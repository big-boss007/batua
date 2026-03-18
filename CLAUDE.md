# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Batua is a full-stack application with a Rust/Axum backend and SvelteKit frontend.

## Backend (Rust)

### Stack

- Rust (Edition 2024), Axum 0.8, Tokio
- PostgreSQL via sqlx 0.8 (with optional reader replica), Redis
- OpenTelemetry 0.31 (OTLP feature is `grpc-tonic`, not `tonic`)
- sqlx 0.8 has built-in tracing — do not add a `"tracing"` feature flag

### Commands

```bash
cargo check          # Compile check
cargo test           # All tests including tracing lint
make generate-types  # Generate Rust structs from YAML specs
```

### Service Architecture

Each service lives under `src/services/{service_name}/` with: `mod.rs`, `handler.rs`, `types.rs`, `storage.rs`, `helpers.rs`, and optional `middleware.rs`, `remote.rs`, `scheduler.rs`.

When adding a new service: create directory → declare in `src/services/mod.rs` → merge router in `src/main.rs` → `get_router()`. Shared middleware goes in `src/helper.rs`.

### Type Generation

Types are generated from YAML specs in `types/*.yaml` (indexed by `types/index.yaml`) into `src/generated/types/`.

**Generate (YAML):** simple Serialize/Deserialize structs without HashMap, sqlx, custom Default, or impl blocks.
**Hand-write:** types needing HashMap, `sqlx::FromRow`, custom Default, self-referential structs, enum variants with data, or impl blocks → `src/services/{service}/types.rs`.

### Type-Crafter Number Formats

| Spec | Rust type | Use for |
|------|-----------|---------|
| `type: number` | `f64` | prices, scores |
| `type: number, format: float` | `f32` | |
| `type: integer` | `i32` | counts, page, limit |
| `type: integer, format: int64` | `i64` | |

### Instrumentation (Mandatory)

Every `pub async fn` in `src/services/` **must** have `#[tracing::instrument]`. Enforced by `tests/lint_tracing.rs`.

- `skip(pool)`, `skip(s3_client)`, `skip(app_state)` for infra params
- `skip` or `skip_all` for secrets
- `err(Debug)` on functions returning `Result` (not on handlers returning `impl IntoResponse`)

### No Panics in Service Code

Never use `.unwrap()` or `.expect()` in `src/services/`. Use `?`, `let-else`, `.unwrap_or_default()`, `match`, or `if let`.

### API Docs

Every endpoint must have documentation in `docs/` following the format in `docs/storefront-home-api.md`: method/path, param tables, curl examples, response JSON, error cases.

## Frontend (SvelteKit)

### Stack

- SvelteKit with Svelte 5 runes, strict TypeScript (`strict: true`)
- CSS custom properties for theming (no Tailwind)
- `@juspay/svelte-ui-components` component library
- Prettier (single quotes, no trailing commas, print width 100) + ESLint

### Commands

```bash
# format, lint, and type-check before commits
```

### Module Structure

Feature modules live at `src/lib/client/modules/<module-name>/` with: `index.ts` (barrel), `store.ts`, `remote.ts`, `utils.ts`, and `ui/` directory.

- Import modules through their barrel (`index.ts`), never from internal files
- `utils.ts` is private by default — promote to barrel only when needed
- One `remote.ts` per module for all API calls

### TypeScript Rules

- `type` keyword only, never `interface`
- No type assertions (`as`), no type predicates (`x is T`)
- `null` for absence, never `undefined`
- `import type { ... }` for type-only imports

### Svelte Rules

- Use runes: `$state()`, `$derived()`, `$props()`, `$bindable()`
- **`$effect` is banned** — use `$derived`, event callbacks, or `use:` actions
- Callbacks as props (`onSearch`, `onFilterChange`), not dispatched events
- Snippets via `Snippet` type and `{@render}`
- Event handlers: lowercase DOM attributes (`onclick`, `oninput`)
- Data fetching in `+page.ts` load functions, not in components

### State Management

| Tier | Where | When |
|------|-------|------|
| URL state | Query params | Filters, search, sort, pagination — must survive refresh |
| Store state | Svelte stores | App-level shared state (sidebar, toasts, auth) |
| Component state | `$state`/`$derived` | Local UI concerns — dies with component |

### Styling

- Design tokens as CSS custom properties on `:root`
- Light/dark via `[data-theme="dark"]` attribute
- Scoped `<style>` blocks in components; global styles only in root CSS
- Check `@juspay/svelte-ui-components` before building custom components
- Theme library components via CSS custom properties, don't fork them

### Import Order

1. Framework (`svelte`, `$app/*`)
2. Third-party libraries
3. Generated types/decoders (`$generated/*`)
4. Module imports (`$lib/client/modules/*`)
5. Shared components (`$lib/components/*`)
6. Relative imports
7. Assets (SVGs with `?raw`)

## Shared Conventions

### Git

- Commit format: `{JIRA_TICKET}: {type}: {short description}` with em-dash bullet body
- Branch naming: `BZN-{number}-{short-description}`
- Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
- Never amend or force-push without explicit permission

### Planning (Skulls MCP)

Every feature starts with a plan using Skulls MCP before writing code. Plans live in `plans/{feature-name}/` with numbered phase files and `checklist.md`. Unused phases are marked `SKIPPED`, not deleted.

### MCP Servers

- **Skulls MCP** — planning and scaffolding (`init_planning` → `select_language` → `get_template`)
- **Svelte MCP** — framework docs; always run `svelte-autofixer` on Svelte code before finishing
- **Svelte UI Components MCP** — `@juspay/svelte-ui-components` docs; check `list_components` before building custom UI

### No Redundant Comments

Do not add inline comments that restate what the code says. Let code speak for itself.
