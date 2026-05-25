# Phase 2: Setup & Configuration

Status: COMPLETED

## SvelteKit Configuration

File: `frontend/svelte.config.js`

```js
adapter: adapter-auto
preprocess: vitePreprocess()
aliases:
  $lib -> src/lib
  $generated -> src/generated
```

The `$lib` alias enables `$lib/client/modules/<name>` imports. The `$generated` alias points to generated types (currently a placeholder at `src/generated/types/index.ts`).

## Vite Configuration

File: `frontend/vite.config.ts`

Minimal config -- just the `sveltekit()` plugin. No custom aliases, proxy, or optimization overrides.

## Path Aliases

| Alias | Target | Purpose |
|---|---|---|
| `$lib` | `src/lib` | Client modules, shared code |
| `$generated` | `src/generated` | Generated types from YAML specs |
| `$app/*` | SvelteKit runtime | `$app/stores`, `$app/navigation`, `$app/environment` |
| `$env/dynamic/public` | SvelteKit env | `PUBLIC_API_BASE_URL` |

## Root Layout

File: `frontend/src/routes/+layout.svelte`

Minimal -- imports `app.css` and renders children via `{@render children()}`. No global providers, no context setup.

## app.css Structure

File: `frontend/src/app.css`

Organized into these sections:

1. **`:root` design tokens** (lines 1-70) -- all spacing, colors, typography, z-index, transitions
2. **`[data-theme='dark']` overrides** (lines 72-85) -- dark color palette
3. **CSS reset** (lines 87-131) -- box-sizing, margins, body font, link styles, img/svg block display
4. **Library component theme overrides** (lines 133-276) -- CSS custom properties for every `@juspay/svelte-ui-components` component (Table, Button, Input, Modal, Pill, Toggle, Tabs, Select, Toast, Pagination, Avatar, Progress, ThemeSwitcher)
5. **Pill variant classes** (lines 278-298) -- `.pill-success`, `.pill-error`, `.pill-warning`, `.pill-info`, `.pill-neutral`
6. **Button variant classes** (lines 300-320) -- `.btn-primary`, `.btn-secondary`, `.btn-danger`, `.btn-ghost`
7. **Dark theme component overrides** (lines 322-356) -- inverted pill colors, dark component backgrounds

## Environment Variables

| Variable | Source | Default | Used In |
|---|---|---|---|
| `PUBLIC_API_BASE_URL` | `$env/dynamic/public` | `http://localhost:3000` | `foundation/remote.ts` -- `buildUrl()` |

## Dependencies (relevant)

- `@sveltejs/kit` -- framework
- `@sveltejs/adapter-auto` -- deployment adapter
- `@sveltejs/vite-plugin-svelte` -- Vite integration
- `@juspay/svelte-ui-components` -- UI component library
- `svelte` (v5) -- with runes support

## Generated Types

File: `frontend/src/generated/types/index.ts`

Currently a placeholder (`export {}`). Types are generated from YAML specs via `make generate-types` in the backend. The frontend does not yet consume generated types -- all types are hand-written in each module's `types.ts`.

## File Count Summary

| Directory | Files |
|---|---|
| `src/routes/` | 24 `+page.svelte`, 4 `+layout.svelte`, 23 `+page.ts`, 3 `+layout.ts` |
| `src/lib/client/modules/` | 11 modules, ~60 `.ts` files, 57 `.svelte` components |
| `src/generated/` | 1 placeholder file |
| Root config | `svelte.config.js`, `vite.config.ts`, `src/app.css` |
