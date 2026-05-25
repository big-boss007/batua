# Phase 7: Remote / Frontend API Calls

## Objective
Update frontend remote functions and decoders to handle new response fields.

## Changes

### `frontend/src/lib/client/modules/storefront/remote.ts`
- `fetchBalance`: response now includes `points_balance`, `cash_balance`, `total_redeemable`, `expiring_soon`
- `lookupMerchant` / layout load: response now includes `points_name`, `points_icon`, `points_to_currency_rate`
- No new API calls needed — just updated decoders for existing endpoints

### `frontend/src/lib/client/modules/admin/remote.ts`
- `fetchMerchantDashboard`: response includes dual-unit metrics
- `fetchMerchant`: includes points config fields

### `frontend/src/lib/client/modules/transactions/remote.ts`
- `fetchBalance`: updated response shape (same as storefront)

### `frontend/src/lib/client/modules/customers/remote.ts`
- Customer detail wallet summary: includes points/cash split

### `frontend/src/lib/client/modules/settings/remote.ts`
- New: `updatePointsConfig(merchantId, { points_name, points_icon, points_to_currency_rate })` — PUT to merchant endpoint

## Validation
- `npx svelte-check --threshold error` passes
- Manual: load storefront and admin, verify data flows through
