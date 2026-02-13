//! Repository 계층: 데이터베이스 CRUD 작업

use crate::common::queries::{DELETE_BOARD, INSERT_BOARD, SELECT_BOARD, UPDATE_BOARD};
use crate::models::board::{Board, DbPool};
use oracle::{ErrorKind, Row}; // ErrorKind 추가
use tracing::{debug, error, info, warn};

pub struct BoardRepository {
    pool: DbPool,
}

impl BoardRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Board>, oracle::Error> {
        info!("[Repository] find_all 호출됨");
        let conn = self.pool.conn.lock().await;
        let rows = conn.query(SELECT_BOARD, &[])?;

        let mut boards = Vec::new();
        for row_result in rows {
            let row: Row = row_result?;
            let board = self.row_to_board(row)?;
            boards.push(board);
        }
        debug!("[Repository] find_all 반환: {}개의 게시글", boards.len());
        Ok(boards)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Board>, oracle::Error> {
        info!("[Repository] find_by_id 호출됨, id={}", id);
        let conn = self.pool.conn.lock().await;
        let sql = "SELECT ID, TITLE, CONTENT, CREATED_AT FROM BOARD WHERE ID = :1";
        let mut rows = conn.query(sql, &[&id])?;

        if let Some(row_result) = rows.next() {
            let row: Row = row_result?;
            let board = self.row_to_board(row)?;
            debug!("[Repository] find_by_id 반환: 게시글 id={} 찾음", board.id);
            Ok(Some(board))
        } else {
            debug!("[Repository] find_by_id 반환: 게시글 id={} 없음", id);
            Ok(None)
        }
    }

    pub async fn insert(&self, title: &str, content: &str) -> Result<i64, oracle::Error> {
        info!("[Repository] insert 호출됨, title={}", title);
        let conn = self.pool.conn.lock().await;
        let stmt = conn.execute(INSERT_BOARD, &[&title, &content])?;
        debug!(
            "[Repository] INSERT 쿼리 실행, 영향 받은 행: {}",
            stmt.row_count()?
        );
        conn.commit()?;

        let sql = "SELECT BOARD_SEQ.CURRVAL FROM DUAL";
        let mut rows = conn.query(sql, &[])?;
        if let Some(row_result) = rows.next() {
            let id: i64 = row_result?.get(0)?;
            info!("[Repository] insert 반환: 게시글 생성 완료 id={}", id);
            Ok(id)
        } else {
            error!("[Repository] ID 조회 실패: BOARD_SEQ.CURRVAL 조회 오류");
            Err(oracle::Error::new(
                ErrorKind::Other, // ErrorKind::Other 사용
                "Failed to get CURRVAL after insert",
            ))
        }
    }

    pub async fn update(&self, id: i64, title: &str, content: &str) -> Result<bool, oracle::Error> {
        info!("[Repository] update 호출됨, id={}, title={}", id, title);
        let conn = self.pool.conn.lock().await;

        // 업데이트 전 게시글 존재 여부 확인
        let check_sql = "SELECT ID FROM BOARD WHERE ID = :1";
        let mut check_rows = conn.query(check_sql, &[&id])?;
        if check_rows.next().is_none() {
            debug!(
                "[Repository] update: 게시글 id={} 업데이트 전 확인 - 존재하지 않음",
                id
            );
            return Ok(false); // 게시글이 존재하지 않으면 업데이트할 필요 없음
        }
        debug!(
            "[Repository] update: 게시글 id={} 업데이트 전 확인 - 존재함",
            id
        );

        let rows_affected = conn
            .execute(UPDATE_BOARD, &[&title, &content, &id])?
            .row_count()?;
        conn.commit()?;
        if rows_affected > 0 {
            debug!("[Repository] update 반환: 게시글 id={} 수정 완료", id);
            Ok(true)
        } else {
            warn!("[Repository] 수정할 게시글을 찾을 수 없음: id={}", id);
            debug!(
                "[Repository] update 반환: 게시글 id={} 수정 실패 (영향 받은 행 없음)",
                id
            );
            Ok(false)
        }
    }

    pub async fn delete(&self, id: i64) -> Result<bool, oracle::Error> {
        info!("[Repository] delete 호출됨, id={}", id);
        let conn = self.pool.conn.lock().await;
        let rows_affected = conn.execute(DELETE_BOARD, &[&id])?.row_count()?;
        conn.commit()?;
        if rows_affected > 0 {
            debug!("[Repository] delete 반환: 게시글 id={} 삭제 완료", id);
            Ok(true)
        } else {
            warn!("[Repository] 삭제할 게시글을 찾을 수 없음: id={}", id);
            debug!(
                "[Repository] delete 반환: 게시글 id={} 삭제 실패 (영향 받은 행 없음)",
                id
            );
            Ok(false)
        }
    }

    fn row_to_board(&self, row: Row) -> Result<Board, oracle::Error> {
        let id: i64 = row.get("ID")?;
        let title: Option<String> = row.get("TITLE")?;
        let content: Option<String> = row.get("CONTENT")?;
        let created_at: Option<oracle::sql_type::Timestamp> = row.get("CREATED_AT")?;

        Ok(Board {
            id,
            title: title.unwrap_or_default(),
            content: content.unwrap_or_default(),
            created_at,
        })
    }
}
