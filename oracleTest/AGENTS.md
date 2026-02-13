# AGENTS.md - Agentic Coding Guidelines

This document provides guidelines for AI agents working in this repository.

## Project Overview

- **Project name**: oracleTest
- **Language**: Rust
- **Edition**: 2024 (nightly/unstable)
- **Core Stack**: Axum (HTTP), Tokio (async), Oracle DB, Tracing (logging)

## Build, Lint, and Test Commands

### Standard Commands
```bash
cargo build              # Build
cargo build --release    # Release
cargo run                # Run server
```

### Linting & Formatting
```bash
cargo fmt                # Format
cargo fmt -- --check    # Check format
cargo clippy             # Lint
cargo clippy --fix      # Auto-fix lint
```

### Testing
```bash
cargo test               # Run tests
cargo test name_here    # Single test
cargo test -- --nocapture  # With output
```

## HTTP API Routes

| Method | Endpoint | Handler |
|--------|----------|---------|
| GET | /boards | list_boards |
| POST | /boards | create_board |
| GET | /boards/{id} | get_board |
| PUT | /boards/{id} | update_board |
| DELETE | /boards/{id} | delete_board |

## Code Style Guidelines

### Naming
- **Variables/functions**: `snake_case`
- **Types/enums**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Files**: `snake_case.rs`

### Imports
- Group: std → external → internal
- Use `crate::` for internal modules

### Module Organization
```
src/
├── main.rs         # Axum server entry
├── config/mod.rs   # Config from env
├── model/mod.rs    # Board, DbPool
├── repository/     # CRUD operations
├── service/        # Business logic
├── controller/     # Request handlers
├── queries.rs      # SQL constants
└── sql/            # *.sql files
```

### Error Handling
- Use `Result<T, E>` with `?`
- Example: `fn main() -> Result<(), Box<dyn std::error::Error>>`

## Important Notes

1. **Config**: Uses `.env` via `dotenv`. See `.env.example`
2. **DB**: Oracle at `127.0.0.1:1521/ORCL` (dev only)
3. **No Tests**: Project has 0 tests
4. **SQL**: In `src/sql/*.sql`, loaded via `include_str!()`
