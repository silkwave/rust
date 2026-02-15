# Gemini Context: oracleTest Rust Project

This document provides instructional context about the `oracleTest` project for Gemini.

## Project Overview

This is a Rust-based web service that implements a simple REST API for a message board. It demonstrates a clean, layered architecture (also known as MVC or hexagonal architecture) for building robust web applications.

*   **Framework**: [Axum](https://github.com/tokio-rs/axum) is used as the web framework for handling HTTP requests and routing.
*   **Database**: The project is configured to connect to an [Oracle Database](https://www.oracle.com/database/) using the `oracle` crate.
*   **Asynchronous Runtime**: [Tokio](https://tokio.rs/) is used as the asynchronous runtime.
*   **Architecture**:
    *   **Layered Structure**: The code is organized into distinct layers: `controllers`, `services`, and `repositories`.
    *   **Dependency Injection**: Dependencies are injected from the top down (`main` -> `services` -> `repositories`), making the code modular and testable.
    *   **Clear Separation of Concerns**:
        *   `main.rs`: Handles server initialization, configuration, and wiring of components.
        *   `routes`: Defines the API endpoints and maps them to controller functions.
        *   `controllers`: Handles HTTP request/response logic, data serialization/deserialization, and calls the service layer.
        *   `services`: Contains the core business logic and validation.
        *   `repositories`: Is the only layer that directly interacts with the database.
        *   `models`: Defines the core data structures (e.g., `Board`) and database connection management.
        *   `sql`: All SQL queries are stored in separate `.sql` files and loaded using `include_str!`, keeping them separate from the Rust code.

## Building and Running

### Prerequisites

*   Rust toolchain (`cargo`)
*   An Oracle Database instance.
*   An `.env` file with the following variables:
    ```bash
    # Server configuration
    SERVER_HOST=127.0.0.1
    SERVER_PORT=3000

    # Database configuration
    DB_USER=your_username
    DB_PASSWORD=your_password
    DB_CONNECT=your_db_connect_string # e.g., localhost:1521/XE

    # Logging
    RUST_LOG=info
    ```

### Commands

*   **Build the project:**
    ```bash
    cargo build
    ```

*   **Run the project:**
    ```bash
    cargo run
    ```
    The server will start on the address specified in the `.env` file (e.g., `http://127.0.0.1:3000`).

*   **Run tests:**
    ```bash
    cargo test
    ```

## Development Conventions

*   **Layered Architecture**: Strictly follow the existing layered architecture. Business logic goes in services, database interaction in repositories, and HTTP handling in controllers.
*   **SQL in Files**: All new SQL queries should be added as new files in the `src/sql/` directory and referenced as constants in `src/common/queries.rs`. Do not write SQL queries directly in Rust functions.
*   **Error Handling**: The service layer uses a custom `ServiceError` enum to abstract away database-specific errors from the controller layer. This practice should be continued.
*   **Dependency Injection**: Use the `Arc` and `State` pattern to pass shared resources like the database pool and services down to the handlers.
*   **Logging**: The `tracing` crate is used for logging. Use the structured logging macros (`info!`, `debug!`, `error!`) to provide context.
*   **Configuration**: All configuration is managed via the `Config` struct in `src/config/mod.rs` and loaded from environment variables using `dotenv`.
