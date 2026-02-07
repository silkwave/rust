// Rust로 구현한 ls 명령어
// 필요한 라이브러리 임포트
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// Unix 시스템에서 파일 권한 정보를 얻기 위한 임포트
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

// 시간을 YYYY-MM-DD HH:MM 형식으로 변환하는 함수
fn format_time(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            // chrono 의존성 없이 간단한 날짜 포맷팅
            let days_since_epoch = secs / 86400; // 1970년 1월 1일부터의 날짜 수
            let year = 1970 + (days_since_epoch / 365) as u32; // 연도 계산
            let day_of_year = (days_since_epoch % 365) as u32; // 그 해의 날짜 수
            let month = (day_of_year / 30) + 1; // 월 계산 (단순화)
            let day = (day_of_year % 30) + 1; // 일 계산 (단순화)
            format!("{}-{:02}-{:02} 12:00", year, month, day)
        }
        Err(_) => "-".to_string(), // 시간을 가져올 수 없을 때
    }
}

// 긴 형식(-l 옵션)으로 파일 정보 출력하는 함수
fn print_long(_path: &Path, meta: &fs::Metadata, name: &str) -> io::Result<()> {
    // Unix 시스템에서 파일 권한 가져오기
    #[cfg(unix)]
    let perm = meta.permissions().mode() & 0o777; // 8진수 권한
    #[cfg(not(unix))]
    let perm = 0; // 비-Unix 시스템에서는 0으로 설정

    let size = meta.len(); // 파일 크기 (바이트)
    let modified = meta.modified().ok().map(format_time).unwrap_or("-".into()); // 수정 시간

    // 형식: 권한  크기  수정시간  파일명
    println!("{:o}\t{:>8}\t{}\t{}", perm, size, modified, name);
    Ok(())
}

// 디렉토리 내용을 읽어서 출력하는 메인 함수
fn list_dir(path: &Path, show_all: bool, long: bool) -> io::Result<()> {
    let entries = fs::read_dir(path)?; // 디렉토리 읽기

    // 각 파일/디렉토리 항목 처리
    for entry in entries {
        let entry = entry?; // 에러 처리
        let path = entry.path(); // 전체 경로
        let name = entry.file_name(); // 파일명
        let name = name.to_string_lossy(); // UTF-8로 변환

        // -a 옵션이 없고 숨김 파일(.)이면 건너뛰기
        if !show_all && name.starts_with('.') {
            continue;
        }

        // -l 옵션에 따라 출력 형식 결정
        if long {
            let meta = entry.metadata()?; // 파일 메타데이터
            print_long(&path, &meta, &name)?; // 긴 형식으로 출력
        } else {
            println!("{}", name); // 파일명만 출력
        }
    }
    Ok(())
}

// 프로그램 진입점
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); // 커맨드 라인 인수

    // 옵션 변수 초기화
    let mut show_all = false; // -a 옵션 (숨김 파일 표시)
    let mut long = false; // -l 옵션 (긴 형식)
    let mut target = "."; // 기본 대상 디렉토리 (현재 디렉토리)

    // 커맨드 라인 인수 분석
    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => show_all = true, // 숨김 파일 표시
            "-l" => long = true,     // 긴 형식
            "-al" | "-la" => {
                // 두 옵션 모두
                show_all = true;
                long = true;
            }
            _ => target = arg, // 디렉토리 경로
        }
    }

    // 디렉토리 목록 출력 실행
    list_dir(Path::new(target), show_all, long)
}
