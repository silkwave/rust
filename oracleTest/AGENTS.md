# 프로젝트 지식 베이스

**생성일:** 2026-02-13
**커밋:** a88f8f3

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

이 프로젝트는 **Rust Axum 웹 프레임워크**와 **Oracle 데이터베이스**를 활용하여 개발된 게시판 REST API 서버입니다. CRUD(Create, Read, Update, Delete) 기능을 제공하며, 명확한 계층형 아키텍처를 통해 유지보수성과 확장성을 고려하여 설계되었습니다.

## 2. 아키텍처

프로젝트는 전형적인 계층형 아키텍처를 따릅니다.
- **Controller (프레젠테이션 계층):** HTTP 요청을 처리하고 응답을 반환합니다. 서비스 계층을 호출합니다.
- **Service (비즈니스 로직 계층):** 핵심 비즈니스 로직을 구현하고 데이터를 검증합니다. 리포지토리 계층을 호출합니다.
- **Repository (데이터 접근 계층):** 데이터베이스와의 상호작용을 담당하며 CRUD 작업을 수행합니다.
- **Model (데이터 모델):** 데이터 구조를 정의합니다.

## 3. 프로젝트 구조

```
./
├── src/
│   ├── main.rs                 # 서버 진입점, Axum 라우트 정의, 애플리케이션 초기화
│   ├── config/mod.rs           # `.env` 파일 기반 환경 설정 로드 및 관리
│   ├── model/mod.rs            # 데이터 모델 (예: Board, DbPool) 정의
│   ├── repository/mod.rs       # 데이터베이스 CRUD 작업 (BoardRepository)
│   ├── service/mod.rs          # 비즈니스 로직 및 유효성 검증 (BoardService)
│   ├── controller/mod.rs       # HTTP 요청 핸들러 (BoardController)
│   └── queries.rs              # SQL 쿼리 상수 정의
├── static/
│   └── index.html              # 간단한 바닐라 JS 기반 프론트엔드 (정적 파일 서빙)
├── Cargo.toml                  # Rust 프로젝트 의존성 및 메타데이터 관리
└── .env                        # 환경 변수 정의 파일 (예: DB 접속 정보, 로깅 레벨)
```

## 4. 코드 맵

| 심볼 | 유형 | 위치 | 역할 | 설명 |
|------|------|------|------|------|
| `AppState` | `Struct` | `main.rs:26` | 애플리케이션 상태 | Axum 핸들러 간 공유될 애플리케이션 상태를 보관 (주로 Controller 인스턴스) |
| `list_boards` | `Fn` | `main.rs:51` | GET `/boards` 핸들러 | 모든 게시글 목록을 조회하여 반환 |
| `get_board` | `Fn` | `main.rs:68` | GET `/boards/{id}` 핸들러 | 특정 ID의 게시글을 조회하여 반환 |
| `create_board` | `Fn` | `main.rs:87` | POST `/boards` 핸들러 | 새로운 게시글을 생성 |
| `update_board` | `Fn` | `main.rs:107` | PUT `/boards/{id}` 핸들러 | 특정 ID의 게시글을 수정 |
| `delete_board` | `Fn` | `main.rs:123` | DELETE `/boards/{id}` 핸들러 | 특정 ID의 게시글을 삭제 |
| `BoardService` | `Struct` | `service/mod.rs:8` | 비즈니스 로직 | 게시글 관련 비즈니스 규칙 및 데이터 검증 처리 |
| `ServiceError` | `Enum` | `service/mod.rs:12` | 오류 유형 | 서비스 계층에서 발생할 수 있는 오류를 정의 |
| `BoardRepository` | `Struct` | `repository/mod.rs:8` | 데이터베이스 작업 | Oracle DB와 직접 통신하여 데이터 CRUD 작업 수행 |

## 5. 주요 컨벤션

*   **언어**: 모든 답변 및 코드 주석은 **한국어**로 작성합니다.
*   **명명 규칙**:
    *   변수 및 함수: `snake_case` (예: `my_variable`, `calculate_sum`)
    *   타입 (구조체, 열거형 등): `PascalCase` (예: `MyStruct`, `ServiceError`)
*   **로깅**:
    *   `[계층명] 메시지` 형식을 사용합니다. (예: `[Controller]`, `[Service]`, `[Repository]`)
    *   `info!`, `error!` 등의 매크로를 사용하여 적절한 로깅 레벨을 지정합니다.

## 6. 금기 사항 (Anti-Patterns)

*   **타입 무시/강제 변환**: Rust의 강력한 타입 시스템을 우회하는 행위 (예: `unsafe` 코드의 불필요한 사용)
*   **오류 무시**: `Result`나 `Option` 타입을 반환하는 함수의 오류를 제대로 처리하지 않고 무시하는 행위
*   **빈 예외 처리**: 오류가 발생했을 때 아무런 동작도 하지 않는 빈 오류 처리 블록
*   **테스트 부재**: 핵심 로직 및 기능에 대한 단위/통합 테스트 코드 작성 부재

## 7. 개발 명령어

```bash
cargo build             # 프로젝트 빌드 (디버그 모드)
cargo build --release   # 프로젝트 릴리스 모드로 빌드 (최적화)
cargo run               # 빌드 후 서버 실행 (Ctrl+C로 종료 가능)
cargo fmt               # Rust 코드 포맷팅 (코드 스타일 일관성 유지)
cargo clippy            # Rust 코드 린트 검사 (잠재적 버그 및 스타일 위반 검출)
cargo test              # 테스트 코드 실행
```

## 8. 참고 사항

*   **환경 설정**: 애플리케이션 실행 전에 프로젝트 루트에 `.env` 파일을 생성하고 필요한 환경 변수를 설정해야 합니다. (`dotenv` 크레이트 사용)
*   **Oracle DB 연결**: Oracle 데이터베이스에 대한 연결 정보는 `.env` 파일에 설정합니다. (예: `DB_USER`, `DB_PASSWORD`, `DB_CONNECT`)
    *   기본 Oracle DB 연결 정보: `127.0.0.1:1521/ORCL`
*   **정적 파일 서빙**: `/` 또는 `/index.html` 경로로 접근하면 `static/index.html` 파일이 서빙됩니다.

## 9. API 테스트 (cURL 명령어)

게시판 API의 주요 CRUD 작업을 테스트하기 위한 `curl` 명령어 예시입니다. `localhost:8080`을 기본 URL로 사용하며 JSON 형식의 데이터를 주고받습니다.

### 1. 게시글 전체 조회 (GET /boards)

```bash
curl -v http://localhost:8080/boards
```

*   설명: 서버에 저장된 모든 게시글 목록을 JSON 형식으로 조회합니다.

### 2. 특정 게시글 조회 (GET /boards/{id})

`{id}` 부분에 조회하고 싶은 게시글의 실제 ID를 입력하세요.

```bash
curl -v http://localhost:8080/boards/29
```

*   설명: 특정 ID를 가진 게시글의 상세 정보를 JSON 형식으로 조회합니다.

### 3. 게시글 생성 (POST /boards)

`--data` 옵션에 JSON 형식으로 제목(`title`)과 내용(`content`)을 입력합니다.

```bash
curl -v -X POST -H "Content-Type: application/json" -d '{
    "title": "새로운 게시글 제목",
    "content": "이것은 새로 작성된 게시글의 내용입니다."
}' http://localhost:8080/boards
```

*   설명: 새로운 게시글을 생성합니다. 성공하면 생성된 게시글의 ID와 함께 게시글 정보가 반환됩니다.

### 4. 게시글 수정 (PUT /boards/{id})

`{id}` 부분에 수정하고 싶은 게시글의 실제 ID를 입력하고, `--data` 옵션에 JSON 형식으로 수정할 제목(`title`)과 내용(`content`)을 입력합니다.

```bash
curl -v -X PUT -H "Content-Type: application/json" -d '{
    "title": "수정된 게시글 제목",
    "content": "이것은 수정된 게시글의 새로운 내용입니다."
}' http://localhost:8080/boards/29
```

*   설명: 특정 ID를 가진 게시글의 제목과 내용을 수정합니다.

### 5. 게시글 삭제 (DELETE /boards/{id})

`{id}` 부분에 삭제하고 싶은 게시글의 실제 ID를 입력하세요.

```bash
curl -v -X DELETE http://localhost:8080/boards/29
```

*   설명: 특정 ID를 가진 게시글을 삭제합니다.
