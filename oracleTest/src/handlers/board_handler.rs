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

/// 게시판 컨트롤러: 비즈니스 로직(Service)과 HTTP 요청(Handler)을 연결하는 역할
pub struct BoardController {
    service: Arc<BoardService>,
}

/// API 응답 모델: 클라이언트에게 반환되는 게시글 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct BoardResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    /// 생성 일시 (문자열 포맷)
    pub created_at: Option<String>,
}

/// 게시글 생성 요청 모델
#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    /// 게시글 제목
    pub title: String,
    /// 게시글 내용
    pub content: String,
}

/// 게시글 수정 요청 모델
#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub title: String,
    pub content: String,
}

#[allow(dead_code)]
impl BoardController {
    /// 컨트롤러 생성자: Service 의존성 주입
    pub fn new(service: Arc<BoardService>) -> Self {
        Self { service }
    }

    /// 내부 로직: 모든 게시글 조회
    pub async fn list_boards_internal(&self) -> Result<Vec<Board>, ServiceError> {
        info!("[Controller] list_boards_internal 호출됨");
        let boards = self.service.get_all_boards().await?;
        Ok(boards)
    }

    /// 내부 로직: 특정 게시글 조회
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

    /// 내부 로직: 게시글 생성
    pub async fn create_board_internal(
        &self,
        title: &str,
        content: &str,
    ) -> Result<i64, ServiceError> {
        info!("[Controller] create_board_internal 호출됨, title={}", title);
        let id = self.service.create_board(title, content).await?;
        Ok(id)
    }

    /// 내부 로직: 게시글 수정
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

    /// 내부 로직: 게시글 삭제
    pub async fn delete_board_internal(&self, id: i64) -> Result<(), ServiceError> {
        info!("[Controller] delete_board_internal 호출됨, id={}", id);
        self.service.delete_board(id).await?;
        Ok(())
    }
}

/// GET /boards
/// 모든 게시글 목록을 조회하여 JSON으로 반환합니다.
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

/// GET /boards/:id
/// 특정 ID의 게시글을 조회합니다. 존재하지 않으면 404를 반환합니다.
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

/// POST /boards
/// 새로운 게시글을 생성합니다.
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

/// PUT /boards/:id
/// 기존 게시글을 수정합니다.
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

/// DELETE /boards/:id
/// 특정 게시글을 삭제합니다.
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

/// GET /
/// 정적 HTML 파일을 서빙합니다.
pub async fn serve_index() -> Result<Html<String>, StatusCode> {
    tokio::fs::read_to_string("static/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::NOT_FOUND)
}
