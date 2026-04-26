# `cp` (Rust 실험 구현)

이 프로젝트는 저수준 `libc` syscall을 사용해서 파일 복사/이동을 **dirfd 기준**으로 수행하는 실험용 CLI입니다.

현재 구현은 GNU coreutils의 `cp`/`mv`와 동작/옵션 호환을 목표로 하지 않습니다.

## 제공 기능

- `cp`: 단일 파일 복사(임시 파일 작성 → `fsync` → `renameat`으로 원자적 교체)
- `mv`: 동일 파일시스템이면 `renameat`, 교차 파일시스템이면 copy-then-unlink
- 심볼릭 링크 레이스 완화
  - 파일 열기에서 `O_NOFOLLOW` 사용
  - (Linux) 부모 디렉토리 fd 확보 시 `openat2(RESOLVE_NO_SYMLINKS|RESOLVE_NO_MAGICLINKS)` 사용(미지원 커널은 폴백)

## 제한 사항

- 디렉토리 복사 미지원
- 권한/타임스탬프/소유권 보존 미지원(대상 파일 모드 기본 `0644`)
- 옵션 파서 미구현(`-r`, `-f`, `-p` 등 없음)
- 내구성(durability)까지 완벽히 보장하려면(전원 장애 등) `renameat` 이후 디렉토리 `fsync`가 추가로 필요할 수 있음

## 사용법

바이너리 이름은 Cargo 설정상 `cp`이며, 서브커맨드로 `cp|mv`를 받습니다.

```bash
# 파일 복사
cargo run -- cp <src> <dst>

# 파일 이동
cargo run -- mv <src> <dst>
```

릴리즈 빌드:

```bash
cargo build --release
./target/release/cp cp <src> <dst>
./target/release/cp mv <src> <dst>
```
