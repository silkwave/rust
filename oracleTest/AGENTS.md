# AGENTS.md - Agentic Coding Guidelines

This document provides guidelines for AI agents working in this repository.

## Project Overview

- **Project name**: oracleTest
- **Language**: Rust
- **Edition**: 2024 (nightly/unstable)
- **Dependencies**: `oracle = "0.6.3"` (Oracle DB driver), `tokio` (async runtime)

## Build, Lint, and Test Commands

### Standard Commands
```bash
# Build the project
cargo build

# Run in release mode
cargo build --release

# Run the application
cargo run
```

### Linting
```bash
# Run clippy linter (recommended before committing)
cargo clippy

# Fix clippy suggestions automatically
cargo clippy --fix

# Check formatting (will show diffs if not formatted)
cargo fmt -- --check

# Auto-format code
cargo fmt
```

### Testing
```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Other Useful Commands
```bash
# Check for security vulnerabilities
cargo audit

# Update dependencies
cargo update

# Show dependency tree
cargo tree
```

## Code Style Guidelines

### General Principles
- Follow standard Rust idioms and conventions
- Use `cargo fmt` for code formatting before committing
- Run `cargo clippy` to catch common mistakes
- Prefer explicit error handling over `unwrap()` (except in tests)

### Naming Conventions
- **Variables/functions**: `snake_case` (e.g., `let conn = ...`, `fn get_data()`)
- **Types/enums**: `PascalCase` (e.g., `struct UserData`, `enum ResultType`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `const MAX_RETRIES: u32 = 3`)
- **Files**: `snake_case.rs` (e.g., `queries.rs`, `user_service.rs`)

### Imports
- Use absolute imports with `crate::` for internal modules
- Group imports: standard library → external crates → internal modules
- Prefer bringing specific items into scope: `use std::fs::File;` not `use std::fs::*;`

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Propagate errors with `?` operator
- Return descriptive error types from libraries when possible
- Example: `fn main() -> Result<(), oracle::Error>`

### Types
- Prefer explicit type annotations for public API functions
- Use generics where appropriate for reusable code
- Avoid `Any` type casts or unsafe code unless absolutely necessary

### Documentation
- Add doc comments (`///`) for public API functions
- Document complex logic with inline comments
- Explain *why* not *what* in comments

### Module Organization
- One module per file, use `mod module_name;` to include
- Use `pub mod` for public modules
- Keep related functionality together
- SQL queries in `src/sql/*.sql` loaded via `include_str!()`

## Current Code Structure

```
oracleTest/
├── Cargo.toml          # Project manifest
├── src/
│   ├── main.rs         # Entry point with Oracle DB operations
│   ├── queries.rs      # SQL query constants
│   └── sql/            # SQL files
│       ├── select_board.sql
│       ├── insert_board.sql
│       ├── update_board.sql
│       └── delete_board.sql
```

## Important Notes

1. **Database Connection**: The code connects to Oracle DB at `127.0.0.1:1521/ORCL` with credentials `docker`/`docker123`. Do not hardcode credentials in production code.

2. **No Tests Exist**: The project currently has 0 tests. Consider adding tests when modifying code.

3. **Edition 2024**: This uses Rust edition 2024 (nightly). Some features may require nightly Rust.

4. **Formatting**: Run `cargo fmt` before committing - current code does not match rustfmt defaults.

## Testing Guidelines

- Write unit tests in the same file using `#[cfg(test)]` modules
- Integration tests go in `tests/` directory
- Use descriptive test names: `#[test] fn test_insert_returns_correct_id()`
- Use `#[should_panic]` for expected panic scenarios
- Keep tests independent and idempotent
