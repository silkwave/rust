# Project conventions (Axum + Oracle)

## Architecture

Controller (HTTP request/response) → Service (business logic + validation) → Repository (DB operations + raw SQL).

## Code style

- Names: `snake_case` for vars/fns/modules; `PascalCase` for types.
- Imports: group external → internal.
- Logging: `tracing` and messages like `[Service] ...`, `[Repository] ...`.
- Error handling:
  - Avoid `unwrap()`/`expect()`.
  - Propagate with `?`.
  - Convert errors at boundaries (Repository/DB → ServiceError → ControllerError/HTTP).

## Common file locations

- Router: `src/routes/mod.rs`
- Handlers: `src/controllers/board_controller.rs`
- DTOs: `src/controllers/dto.rs`
- Controller error mapping: `src/controllers/error.rs`
- Service: `src/services/board_service.rs`
- Repository: `src/repositories/board_repository.rs`
- SQL files: `src/sql/*.sql` (loaded via `include_str!`)
- App state: `src/common/app_state.rs`

## Validation rules of thumb

- Validate in Service, not Controller.
- Use `ServiceError::InvalidInput(String)` for 400-class problems.
- Prefer explicit constraints (min/max length, page/size bounds, required fields).

