//! `cp`/`mv`를 조금 더 안전하게 수행하기 위한 간단한 CLI입니다.
//!
//! 목표
//! - 심볼릭 링크 추적 방지(`O_NOFOLLOW`)
//! - `cp`는 임시 파일에 기록 후 `renameat`으로 원자적 교체
//! - `mv`는 동일 파일시스템이면 `renameat`, 교차 파일시스템이면 copy-then-unlink
//!
//! 제한/주의
//! - 디렉토리 복사는 지원하지 않습니다.
//! - 권한/타임스탬프/소유권 보존은 하지 않습니다(대상 파일 모드는 기본 `0644`).
//! - 저수준 `libc` 호출을 사용하므로, 에러는 OS 에러(`errno`) 기반으로 반환됩니다.
//!
//! 배경(참고)
//! - Ubuntu 26.04 LTS에서는 `cp`/`mv`/`rm`이 여전히 GNU coreutils로 제공됩니다.
//! - rust-coreutils(uutils) 쪽 `cp`/`mv`/`rm`에는 TOCTOU(time-of-check to time-of-use) 성격의
//!   미해결 이슈가 남아 있어(2026-04-22 기준 8개) 기본 제공 대상에서 제외된 것으로 알려져 있습니다.

use std::ffi::CString;
use std::io;
use std::path::Path;

use libc::{
    AT_FDCWD, O_CREAT, O_DIRECTORY, O_EXCL, O_NOFOLLOW, O_RDONLY, O_WRONLY,
    c_int, close, fsync, fstat, openat, read, renameat, stat, unlinkat, write,
};

/// 파일 복사에 사용할 버퍼 크기(바이트).
const BUF_SIZE: usize = 8192;

// ────────────────────────────────────────────────
// 경로 해석(TOCTOU 완화)
// ────────────────────────────────────────────────
/// 부모 디렉토리를 "경로 문자열"이 아니라 "디렉토리 fd"로 고정(anchor)하기 위한 헬퍼입니다.
///
/// 핵심은 "검사(check) 후 사용(use)" 사이에 경로가 바뀌는(TOCTOU) 레이스를 피하는 것입니다.
/// - Linux에서는 `openat2(2)` + `RESOLVE_NO_SYMLINKS`로 경로 해석 단계에서 심볼릭 링크를 차단하고,
///   성공 시 디렉토리 inode에 고정된 fd를 얻습니다.
/// - 그 다음 파일 작업은 반드시 `openat/renameat/unlinkat`처럼 *fd 기준* syscall로 수행합니다.
///
/// 참고: `openat2`는 Linux 커널 5.6+가 필요합니다. (미지원이면 기존 방식으로 폴백)
#[cfg(target_os = "linux")]
fn open_dirfd_no_symlinks(path: &str) -> io::Result<OwnedFd> {
    #[repr(C)]
    struct OpenHow {
        flags: u64,
        mode: u64,
        resolve: u64,
    }

    // linux/openat2.h 의 RESOLVE_* 값들
    //
    // - `RESOLVE_NO_SYMLINKS`: 경로 해석 과정에서 심볼릭 링크를 만나면 실패합니다.
    // - `RESOLVE_NO_MAGICLINKS`: /proc/<pid>/fd/* 같은 "매직 링크"를 거부합니다.
    //
    // 이 조합은 "부모 디렉토리 경로 문자열을 따라가는 동안" 공격자가 심볼릭 링크로 바꿔치기하여
    // 의도하지 않은 디렉토리로 유도하는 TOCTOU 류 공격을 완화합니다.
    const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
    const RESOLVE_NO_SYMLINKS: u64 = 0x04;

    let c_path = cstr(path)?;
    let how = OpenHow {
        flags: (O_RDONLY | O_DIRECTORY | libc::O_CLOEXEC) as u64,
        mode: 0,
        resolve: RESOLVE_NO_SYMLINKS | RESOLVE_NO_MAGICLINKS,
    };

    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2 as libc::c_long,
            AT_FDCWD as libc::c_long,
            c_path.as_ptr() as libc::c_long,
            (&how as *const OpenHow) as libc::c_long,
            std::mem::size_of::<OpenHow>() as libc::c_long,
        )
    };

    if fd < 0 {
        let err = io::Error::last_os_error();
        // 커널이 openat2를 지원하지 않는 경우(ENOSYS)에는 기존 방식으로 폴백합니다.
        //
        // 폴백 경로는 "부모 디렉토리 문자열"을 다시 해석해야 하므로, 이 단계의 심볼릭 링크 레이스를
        // openat2만큼 강하게 막을 수는 없습니다. (그래도 이후 파일 조작은 dirfd 기반으로 수행)
        //
        // 참고: 일부 환경(예: WSL2)에서는 상대 경로(`.` 등) + resolve 플래그 조합에서 `ENOENT`가
        // 발생하는 사례가 있어, "폴백이 성공하면 폴백을 사용"하도록 처리합니다.
        if matches!(
            err.raw_os_error(),
            Some(libc::ENOSYS | libc::EINVAL | libc::EPERM | libc::ENOENT)
        ) {
            if let Ok(fallback) =
                OwnedFd::open_at(AT_FDCWD, path, O_RDONLY | O_DIRECTORY | libc::O_CLOEXEC, 0)
            {
                return Ok(fallback);
            }
        }
        return Err(err);
    }

    Ok(OwnedFd(fd as c_int))
}

#[cfg(not(target_os = "linux"))]
fn open_dirfd_no_symlinks(path: &str) -> io::Result<OwnedFd> {
    OwnedFd::open_at(AT_FDCWD, path, O_RDONLY | O_DIRECTORY, 0)
}

// ────────────────────────────────────────────────
// RAII fd 래퍼: 스코프를 벗어나면 자동 close
// ────────────────────────────────────────────────
/// OS 파일 디스크립터를 소유(ownership)하는 간단한 RAII 래퍼입니다.
///
/// - 생성에 성공하면 `Drop`에서 반드시 `close(2)`됩니다.
/// - 에러 시 `io::Error`로 변환하여 호출자에게 전달합니다.
struct OwnedFd(c_int);

impl OwnedFd {
    /// `openat(2)` 기반으로 `dirfd + name`을 엽니다.
    ///
    /// `name`은 NUL 바이트를 포함할 수 없으며, 실패 시 `io::Error`를 반환합니다.
    fn open_at(dirfd: c_int, name: &str, flags: c_int, mode: u32) -> io::Result<Self> {
        let c = cstr(name)?;
        let fd = unsafe { openat(dirfd, c.as_ptr(), flags, mode as libc::mode_t) };
        if fd < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(fd))
        }
    }

    /// 내부 raw fd를 그대로 반환합니다.
    fn raw(&self) -> c_int { self.0 }

    /// `fstat(2)`를 호출하여 메타데이터를 얻습니다.
    fn fstat(&self) -> io::Result<stat> {
        let mut st: stat = unsafe { std::mem::zeroed() };
        if unsafe { fstat(self.0, &mut st) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(st)
    }
}

impl Drop for OwnedFd {
    /// 소유한 fd를 닫습니다. (닫기 실패는 무시)
    fn drop(&mut self) {
        if self.0 >= 0 {
            unsafe { close(self.0) };
        }
    }
}

// ────────────────────────────────────────────────
// 공통 유틸리티
// ────────────────────────────────────────────────
/// Rust 문자열을 C 문자열(`CString`)로 변환합니다.
///
/// NUL 바이트가 포함되면 `CString::new`가 실패하므로 `io::Error`로 변환합니다.
fn cstr(s: &str) -> io::Result<CString> {
    CString::new(s).map_err(io::Error::other)
}

fn trace_enabled() -> bool {
    std::env::var_os("CP_TRACE").is_some()
}

fn trace(msg: &str) {
    if trace_enabled() {
        eprintln!("[trace] {msg}");
    }
}

fn trace_result<T>(step: &str, res: io::Result<T>) -> io::Result<T> {
    match res {
        Ok(v) => Ok(v),
        Err(e) => {
            if trace_enabled() {
                eprintln!("[trace] {step}: {e}");
            }
            Err(e)
        }
    }
}

/// 경로를 `(부모 디렉토리, 파일명)`으로 분리합니다.
///
/// - `parent`가 없으면 `"."`(현재 디렉토리)로 간주합니다.
/// - 파일명이 UTF-8이 아니면 에러를 반환합니다.
fn split_path(path: &Path) -> io::Result<(String, String)> {
    let parent = match path.parent() {
        None => Path::new("."),
        Some(p) if p.as_os_str().is_empty() => Path::new("."),
        Some(p) => p,
    };
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::other("유효하지 않은 파일명"))?
        .to_string();
    let parent_s = parent
        .to_str()
        .ok_or_else(|| io::Error::other("유효하지 않은 경로"))?
        .to_string();
    Ok((parent_s, name))
}

/// 대상 파일명에 기반한 임시 파일명을 생성합니다.
///
/// `getrandom(2)`로 랜덤 값을 얻어 충돌 확률을 낮춥니다.
fn generate_tmp_name(base: &str) -> io::Result<String> {
    let mut rnd: u64 = 0;
    let ret = unsafe { libc::getrandom(&mut rnd as *mut u64 as *mut libc::c_void, 8, 0) };
    if ret < 0 { return Err(io::Error::last_os_error()); }
    Ok(format!(".tmp.{}.{}.{:016x}", base, std::process::id(), rnd))
}

/// `read(2)`/`write(2)`로 데이터를 스트리밍 복사합니다.
///
/// - `write`는 부분 쓰기가 가능하므로, 요청 길이를 모두 쓸 때까지 반복합니다.
fn copy_data(src_fd: c_int, dst_fd: c_int) -> io::Result<()> {
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let n = unsafe { read(src_fd, buf.as_mut_ptr() as *mut _, BUF_SIZE) };
        if n < 0 { return Err(io::Error::last_os_error()); }
        if n == 0 { break; }

        let mut written: isize = 0;
        while (written as usize) < (n as usize) {
            let w = unsafe {
                write(dst_fd, buf[written as usize..(n as usize)].as_ptr() as *const _, (n - written) as usize)
            };
            if w < 0 { return Err(io::Error::last_os_error()); }
            written += w;
        }
    }
    Ok(())
}

// ────────────────────────────────────────────────
// SAFE CP: 확보된 dirfd 기반 원자적 복사
// ────────────────────────────────────────────────
/// `src_dirfd/src_name`을 `dst_dirfd/dst_name`으로 안전하게 복사합니다.
///
/// 동작 개요
/// 1) 원본을 `O_NOFOLLOW`로 열어 심볼릭 링크 추적을 방지합니다.
/// 2) 대상 디렉토리에 임시 파일(`O_EXCL`)을 생성해 내용을 기록합니다.
/// 3) `fsync` 후 임시 파일을 `renameat`으로 최종 이름으로 원자적 교체합니다.
///
/// 참고
/// - 디렉토리는 복사하지 않습니다.
/// - 대상 파일 모드는 고정(`0644`)입니다.
fn safe_cp_at(src_dirfd: c_int, src_name: &str, dst_dirfd: c_int, dst_name: &str) -> io::Result<()> {
    // 원본 열기: `O_NOFOLLOW`로 심볼릭 링크를 따라가지 않습니다.
    // - 경로 기반 stat 후 open 같은 "check-then-open" 패턴을 피하고, open 자체에서 정책을 강제합니다.
    trace(&format!("safe_cp_at src_dirfd={src_dirfd} src_name={src_name} dst_dirfd={dst_dirfd} dst_name={dst_name}"));
    let src_fd = trace_result(
        "open src file",
        OwnedFd::open_at(src_dirfd, src_name, O_RDONLY | O_NOFOLLOW, 0),
    )?;
    let st = src_fd.fstat()?;
    if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
        return Err(io::Error::other("디렉토리 복사는 지원하지 않습니다."));
    }

    let tmp_name = generate_tmp_name(dst_name)?;
    trace(&format!("tmp_name={tmp_name}"));
    // 임시 파일 생성:
    // - `O_EXCL`로 "같은 이름이 이미 존재"하면 실패 → 타 프로세스가 같은 tmp_name을 선점하는 것을 방지.
    // - `O_NOFOLLOW`로 심볼릭 링크를 따라가지 않음(임시 엔트리를 symlink로 바꿔치기하는 공격 완화).
    let dst_fd = trace_result(
        "create tmp file",
        OwnedFd::open_at(dst_dirfd, &tmp_name, O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW, 0o644),
    )?;

    // 데이터 복사 시도
    if let Err(e) = copy_data(src_fd.raw(), dst_fd.raw()) {
        // 복사 도중 실패하면 임시 파일을 정리하고 원본 에러를 반환합니다.
        let _ = unsafe { unlinkat(dst_dirfd, cstr(&tmp_name).unwrap().as_ptr(), 0) };
        return Err(e);
    }

    // 디스크 동기화 및 원자적 교체
    // 주의: 이 `fsync`는 "임시 파일의 데이터"를 디스크에 내리는 것입니다.
    // 전원 장애까지 고려한 내구성(durability)을 원하면, `renameat` 이후에
    // 대상 디렉토리 fd에 대한 `fsync`도 추가로 필요할 수 있습니다.
    if unsafe { fsync(dst_fd.raw()) } != 0 {
        // 동기화 실패 역시 부분 파일이 남지 않도록 정리합니다.
        let _ = unsafe { unlinkat(dst_dirfd, cstr(&tmp_name).unwrap().as_ptr(), 0) };
        return Err(io::Error::last_os_error());
    }

    // 가능한 한 빨리 원자적 교체를 수행합니다.
    // fd를 닫을 필요는 없으며(닫아도 의미상 문제는 없지만), 닫는 동안 스케줄링으로 인해
    // 임시 경로가 외부에 노출되는 시간을 불필요하게 늘리지 않도록 즉시 rename합니다.
    let c_tmp = cstr(&tmp_name)?;
    let c_final = cstr(dst_name)?;
    if unsafe { renameat(dst_dirfd, c_tmp.as_ptr(), dst_dirfd, c_final.as_ptr()) } != 0 {
        // 최종 교체 실패 시에도 임시 파일을 정리합니다.
        let _ = unsafe { unlinkat(dst_dirfd, c_tmp.as_ptr(), 0) };
        return Err(io::Error::last_os_error());
    }

    drop(dst_fd);
    Ok(())
}

// ────────────────────────────────────────────────
// SAFE MV: 동일/교차 파일 시스템 자동 판별
// ────────────────────────────────────────────────
/// `src`를 `dst`로 안전하게 이동합니다.
///
/// - 동일 파일시스템이면 `renameat`을 사용해 원자적으로 이름을 바꿉니다.
/// - 교차 파일시스템(장치 번호 `st_dev`가 다름)이면 `copy` 후 `unlink`합니다.
///
/// 주의
/// - 교차 FS 이동은 원자적이지 않습니다(복사 후 삭제).
pub fn safe_mv(src: &Path, dst: &Path) -> io::Result<()> {
    let (src_parent, src_n) = split_path(src)?;
    let (dst_parent, dst_n) = split_path(dst)?;

    let src_dirfd = open_dirfd_no_symlinks(&src_parent)?;
    let dst_dirfd = open_dirfd_no_symlinks(&dst_parent)?;

    // 장치 번호(st_dev) 비교를 위해 fstat 호출
    let src_target_fd = OwnedFd::open_at(src_dirfd.raw(), &src_n, O_RDONLY | O_NOFOLLOW, 0)?;
    let st_src = src_target_fd.fstat()?;
    drop(src_target_fd); // rename/unlink를 위해 닫음
    let st_dst_dir = dst_dirfd.fstat()?;

    if st_src.st_dev == st_dst_dir.st_dev {
        // 1. 동일 FS: 원자적 이름 변경
        if unsafe { renameat(src_dirfd.raw(), cstr(&src_n)?.as_ptr(), dst_dirfd.raw(), cstr(&dst_n)?.as_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
    } else {
        // 2. 교차 FS: Copy-then-Unlink
        safe_cp_at(src_dirfd.raw(), &src_n, dst_dirfd.raw(), &dst_n)?;
        if unsafe { unlinkat(src_dirfd.raw(), cstr(&src_n)?.as_ptr(), 0) } != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn main() {
    // 간단한 CLI:
    //   cp cp <src> <dst>
    //   cp mv <src> <dst>
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cp [cp|mv] <src> <dst>");
        std::process::exit(1);
    }

    let cmd = &args[1];
    let src = Path::new(&args[2]);
    let dst = Path::new(&args[3]);

    let res = match cmd.as_str() {
        "cp" => (|| -> io::Result<()> {
            // dirfd를 먼저 확보한 뒤, 파일명만 `openat/renameat/unlinkat`에 전달합니다.
            // (Linux에서는 openat2로 parent 경로의 심볼릭 링크를 차단하여 TOCTOU를 더 줄입니다.)
            let (s_parent, s_n) = split_path(src)?;
            let (d_parent, d_n) = split_path(dst)?;
            trace(&format!("cp src_parent={s_parent} src_name={s_n} dst_parent={d_parent} dst_name={d_n}"));
            let s_dfd = trace_result("open src parent dir", open_dirfd_no_symlinks(&s_parent))?;
            let d_dfd = trace_result("open dst parent dir", open_dirfd_no_symlinks(&d_parent))?;
            safe_cp_at(s_dfd.raw(), &s_n, d_dfd.raw(), &d_n)
        })(),
        "mv" => safe_mv(src, dst),
        _ => { eprintln!("알 수 없는 명령"); std::process::exit(1); }
    };

    if let Err(e) = res {
        eprintln!("실행 오류: {e}");
        std::process::exit(1);
    } else {
        println!("성공적으로 완료되었습니다.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_bare_filename_uses_dot_parent() {
        let (parent, name) = split_path(Path::new("test.sh")).unwrap();
        assert_eq!(parent, ".");
        assert_eq!(name, "test.sh");
    }

    #[test]
    fn split_path_nested_path_splits_parent_and_name() {
        let (parent, name) = split_path(Path::new("a/b.txt")).unwrap();
        assert_eq!(parent, "a");
        assert_eq!(name, "b.txt");
    }
}
