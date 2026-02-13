//! 애플리케이션 상태 관리 모듈
//!
//! Axum 핸들러 간에 공유되는 전역 상태를 정의합니다.

use crate::handlers::board_handler::BoardController;
use std::sync::Arc;

/// 애플리케이션 전역 상태 구조체
///
/// 이 구조체는 Axum의 `State` 추출기를 통해 핸들러에 주입됩니다.
#[derive(Clone)]
pub struct AppState {
    /// 게시판 컨트롤러 (스레드 안전한 공유를 위해 Arc 사용)
    pub controller: Arc<BoardController>,
}
