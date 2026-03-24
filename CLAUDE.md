# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Batua (Breeze Retention Suite) is a full-stack wallet/loyalty/gift-cards/referrals/campaigns/memberships SaaS platform for Indian Shopify D2C merchants. Rust/Axum backend, SvelteKit frontend, PostgreSQL database.

Three interfaces: `/admin/*` (merchant admin), `/platform/*` (super-admin), `/s/{slug}` (customer storefront).

## Backend (Rust)

### Stack

- Rust (Edition 2024), Axum 0.8, Tokio
- PostgreSQL via sqlx 0.8 (with optional reader replica), Redis
- OpenTelemetry 0.31 (OTLP feature is `grpc-tonic`, not `tonic`)
- sqlx 0.8 has built-in tracing — do not add a `"tracing"` feature flag
- Database name: `batua`

### Commands

```bash
cargo check                        # Compile check
cargo test                         # All tests including tracing lint
cargo run                          # Start backend on :3000
make dev                           # Start backend (:3000) + frontend (:5174) together
make reset-and-seed                # Drop/recreate DB, run migrations, seed data
make reset-db                      # Drop/recreate DB + run migrations only
psql -d batua -f migrations/X.sql  # Run a single migration
```

### Service Architecture

Each service lives under `src/services/{service_name}/` with: `mod.rs`, `handler.rs`, `types.rs`, `storage.rs`, `helpers.rs`, and optional `middleware.rs`, `remote.rs`, `scheduler.rs`.

When adding a new service: create directory → declare in `src/services/mod.rs` → merge router in `src/main.rs` → `get_router()`. Shared middleware goes in `src/helper.rs`.

14 services: `admin`, `campaigns`, `cod`, `earn`, `events`, `gift_cards`, `identity`, `ledger`, `loyalty`, `notifications`, `redemption`, `referrals`, `rules`, `wallets`.

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
cd frontend
npm run dev                          # Start dev server on :5173
npm run build                        # Production build
npx svelte-check --threshold error   # Type check (use this to verify before committing)
npm run lint                         # ESLint
npm run format                       # Prettier format
npm run format:check                 # Check formatting without writing
```

### Module Structure

Feature modules live at `src/lib/client/modules/<module-name>/` with: `index.ts` (barrel), `store.ts`, `remote.ts`, `utils.ts`, and `ui/` directory.

11 modules: `foundation`, `admin`, `platform`, `transactions`, `customers`, `gift-cards`, `referrals`, `rules`, `settings`, `analytics`, `storefront`.

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

- Design tokens as CSS custom properties on `:root` in `app.css`
- Light/dark via `[data-theme="dark"]` attribute
- Scoped `<style>` blocks in components; global styles only in root CSS
- **IMPORTANT**: Storefront (`/s/{slug}`) uses hardcoded hex colors (e.g. `#1a1d27`, `#2a2d3a`, `#4ade80`) not CSS variables — this is intentional for the card design

### Component Library — MANDATORY CHECK

**BLOCKING: Before writing ANY UI element, call `list_components` from Svelte UI Components MCP.** If the library has it (Button, Input, Select, Toggle, Pill, Progress, Table, Modal, Tabs, Pagination, Avatar, Tooltip, etc.), use it. Do not build custom versions.

- Call `get_component_docs` for exact prop names, types, and CSS variables
- **CAVEAT**: The Select component's actual TypeScript API (`items: SelectItem[]`, `value: string[]`, `onchange`) differs from MCP docs. Always check `node_modules/@juspay/svelte-ui-components/dist/{Component}/properties.d.ts` for the real types
- Theme via CSS custom properties in `app.css` — global variant classes exist: `btn-primary`, `btn-secondary`, `btn-danger`, `btn-ghost`, `pill-success`, `pill-error`, `pill-warning`, `pill-info`, `pill-neutral`

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

### Planning (Skulls MCP) — MANDATORY

**BLOCKING REQUIREMENT: Do NOT write any code (Edit, Write) for a new feature, refactoring, or multi-file change until a plan exists in `plans/{feature-name}/`.**

Workflow — every step is required:

1. `init_planning` → `select_language` → `get_template` from Skulls MCP
2. Create plan directory: `plans/{feature-name}/`
3. Write all phase files (`00-overview.md` through template phases) + `checklist.md`
4. Show the plan to the user and get explicit confirmation
5. **ONLY THEN** start writing code
6. Track progress against `checklist.md` during implementation

Rules:
- Plans live in `plans/{feature-name}/` with numbered phase files and `checklist.md`
- Unused phases are marked `SKIPPED`, not deleted
- This applies to features, refactors, migrations, and any work touching 3+ files
- Bug fixes touching 1-2 files are exempt
- If you catch yourself writing code without a plan, STOP and create the plan first

### MCP Servers

- **Skulls MCP** — planning and scaffolding (`init_planning` → `select_language` → `get_template`)
- **Svelte MCP** — framework docs; always run `svelte-autofixer` on Svelte code before finishing
- **Svelte UI Components MCP** — `@juspay/svelte-ui-components` docs; check `list_components` before building custom UI

### Testing Before Confirming

Always verify features work end-to-end in the browser (via devtools, screenshots, or network requests) before telling the user it's done. Test interactive behaviors (collapse/expand, toggle, form submit) not just rendering. Check the user's URL/port matches your test server.

### No Redundant Comments

Do not add inline comments that restate what the code says. Let code speak for itself.
