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
# 린트 체크 (경고 확인)
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

### 임포트 구성 순서
```rust
// 1. 표준 라이브러리 (현재 없음)
// use std::collections::HashMap;

// 2. 외부 크레이트 (알파벳 순)
use askama::Template;
use axum::{
    extract::Path,
    Form,
    response::Html,
    routing::{get, post},
    Router,
};

// 3. 로컬 모듈
use crate::models::{CreateUserRequest, UserResponse, create_user, get_user_by_id};
use crate::templates::{IndexTemplate, UsersTemplate};
```

### 네이밍 규칙
```rust
// 타입과 트레이트: PascalCase
pub struct UserResponse {}
pub trait UserRepository {}

// 함수와 메서드: snake_case
fn create_user() {}
pub async fn handle_create_user() {}

// 변수와 필드: snake_case
let user_id = 1;
let created_at = chrono::Utc::now();

// 템플릿 구조체: PascalCase + Template 접미사
pub struct IndexTemplate {}
pub struct UsersTemplate {}

// 상수: SCREAMING_SNAKE_CASE
const MAX_USERS: usize = 1000;
```

### 코드 구조 규칙
- **들여쓰기**: 4스페이스
- **라인 길이**: 100자 이하 권장
- **함수 길이**: 30라인 이하 권장
- **파일 길이**: 300라인 이하 권장

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
```

## 🏗️ 프로젝트 아키텍처

### 모듈 구조
```
src/
├── main.rs          # 애플리케이션 진입점
├── models/mod.rs     # 데이터 모델, 서비스 함수
├── routes/mod.rs     # HTTP 핸들러, 라우팅
└── templates/mod.rs  # Askama 템플릿 구조체

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
// 템플릿 구조체
#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<UserResponse>,
}

// 핸들러에서 사용
pub async fn list_users() -> Html<String> {
    let template = UsersTemplate { users };
    Html(template.render().unwrap())
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
cargo test models::tests                    # 특정 모듈
```

## ⚠️ 에러 핸들링 규칙

### 현재 상태와 개선 방향
```rust
// 현재 방식
let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

// 권장 개선 방식
let listener = tokio::net::TcpListener::bind(bind_addr).await?;
```

## 🎨 프론트엔드 개발

### 정적 파일 구조
```
static/
├── css/style.css    # 전역 스타일
└── js/app.js       # 프론트엔드 로직
```

### Askama 템플릿 문법
```html
{{ user.username }}  <!-- 변수 출력 -->
{% for user in users %} <!-- 루프 -->
{% endfor %}
```

## 🔄 개발 워크플로우

### 코드 검증 체크리스트
```bash
cargo check && cargo fmt && cargo clippy && cargo test && cargo build --release
```

---

이 가이드는 현재 코드베이스의 실제 패턴을 기반으로 작성되었습니다. 에이전트들은 이 가이드를 따라 일관된 코드 스타일을 유지하고 생산적으로 기여할 수 있습니다.