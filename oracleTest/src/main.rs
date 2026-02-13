mod controller;
mod model;
mod queries;
mod repository;
mod service;

use controller::BoardController;
use model::create_pool;
use repository::{BoardRepository, create_connection};
use service::BoardService;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const DB_USER: &str = "docker";
const DB_PASSWORD: &str = "docker123";
const DB_CONNECT: &str = "127.0.0.1:1521/ORCL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer())
        .init();

    info!("Starting Oracle MVC Board Application");

    let conn = create_connection(DB_USER, DB_PASSWORD, DB_CONNECT)?;
    let pool = create_pool(conn);

    let repository = Arc::new(BoardRepository::new(pool));
    let service = Arc::new(BoardService::new(repository));
    let controller = Arc::new(BoardController::new(service));

    println!("=== Rust Oracle MVC Board Demo ===");

    controller
        .create_board("First Post", "Hello from Rust MVC!")
        .await;
    controller
        .create_board("Second Post", "Another post with more content.")
        .await;

    controller.list_boards().await;

    controller.get_board(1).await;

    controller
        .update_board(1, "Updated First Post", "Content has been updated!")
        .await;

    controller.list_boards().await;

    controller.delete_board(2).await;

    controller.list_boards().await;

    Ok(())
}
