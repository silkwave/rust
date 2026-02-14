use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    common::app_state::AppState,
    models::board::Board,
    services::board_service::{BoardService, ServiceError},
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

async fn get_board_internal(
    service: &BoardService,
    id: i64,
) -> Result<Option<Board>, ServiceError> {
    info!("[Controller] get_board_internal 호출됨, id={}", id);
    let result = service.get_board(id).await;
    let board = match result {
        Ok(b) => Some(b),
        Err(ServiceError::NotFound) => None,
        Err(e) => return Err(e),
    };
    Ok(board)
}

async fn create_board_internal(
    service: &BoardService,
    title: &str,
    content: &str,
) -> Result<i64, ServiceError> {
    info!("[Controller] create_board_internal 호출됨, title={}", title);
    let id = service.create_board(title, content).await?;
    Ok(id)
}

async fn update_board_internal(
    service: &BoardService,
    id: i64,
    title: &str,
    content: &str,
) -> Result<(), ServiceError> {
    info!("[Controller] update_board_internal 호출됨, id={}", id);
    service.update_board(id, title, content).await?;
    Ok(())
}

async fn delete_board_internal(
    service: &BoardService,
    id: i64,
) -> Result<(), ServiceError> {
    info!("[Controller] delete_board_internal 호출됨, id={}", id);
    service.delete_board(id).await?;
    Ok(())
}

pub async fn list_boards(
    State(state): State<AppState>,
    Query(cursor): Query<CursorRequest>,
) -> Result<Json<CursorResponse>, StatusCode> {
    let size = cursor.size.unwrap_or(10);

    let (boards, next_cursor) = state.service.get_boards_cursor(cursor.last_id, size).await.map_err(|e| {
        error!("게시글 목록 조회 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let has_more = boards.len() as i64 == size;
    let data: Vec<BoardResponse> = boards
        .into_iter()
        .map(|b| BoardResponse {
            id: b.id,
            title: b.title,
            content: b.content,
            created_at: b.created_at.map(|ts| ts.to_string()),
        })
        .collect();

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
    let board = get_board_internal(&state.service, id).await.map_err(|e| {
        error!("게시글 조회 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    match board {
        Some(b) => Ok(Json(BoardResponse {
            id: b.id,
            title: b.title,
            content: b.content,
            created_at: b.created_at.map(|ts| ts.to_string()),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_board(
    State(state): State<AppState>,
    Json(req): Json<CreateBoardRequest>,
) -> Result<Json<BoardResponse>, StatusCode> {
    let id = create_board_internal(&state.service, &req.title, &req.content)
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
    update_board_internal(&state.service, id, &req.title, &req.content)
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
    delete_board_internal(&state.service, id).await.map_err(|e| {
        error!("게시글 삭제 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::OK)
}

pub async fn serve_index() -> Result<Html<String>, StatusCode> {
    tokio::fs::read_to_string("static/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::NOT_FOUND)
}
