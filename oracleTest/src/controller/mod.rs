//! Controller 계층: 요청 처리 및 출력

use crate::model::Board;
use crate::service::{BoardService, ServiceError};
use std::sync::Arc;

pub struct BoardController {
    service: Arc<BoardService>,
}

impl BoardController {
    pub fn new(service: Arc<BoardService>) -> Self {
        Self { service }
    }

    pub async fn list_boards(&self) {
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
        match self.service.get_board(id).await {
            Ok(board) => {
                println!("\n=== Board Detail ===");
                self.print_board(&board);
            }
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn create_board(&self, title: &str, content: &str) {
        match self.service.create_board(title, content).await {
            Ok(id) => println!("\nCreated board with ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn update_board(&self, id: i64, title: &str, content: &str) {
        match self.service.update_board(id, title, content).await {
            Ok(_) => println!("\nUpdated board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
    }

    pub async fn delete_board(&self, id: i64) {
        match self.service.delete_board(id).await {
            Ok(_) => println!("\nDeleted board ID: {}", id),
            Err(e) => self.print_error(&e),
        }
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
        println!("\nError: {:?}", err);
    }
}
