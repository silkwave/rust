//! Model 계층: 데이터 구조체 및 DB 연결 관리

use oracle::{Connection, Connector};
use std::sync::Arc;
use tokio::sync::Mutex;

/// 게시판 데이터 모델
#[derive(Debug, Clone)]
pub struct Board {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<oracle::sql_type::Timestamp>,
}

impl Board {
    #[allow(dead_code)]
    pub fn new(id: i64, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            created_at: None,
        }
    }
}

/// 데이터베이스 연결을 관리하는 구조체
pub struct DbConnection {
    pub conn: Mutex<Connection>,
}

impl DbConnection {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }
}

/// 스레드安全的인 DB 연결 타입
pub type DbPool = Arc<DbConnection>;

/// DB 연결 풀 생성
pub fn create_pool(conn: Connection) -> DbPool {
    Arc::new(DbConnection::new(conn))
}

pub fn create_connection(
    user: &str,
    password: &str,
    db: &str,
) -> Result<Connection, oracle::Error> {
    Connector::new(user, password, db).connect()
}
