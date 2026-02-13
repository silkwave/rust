use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub server_host: String,
    #[serde(default = "default_port")]
    pub server_port: u16,
    #[serde(default = "default_log")]
    pub rust_log: String,
    #[serde(default = "default_db_user")]
    pub db_user: String,
    #[serde(default = "default_db_password")]
    pub db_password: String,
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
