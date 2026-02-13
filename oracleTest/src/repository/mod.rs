//! Repository 계층: 데이터베이스 CRUD 작업

use crate::model::{Board, DbPool};
use crate::queries::{DELETE_BOARD, INSERT_BOARD, SELECT_BOARD, UPDATE_BOARD};
use oracle::{Connection, Connector, Row};
use tracing::{debug, error, info};

pub struct BoardRepository {
    pool: DbPool,
}

impl BoardRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Board>, oracle::Error> {
        info!("Fetching all boards from database");
        let conn = self.pool.conn.lock().await;
        let rows = conn.query(SELECT_BOARD, &[])?;

        let mut boards = Vec::new();
        for row_result in rows {
            let row: Row = row_result?;
            let board = self.row_to_board(row)?;
            boards.push(board);
        }
        debug!("Retrieved {} boards", boards.len());
        Ok(boards)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Board>, oracle::Error> {
        info!("Fetching board with id: {}", id);
        let conn = self.pool.conn.lock().await;
        let sql = "SELECT ID, TITLE, CONTENT, CREATED_AT FROM BOARD WHERE ID = :1";
        let mut rows = conn.query(sql, &[&id])?;

        if let Some(row_result) = rows.next() {
            let row: Row = row_result?;
            let board = self.row_to_board(row)?;
            debug!("Found board: id={}", board.id);
            Ok(Some(board))
        } else {
            debug!("Board not found: id={}", id);
            Ok(None)
        }
    }

    pub async fn insert(&self, title: &str, content: &str) -> Result<i64, oracle::Error> {
        info!("Inserting new board: title={}", title);
        let conn = self.pool.conn.lock().await;
        conn.execute(INSERT_BOARD, &[&title, &content])?;
        conn.commit()?;

        let sql = "SELECT BOARD_SEQ.CURRVAL FROM DUAL";
        let mut rows = conn.query(sql, &[])?;
        if let Some(row_result) = rows.next() {
            let id: i64 = row_result?.get(0)?;
            info!("Board inserted successfully: id={}", id);
            Ok(id)
        } else {
            error!("Failed to get inserted board ID");
            Ok(0)
        }
    }

    pub async fn update(&self, id: i64, title: &str, content: &str) -> Result<bool, oracle::Error> {
        info!("Updating board: id={}, title={}", id, title);
        let conn = self.pool.conn.lock().await;
        conn.execute(UPDATE_BOARD, &[&title, &content, &id])?;
        conn.commit()?;
        debug!("Board updated: id={}", id);
        Ok(true)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, oracle::Error> {
        info!("Deleting board: id={}", id);
        let conn = self.pool.conn.lock().await;
        conn.execute(DELETE_BOARD, &[&id])?;
        conn.commit()?;
        debug!("Board deleted: id={}", id);
        Ok(true)
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

pub fn create_connection(
    user: &str,
    password: &str,
    db: &str,
) -> Result<Connection, oracle::Error> {
    Connector::new(user, password, db).connect()
}
