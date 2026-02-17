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
use r2d2::Pool;
use r2d2_oracle::OracleConnectionManager;
use repositories::board_repository::BoardRepository;
use services::board_service::BoardService;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 애플리케이션의 진입점
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 환경 설정 로드
    // .env 파일에서 환경 변수를 로드하여 애플리케이션 설정을 초기화합니다.
    let config = Config::from_env();

    // 2. 로깅 초기화 (tracing-subscriber 사용)
    // `tracing-subscriber`를 사용하여 애플리케이션의 로깅 시스템을 설정합니다.
    // `RUST_LOG` 환경 변수를 기반으로 로그 필터를 적용합니다.
    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(fmt::layer())
        .init();

    info!("Oracle MVC Board Application 시작");
    info!("서버 설정: {}:{}", config.server_host, config.server_port);

    // 3. 데이터베이스 연결 및 풀 생성
    // Oracle 데이터베이스에 연결하고 `r2d2` 풀을 사용하여 효율적인 연결 관리를 설정합니다.
    // `max_size`는 최대 동시 연결 수를 정의합니다.
    let manager =
        OracleConnectionManager::new(&config.db_user, &config.db_password, &config.db_connect);
    let pool = Pool::builder()
        .max_size(10) // 최대 연결 수 설정
        .build(manager)?;

    // 4. 의존성 주입 (Repository -> Service)
    // `BoardRepository`와 `BoardService` 인스턴스를 생성하고, `Arc`를 사용하여
    // 여러 스레드에서 공유될 수 있도록 합니다. 서비스 계층은 리포지토리 계층에 의존합니다.
    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));

    // 5. 애플리케이션 상태 생성 (Service 공유)
    // Axum `State`를 통해 애플리케이션 전반에 걸쳐 `BoardService`를 공유할 수 있도록 `AppState`를 생성합니다.
    let state = AppState { service };

    // 6. 라우터 설정 (미들웨어 및 상태 주입)
    // `api_routes` 함수를 호출하여 모든 API 라우트를 정의하고, `log_middleware`를 적용하여
    // 모든 요청에 대한 로깅을 처리합니다. `AppState`를 라우터에 주입하여 핸들러 함수에서
    // 서비스에 접근할 수 있도록 합니다.
    let app = api_routes()
        .layer(axum_middleware::from_fn(log_middleware))
        .with_state(state); // ✅ State는 여기 단 한 번

    // 7. 서버 바인딩 및 실행
    // 설정된 호스트와 포트로 Axum 서버를 바인딩하고 비동기적으로 실행합니다.
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("서버 시작: http://{}", addr);

    // 8. 우아한 종료 (Graceful Shutdown) 처리
    // Ctrl+C 신호(SIGINT)를 감지하여 서버를 안전하게 종료하고, 종료 시점의 메모리 사용량을 기록합니다.
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
