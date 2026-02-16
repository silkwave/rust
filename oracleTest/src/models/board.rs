//! Model 계층: 데이터 구조체

/// 게시판 데이터 모델
#[derive(Debug, Clone)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<oracle::sql_type::Timestamp>,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(id: i64, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            created_at: None,
        }
    }
}
