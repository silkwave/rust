# 🛒 Rust 전자상거래 시스템 - 에이전트 개발 가이드

## 🚀 빌드/테스트/린트 명령어

### 기본 개발 명령어
```bash
# 개발 서버 실행
cargo run

# 프로덕션 빌드 및 실행
cargo build --release
./target/release/rust-ecommerce

# 의존성 확인
cargo check
```

### 코드 품질 검증
```bash
# 린트 체크
cargo clippy

# 코드 포맷팅
cargo fmt
cargo fmt --check  # 포맷팅 검증만

# 테스트 실행 (현재 테스트 없음)
cargo test

# 단일 테스트 실행 (테스트 추가 후)
cargo test test_user_creation -- --nocapture

# 특정 모듈 테스트
cargo test models::tests
```

### 빌드 최적화
```bash
# 디버그 빌드
cargo build

# 프로덕션 빌드
cargo build --release

# 타겟 지정 빌드
cargo build --release --target x86_64-unknown-linux-gnu
```

## 📝 코드 스타일 가이드라인

### 임포트 구성 순서 (실제 패턴)
```rust
// 1. 로컬 모듈 (현재 패턴)
use crate::logging::{log_request_start, log_request_complete};
use crate::models::{CreateUserRequest, UserResponse};
use crate::templates::{IndexTemplate, UsersTemplate};

// 2. 외부 크레이트 (알파벳 순)
use askama::Template;
use axum::{extract::Path, Form, response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};

// 3. 표준 라이브러리 (현재 없음)
// use std::collections::HashMap;
```

### 네이밍 규칙
```rust
// 타입과 트레이트: PascalCase
pub struct UserResponse {}
pub struct IndexTemplate {}
pub struct CreateUserRequest {}

// 함수와 메서드: snake_case
fn create_user() {}
pub async fn handle_create_user() {}
pub fn log_user_creation() {}

// 변수와 필드: snake_case
let user_id = 1;
let created_at = chrono::Utc::now();

// 상수: SCREAMING_SNAKE_CASE
const MAX_USERS: usize = 1000;
```

### 코드 구조 규칙
- **들여쓰기**: 4스페이스
- **라인 길이**: 100자 이하 권장
- **함수 길이**: 20라인 이하 권장
- **파일 길이**: 150라인 이하 권장

### 주석 규칙
```rust
/// 함수 레벨 문서 주석
/// 사용자를 생성하고 데이터베이스에 저장합니다.
pub async fn create_user() -> UserResponse {

// 라인 주석: 한글 설명 가능
// 🛒 전자상거래 핵심 로직

// TODO: 개선이 필요한 부분
// FIXME: 버그가 있는 부분
// NOTE: 중요한 설명
```

### 에모지 사용 가이드
```rust
// 일관된 에모지 사용 (프로젝트 특성)
println!("🛒 전자상거래 시스템 시작");
println!("👤 사용자 생성: {}", username);
println!("✅ 작업 완료");
println!("❌ 에러 발생");

// 추적 로깅 에모지
log_request_start()  // 📥 요청 시작
log_request_complete() // 📤 요청 완료
log_user_creation()   // 👤 사용자 생성
log_user_not_found()  // ❌ 사용자 없음
```

## 🏗️ 프로젝트 아키텍처

### 모듈 구조
```
src/
├── main.rs          # 애플리케이션 진입점 (40라인)
├── models/mod.rs     # 데이터 모델, 서비스 함수 (54라인)
├── routes/mod.rs     # HTTP 핸들러, 라우팅 (129라인)
├── logging/mod.rs    # 추적 로깅 유틸리티 (66라인)
└── templates/mod.rs  # Askama 템플릿 구조체 (43라인)

templates/            # HTML 템플릿 파일
static/              # 정적 파일 (CSS, JS)
├── css/style.css     # 전역 스타일
└── js/app.js        # 프론트엔드 로직
```

### 데이터 모델 패턴
```rust
// 요청 구조체
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
}

// 응답 구조체
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}
```

### Askama 템플릿 패턴
```rust
// 데이터 없는 템플릿
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

// 데이터 있는 템플릿
#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserResponse>,
}

// 핸들러에서 사용
pub async fn list_users() -> Html<String> {
    log_request_start("/users", "GET");
    log_template_render("users.html");
    
    let template = UsersTemplate { users };
    let result = Html(template.render().unwrap());
    log_request_complete("/users", "GET", 200);
    result
}
```

### 추적 로깅 패턴
```rust
// 로거 초기화
tracing_subscriber::fmt()
    .with_target(true)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .with_max_level(tracing::Level::INFO)
    .init();

// 로깅 함수
#[instrument]
pub fn log_user_creation(username: &str, email: &str) {
    info!("👤 사용자 생성 - 사용자: {}, 이메일: {}", username, email);
}
```

## 🧪 테스트 작성 가이드

### 테스트 구조 (현재 없음 - 추가 필요)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_creation() {
        let request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let result = create_user(request).await;
        assert_eq!(result.username, "testuser");
    }
}
```

### 테스트 실행 명령어
```bash
cargo test                              # 전체 테스트
cargo test test_user_creation -- --nocapture  # 단일 테스트
```

## ⚠️ 에러 핸들링 규칙

### 현재 상태 (실제 패턴)
```rust
// 현재 방식 (unwrap() 남용)
let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
Html(template.render().unwrap())

// 유일한 적절한 에러 핸들링
if let Err(e) = axum::serve(listener, app).await {
    error!("🔥 서버 실행 실패: {}", e);
    std::process::exit(1);
}
```

### 개선 방향
```rust
// 권장 개선 방식
let listener = tokio::net::TcpListener::bind(bind_addr)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", bind_addr, e))?;

let rendered = template.render()
    .map_err(|e| anyhow::anyhow!("Template render failed: {}", e))?;
Html(rendered)
```

## 🔗 의존성 관리

### 주요 의존성
- **axum 0.7**: 웹 프레임워크
- **askama 0.12**: 템플릿 엔진
- **tokio**: 비동기 런타임
- **tracing**: 추적 로깅
- **serde**: 직렬화

### 의존성 추가
```bash
cargo add serde --features derive
cargo add askama --features "with-axum"
```

## 🔄 개발 워크플로우

### 코드 검증 체크리스트
```bash
cargo check && cargo fmt && cargo clippy && cargo test && cargo build --release
```

---

이 가이드는 현재 코드베이스의 실제 패턴을 기반으로 작성되었습니다.