//! Controller 계층: 요청 처리 및 출력
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    common::app_state::AppState,
    models::board::Board,
    services::board_service::{BoardService, ServiceError},
};

pub struct BoardController {
    service: Arc<BoardService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
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

#[allow(dead_code)]
impl BoardController {
    pub fn new(service: Arc<BoardService>) -> Self {
        Self { service }
    }

    pub async fn list_boards_internal(&self) -> Result<Vec<Board>, ServiceError> {
        info!("[Controller] list_boards_internal 호출됨");
        let boards = self.service.get_all_boards().await?;
        Ok(boards)
    }

    pub async fn get_board_internal(&self, id: i64) -> Result<Option<Board>, ServiceError> {
        info!("[Controller] get_board_internal 호출됨, id={}", id);
        let result = self.service.get_board(id).await;
        let board = match result {
            Ok(b) => Some(b),
            Err(ServiceError::NotFound) => None,
            Err(e) => return Err(e),
        };
        Ok(board)
    }

    pub async fn create_board_internal(
        &self,
        title: &str,
        content: &str,
    ) -> Result<i64, ServiceError> {
        info!("[Controller] create_board_internal 호출됨, title={}", title);
        let id = self.service.create_board(title, content).await?;
        Ok(id)
    }

    pub async fn update_board_internal(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<(), ServiceError> {
        info!("[Controller] update_board_internal 호출됨, id={}", id);
        self.service.update_board(id, title, content).await?;
        Ok(())
    }

    pub async fn delete_board_internal(&self, id: i64) -> Result<(), ServiceError> {
        info!("[Controller] delete_board_internal 호출됨, id={}", id);
        self.service.delete_board(id).await?;
        Ok(())
    }
}

pub async fn list_boards(
    State(state): State<AppState>,
) -> Result<Json<Vec<BoardResponse>>, StatusCode> {
    let boards = state.controller.list_boards_internal().await.map_err(|e| {
        error!("게시글 목록 조회 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let response: Vec<BoardResponse> = boards
        .into_iter()
        .map(|b| BoardResponse {
            id: b.id,
            title: b.title,
            content: b.content,
            created_at: b.created_at.map(|ts| ts.to_string()),
        })
        .collect();
    Ok(Json(response))
}

pub async fn get_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BoardResponse>, StatusCode> {
    let board = state.controller.get_board_internal(id).await.map_err(|e| {
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
    let id = state
        .controller
        .create_board_internal(&req.title, &req.content)
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
    state
        .controller
        .update_board_internal(id, &req.title, &req.content)
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
    state
        .controller
        .delete_board_internal(id)
        .await
        .map_err(|e| {
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
