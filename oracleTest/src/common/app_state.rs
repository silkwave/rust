use crate::services::board_service::BoardService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<BoardService>,
}
