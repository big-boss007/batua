Development Rules

Rules and conventions followed in this project. All contributors (human and AI) must adhere to these.

---

## 1. Planning Before Code

Every feature or endpoint implementation **must** start with a plan using the Skulls MCP workflow before writing any code.

- Plans live in `plans/{feature-name}/` with numbered phase files (`00-overview.md` through `10-database.md`) and a `checklist.md`
- Unused phases are marked `## Status: SKIPPED` with a reason, not deleted
- The plan separates generated types from hand-written types (see [Type Generation](#3-type-generation))

---

## 2. Service Architecture

Each service is a self-contained module under `src/services/{service_name}/`:

```
src/services/{service_name}/
├── mod.rs          # Router and module exports
├── handler.rs      # HTTP request handlers
├── types.rs        # Request/Response types
├── storage.rs      # Database operations
├── helpers.rs      # Business logic
├── middleware.rs   # Custom middleware (optional)
├── remote.rs       # External API calls (optional)
└── scheduler.rs    # Scheduled tasks (optional)
```

When adding a new service:
1. Create the directory under `src/services/`
2. Declare the module in `src/services/mod.rs`
3. Merge the router in `src/main.rs` → `get_router()`

Shared middleware lives in `src/helper.rs`, not `src/main.rs`.

---

## 3. Type Generation

Types are generated from YAML specs, not hand-written.

- YAML specs live in `types/*.yaml`, indexed by `types/index.yaml`
- Run `make generate-types` to produce Rust structs in `src/generated/types/`
- Plans must explicitly separate **Generated (YAML)** types from **Hand-written (Rust)** types

**When to generate (YAML):** simple Serialize/Deserialize structs without HashMap, sqlx, custom Default, or impl blocks.

**When to hand-write:** types needing HashMap, `sqlx::FromRow`, custom Default, self-referential structs, enum variants with data, or impl blocks. These go in `src/services/{service}/types.rs`.

### Type-crafter number formats

| Spec | Rust type | Use for |
|------|-----------|---------|
| `type: number` | `f64` | prices, scores |
| `type: number, format: float` | `f32` | |
| `type: integer` | `i32` | counts, page, limit |
| `type: integer, format: int64` | `i64` | |

---

## 4. Observability

Every `pub async fn` in `src/services/` **must** have `#[tracing::instrument]`. This is enforced by `tests/lint_tracing.rs` and CI will fail without it.

### Instrumentation rules

| Parameter type | Action |
|---|---|
| `pool: &PgPool` | `skip(pool)` |
| `s3_client: &S3Client` | `skip(s3_client)` |
| `State(app_state)` | `skip(app_state)` |
| Secrets (tokens, keys) | `skip(secret_name)` or `skip_all` |
| Business fields (IDs, URLs) | Keep visible, or add via `fields(...)` |
| Functions returning `Result` | Add `err(Debug)` |

Note: `err(Debug)` only works on functions returning `Result`, not handlers returning `impl IntoResponse`.

---

## 5. No Panics in Service Code

Never use `.unwrap()` or `.expect()` in `src/services/`. Use safe alternatives:

- `?` with proper error conversion
- `let Some(...) = value else { return ... };`
- `.unwrap_or_default()`
- `.unwrap_or_else(|| ...)`
- `match` / `if let`

Even if it "can't fail" logically, use a safe fallback. Panics crash the server.

---

## 6. No Redundant Comments

Do not add inline comments that restate what the code already says. No type annotations in comments. Keep code clean and let it speak for itself.

---

## 7. API Documentation

Every new endpoint **must** have API documentation in the `docs/` directory.

Docs follow a consistent format (see `docs/storefront-home-api.md` for reference):
- Endpoint method and path
- Request body / query params table (field, type, required, default, description)
- Curl examples
- Response JSON examples
- Error cases

Endpoints without docs are considered incomplete.

---

## 8. Git Conventions

### Commit messages

Format: `{JIRA_TICKET}: {type}: {short description}`

Body: bullet points with em dash (`---`) prefix summarizing key changes.

```
BZN-49044: feat: add home page products endpoint

--- POST /storefront/products/home --- products grouped by L0 categories
--- OpenSearch msearch for batched multi-category queries in a single HTTP call
--- Lightweight HomeProduct response (id, title, image, price, stock)
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### Branch naming

Branches are named after Jira tickets: `BZN-{number}-{short-description}`

### Rules

- Never amend a commit without explicit permission
- Never force-push without explicit permission
- Never run destructive git operations (`reset --hard`, `checkout .`, `clean -f`) without asking
- Always create new commits rather than amending by default

---

## 9. Environment & Tooling

- **Language:** Rust (Edition 2024)
- **Framework:** Axum 0.8
- **Database:** PostgreSQL via sqlx, with optional reader replica
- **Cache:** Redis
- **Async runtime:** Tokio
- **Log formats:** `json` (production), `tree` or `pretty` (development) via `LOG_FORMAT` env var
- **OpenTelemetry:** enabled when `OTEL_EXPORTER_OTLP_ENDPOINT` is set

### Key crate notes

- sqlx 0.8 has built-in tracing support; do not add a `"tracing"` feature flag
- OpenTelemetry 0.31: the OTLP feature is `grpc-tonic` (not `tonic`)

---

## 10. Testing

- `cargo test` runs all tests including the tracing lint check
- All `pub async fn` instrumentation is verified by `tests/lint_tracing.rs`
- Run `cargo check` before committing to catch compilation errors early
