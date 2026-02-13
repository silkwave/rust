//! Controller 계층: 요청 처리 및 출력

use crate::model::Board;
use crate::service::{BoardService, ServiceError};
use std::sync::Arc;
use tracing::{error, info};

pub struct BoardController {
    service: Arc<BoardService>,
}

impl BoardController {
    pub fn new(service: Arc<BoardService>) -> Self {
        Self { service }
    }

    pub async fn list_boards(&self) {
        info!("Controller: Listing all boards");
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
        info!("Controller: Getting board id={}", id);
        match self.service.get_board(id).await {
            Ok(board) => {
                println!("\n=== Board Detail ===");
                self.print_board(&board);
            }
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn create_board(&self, title: &str, content: &str) {
        info!("Controller: Creating board title={}", title);
        match self.service.create_board(title, content).await {
            Ok(id) => println!("\nCreated board with ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn update_board(&self, id: i64, title: &str, content: &str) {
        info!("Controller: Updating board id={}", id);
        match self.service.update_board(id, title, content).await {
            Ok(_) => println!("\nUpdated board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn delete_board(&self, id: i64) {
        info!("Controller: Deleting board id={}", id);
        match self.service.delete_board(id).await {
            Ok(_) => println!("\nDeleted board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn list_boards_internal(&self) -> Result<Vec<Board>, ServiceError> {
        info!("Controller: Listing all boards (internal)");
        self.service.get_all_boards().await
    }

    pub async fn get_board_internal(&self, id: i64) -> Result<Option<Board>, ServiceError> {
        info!("Controller: Getting board id={} (internal)", id);
        self.service.get_board(id).await.map(Some).or(Ok(None))
    }

    pub async fn create_board_internal(&self, title: &str, content: &str) -> Result<i64, ServiceError> {
        info!("Controller: Creating board title={} (internal)", title);
        self.service.create_board(title, content).await
    }

    pub async fn update_board_internal(&self, id: i64, title: &str, content: &str) -> Result<(), ServiceError> {
        info!("Controller: Updating board id={} (internal)", id);
        self.service.update_board(id, title, content).await?;
        Ok(())
    }

    pub async fn delete_board_internal(&self, id: i64) -> Result<(), ServiceError> {
        info!("Controller: Deleting board id={} (internal)", id);
        self.service.delete_board(id).await?;
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
        error!("Controller: Error occurred - {:?}", err);
        println!("\nError: {:?}", err);
    }
}
