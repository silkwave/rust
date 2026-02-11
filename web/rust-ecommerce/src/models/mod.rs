use crate::logging::log_user_creation;
use serde::{Deserialize, Serialize};

// 사용자 데이터 모델
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

// 서비스 함수들
pub async fn create_user(request: CreateUserRequest) -> UserResponse {
    log_user_creation(&request.username, &request.email);

    UserResponse {
        id: 1,
        username: request.username,
        email: request.email,
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}

pub async fn get_user_by_id(user_id: i64) -> Option<UserResponse> {
    if user_id == 1 {
        println!("👁 사용자 조회: ID {}", user_id);
        Some(UserResponse {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    } else {
        None
    }
}
