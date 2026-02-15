use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    common::app_state::AppState, models::board::Board, services::board_service::ServiceError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CursorRequest {
    pub last_id: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CursorResponse {
    pub data: Vec<BoardResponse>,
    pub pagination: CursorPagination,
}

#[derive(Debug, Serialize)]
pub struct CursorPagination {
    pub last_id: Option<i64>,
    pub next_cursor: Option<i64>,
    pub size: i64,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub title: String,
    pub content: String,
}

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

pub async fn list_boards(
    State(state): State<AppState>,
    Query(cursor): Query<CursorRequest>,
) -> Result<Json<CursorResponse>, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");      
    info!("[Controller] list_boards 호출됨, cursor={:?}", cursor);

    let size = cursor.size.unwrap_or(10);

    let (boards, next_cursor) = state
        .service
        .get_boards_cursor(cursor.last_id, size)
        .await
        .map_err(|e| {
            error!("게시글 목록 조회 실패: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let has_more = boards.len() as i64 == size;
    let data = boards.into_iter().map(BoardResponse::from).collect();

    Ok(Json(CursorResponse {
        data,
        pagination: CursorPagination {
            last_id: cursor.last_id,
            next_cursor,
            size,
            has_more,
        },
    }))
}

pub async fn get_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BoardResponse>, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");      
    info!("[Controller] get_board 호출됨, id={}", id);
    match state.service.get_board(id).await {
        Ok(board) => Ok(Json(BoardResponse::from(board))),
        Err(ServiceError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("게시글 조회 실패: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_board(
    State(state): State<AppState>,
    Json(req): Json<CreateBoardRequest>,
) -> Result<Json<BoardResponse>, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");      
    info!("[Controller] create_board 호출됨, title={}", req.title);
    let id = state
        .service
        .create_board(&req.title, &req.content)
        .await
        .map_err(|e| {
            error!("게시글 생성 실패: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(BoardResponse {
        id,
        title: req.title,
        content: req.content,
        created_at: None,
    }))
}

pub async fn update_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(req): Json<UpdateBoardRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");      
    info!("[Controller] update_board 호출됨, id={}", id);
    state
        .service
        .update_board(id, &req.title, &req.content)
        .await
        .map_err(|e| {
            error!("게시글 수정 실패: {:?}", e);
            match e {
                ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;
    Ok(StatusCode::OK)
}

pub async fn delete_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");      
    info!("[Controller] delete_board 호출됨, id={}", id);
    state.service.delete_board(id).await.map_err(|e| {
        error!("게시글 삭제 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::OK)
}

pub async fn serve_index() -> Result<Html<String>, StatusCode> {
    info!("[Controller] =================================");        
    info!("[Controller] =================================");        
    info!("[Controller] =================================");                
    info!("[Controller] static/index.html 호출됨");    
    tokio::fs::read_to_string("static/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::NOT_FOUND)
}
