# 프로젝트 지식 베이스

**생성일:** 2026-02-13
**커밋:** a88f8f3

## 개요

Rust Axum 기반 Oracle DB 게시판 REST API

## 구조

```
./
├── src/
│   ├── main.rs           # 서버 진입점, 라우트 정의
│   ├── config/mod.rs     # .env 설정 로드
│   ├── model/mod.rs     # Board, DbPool 구조체
│   ├── repository/mod.rs # DB CRUD operations
│   ├── service/mod.rs   # 비즈니스 로직, 검증
│   ├── controller/mod.rs # 요청 핸들러
│   └── queries.rs       # SQL 상수
├── static/index.html     # Vue 없는 바닐라 JS 프론트엔드
├── Cargo.toml
└── .env
```

## 코드 맵

| 심볼 | 유형 | 위치 | 역할 |
|------|------|------|------|
| AppState | Struct | main.rs:26 | controller 상태 보관 |
| list_boards | Fn | main.rs:51 | GET /boards |
| get_board | Fn | main.rs:68 | GET /boards/{id} |
| create_board | Fn | main.rs:87 | POST /boards |
| update_board | Fn | main.rs:107 | PUT /boards/{id} |
| delete_board | Fn | main.rs:123 | DELETE /boards/{id} |
| BoardService | Struct | service/mod.rs:8 | 비즈니스 로직 |
| ServiceError | Enum | service/mod.rs:12 | 오류 유형 |
| BoardRepository | Struct | repository/mod.rs:8 | DB 작업 |

## 컨벤션

- **답변:** 한글
- **주석:** 한글
- **명명:** snake_case (변수/함수), PascalCase (타입)
- **로깅:** `[Layer] 메시지` 형식 (예: `[Controller]`, `[Service]`)

## 금기 사항

- 타입 오류 감싸기 (`as any`, `@ts-ignore`) - Rust에서는 불가
- 빈 catch 블록
- 테스트 없음

## 명령어

```bash
cargo build              # 빌드
cargo build --release    # 릴리스
cargo run                # 서버 실행 (Ctrl+C 지원)
cargo fmt                # 포맷
cargo clippy             # 린트
```

## 참고

- `.env` 파일 필요 (dotenv)
- Oracle DB: `127.0.0.1:1521/ORCL`
- 정적 파일: `/`, `/index.html` → `static/index.html`
