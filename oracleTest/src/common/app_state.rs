use crate::services::board_service::BoardService;
use std::sync::Arc;

/// 애플리케이션의 공유 상태를 나타내는 구조체.
/// 모든 핸들러에서 접근할 수 있도록 `BoardService` 인스턴스를 포함합니다.
#[derive(Clone)]
pub struct AppState {
    /// `BoardService` 인스턴스를 `Arc`로 래핑하여 여러 스레드에서 안전하게 공유하고 접근할 수 있도록 합니다.
    pub service: Arc<BoardService>,
}
