mod logging;
mod models;
mod routes;
mod templates;

use axum::Router;
use logging::{init_tracing, log_server_start};
use routes::user_routes;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // 로깅 시스템 초기화
    init_tracing();
    info!("🛒 전자상거래 Rust 웹 애플리케이션!");

    // CORS 설정
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(user_routes())
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .layer(cors);

    let bind_addr = "0.0.0.0:3000";
    log_server_start(bind_addr);
    info!("📱 접속 주소: http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    info!("✅ TCP 리스너 바인딩 성공");

    if let Err(e) = axum::serve(listener, app).await {
        error!("🔥 서버 실행 실패: {}", e);
        std::process::exit(1);
    }
}
