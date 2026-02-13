//! Service 계층: 비즈니스 로직 및 유효성 검사

use crate::models::board::Board;
use crate::repositories::board_repository::BoardRepository;
use std::sync::Arc;
use tracing::{debug, info, warn};

pub struct BoardService {
    repository: Arc<BoardRepository>,
}

#[derive(Debug)]
#[allow(dead_code)] // Debug 트레이트 구현 시 필드가 '사용되지 않는 코드'로 경고 발생 방지
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
        info!("[Service] get_all_boards 호출됨");
        let boards = self.repository.find_all().await?;
        debug!("[Service] get_all_boards 반환: {}개의 게시글", boards.len());
        Ok(boards)
    }

    pub async fn get_board(&self, id: i64) -> Result<Board, ServiceError> {
        info!("[Service] get_board 호출됨, id={}", id);
        if id <= 0 {
            warn!("[Service] 유효하지 않은 ID: {}", id);
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        debug!("[Service] ID 유효성 검사 통과: {}", id);
        let board = self.repository.find_by_id(id).await?;
        match board {
            Some(b) => {
                debug!("[Service] get_board 반환: 게시글 id={} 찾음", id);
                Ok(b)
            }
            None => {
                debug!("[Service] get_board 반환: 게시글 id={} 없음 (NotFound)", id);
                Err(ServiceError::NotFound)
            }
        }
    }

    pub async fn create_board(&self, title: &str, content: &str) -> Result<i64, ServiceError> {
        info!("[Service] create_board 호출됨, title={}", title);
        self.validate_title(title)?;
        self.validate_content(content)?;
        debug!("[Service] 제목 및 내용 유효성 검사 통과");

        let id = self.repository.insert(title, content).await?;
        info!("[Service] create_board 반환: 게시글 생성 완료 id={}", id);
        Ok(id)
    }

    pub async fn update_board(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<bool, ServiceError> {
        info!("[Service] update_board 호출됨, id={}, title={}", id, title);
        if id <= 0 {
            warn!("[Service] 유효하지 않은 ID: {}", id);
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        self.validate_title(title)?;
        self.validate_content(content)?;
        debug!("[Service] ID, 제목, 내용 유효성 검사 통과");

        let updated = self.repository.update(id, title, content).await?;
        if !updated {
            warn!("[Service] 수정할 게시글 없음 id={}", id);
            debug!(
                "[Service] update_board 반환: 게시글 id={} 수정 실패 (NotFound)",
                id
            );
            return Err(ServiceError::NotFound);
        }
        info!("[Service] update_board 반환: 게시글 수정 완료 id={}", id);
        Ok(true)
    }

    pub async fn delete_board(&self, id: i64) -> Result<bool, ServiceError> {
        info!("[Service] delete_board 호출됨, id={}", id);
        if id <= 0 {
            warn!("[Service] 유효하지 않은 ID: {}", id);
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        debug!("[Service] ID 유효성 검사 통과: {}", id);
        let deleted = self.repository.delete(id).await?;
        if !deleted {
            warn!("[Service] 삭제할 게시글 없음 id={}", id);
            debug!(
                "[Service] delete_board 반환: 게시글 id={} 삭제 실패 (NotFound)",
                id
            );
            return Err(ServiceError::NotFound);
        }
        info!("[Service] delete_board 반환: 게시글 삭제 완료 id={}", id);
        Ok(true)
    }

    fn validate_title(&self, title: &str) -> Result<(), ServiceError> {
        if title.trim().is_empty() {
            debug!("[Service] 제목 유효성 검사 실패: 빈 제목");
            return Err(ServiceError::InvalidInput("제목은 필수입니다".to_string()));
        }
        if title.len() > 200 {
            debug!(
                "[Service] 제목 유효성 검사 실패: 너무 김 ({}자)",
                title.len()
            );
            return Err(ServiceError::InvalidInput(
                "제목이 너무 길습니다 (최대 200자)".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_content(&self, content: &str) -> Result<(), ServiceError> {
        if content.trim().is_empty() {
            debug!("[Service] 내용 유효성 검사 실패: 빈 내용");
            return Err(ServiceError::InvalidInput("내용은 필수입니다".to_string()));
        }
        Ok(())
    }
}
