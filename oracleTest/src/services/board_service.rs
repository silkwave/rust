//! Service 계층: 비즈니스 로직 및 유효성 검사

use crate::models::board::Board;
use crate::repositories::board_repository::BoardRepository;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// 게시판 비즈니스 로직을 담당하는 서비스 구조체
pub struct BoardService {
    repository: Arc<BoardRepository>,
}

/// 서비스 계층에서 발생할 수 있는 에러 정의
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
    /// 서비스 생성자: Repository 의존성 주입
    pub fn new(repository: Arc<BoardRepository>) -> Self {
        Self { repository }
    }

    /// 커서 기반 페이징 조회
    pub async fn get_boards_cursor(
        &self,
        last_id: Option<i64>,
        size: i64,
    ) -> Result<(Vec<Board>, Option<i64>), ServiceError> {
        info!(
            "[Service] get_boards_cursor 호출됨, last_id={:?}, size={}",
            last_id, size
        );

        if size <= 0 {
            return Err(ServiceError::InvalidInput(
                "size는 1 이상이어야 합니다".to_string(),
            ));
        }

        let boards = self.repository.find_by_cursor(last_id, size).await?;

        let next_cursor = boards.last().map(|b| b.id);
        let total = self.repository.count_all().await?;

        debug!(
            "[Service] get_boards_cursor 반환: {}개 (total={})",
            boards.len(),
            total
        );
        Ok((boards, next_cursor))
    }

    /// 특정 게시글 조회 로직 (ID 유효성 검사 포함)
    pub async fn get_board(&self, id: i64) -> Result<Board, ServiceError> {
        info!("[Service] get_board 호출됨, id={}", id);
        self.validate_id(id)?;
        debug!("[Service] ID 유효성 검사 통과: {}", id);
        match self.repository.find_by_id(id).await? {
            Some(board) => {
                debug!("[Service] get_board 반환: 게시글 id={} 찾음", id);
                Ok(board)
            }
            None => {
                debug!("[Service] get_board 반환: 게시글 id={} 없음 (NotFound)", id);
                Err(ServiceError::NotFound)
            }
        }
    }

    /// 게시글 생성 로직 (제목/내용 유효성 검사 포함)
    pub async fn create_board(&self, title: &str, content: &str) -> Result<i64, ServiceError> {
        info!("[Service] create_board 호출됨, title={}", title);
        self.validate_title(title)?;
        self.validate_content(content)?;
        debug!("[Service] 제목 및 내용 유효성 검사 통과");

        let id = self.repository.insert(title, content).await?;
        info!("[Service] create_board 반환: 게시글 생성 완료 id={}", id);
        Ok(id)
    }

    /// 게시글 수정 로직
    pub async fn update_board(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<bool, ServiceError> {
        info!("[Service] update_board 호출됨, id={}, title={}", id, title);
        self.validate_id(id)?;
        self.validate_title(title)?;
        self.validate_content(content)?;
        debug!("[Service] ID, 제목, 내용 유효성 검사 통과");

        let updated = self.repository.update(id, title, content).await?;
        if updated {
            info!("[Service] update_board 반환: 게시글 수정 완료 id={}", id);
            Ok(true)
        } else {
            warn!("[Service] 수정할 게시글 없음 id={}", id);
            debug!(
                "[Service] update_board 반환: 게시글 id={} 수정 실패 (NotFound)",
                id
            );
            Err(ServiceError::NotFound)
        }
    }

    /// 게시글 삭제 로직
    pub async fn delete_board(&self, id: i64) -> Result<bool, ServiceError> {
        info!("[Service] delete_board 호출됨, id={}", id);
        self.validate_id(id)?;
        debug!("[Service] ID 유효성 검사 통과: {}", id);
        let deleted = self.repository.delete(id).await?;
        if deleted {
            info!("[Service] delete_board 반환: 게시글 삭제 완료 id={}", id);
            Ok(true)
        } else {
            warn!("[Service] 삭제할 게시글 없음 id={}", id);
            debug!(
                "[Service] delete_board 반환: 게시글 id={} 삭제 실패 (NotFound)",
                id
            );
            Err(ServiceError::NotFound)
        }
    }

    /// 제목 유효성 검사: 비어있거나 너무 긴 경우 에러
    fn validate_id(&self, id: i64) -> Result<(), ServiceError> {
        if id <= 0 {
            warn!("[Service] 유효하지 않은 ID: {}", id);
            return Err(ServiceError::InvalidInput("Invalid ID".to_string()));
        }
        Ok(())
    }

    /// 제목 유효성 검사: 비어있거나 너무 긴 경우 에러
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

    /// 내용 유효성 검사: 비어있는 경우 에러
    fn validate_content(&self, content: &str) -> Result<(), ServiceError> {
        if content.trim().is_empty() {
            debug!("[Service] 내용 유효성 검사 실패: 빈 내용");
            return Err(ServiceError::InvalidInput("내용은 필수입니다".to_string()));
        }
        Ok(())
    }
}
