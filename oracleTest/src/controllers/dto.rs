//! Controller 계층에서 사용하는 데이터 전송 객체 (DTO) 모음

use crate::models::board::Board;
use serde::{Deserialize, Serialize};

/// 게시글 응답을 위한 DTO
#[derive(Debug, Serialize)]
pub struct BoardResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
}

/// Board 모델을 BoardResponse DTO로 변환
impl From<Board> for BoardResponse {
    fn from(board: Board) -> Self {
        Self {
            id: board.id,
            title: board.title,
            content: board.content,
            created_at: board.created_at.map(|ts| ts.to_string()),
        }
    }
}

/// 커서 기반 페이징 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CursorRequest {
    pub last_id: Option<i64>,
    pub size: Option<i64>,
}

/// 커서 기반 페이징 응답 DTO
#[derive(Debug, Serialize)]
pub struct CursorResponse {
    pub data: Vec<BoardResponse>,
    pub pagination: CursorPagination,
}

/// 페이징 정보 DTO
#[derive(Debug, Serialize)]
pub struct CursorPagination {
    pub last_id: Option<i64>,
    pub next_cursor: Option<i64>,
    pub size: i64,
    pub has_more: bool,
}

/// 게시글 생성을 위한 요청 DTO
#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub title: String,
    pub content: String,
}

/// 게시글 수정을 위한 요청 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub title: String,
    pub content: String,
}
