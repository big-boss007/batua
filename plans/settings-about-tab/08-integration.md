# Phase 8: Integration

## Objective
Register the new tab so it is selectable and deep-linkable.

## Tasks
- Add `'about'` to the `tabIds` const tuple.
- Add `'About'` to the `tabItems` array (last position).
- Confirm `+page.ts` needs no change: it returns the raw `tab` query param, and
  `tabIds.indexOf('about')` resolves the index; an unknown tab still falls back to
  index 0 via `Math.max(0, ...)`.

## Outputs
- Updated `tabIds` / `tabItems` in `+page.svelte`.

## Validation
- The "About" tab appears in the tab bar after "Notifications".
- Selecting it sets `?tab=about`; reloading that URL re-opens the About tab.
- Run `npx svelte-check --threshold error` — passes.
- Browser test: open `/admin/settings?tab=about`, click "Visit Website", confirm the
  bundled site opens in a new tab.
