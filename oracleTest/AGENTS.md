# AGENTS.md - Developer Knowledge Base

**Last Updated:** 2026-02-17
**Project:** Rust Axum + Oracle Board API

## 1. Quick Reference

| Command | Description |
|---------|-------------|
| `cargo build` | Build debug binary |
| `cargo build --release` | Build optimized binary |
| `cargo run` | Run server (requires `.env` file) |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint code |
| `cargo test` | Run all tests |
| `cargo test <test_name>` | Run single test by name |
| `cargo test -- --nocapture` | Run tests with output |

## 2. Architecture

```
Controller (board_controller.rs) → Service (board_service.rs) → Repository (board_repository.rs)
```

- **Controller**: HTTP request/response handling, extracts query/path params
- **Service**: Business logic, validation, orchestrates repositories
- **Repository**: Database operations, raw SQL execution

## 3. Code Style Guidelines

### Naming Conventions
- **Variables/Functions**: `snake_case` (e.g., `get_board`, `page_size`)
- **Types (Structs/Enums)**: `PascalCase` (e.g., `BoardService`, `ServiceError`)
- **Modules**: `snake_case` (e.g., `board_service`)

### Imports
```rust
// Standard: group by external → internal → parent
use axum::{
    Json,
    extract::{Path, Query, State},
};
use tracing::{debug, info, warn};

use crate::models::board::Board;
use crate::services::board_service::ServiceError;
```

### Error Handling
- Use custom error enums with `From<T>` implementations
- Propagate errors with `?` operator
- Convert at layer boundaries (ServiceError → ControllerError)

```rust
// Service layer - define errors
#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    InvalidInput(String),
    DatabaseError(oracle::Error),
}

impl From<oracle::Error> for ServiceError {
    fn from(err: oracle::Error) -> Self {
        ServiceError::DatabaseError(err)
    }
}
```

### Logging
- Use `tracing` crate
- Format: `[Layer] message` (e.g., `[Service] get_board called, id=5`)
- Levels: `debug!`, `info!`, `warn!`, `error!`

```rust
info!("[Service] create_board called, title={}", title);
debug!("[Repository] query returned {} rows", rows.len());
```

### Async/Await
- All database operations are async
- Use `Arc<T>` for shared state
- Prefer `&self` for methods that don't need ownership

### Validation
- Validate input in Service layer
- Return `ServiceError::InvalidInput(String)` for bad input

## 4. Project Structure

```
src/
├── main.rs              # Entry point, router setup
├── config/mod.rs        # .env loading
├── common/
│   ├── mod.rs
│   ├── app_state.rs     # AppState (Arc<BoardService>)
│   └── queries.rs       # SQL with include_str!
├── models/
│   └── board.rs         # Board, BoardListItem structs
├── controllers/
│   ├── mod.rs           # ControllerError, handler exports
│   ├── board_controller.rs  # HTTP handlers
│   ├── dto.rs           # Request/Response DTOs
│   └── error.rs        # ControllerError impl
├── services/
│   ├── mod.rs
│   └── board_service.rs # Business logic
├── repositories/
│   ├── mod.rs
│   └── board_repository.rs # DB operations
├── routes/mod.rs        # Router configuration
├── middleware/
│   └── logging.rs      # Tower HTTP tracing
└── sql/
    ├── select_board.sql
    ├── insert_board.sql
    ├── update_board.sql
    └── delete_board.sql
static/index.html        # Frontend
.env                     # Environment (required)
```

## 5. Anti-Patterns (NEVER DO)

| Pattern | Forbidden | Use Instead |
|---------|-----------|-------------|
| Type suppression | `as any`, unsafe casts | Proper error handling |
| Error ignoring | `unwrap()`, `expect()` | `?` with proper error types |
| Empty catch | `catch { }` | Log and return error |
| Sync DB in async | blocking calls in async | Always use async Oracle methods |

## 6. Dependencies

Key crates (see `Cargo.toml`):
- **axum 0.7** - Web framework
- **oracle 0.5.8** - Oracle DB driver
- **tokio 1** - Async runtime
- **tracing 0.1** - Logging
- **serde 1** - Serialization
- **dotenv 0.15** - Env vars

**Note:** Edition is `2024` (unstable Rust).

## 7. Environment Variables

Create `.env` before running:
```bash
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=debug
DB_USER=docker
DB_PASSWORD=docker123
DB_CONNECT=127.0.0.1:1521/ORCL
```

## 8. Running Tests

```bash
# All tests
cargo test

# Single test (partial match)
cargo test get_board

# With output
cargo test -- --nocapture

# With logging
RUST_LOG=debug cargo test
```

## 9. Common Tasks

### Adding a new endpoint:
1. Add DTO in `controllers/dto.rs`
2. Add handler in `controllers/board_controller.rs`
3. Add business logic in `services/board_service.rs`
4. Add DB method in `repositories/board_repository.rs`
5. Register route in `routes/mod.rs`

### Adding validation:
1. Add validation method in Service layer
2. Return `ServiceError::InvalidInput` on failure
3. Controller automatically converts to HTTP 400

## 10. API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/boards` | List boards (query: `page`, `size`) |
| GET | `/boards/:id` | Get single board |
| POST | `/boards` | Create board (JSON: `title`, `content`) |
| PUT | `/boards/:id` | Update board |
| DELETE | `/boards/:id` | Delete board |
| GET | `/` | Serve static index.html |
