use crate::handlers::board_handler::BoardController;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub controller: Arc<BoardController>,
}
