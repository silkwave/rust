//! Repository 계층: 데이터베이스 CRUD 작업

use crate::common::queries::{
    DELETE_BOARD, INSERT_BOARD, SELECT_BOARD_BY_ID, SELECT_BOARD_COUNT,
    SELECT_BOARD_SEQ_CURRVAL, SELECT_BOARD_PAGED, UPDATE_BOARD,
};
use crate::models::board::{Board, DbPool};
use oracle::{ErrorKind, Row};
use tracing::{debug, info, warn};

/// 게시판 데이터베이스 접근 객체 (DAO)
pub struct BoardRepository {
    pool: DbPool,
}

impl BoardRepository {
    /// 새로운 Repository 인스턴스 생성
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// 전체 게시글 수 조회
    pub async fn count_all(&self) -> Result<u32, oracle::Error> {
        info!("[Repo] count_all 호출");
        let conn = self.pool.conn.lock().await;
        // 쿼리 실행 후 첫 번째 행의 첫 번째 컬럼 값을 가져옴
        conn.query_row_as::<u32>(SELECT_BOARD_COUNT, &[])
    }

    /// 페이지네이션을 사용하여 게시글 목록 조회
    pub async fn find_paged(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Board>, oracle::Error> {
        info!("[Repo] find_paged 호출: offset={}, limit={}", offset, limit);
        let conn = self.pool.conn.lock().await;

        let rows = conn.query(SELECT_BOARD_PAGED, &[&(offset as i64), &(limit as i64)])?;

        rows.map(|row_result| self.row_to_board(row_result?))
            .collect()
    }

    /// ID로 단일 게시글 조회
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Board>, oracle::Error> {
        info!("[Repo] find_by_id 호출: id={}", id);
        let conn = self.pool.conn.lock().await;
        let mut rows = conn.query(SELECT_BOARD_BY_ID, &[&id])?;
        rows.next()
            .map(|row_result| self.row_to_board(row_result?))
            .transpose()
    }

    /// 새 게시글 추가 후 생성된 ID 반환
    pub async fn insert(&self, title: &str, content: &str) -> Result<i64, oracle::Error> {
        info!("[Repo] insert 호출: title={}", title);
        let conn = self.pool.conn.lock().await;
        
        let stmt = conn.execute(INSERT_BOARD, &[&title, &content])?;
        debug!("[Repo] INSERT 실행, 영향 받은 행: {}", stmt.row_count()?);
        conn.commit()?;

        let mut rows = conn.query(SELECT_BOARD_SEQ_CURRVAL, &[])?;
        rows.next()
            .transpose()?
            .map_or_else(
                || Err(oracle::Error::new(ErrorKind::Other, "Failed to get CURRVAL after insert")),
                |row| row.get(0)
            )
    }

    /// 게시글 수정
    pub async fn update(&self, id: i64, title: &str, content: &str) -> Result<bool, oracle::Error> {
        info!("[Repo] update 호출: id={}, title={}", id, title);
        let conn = self.pool.conn.lock().await;

        let rows_affected = conn.execute(UPDATE_BOARD, &[&title, &content, &id])?.row_count()?;
        conn.commit()?;

        if rows_affected == 0 {
            warn!("[Repo] 수정할 게시글 없음: id={}", id);
        }
        
        Ok(rows_affected > 0)
    }

    /// 게시글 삭제
    pub async fn delete(&self, id: i64) -> Result<bool, oracle::Error> {
        info!("[Repo] delete 호출: id={}", id);
        let conn = self.pool.conn.lock().await;
        
        let rows_affected = conn.execute(DELETE_BOARD, &[&id])?.row_count()?;
        conn.commit()?;

        if rows_affected == 0 {
            warn!("[Repo] 삭제할 게시글 없음: id={}", id);
        }

        Ok(rows_affected > 0)
    }

    /// DB Row를 Board 구조체로 변환하는 헬퍼 함수
    fn row_to_board(&self, row: Row) -> Result<Board, oracle::Error> {
        Ok(Board {
            id: row.get("ID")?,
            title: row.get::<&str, Option<String>>("TITLE")?.unwrap_or_default(),
            content: row.get::<&str, Option<String>>("CONTENT")?.unwrap_or_default(),
            created_at: row.get("CREATED_AT")?,
        })
    }
}
