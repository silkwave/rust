//! Service 계층: 비즈니스 로직 및 유효성 검사

use crate::model::Board;
use crate::repository::BoardRepository;
use std::sync::Arc;

pub struct BoardService {
    repository: Arc<BoardRepository>,
}

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    InvalidInput(String),
    DatabaseError(oracle::Error),
}

impl From<oracle::Error> for ServiceError {
    fn from(err: oracle::Error) -> Self {
        ServiceError::DatabaseError(err)
    }
}

impl BoardService {
    pub fn new(repository: Arc<BoardRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_boards(&self) -> Result<Vec<Board>, ServiceError> {
        let boards = self.repository.find_all().await?;
        Ok(boards)
    }

    pub async fn get_board(&self, id: i64) -> Result<Board, ServiceError> {
        if id <= 0 {
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        let board = self.repository.find_by_id(id).await?;
        board.ok_or(ServiceError::NotFound)
    }

    pub async fn create_board(&self, title: &str, content: &str) -> Result<i64, ServiceError> {
        self.validate_title(title)?;
        self.validate_content(content)?;

        let id = self.repository.insert(title, content).await?;
        Ok(id)
    }

    pub async fn update_board(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<bool, ServiceError> {
        if id <= 0 {
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        self.validate_title(title)?;
        self.validate_content(content)?;

        let updated = self.repository.update(id, title, content).await?;
        if !updated {
            return Err(ServiceError::NotFound);
        }
        Ok(true)
    }

    pub async fn delete_board(&self, id: i64) -> Result<bool, ServiceError> {
        if id <= 0 {
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        let deleted = self.repository.delete(id).await?;
        if !deleted {
            return Err(ServiceError::NotFound);
        }
        Ok(true)
    }

    fn validate_title(&self, title: &str) -> Result<(), ServiceError> {
        if title.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 200 {
            return Err(ServiceError::InvalidInput(
                "Title too long (max 200 chars)".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_content(&self, content: &str) -> Result<(), ServiceError> {
        if content.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Content cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
