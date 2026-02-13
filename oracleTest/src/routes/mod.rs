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

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/boards", get(list_boards))
        .route("/boards", post(create_board))
        .route("/boards/:id", get(get_board))
        .route("/boards/:id", put(update_board))
        .route("/boards/:id", delete(delete_board))
}
