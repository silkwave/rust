# Board API surface (typical)

This reference is a quick checklist for CRUD-style changes; always confirm the repo’s existing routes/DTO shapes before implementing.

## Endpoints (common set)

- `GET /boards` (query: `page`, `size`) — list
- `GET /boards/:id` — get one
- `POST /boards` (JSON: `title`, `content`) — create
- `PUT /boards/:id` — update
- `DELETE /boards/:id` — delete
- `GET /` — static index

## “Thin controller” reminder

- Controller:
  - Extract params/body.
  - Call service.
  - Map service errors to HTTP response.
- Service:
  - Validate input, enforce constraints.
  - Decide NotFound vs InvalidInput vs DB error.
- Repository:
  - Execute SQL and map row(s) to model(s).
  - Keep SQL in `src/sql/` and load via `include_str!`.

