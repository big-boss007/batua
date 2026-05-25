# Phase 1: Planning & Architecture Decisions

Status: COMPLETED

## Module Structure Pattern

Every module follows the same file layout:

```
src/lib/client/modules/<name>/
  index.ts      -- barrel file; the public API of the module
  types.ts      -- type definitions (using `type` keyword only, never `interface`)
  store.ts      -- Svelte stores (writable, derived)
  remote.ts     -- API calls using foundation's apiCaller
  utils.ts      -- (optional) pure helper functions
  ui/
    index.ts    -- component barrel (re-exports default exports from .svelte files)
    *.svelte    -- Svelte 5 components
```

Key rule: consumers import from the barrel (`$lib/client/modules/<name>`) or from `$lib/client/modules/<name>/ui`. They never reach into internal files like `types.ts` or `store.ts` directly.

## Barrel Export Design

Each `index.ts` follows a consistent export grouping:

1. **Types** -- `export type { ... } from './types'` (type-only re-exports)
2. **Remote functions** -- `export { ... } from './remote'`
3. **Store instances** -- `export { ... } from './store'`
4. **Utilities** -- `export { ... } from './utils'` (when present)

The `ui/index.ts` uses `export { default as ComponentName } from './ComponentName.svelte'` pattern.

Utils files are private by default. Only functions needed outside the module are re-exported through the barrel.

## API Client Architecture (foundation/remote.ts)

The `APICaller` class provides typed HTTP methods:

- `get<T>(path, decoder, params?)` -> `APIResult<T>`
- `post<T>(path, body, decoder)` -> `APIResult<T>`
- `put<T>(path, body, decoder)` -> `APIResult<T>`
- `patch<T>(path, body, decoder)` -> `APIResult<T>`
- `delete<T>(path, decoder)` -> `APIResult<T>`

Every method returns `APIResult<T>`, a tagged union:
```ts
type APIResult<T> = APISuccess<T> | APIError
// APISuccess: { tag: 'success', data: T, status: number }
// APIError:   { tag: 'error', message: string, status: number, body: unknown }
```

Each `remote.ts` defines its own decoder functions (e.g., `decodeGiftCard`, `decodeMerchant`) that take `unknown` and return the typed value. This provides a manual type-safety boundary at the API layer.

Base URL comes from `env.PUBLIC_API_BASE_URL` (SvelteKit's `$env/dynamic/public`), falling back to `http://localhost:3000`.

## Theming Strategy

### CSS Custom Properties

All design tokens are defined as CSS custom properties on `:root` in `src/app.css`:

- **Colors**: `--color-bg`, `--color-surface`, `--color-surface-2`, `--color-text`, `--color-text-muted`, `--color-border`, `--color-primary`, `--color-success`, `--color-error`, `--color-warning`, `--color-info`
- **Spacing**: 4px grid from `--space-1` (4px) through `--space-16` (64px)
- **Radii**: `--radius-sm` (4px), `--radius-md` (8px), `--radius-lg` (12px), `--radius-full` (9999px)
- **Shadows**: `--shadow-sm`, `--shadow-md`, `--shadow-lg`
- **Typography**: `--font-sans` (Inter), `--font-mono`, sizes from `--font-size-xs` (12px) to `--font-size-3xl` (30px), weights from normal to bold
- **Z-index layers**: `--z-base` (0) through `--z-toast` (500)
- **Transitions**: `--transition-fast` (150ms), `--transition-base` (200ms), `--transition-slow` (300ms)

### Dark Theme

Dark theme overrides all color tokens under `[data-theme='dark']` selector. The `themeStore` in foundation handles:
- Persisting preference to `localStorage` key `batua-theme`
- Setting `data-theme` attribute on `document.documentElement`
- Toggle and explicit set methods

Storefront layout detects `prefers-color-scheme: dark` media query and applies automatically.

## @juspay/svelte-ui-components Integration

Library components are themed exclusively through CSS custom properties in `app.css`. Components used:

| Library Component | Where Used | CSS Custom Properties |
|---|---|---|
| `Table` | MerchantTable, TransactionTable, RulesList, CampaignsList, EventsTable | `--table-*` (17 variables) |
| `Button` | ShareButtons, forms, actions | `--button-*` (7 variables) |
| `Input` | MerchantTable search, forms | `--input-*` (11 variables) |
| `Select` | MerchantSelector, TransactionFilters | `--select-*` (9 variables) |
| `Pill` | Status badges, tier badges, plan badges | `--pill-*` (5 variables) |
| `Toggle` | Rule active/inactive toggle | `--slider-*`, `--toggle-*` |
| `Progress` | TierProgress, BalanceCard, TierDistributionChart, GiftCardStatus, TierCard | `--progress-*` (6 variables) |
| `Pagination` | TransactionTable | `--pagination-*` (5 variables) |
| `Modal` | Not directly used in read components but themed | `--modal-*` (12 variables) |
| `ThemeSwitcher` | Admin layout, Platform layout | `--theme-switcher-*` (5 variables) |
| `Tabs` | Themed but not spotted in read components | `--tabs-*` (7 variables) |
| `Avatar` | Themed | `--avatar-*` (3 variables) |
| `RelativeTime` | TransactionCard | (no custom properties) |

Variant classes defined in `app.css`:
- Pill: `.pill-success`, `.pill-error`, `.pill-warning`, `.pill-info`, `.pill-neutral`
- Button: `.btn-primary`, `.btn-secondary`, `.btn-danger`, `.btn-ghost`

Dark-theme overrides for pill variants use inverted palettes (dark background, bright text).

## Component Design Patterns

1. **Props via `$props()`** -- all components use Svelte 5 runes, destructured with type annotations
2. **Callbacks as props** -- `onEdit`, `onToggle`, `onPageChange`, `onSubmit`, `onChange`, `onCopy` (never event dispatching)
3. **`$derived` for computed values** -- formatting, filtering, conditional classes
4. **`$derived.by()`** -- for complex derivations with control flow (e.g., status class computation)
5. **Scoped `<style>`** -- all components use scoped styles; no CSS modules or utility classes
6. **Snippets** -- used in Table components via `{#snippet cell()}` and `{#snippet empty()}`
7. **No `$effect`** -- banned per project rules

## Svelte 5 Runes Adoption

- `$state()` for local mutable state
- `$derived()` / `$derived.by()` for computed values
- `$props()` for component inputs
- `$bindable()` for two-way bindings (used sparingly)
- Store subscriptions via `.subscribe()` callback pattern in layout files
