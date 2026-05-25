# Phase 4: Handlers

## Objective
Update API handlers to return dual-unit data and merchant points config.

## Changes

### `src/services/ledger/handler.rs` — balance endpoint
- Call `get_expiring_soon` alongside `get_balance`
- Include `expiring_soon` in the response (or as a separate field on WalletBalance)

### `src/services/admin/handler.rs` — dashboard/stats
- Return both earning_unit and currency_equivalent sums for points metrics
- The dashboard queries already aggregate — just add the second SUM column

### Merchant endpoints (admin + storefront)
- `src/services/admin/handler.rs` — merchant response already serializes from DB, will automatically include new columns after migration + type update
- Storefront layout data (`/s/[slug]` load function) — ensure merchant response includes `points_name`, `points_icon`, `points_to_currency_rate`

### Storefront eligibility endpoint
- `src/services/redemption/handler.rs` — `check_eligibility` returns `RedemptionEligibility` which has `total_eligible` (in earning_unit). No change needed — the storefront will convert to ₹ using the merchant rate.

## No new endpoints
All changes are to existing endpoint responses. No new routes needed.

## Validation
- `cargo check` passes
- `cargo test` passes
- Manual: `curl localhost:3000/ledger/{wallet_id}/balance` returns new fields
