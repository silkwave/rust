# Repository Guidelines

## 프로젝트 구조 및 모듈 구성
- `src/main.rs`에 `map!` 매크로, `Value` enum, 데모용 `run()` 로직이 있습니다.
- `Cargo.toml`은 단일 Rust 바이너리 크레이트를 정의합니다(`edition = "2024"`).
- `Cargo.lock`은 재현 가능한 빌드를 위해 유지합니다.
- `target/`는 빌드 산출물이며 리뷰 대상에서 제외합니다.

## 빌드/테스트/개발 명령
- `cargo build`: 디버그 빌드 생성
- `cargo run`: 빌드 후 데모 실행
- `cargo test`: 테스트 실행(현재는 테스트 없음)
- `cargo build --release`: 최적화 바이너리 생성(`target/release/`)

## 코딩 스타일 및 네이밍
- `rustfmt` 기본 규칙을 따릅니다. 변경 전 `cargo fmt` 실행.
- 함수/변수는 `snake_case`, 타입/enum은 `CamelCase` 사용.
- 매크로는 작고 명확하게 유지하고 간단한 예시 주석을 포함합니다.
- 주석은 의도와 이유 위주로 간결하게 작성합니다.

## 테스트 가이드
- 현재 테스트 모듈이 없습니다. 테스트를 추가할 때는 다음을 권장합니다.
- 단위 테스트: `src/main.rs` 하단에 `#[cfg(test)]` 모듈을 추가.
- 통합 테스트: 새 파일을 `tests/` 디렉터리에 추가(예: `tests/map_macro.rs`).
- 테스트 이름은 동작을 드러내는 형태로 작성(예: `test_map_macro_inserts_values`).
- 변경 후 로컬에서 `cargo test`를 실행하고, 필요 시 `cargo fmt`와 `cargo clippy`도 함께 실행합니다.

## 커밋 및 PR 가이드
- 최근 커밋 메시지는 짧은 라벨(예: `Messenger`)과 Conventional Commit 형식이 혼재합니다.
- 앞으로는 가능하면 Conventional Commit을 사용합니다. 예: `feat: add value display formatting`, `fix: handle empty map`.
- PR에는 요약, 수행한 테스트(`cargo test`), 동작 변경 시 출력 변화 요약을 포함합니다.

## 에이전트 안내
- 이 저장소에서 작업할 때는 `AGENTS.md` 지침을 따릅니다.
