# AGENTS.md - 에이전트 코딩 가이드라인

이 문서는 이 저장소에서 작업하는 AI 에이전트를 위한 가이드라인입니다.

### 답변 
한글로 답변 합니다. 

### 주석 
한글주석. 

## 프로젝트 개요

- **프로젝트 이름**: oracleTest
- **언어**: Rust
- **에디션**: 2024 (nightly/unstable)
- **핵심 스택**: Axum (HTTP), Tokio (async), Oracle DB, Tracing (로깅)

## 빌드, 린트 및 테스트 명령어

### 표준 명령어
```bash
cargo build              # 빌드
cargo build --release    # 릴리스
cargo run                # 서버 실행
```

### 린트 & 포맷팅
```bash
cargo fmt                # 포맷
cargo fmt -- --check    # 포맷 확인
cargo clippy             # 린트
cargo clippy --fix      # 린트 자동 수정
```

### 테스트
```bash
cargo test               # 테스트 실행
cargo test name_here    # 단일 테스트
cargo test -- --nocapture  # 출력 포함
```

## HTTP API 라우트

| 메서드 | 엔드포인트 | 핸들러 |
|--------|------------|---------|
| GET | /boards | list_boards |
| POST | /boards | create_board |
| GET | /boards/{id} | get_board |
| PUT | /boards/{id} | update_board |
| DELETE | /boards/{id} | delete_board |

## 코딩 스타일 가이드라인

###命名
- **변수/함수**: `snake_case`
- **타입/열거형**: `PascalCase`
- **상수**: `SCREAMING_SNAKE_CASE`
- **파일**: `snake_case.rs`

### 가져오기(Imports)
- 그룹: std → external → internal
- 내부 모듈은 `crate::` 사용

### 모듈 구성
```
src/
├── main.rs         # Axum 서버 진입점
├── config/mod.rs   # 환경변수 설정
├── model/mod.rs    # Board, DbPool
├── repository/     # CRUD 작업
├── service/        # 비즈니스 로직
├── controller/     # 요청 핸들러
├── queries.rs      # SQL 상수
└── sql/            # *.sql 파일
```

### 오류 처리
- `Result<T, E>`와 `?` 사용
- 예시: `fn main() -> Result<(), Box<dyn std::error::Error>>`

## 중요 참고사항

1. **설정**: `dotenv`를 통해 `.env` 사용. `.env.example` 참조
2. **DB**: Oracle `127.0.0.1:1521/ORCL` (개발 전용)
3. **테스트**: 프로젝트에 테스트 없음 (0개)
4. **SQL**: `src/sql/*.sql`, `include_str!()`로 로드
