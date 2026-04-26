---
name: axum-oracle-board-api
description: Build and maintain a Rust Axum (0.7) REST API backed by Oracle using a layered Controller→Service→Repository architecture. Use when adding or changing endpoints (CRUD), DTOs, validation, pagination, error mapping, tracing logs, SQL files (include_str!), or Oracle async repository methods for the “Board API” style project.
---

# Axum + Oracle Board API

Follow the repo’s conventions: Controller (HTTP) → Service (business/validation) → Repository (DB/SQL).

## Quick workflow (most changes)

1. Locate the layer you need to touch:
   - Route wiring: `src/routes/mod.rs`
   - Handlers: `src/controllers/board_controller.rs`
   - DTOs: `src/controllers/dto.rs`
   - Business logic: `src/services/board_service.rs`
   - DB access + SQL: `src/repositories/board_repository.rs`, `src/sql/*.sql`
2. Make changes in the smallest correct layer:
   - Parse/extract in Controller, validate in Service, execute SQL in Repository.
3. Preserve error boundaries:
   - Repository returns repo/db error type → Service maps to `ServiceError` → Controller maps to `ControllerError`/HTTP.
4. Log with `tracing` using the format `[Layer] message ...` (`info!`, `debug!`, `warn!`, `error!`).
5. Avoid forbidden patterns: no `unwrap()`/`expect()`, no ignored errors.

## Checklists

### Adding an endpoint

1. Add/adjust request/response structs in `src/controllers/dto.rs` (serde).
2. Add handler in `src/controllers/board_controller.rs`:
   - Use `extract::{Path, Query, State}` and `Json`.
   - Keep it thin: call service and convert errors.
3. Add validation + orchestration in `src/services/board_service.rs`:
   - Reject bad input with `ServiceError::InvalidInput(String)`.
4. Add DB method in `src/repositories/board_repository.rs`:
   - Use async Oracle APIs.
   - Put SQL in `src/sql/*.sql` and load via `include_str!`.
5. Register route in `src/routes/mod.rs`.

### Pagination (list endpoints)

- Prefer explicit `page` + `size` query params with sane defaults.
- Validate `page >= 1`, `1 <= size <= max` in Service.
- Return items + page metadata in response DTO (if the project already does this, match existing shape).

## Progressive references (load only if needed)

- Project conventions + layer responsibilities: `skills/axum-oracle-board-api/references/project-conventions.md`
- Endpoint inventory + typical DTOs: `skills/axum-oracle-board-api/references/board-api-surface.md`

