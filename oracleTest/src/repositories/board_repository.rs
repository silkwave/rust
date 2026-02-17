//! Repository 계층: 데이터베이스 CRUD 작업

use crate::common::queries::{
    DELETE_BOARD, INSERT_BOARD, SELECT_BOARD_BY_ID, SELECT_BOARD_COUNT, SELECT_BOARD_PAGED,
    SELECT_BOARD_SEQ_CURRVAL, UPDATE_BOARD,
};
use crate::models::board::{Board, BoardListItem};
use oracle::Row;
use oracle::sql_type::ToSql;
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use tokio::task::spawn_blocking;
use tracing::{debug, info, warn};

/// 게시판 데이터베이스 접근 객체 (DAO).
/// Oracle 데이터베이스에 대한 CRUD(Create, Read, Update, Delete) 작업을 담당합니다.
pub struct BoardRepository {
    pool: Pool<OracleConnectionManager>,
}

impl BoardRepository {
    /// `r2d2::Error`를 `oracle::Error`로 매핑하는 헬퍼 함수
    fn map_pool_err(err: r2d2::Error) -> oracle::Error {
        oracle::Error::InternalError(err.to_string())
    }

    /// `tokio::task::JoinError`를 `oracle::Error`로 매핑하는 헬퍼 함수
    fn map_join_err(err: tokio::task::JoinError) -> oracle::Error {
        oracle::Error::InternalError(err.to_string())
    }

    /// 새로운 Repository 인스턴스 생성
    pub fn new(pool: Pool<OracleConnectionManager>) -> Self {
        Self { pool }
    }

    /// 전체 게시글 수 조회
    pub async fn count_all(&self) -> Result<u32, oracle::Error> {
        info!("[Repo] count_all 호출");
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;
            debug!("[Repo][SQL] {}", SELECT_BOARD_COUNT.trim());
            // 쿼리 실행 후 첫 번째 행의 첫 번째 컬럼 값을 가져옴
            conn.query_row_as::<u32>(SELECT_BOARD_COUNT, &[])
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// 페이지네이션을 사용하여 게시글 목록 조회
    pub async fn find_paged(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<BoardListItem>, oracle::Error> {
        info!("[Repo] find_paged 호출: offset={}, limit={}", offset, limit);
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;
            let start_row = i64::from(offset);
            let end_row = i64::from(offset.saturating_add(limit));
            let params: [(&str, &dyn ToSql); 2] =
                [("start_row", &start_row), ("end_row", &end_row)];
            debug!("[Repo][SQL] {}", SELECT_BOARD_PAGED.trim());
            debug!("[Repo][BIND] start_row={}, end_row={}", start_row, end_row);
            let rows = conn.query_named(SELECT_BOARD_PAGED, &params)?;

            rows.map(|row_result| Self::row_to_board_list_item(row_result?))
                .collect()
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// ID로 단일 게시글 조회
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Board>, oracle::Error> {
        info!("[Repo] find_by_id 호출: id={}", id);
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;
            let params: [(&str, &dyn ToSql); 1] = [("id", &id)];
            debug!("[Repo][SQL] {}", SELECT_BOARD_BY_ID.trim());
            debug!("[Repo][BIND] id={}", id);
            let mut rows = conn.query_named(SELECT_BOARD_BY_ID, &params)?;
            rows.next()
                .map(|row_result| Self::row_to_board(row_result?))
                .transpose()
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// 새 게시글 추가 후 생성된 ID 반환
    pub async fn insert(&self, title: String, content: String) -> Result<i64, oracle::Error> {
        info!("[Repo] insert 호출: title={}", title); // title은 여기서 사용 후 이동됨
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;

            let params: [(&str, &dyn ToSql); 2] = [("title", &title), ("content", &content)];
            debug!("[Repo][SQL] {}", INSERT_BOARD.trim());
            debug!(
                "[Repo][BIND] title={}, content_len={}",
                title,
                content.chars().count()
            );
            let stmt = conn.execute_named(INSERT_BOARD, &params)?;
            debug!("[Repo] INSERT 실행, 영향 받은 행: {}", stmt.row_count()?);
            // 트랜잭션 커밋
            conn.commit()?;

            debug!("[Repo][SQL] {}", SELECT_BOARD_SEQ_CURRVAL.trim());
            let mut rows = conn.query(SELECT_BOARD_SEQ_CURRVAL, &[])?;
            rows.next().transpose()?.map_or_else(
                || {
                    Err(oracle::Error::InternalError(
                        "Failed to get CURRVAL after insert".to_string(),
                    ))
                },
                |row| row.get::<usize, i64>(0),
            )
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// 게시글 수정
    pub async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
    ) -> Result<bool, oracle::Error> {
        info!("[Repo] update 호출: id={}, title={}", id, title);
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;

            let params: [(&str, &dyn ToSql); 3] =
                [("title", &title), ("content", &content), ("id", &id)];
            debug!("[Repo][SQL] {}", UPDATE_BOARD.trim());
            debug!(
                "[Repo][BIND] id={}, title={}, content_len={}",
                id,
                title,
                content.chars().count()
            );
            let rows_affected = conn.execute_named(UPDATE_BOARD, &params)?.row_count()?;
            // 트랜잭션 커밋
            conn.commit()?;

            if rows_affected == 0 {
                warn!("[Repo] 수정할 게시글 없음: id={}", id);
            }

            Ok(rows_affected > 0)
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// 게시글 삭제
    pub async fn delete(&self, id: i64) -> Result<bool, oracle::Error> {
        info!("[Repo] delete 호출: id={}", id);
        let pool = self.pool.clone();

        spawn_blocking(move || {
            let conn = pool.get().map_err(Self::map_pool_err)?;

            let params: [(&str, &dyn ToSql); 1] = [("id", &id)];
            debug!("[Repo][SQL] {}", DELETE_BOARD.trim());
            debug!("[Repo][BIND] id={}", id);
            let rows_affected = conn.execute_named(DELETE_BOARD, &params)?.row_count()?;
            // 트랜잭션 커밋
            conn.commit()?;

            if rows_affected == 0 {
                warn!("[Repo] 삭제할 게시글 없음: id={}", id);
            }

            Ok(rows_affected > 0)
        })
        .await
        .map_err(Self::map_join_err)?
    }

    /// DB Row를 Board 구조체로 변환하는 헬퍼 함수.
    /// `spawn_blocking` 내부에서 사용하기 위해 `&self` 의존성을 제거했습니다.
    fn row_to_board(row: Row) -> Result<Board, oracle::Error> {
        Ok(Board {
            id: row.get("ID")?,
            title: row
                .get::<&str, Option<String>>("TITLE")?
                .unwrap_or_default(),
            content: row
                .get::<&str, Option<String>>("CONTENT")?
                .unwrap_or_default(),
            created_at: row.get("CREATED_AT")?,
        })
    }

    /// DB Row를 `BoardListItem` 구조체로 변환하는 헬퍼 함수.
    /// `spawn_blocking` 내부에서 사용하기 위해 `&self` 의존성을 제거했습니다.
    fn row_to_board_list_item(row: Row) -> Result<BoardListItem, oracle::Error> {
        Ok(BoardListItem {
            id: row.get("ID")?,
            title: row
                .get::<&str, Option<String>>("TITLE")?
                .unwrap_or_default(),
            content: row
                .get::<&str, Option<String>>("CONTENT")?
                .unwrap_or_default(),
            created_at: row.get("CREATED_AT")?,
        })
    }
}
