//! `board` 리소스에 대한 HTTP 요청을 처리하는 핸들러 함수들

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Html,
};
use tracing::info;

use crate::common::app_state::AppState;

use super::{
    dto::{
        BoardResponse, CreateBoardRequest, PaginationMeta, PaginationRequest, PaginationResponse,
        UpdateBoardRequest,
    },
    error::ControllerError,
};

/// 게시글 목록을 페이지네이션으로 조회합니다.
pub async fn list_boards(
    State(state): State<AppState>,
    Query(pagination_req): Query<PaginationRequest>,
) -> Result<Json<PaginationResponse>, ControllerError> {
    info!(
        "[Controller] list_boards 호출됨, pagination_req={:?}",
        pagination_req
    );
    let page = pagination_req.page.unwrap_or(1); // 기본 1페이지
    let size = pagination_req.size.unwrap_or(10); // 기본 10개

    // 서비스 계층을 호출하여 데이터를 가져옵니다.
    let (boards, total_pages) = state.service.get_boards_paged(page, size).await?;

    let data = boards.into_iter().map(BoardResponse::from).collect();

    Ok(Json(PaginationResponse {
        data,
        pagination: PaginationMeta {
            current_page: page,
            total_pages,
            size,
        },
    }))
}

/// 특정 ID의 게시글을 조회합니다.
pub async fn get_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BoardResponse>, ControllerError> {
    info!("[Controller] get_board 호출됨, id={}", id);
    let board = state.service.get_board(id).await?;
    Ok(Json(BoardResponse::from(board)))
}

/// 새로운 게시글을 생성합니다.
pub async fn create_board(
    State(state): State<AppState>,
    Json(req): Json<CreateBoardRequest>,
) -> Result<(StatusCode, Json<BoardResponse>), ControllerError> {
    info!("[Controller] create_board 호출됨, title={}", req.title);
    let board = state.service.create_board(&req.title, &req.content).await?;
    Ok((StatusCode::CREATED, Json(BoardResponse::from(board))))
}

/// 기존 게시글을 수정합니다.
pub async fn update_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(req): Json<UpdateBoardRequest>,
) -> Result<StatusCode, ControllerError> {
    info!("[Controller] update_board 호출됨, id={}", id);
    state
        .service
        .update_board(id, &req.title, &req.content)
        .await?;
    Ok(StatusCode::OK)
}

/// 특정 ID의 게시글을 삭제합니다.
pub async fn delete_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<StatusCode, ControllerError> {
    info!("[Controller] delete_board 호출됨, id={}", id);
    state.service.delete_board(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// 정적 파일을 서빙합니다 (예: index.html).
pub async fn serve_index() -> Result<Html<String>, ControllerError> {
    info!("[Controller] static/index.html 호출됨");
    let content = tokio::fs::read_to_string("static/index.html").await?;
    Ok(Html(content))
}
