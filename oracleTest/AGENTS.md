# 프로젝트 지식 베이스

**생성일:** 2026-02-13
**최종 업데이트:** 2026-02-14

## 목차
1.  [개요](#1-개요)
2.  [아키텍처](#2-아키텍처)
3.  [프로젝트 구조](#3-프로젝트-구조)
4.  [코드 맵](#4-코드-맵)
5.  [주요 컨벤션](#5-주요-컨벤션)
6.  [금기 사항 (Anti-Patterns)](#6-금기-사항-anti-patterns)
7.  [개발 명령어](#7-개발-명령어)
8.  [참고 사항](#8-참고-사항)
9.  [API 테스트 (cURL 명령어)](#9-api-테스트-curl-명령어)

## 1. 개요

이 프로젝트는 **Rust Axum 웹 프레임워크**와 **Oracle 데이터베이스**를 활용하여 개발된 게시판 REST API 서버입니다. CRUD(Create, Read, Update, Delete) 기능을 제공하며, Spring MVC 패턴을 적용한 명확한 계층형 아키텍처를 통해 유지보수성과 확장성을 고려하여 설계되었습니다.

## 2. 아키텍처

프로젝트는 Spring MVC 패턴을 따릅니다.
- **Controller (Presentation Layer):** HTTP 요청을 처리하고 응답을 반환합니다. 서비스 계층을 호출합니다.
- **Service (Business Logic Layer):** 핵심 비즈니스 로직을 구현하고 데이터를 검증합니다. 리포지토리 계층을 호출합니다.
- **Repository (Data Access Layer):** 데이터베이스와의 상호작용을 담당하며 CRUD 작업을 수행합니다.
- **Model (Data Model):** 데이터 구조를 정의합니다.

## 3. 프로젝트 구조

```
./
├── src/
│   ├── main.rs                    # 서버 진입점, Axum 라우트 정의
│   ├── config/mod.rs              # .env 파일 기반 환경 설정
│   ├── common/
│   │   ├── mod.rs                 # 공통 모듈
│   │   ├── app_state.rs          # 애플리케이션 상태 (Service 공유)
│   │   └── queries.rs             # SQL 쿼리 로드 (include_str!)
│   ├── models/
│   │   ├── mod.rs                 # 모델 모듈
│   │   └── board.rs              # Board 구조체, DB 연결 관리
│   ├── repositories/
│   │   ├── mod.rs                 # 리포지토리 모듈
│   │   └── board_repository.rs    # Oracle CRUD 작업
│   ├── services/
│   │   ├── mod.rs                 # 서비스 모듈
│   │   └── board_service.rs       # 비즈니스 로직, 유효성 검증
│   ├── controllers/
│   │   ├── mod.rs                 # 컨트롤러 모듈
│   │   └── board_controller.rs   # HTTP 요청 핸들러
│   ├── routes/
│   │   └── mod.rs                 # 라우트 설정
│   ├── middleware/
│   │   └── logging.rs             # 로깅 미들웨어
│   └── sql/
│       ├── select_board.sql       # 게시글 조회 (커서 기반 페이징)
│       ├── insert_board.sql       # 게시글 생성
│       ├── update_board.sql       # 게시글 수정
│       └── delete_board.sql       # 게시글 삭제
├── static/
│   └── index.html                 # Vanilla JS 프론트엔드 (커서 페이징 지원)
├── Cargo.toml
└── .env
```

## 4. 코드 맵

| 심볼 | 유형 | 위치 | 역할 | 설명 |
|------|------|------|------|------|
| `AppState` | `Struct` | `common/app_state.rs` | 애플리케이션 상태 | Service를 공유하기 위한 상태 |
| `list_boards` | `Fn` | `controllers/board_controller.rs` | GET `/boards` | 커서 기반 페이징 조회 |
| `get_board` | `Fn` | `controllers/board_controller.rs` | GET `/boards/:id` | 특정 ID 조회 |
| `create_board` | `Fn` | `controllers/board_controller.rs` | POST `/boards` | 게시글 생성 |
| `update_board` | `Fn` | `controllers/board_controller.rs` | PUT `/boards/:id` | 게시글 수정 |
| `delete_board` | `Fn` | `controllers/board_controller.rs` | DELETE `/boards/:id` | 게시글 삭제 |
| `BoardService` | `Struct` | `services/board_service.rs` | 비즈니스 로직 | 게시글 관련 비즈니스 규칙 및 데이터 검증 |
| `ServiceError` | `Enum` | `services/board_service.rs` | 오류 유형 | 서비스 계층에서 발생할 수 있는 오류 |
| `BoardRepository` | `Struct` | `repositories/board_repository.rs` | 데이터베이스 작업 | Oracle DB와 직접 통신하여 CRUD 작업 수행 |
| `Board` | `Struct` | `models/board.rs` | 데이터 모델 | 게시글 데이터 구조체 |

## 5. 주요 컨벤션

*   **언어**: 모든 답변 및 코드 주석은 **한국어**로 작성합니다.
*   **명명 규칙**:
    *   변수 및 함수: `snake_case` (예: `my_variable`, `calculate_sum`)
    *   타입 (구조체, 열거형 등): `PascalCase` (예: `MyStruct`, `ServiceError`)
*   **로깅**:
    *   `[계층명] 메시지` 형식을 사용합니다. (예: `[Controller]`, `[Service]`, `[Repository]`)
    *   `info!`, `error!` 등의 매크로를 사용하여 적절한 로깅 레벨을 지정합니다.

## 6. 금기 사항 (Anti-Patterns)

*   **타입 무시/강제 변환**: Rust의 강력한 타입 시스템을 우회하는 행위 (예: `as any`, `@ts-ignore`)
*   **오류 무시**: `Result`나 `Option` 타입을 반환하는 함수의 오류를 제대로 처리하지 않고 무시하는 행위
*   **빈 예외 처리**: 오류가 발생했을 때 아무런 동작도 하지 않는 빈 오류 처리 블록
*   **테스트 부재**: 핵심 로직 및 기능에 대한 단위/통합 테스트 코드 작성 부재

## 7. 개발 명령어

```bash
cargo build             # 프로젝트 빌드 (디버그 모드)
cargo build --release  # 프로젝트 릴리스 모드로 빌드 (최적화)
cargo run              # 빌드 후 서버 실행 (Ctrl+C로 종료 가능)
cargo fmt              # Rust 코드 포맷팅
cargo clippy           # Rust 코드 린트 검사
cargo test             # 테스트 코드 실행
```

## 8. 참고 사항

*   **환경 설정**: 애플리케이션 실행 전에 `.env` 파일을 생성해야 합니다. (`dotenv` 크레이트 사용)
*   **Oracle DB 연결**: `.env` 파일에 연결 정보 설정 (예: `DB_USER`, `DB_PASSWORD`, `DB_CONNECT`)
    *   기본: `127.0.0.1:1521/ORCL`
*   **정적 파일 서빙**: `/` 또는 `/index.html` 경로로 접근하면 `static/index.html` 파일이 서빙됩니다.
*   **커서 기반 페이징**: Oracle의 `OFFSET-FETCH` 대신 `WHERE id < :last_id` 방식으로 커서 페이징을 구현하여 대규모 데이터에서 성능을 최적화합니다.
*   **연결 관리**: 단일 연결 + Mutex 방식 사용 (단순하고 효과적인 동시성 제어)

## 9. API 테스트 (cURL 명령어)

게시판 API의 주요 CRUD 작업을 테스트하기 위한 `curl` 명령어 예시입니다. `localhost:8080`을 기본 URL로 사용하며 JSON 형식의 데이터를 주고받습니다.

### 1. 게시글 목록 조회 - 커서 기반 페이징 (GET /boards)

첫 페이지 조회:
```bash
curl -v "http://localhost:8080/boards?size=10"
```

응답 형식:
```json
{
  "data": [
    { "id": 10, "title": "...", "content": "...", "created_at": "..." }
  ],
  "pagination": {
    "last_id": null,
    "next_cursor": 5,
    "size": 10,
    "has_more": true
  }
}
```

다음 페이지 조회 (마지막 ID 사용):
```bash
curl -v "http://localhost:8080/boards?last_id=5&size=10"
```

### 2. 특정 게시글 조회 (GET /boards/{id})

`{id}` 부분에 조회하고 싶은 게시글의 실제 ID를 입력하세요.

```bash
curl -v http://localhost:8080/boards/29
```

### 3. 게시글 생성 (POST /boards)

`--data` 옵션에 JSON 형식으로 제목(`title`)과 내용(`content`)을 입력합니다.

```bash
curl -v -X POST -H "Content-Type: application/json" -d '{
    "title": "새로운 게시글 제목",
    "content": "이것은 새로 작성된 게시글의 내용입니다."
}' http://localhost:8080/boards
```

### 4. 게시글 수정 (PUT /boards/{id})

`{id}` 부분에 수정하고 싶은 게시글의 실제 ID를 입력하고, `--data` 옵션에 JSON 형식으로 수정할 제목(`title`)과 내용(`content`)을 입력합니다.

```bash
curl -v -X PUT -H "Content-Type: application/json" -d '{
    "title": "수정된 게시글 제목",
    "content": "이것은 수정된 게시글의 새로운 내용입니다."
}' http://localhost:8080/boards/29
```

### 5. 게시글 삭제 (DELETE /boards/{id})

`{id}` 부분에 삭제하고 싶은 게시글의 실제 ID를 입력하세요.

```bash
curl -v -X DELETE http://localhost:8080/boards/29
```
