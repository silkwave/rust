mod config;
mod controller;
mod model;
mod queries;
mod repository;
mod service;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
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
use tracing::{error, info};
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
        error!("Failed to list boards: {:?}", e);
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
        error!("Failed to get board: {:?}", e);
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
            error!("Failed to create board: {:?}", e);
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
            error!("Failed to update board: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::OK)
}

async fn delete_board(Path(id): Path<i64>, State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    state.controller.delete_board_internal(id).await.map_err(|e| {
        error!("Failed to delete board: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(fmt::layer())
        .init();

    info!("Starting Oracle MVC Board Application");
    info!("Server config: {}:{}", config.server_host, config.server_port);

    let conn = create_connection(&config.db_user, &config.db_password, &config.db_connect)?;
    let pool = create_pool(conn);

    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));
    let controller = Arc::new(BoardController::new(service));

    let state = AppState { controller };

    let app = Router::new()
        .route("/boards", get(list_boards))
        .route("/boards", post(create_board))
        .route("/boards/{id}", get(get_board))
        .route("/boards/{id}", put(update_board))
        .route("/boards/{id}", delete(delete_board))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
