# Phase 0: Middleware -- COMPLETED

Middleware is applied in `src/main.rs` as tower layers on the router, and shared utilities live in `src/helper.rs`.

## Applied Layers (in order, outermost first)

1. **TraceLayer** (`tower_http::trace::TraceLayer::new_for_http()`) -- HTTP request/response tracing integrated with the tracing subscriber
2. **CorsLayer** (`tower_http::cors::CorsLayer::permissive()`) -- Permissive CORS for development; allows all origins, methods, and headers
3. **SetRequestIdLayer** (`src/helper.rs`) -- Generates a UUID v4 `x-request-id` header on every incoming request
4. **PropagateRequestIdLayer** (`src/helper.rs`) -- Copies the `x-request-id` from the request to the response

## Request ID Generator

`src/helper.rs`:
- `RequestIdGenerator` implements `MakeRequestId` from tower-http
- Generates UUID v4 for each request
- `set_request_id_layer()` and `propagate_request_id_layer()` factory functions

## Structured Logging

Configured in `main()`:
- `LOG_FORMAT=json` -- JSON-formatted logs (production)
- `LOG_FORMAT=pretty` (default) -- Human-readable pretty logs (development)
- `RUST_LOG` / `EnvFilter` -- Controls log level, defaults to `info`
- Uses `tracing_subscriber::registry()` with `EnvFilter` + `fmt::layer()`

## Tracing Instrumentation

All `pub async fn` in `src/services/` are annotated with `#[tracing::instrument]`:
- Handlers: `skip(app_state)`
- Storage functions: `skip(pool), err(Debug)`
- Helpers: `skip(pool)` or `skip(pool, event)` as appropriate, `err(Debug)` on Result-returning functions
- Enforced by `tests/lint_tracing.rs`

## Error Handling

`src/error.rs` -- `AppError` implements `IntoResponse`:
- Database/Redis errors -> 500 Internal Server Error
- NotFound -> 404
- BadRequest -> 400
- Conflict -> 409
- Unauthorized -> 401
- All responses use `{"error": "message"}` JSON format
