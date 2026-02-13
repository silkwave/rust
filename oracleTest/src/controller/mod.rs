//! Controller 계층: 요청 처리 및 출력

use crate::model::Board;
use crate::service::{BoardService, ServiceError};
use std::sync::Arc;
use tracing::{debug, error, info};

pub struct BoardController {
    service: Arc<BoardService>,
}

impl BoardController {
    pub fn new(service: Arc<BoardService>) -> Self {
        Self { service }
    }

    pub async fn list_boards(&self) {
        info!("[Controller] 게시글 목록 조회");
        match self.service.get_all_boards().await {
            Ok(boards) => {
                println!("\n=== Board List ({} items) ===", boards.len());
                for board in &boards {
                    self.print_board(board);
                }
            }
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn get_board(&self, id: i64) {
        info!("[Controller] 게시글 조회 id={}", id);
        match self.service.get_board(id).await {
            Ok(board) => {
                println!("\n=== Board Detail ===");
                self.print_board(&board);
            }
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn create_board(&self, title: &str, content: &str) {
        info!("[Controller] 게시글 생성 title={}", title);
        match self.service.create_board(title, content).await {
            Ok(id) => println!("\nCreated board with ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn update_board(&self, id: i64, title: &str, content: &str) {
        info!("[Controller] 게시글 수정 id={}", id);
        match self.service.update_board(id, title, content).await {
            Ok(_) => println!("\nUpdated board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn delete_board(&self, id: i64) {
        info!("[Controller] 게시글 삭제 id={}", id);
        match self.service.delete_board(id).await {
            Ok(_) => println!("\nDeleted board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn list_boards_internal(&self) -> Result<Vec<Board>, ServiceError> {
        info!("[Controller] list_boards_internal 호출됨");
        let boards = self.service.get_all_boards().await?;
        debug!("[Controller] list_boards_internal 반환: {}개의 게시글", boards.len());
        Ok(boards)
    }

    pub async fn get_board_internal(&self, id: i64) -> Result<Option<Board>, ServiceError> {
        info!("[Controller] get_board_internal 호출됨, id={}", id);
        let result = self.service.get_board(id).await;
        let board = match result {
            Ok(b) => {
                debug!("[Controller] get_board_internal 반환: 게시글 id={} 찾음", id);
                Some(b)
            },
            Err(ServiceError::NotFound) => {
                debug!("[Controller] get_board_internal 반환: 게시글 id={} 없음", id);
                None
            },
            Err(e) => return Err(e), // 다른 종류의 에러는 그대로 반환
        };
        Ok(board)
    }

    pub async fn create_board_internal(&self, title: &str, content: &str) -> Result<i64, ServiceError> {
        info!("[Controller] create_board_internal 호출됨, title={}", title);
        let id = self.service.create_board(title, content).await?;
        debug!("[Controller] create_board_internal 반환: 생성된 게시글 id={}", id);
        Ok(id)
    }

    pub async fn update_board_internal(&self, id: i64, title: &str, content: &str) -> Result<(), ServiceError> {
        info!("[Controller] update_board_internal 호출됨, id={}", id);
        let updated = self.service.update_board(id, title, content).await?;
        if updated {
            debug!("[Controller] update_board_internal 반환: 게시글 id={} 수정 성공", id);
        } else {
            debug!("[Controller] update_board_internal 반환: 게시글 id={} 수정 실패 (NotFound)", id);
        }
        Ok(())
    }

    pub async fn delete_board_internal(&self, id: i64) -> Result<(), ServiceError> {
        info!("[Controller] delete_board_internal 호출됨, id={}", id);
        let deleted = self.service.delete_board(id).await?;
        if deleted {
            debug!("[Controller] delete_board_internal 반환: 게시글 id={} 삭제 성공", id);
        } else {
            debug!("[Controller] delete_board_internal 반환: 게시글 id={} 삭제 실패 (NotFound)", id);
        }
        Ok(())
    }

    fn print_board(&self, board: &Board) {
        println!("----------------------------------------");
        println!("ID:      {}", board.id);
        println!("Title:   {}", board.title);
        println!("Content: {}", board.content);
        if let Some(created) = &board.created_at {
            println!("Created: {}", created);
        }
    }

    fn print_error(&self, err: &ServiceError) {
        error!("[Controller] 오류 발생: {:?}", err);
        println!("\nError: {:?}", err);
    }
}
