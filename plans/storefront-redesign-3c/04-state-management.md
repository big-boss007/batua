# Phase 4: State Management

## Objective

Store customer name and any derived data needed by the new layout.

## Changes

### In `+page.svelte` (local state):
- Add `customerName: string | null` — set from `lookupCustomer()` result (already returns `CustomerIdentity` with `name`)
- Add `memberSince: string | null` — if available from API, otherwise omit
- Running balance + date groups: computed via `$derived` from `entries` state

### No store changes needed
- `customerPhone` and `merchantContext` stores remain as-is
- Customer name doesn't need to persist across page navigations (re-fetched on phone entry)

## Tasks

- [ ] Capture `customer.name` from `lookupCustomer()` response in page state
- [ ] Add `$derived` for date-grouped entries
- [ ] Add `$derived` for running balance calculation
