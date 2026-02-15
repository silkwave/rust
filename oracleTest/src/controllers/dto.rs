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

/// 페이지네이션 요청 DTO
#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

/// 페이지네이션 응답 DTO
#[derive(Debug, Serialize)]
pub struct PaginationResponse {
    pub data: Vec<BoardResponse>,
    pub pagination: PaginationMeta,
}

/// 페이지네이션 메타데이터 DTO
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub total_pages: u32,
    pub size: u32,
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
