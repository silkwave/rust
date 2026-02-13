//! 환경 설정 모듈: 애플리케이션 실행에 필요한 설정을 관리

use serde::Deserialize;
use std::env;

/// 애플리케이션 환경 설정 구조체
///
/// `.env` 파일 또는 시스템 환경 변수로부터 설정을 로드합니다.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 서버 호스트 주소 (예: 0.0.0.0)
    #[serde(default = "default_host")]
    pub server_host: String,
    /// 서버 포트 번호 (예: 8080)
    #[serde(default = "default_port")]
    pub server_port: u16,
    /// 로깅 레벨 (예: info, debug)
    #[serde(default = "default_log")]
    pub rust_log: String,
    /// 데이터베이스 사용자명
    #[serde(default = "default_db_user")]
    pub db_user: String,
    /// 데이터베이스 비밀번호
    #[serde(default = "default_db_password")]
    pub db_password: String,
    /// 데이터베이스 접속 문자열 (예: localhost:1521/ORCL)
    #[serde(default = "default_db_connect")]
    pub db_connect: String,
}

fn default_host() -> String {
    env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn default_port() -> u16 {
    env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn default_log() -> String {
    env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string())
}

fn default_db_user() -> String {
    env::var("DB_USER").unwrap_or_else(|_| "docker".to_string())
}

fn default_db_password() -> String {
    env::var("DB_PASSWORD").unwrap_or_else(|_| "docker123".to_string())
}

fn default_db_connect() -> String {
    env::var("DB_CONNECT").unwrap_or_else(|_| "127.0.0.1:1521/ORCL".to_string())
}

impl Config {
    /// 환경 변수에서 설정을 로드하여 Config 인스턴스를 생성합니다.
    ///
    /// 1. `.env` 파일이 있다면 로드합니다.
    /// 2. 환경 변수가 없으면 기본값을 사용합니다.
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            server_host: default_host(),
            server_port: default_port(),
            rust_log: default_log(),
            db_user: default_db_user(),
            db_password: default_db_password(),
            db_connect: default_db_connect(),
        }
    }
}
