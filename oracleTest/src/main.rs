mod config;
mod controller;
mod model;
mod queries;
mod repository;
mod service;

use axum::{
    extract::{Path, State},
    http::{Request, StatusCode}, // Request 추가
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use config::Config;
use controller::BoardController;
use model::create_pool;
use repository::{BoardRepository, create_connection};
use serde::{Deserialize, Serialize};
use service::BoardService;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info}; // debug 추가
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
struct AppState {
    controller: Arc<BoardController>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoardResponse {
    id: i64,
    title: String,
    content: String,
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateBoardRequest {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBoardRequest {
    title: String,
    content: String,
}

async fn list_boards(State(state): State<AppState>) -> Result<Json<Vec<BoardResponse>>, StatusCode> {
    let boards = state.controller.list_boards_internal().await.map_err(|e| {
        error!("게시글 목록 조회 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let response: Vec<BoardResponse> = boards
        .into_iter()
        .map(|b| BoardResponse {
            id: b.id,
            title: b.title,
            content: b.content,
            created_at: b.created_at.map(|ts| ts.to_string()),
        })
        .collect();
    Ok(Json(response))
}

async fn get_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BoardResponse>, StatusCode> {
    let board = state.controller.get_board_internal(id).await.map_err(|e| {
        error!("게시글 조회 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    match board {
        Some(b) => Ok(Json(BoardResponse {
            id: b.id,
            title: b.title,
            content: b.content,
            created_at: b.created_at.map(|ts| ts.to_string()),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_board(
    State(state): State<AppState>,
    Json(req): Json<CreateBoardRequest>,
) -> Result<Json<BoardResponse>, StatusCode> {
    let id = state
        .controller
        .create_board_internal(&req.title, &req.content)
        .await
        .map_err(|e| {
            error!("게시글 생성 실패: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(BoardResponse {
        id,
        title: req.title,
        content: req.content,
        created_at: None,
    }))
}

async fn update_board(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(req): Json<UpdateBoardRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .controller
        .update_board_internal(id, &req.title, &req.content)
        .await
        .map_err(|e| {
            error!("게시글 수정 실패: {:?}", e);
            match e {
                service::ServiceError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;
    Ok(StatusCode::OK)
}

async fn delete_board(Path(id): Path<i64>, State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    state.controller.delete_board_internal(id).await.map_err(|e| {
        error!("게시글 삭제 실패: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::OK)
}

async fn serve_index() -> Result<Html<String>, StatusCode> {
    tokio::fs::read_to_string("static/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::NOT_FOUND)
}

// HTTP 요청 헤더를 로깅하는 함수
fn on_request_log(request: &Request<axum::body::Body>, _span: &tracing::Span) {
    debug!("--- HTTP 요청 수신 ---");
    debug!("메서드: {:?}", request.method());
    debug!("URI: {:?}", request.uri());
    debug!("버전: {:?}", request.version());
    debug!("헤더:");
    for (key, value) in request.headers() {
        debug!("  {}: {:?}", key, value);
    }
    debug!("--------------------");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(fmt::layer())
        .init();

    info!("Oracle MVC Board Application 시작");
    info!("서버 설정: {}:{}", config.server_host, config.server_port);

    // 이터레이터 예제 시작
    info!("--- Rust 이터레이터 예제 시작 ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    info!("원본 숫자: {:?}", numbers);

    // 1. iter()를 사용하여 컬렉션 순회
    info!("iter()를 사용한 순회:");
    for num in numbers.iter() {
        info!("  현재 숫자: {}", num);
    }

    // 2. map()을 사용하여 각 요소를 변환 (두 배로 만들기)
    let doubled_numbers: Vec<i32> = numbers.iter().map(|&num| num * 2).collect();
    info!("map()을 사용하여 두 배로 만든 숫자: {:?}", doubled_numbers);

    // 3. filter()를 사용하여 특정 조건의 요소만 필터링 (짝수만)
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&num| num % 2 == 0).copied().collect();
    info!("filter()를 사용하여 짝수만 필터링: {:?}", even_numbers);

    // 4. map()과 filter()를 함께 사용하여 변환 및 필터링 후 합계 계산
    let sum_of_even_doubled_numbers: i32 = numbers
        .iter()
        .map(|&num| num * 2) // 각 숫자를 두 배로 만들고
        .filter(|&num| num % 2 == 0) // 그 중에서 짝수만 필터링 (두 배로 만들었으므로 모든 숫자는 짝수가 됨)
        .sum(); // 합계를 계산
    info!(
        "두 배로 만든 짝수의 합계: {:?}",
        sum_of_even_doubled_numbers
    );

    // 5. take()와 skip()을 사용하여 일부 요소 건너뛰고 가져오기
    let skipped_and_taken: Vec<i32> = numbers.iter().skip(3).take(4).copied().collect();
    info!(
        "skip(3)와 take(4)를 사용한 결과: {:?}",
        skipped_and_taken
    );

    info!("--- Rust 이터레이터 예제 종료 ---");
    // 이터레이터 예제 종료

    let conn = create_connection(&config.db_user, &config.db_password, &config.db_connect)?;
    let pool = create_pool(conn);

    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));
    let controller = Arc::new(BoardController::new(service));

    let state = AppState { controller };

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/boards", get(list_boards))
        .route("/boards", post(create_board))
        .route("/boards/{id}", get(get_board))
        .route("/boards/{id}", put(update_board))
        .route("/boards/{id}", delete(delete_board))
        .layer(TraceLayer::new_for_http().on_request(on_request_log)) // on_request_log 추가
        .with_state(state);

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
