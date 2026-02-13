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
    /// Oracle 연결 객체 (스레드 안전을 위해 Mutex로 감쌈)
    pub conn: Mutex<Connection>,
}

impl DbConnection {
    /// 새로운 DbConnection 인스턴스 생성
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }
}

/// 스레드 안전한 DB 연결 타입 (Arc로 공유)
pub type DbPool = Arc<DbConnection>;

/// DB 연결 풀 생성
pub fn create_pool(conn: Connection) -> DbPool {
    Arc::new(DbConnection::new(conn))
}

/// 데이터베이스 연결 생성 함수
pub fn create_connection(
    user: &str,
    password: &str,
    db: &str,
) -> Result<Connection, oracle::Error> {
    Connector::new(user, password, db).connect()
}
