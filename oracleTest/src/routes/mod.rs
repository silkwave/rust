use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    common::app_state::AppState,
    controllers::board_controller::{
        create_board, delete_board, get_board, list_boards, serve_index, update_board,
    },
};

pub fn api_routes() -> Router<AppState> {
    /// 모든 API 라우트를 정의하고, 애플리케이션 상태(AppState)를 공유하는 라우터 인스턴스를 반환합니다.
    /// 각 라우트는 특정 HTTP 메서드와 엔드포인트에 핸들러 함수를 매핑합니다.
    Router::new()
        .route("/", get(serve_index)) // 루트 경로로 index.html 정적 파일을 서빙합니다.
        .route("/index.html", get(serve_index)) // `/index.html` 경로로 index.html 정적 파일을 서빙합니다.
        .route("/boards", get(list_boards)) // 모든 게시글 목록을 페이지네이션으로 조회합니다.
        .route("/boards", post(create_board)) // 새로운 게시글을 생성합니다.
        .route("/boards/:id", get(get_board)) // 특정 ID의 게시글을 조회합니다.
        .route("/boards/:id", put(update_board)) // 특정 ID의 게시글을 수정합니다.
        .route("/boards/:id", delete(delete_board)) // 특정 ID의 게시글을 삭제합니다.
}
