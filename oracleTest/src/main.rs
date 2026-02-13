mod common;
mod config;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use crate::common::app_state::AppState;
use crate::middleware::logging::log_middleware;
use crate::routes::api_routes;
use axum::middleware as axum_middleware;
use config::Config;
use handlers::board_handler::BoardController;
use models::board::{create_connection, create_pool};
use repositories::board_repository::BoardRepository;
use services::board_service::BoardService;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(fmt::layer())
        .init();

    info!("Oracle MVC Board Application 시작");
    info!("서버 설정: {}:{}", config.server_host, config.server_port);

    let conn = create_connection(&config.db_user, &config.db_password, &config.db_connect)?;
    let pool = create_pool(conn);

    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));
    let controller = Arc::new(BoardController::new(service));

    let state = AppState { controller };

    let app = api_routes()
        .layer(axum_middleware::from_fn(log_middleware))
        .with_state(state); // ✅ State는 여기 단 한 번

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("서버 시작: http://{}", addr);

    tokio::select! {
        _ = async {
            axum::serve(listener, app).await.ok();
        } => {}
        _ = tokio::signal::ctrl_c() => {
            info!("서버 종료 중...");
        }
    }

    Ok(())
}
