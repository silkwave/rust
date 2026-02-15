//! 메인 엔트리 포인트: 애플리케이션 초기화 및 서버 실행

//! 메인 엔트리 포인트: 애플리케이션 초기화 및 서버 실행

mod common;
mod config;
mod controllers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use crate::common::app_state::AppState;
use crate::common::utils::current_rss_kb;
use crate::middleware::logging::log_middleware;
use crate::routes::api_routes;
use axum::middleware as axum_middleware;
use config::Config;
use models::board::{create_connection, create_pool};
use repositories::board_repository::BoardRepository;
use services::board_service::BoardService;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 애플리케이션의 진입점
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 환경 설정 로드
    let config = Config::from_env();

    // 2. 로깅 초기화 (tracing-subscriber 사용)
    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(fmt::layer())
        .init();

    info!("Oracle MVC Board Application 시작");
    info!("서버 설정: {}:{}", config.server_host, config.server_port);

    // 3. 데이터베이스 연결 및 풀 생성
    let conn = create_connection(&config.db_user, &config.db_password, &config.db_connect)?;
    let pool = create_pool(conn);

    // 4. 의존성 주입 (Repository -> Service)
    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));

    // 5. 애플리케이션 상태 생성 (Service 공유)
    let state = AppState { service };

    // 6. 라우터 설정 (미들웨어 및 상태 주입)
    let app = api_routes()
        .layer(axum_middleware::from_fn(log_middleware))
        .with_state(state); // ✅ State는 여기 단 한 번

    // 7. 서버 바인딩 및 실행
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("서버 시작: http://{}", addr);

    // 8. 우아한 종료 (Graceful Shutdown) 처리
    tokio::select! {
        _ = async {
            axum::serve(listener, app).await.ok();
        } => {}
        _ = tokio::signal::ctrl_c() => {
            info!("서버 종료 중...");
            info!("종료 시 메모리 사용량: {} KB", current_rss_kb());
        }
    }

    Ok(())
}
