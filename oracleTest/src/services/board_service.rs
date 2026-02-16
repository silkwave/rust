//! Service 계층: 비즈니스 로직 및 유효성 검사

use crate::models::board::{Board, BoardListItem};
use crate::repositories::board_repository::BoardRepository;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// 게시판 비즈니스 로직을 담당하는 서비스 구조체
pub struct BoardService {
    repository: Arc<BoardRepository>,
}

/// 서비스 계층에서 발생할 수 있는 에러 정의
#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    InvalidInput(String),
    DatabaseError(oracle::Error),
}

/// oracle::Error를 ServiceError로 자동 변환
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

    /// 페이지네이션을 사용하여 게시글 목록 조회
    pub async fn get_boards_paged(
        &self,
        page: u32,
        size: u32,
    ) -> Result<(Vec<BoardListItem>, u32), ServiceError> {
        info!(
            "[Service] get_boards_paged 호출: page={}, size={}",
            page, size
        );
        self.validate_page(page)?;
        self.validate_size(size)?;

        let total_boards = self.repository.count_all().await?;
        let total_pages = total_boards.div_ceil(size);

        // 요청한 페이지가 총 페이지 수를 초과하는 경우
        if page > total_pages && total_pages > 0 {
            warn!(
                "[Service] 요청 페이지 초과: page={}, total_pages={}",
                page, total_pages
            );
            return Err(ServiceError::InvalidInput(format!(
                "요청 페이지가 너무 큽니다. 최대 페이지: {}",
                total_pages
            )));
        }

        let offset = (page - 1) * size;
        let boards = self.repository.find_paged(offset, size).await?;

        debug!(
            "[Service] get_boards_paged 반환: {}개, 총 페이지: {}",
            boards.len(),
            total_pages
        );
        Ok((boards, total_pages))
    }

    /// 특정 게시글 조회 로직 (ID 유효성 검사 포함)
    pub async fn get_board(&self, id: i64) -> Result<Board, ServiceError> {
        info!("[Service] get_board 호출됨, id={}", id);
        self.validate_id(id)?;

        self.repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)
    }

    /// 게시글 생성 로직 (제목/내용 유효성 검사 포함)
    pub async fn create_board(&self, title: &str, content: &str) -> Result<Board, ServiceError> {
        info!("[Service] create_board 호출됨, title={}", title);
        self.validate_title(title)?;
        self.validate_content(content)?;

        let id = self
            .repository
            .insert(title.to_string(), content.to_string())
            .await?;
        info!("[Service] 게시글 생성 완료 id={}, 다시 조회합니다.", id);

        // 생성된 게시글을 다시 조회하여 완전한 객체로 반환
        self.get_board(id).await
    }

    /// 게시글 수정 로직
    pub async fn update_board(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<(), ServiceError> {
        info!("[Service] update_board 호출됨, id={}, title={}", id, title);
        self.validate_id(id)?;
        self.validate_title(title)?;
        self.validate_content(content)?;

        let updated = self
            .repository
            .update(id, title.to_string(), content.to_string())
            .await?;
        if updated {
            info!("[Service] update_board 반환: 게시글 수정 완료 id={}", id);
            Ok(())
        } else {
            warn!("[Service] 수정할 게시글 없음 id={}", id);
            Err(ServiceError::NotFound)
        }
    }

    /// 게시글 삭제 로직
    pub async fn delete_board(&self, id: i64) -> Result<(), ServiceError> {
        info!("[Service] delete_board 호출됨, id={}", id);
        self.validate_id(id)?;

        if self.repository.delete(id).await? {
            info!("[Service] delete_board 반환: 게시글 삭제 완료 id={}", id);
            Ok(())
        } else {
            warn!("[Service] 삭제할 게시글 없음 id={}", id);
            Err(ServiceError::NotFound)
        }
    }

    // --- 유효성 검사 헬퍼 함수들 ---

    fn validate_id(&self, id: i64) -> Result<(), ServiceError> {
        if id > 0 {
            Ok(())
        } else {
            warn!("[Service] 유효하지 않은 ID: {}", id);
            Err(ServiceError::InvalidInput(
                "ID는 0보다 커야 합니다.".to_string(),
            ))
        }
    }

    fn validate_page(&self, page: u32) -> Result<(), ServiceError> {
        if page > 0 {
            Ok(())
        } else {
            warn!("[Service] 유효하지 않은 페이지 번호: {}", page);
            Err(ServiceError::InvalidInput(
                "페이지 번호는 0보다 커야 합니다.".to_string(),
            ))
        }
    }

    fn validate_size(&self, size: u32) -> Result<(), ServiceError> {
        if size > 0 {
            Ok(())
        } else {
            warn!("[Service] 유효하지 않은 size: {}", size);
            Err(ServiceError::InvalidInput(
                "size는 0보다 커야 합니다.".to_string(),
            ))
        }
    }

    fn validate_title(&self, title: &str) -> Result<(), ServiceError> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err(ServiceError::InvalidInput("제목은 필수입니다.".to_string()));
        }
        if trimmed_title.chars().count() > 200 {
            return Err(ServiceError::InvalidInput(
                "제목이 너무 길습니다 (최대 200자)".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_content(&self, content: &str) -> Result<(), ServiceError> {
        if content.trim().is_empty() {
            return Err(ServiceError::InvalidInput("내용은 필수입니다.".to_string()));
        }
        Ok(())
    }
}
