# Phase 2: Storage Layer

## Objective
Update balance queries to return both earning_unit and currency_equivalent sums, plus expiring-soon data.

## `src/services/ledger/storage.rs`

### `build_wallet_balance` changes
Currently sums only `earning_unit`. Change to:

1. The SQL aggregate query (`BucketAggregateRow`) needs to return both:
   - `total_earning_units` (SUM of earning_unit)
   - `total_currency` (SUM of currency_equivalent)

2. `build_wallet_balance` constructs:
   - Per bucket: `displayed` and `spendable` (keep as earning_unit for backward compat with balance calc)
   - Per bucket: NEW `earning_unit_total` field
   - NEW aggregate: `points_balance` = sum of earning_unit across points buckets
   - NEW aggregate: `cash_balance` = sum of currency_equivalent across cash buckets (gift_card, customer_funded, refund_credit)
   - NEW aggregate: `total_redeemable` = (points_balance × merchant_rate) + cash_balance

   **Note**: `build_wallet_balance` doesn't have access to merchant rate. Two options:
   - A) Pass rate as parameter (requires callers to look up merchant)
   - B) Return raw points + cash sums, let the handler/frontend multiply

   **Decision: Option B** — return raw values, frontend handles conversion using merchant config it already has. Backend just classifies and sums.

### New query: `get_expiring_soon`
```sql
SELECT
    COALESCE(SUM(earning_unit), 0) AS amount,
    COALESCE(SUM(currency_equivalent), 0) AS currency,
    MIN(expires_at) AS nearest_expiry,
    COUNT(*) AS count
FROM ledger_entries
WHERE wallet_id = $1
  AND state = 'active'
  AND expires_at IS NOT NULL
  AND expires_at > now()
  AND expires_at <= now() + interval '30 days'
  AND movement_type = 'in'
```

## `src/services/admin/storage.rs`
Dashboard stats queries need to return both earning_unit sums and currency_equivalent sums for points-denominated metrics.

## Validation
- `cargo check` passes
- Existing balance tests still pass (`cargo test`)
