//! 라우팅 설정 모듈: API 엔드포인트와 핸들러 매핑

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    common::app_state::AppState,
    handlers::board_handler::{
        create_board, delete_board, get_board, list_boards, serve_index, update_board,
    },
};

/// API 라우터 설정
///
/// 각 HTTP 메서드와 경로를 해당 핸들러 함수에 매핑합니다.
pub fn api_routes() -> Router<AppState> {
    Router::new()
        // 정적 파일 서빙 (메인 페이지)
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        // 게시판 CRUD 엔드포인트
        .route("/boards", get(list_boards))         // 목록 조회
        .route("/boards", post(create_board))       // 생성
        .route("/boards/:id", get(get_board))       // 상세 조회
        .route("/boards/:id", put(update_board))    // 수정
        .route("/boards/:id", delete(delete_board)) // 삭제
}
