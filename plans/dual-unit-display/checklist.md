# Dual Unit Display — Implementation Checklist

## Phase 10: Database
- [ ] Write migration `20260322000001_merchant_points_config.sql`
- [ ] Run migration against dev DB
- [ ] Update seed script with varied points configs
- [ ] Re-seed and verify

## Phase 1: Types
- [ ] `BucketType::is_points_bucket()` method in `src/services/ledger/types.rs`
- [ ] `ExpiringSoon` struct in `src/services/ledger/types.rs`
- [ ] `WalletBalance` — add `points_balance`, `cash_balance`, `total_redeemable`
- [ ] `BucketBalance` — add `earning_unit_total`
- [ ] Merchant types — include `points_name`, `points_icon`, `points_to_currency_rate`
- [ ] Frontend types — `storefront/types.ts`, `transactions/types.ts`, `customers/types.ts`, `admin/types.ts`
- [ ] `cargo check` passes

## Phase 2: Storage
- [ ] Update `BucketAggregateRow` to return both earning_unit and currency_equivalent sums
- [ ] Update `build_wallet_balance` to classify buckets and compute points/cash split
- [ ] Add `get_expiring_soon` query
- [ ] `cargo test` — all existing tests pass

## Phase 3: Helpers
- [ ] No logic changes needed (verified)

## Phase 4: Handlers
- [ ] Balance endpoint returns `expiring_soon`
- [ ] Dashboard endpoint returns dual-unit metrics
- [ ] Merchant endpoint includes points config
- [ ] `cargo test` passes

## Phase 7: Frontend Remote / API
- [ ] Update storefront `fetchBalance` decoder
- [ ] Update storefront merchant loader to include points config
- [ ] Update admin `fetchMerchantDashboard` decoder
- [ ] Add `updatePointsConfig` in settings remote
- [ ] `npx svelte-check --threshold error` passes

## Phase 9A: Frontend Foundation
- [ ] `formatPoints(value, icon)` function in `foundation/utils.ts`
- [ ] Export from `foundation/index.ts`
- [ ] `isPointsBucket()` / `isCashBucket()` helper
- [ ] `POINTS_BUCKETS` / `CASH_BUCKETS` constants

## Phase 9B: Frontend Storefront (Design 5)
- [ ] `BalanceCard.svelte` — full rewrite (stacked sections, 8 states)
- [ ] Remove `StatGrid.svelte`
- [ ] `TransactionCard.svelte` — native unit display
- [ ] `TransactionList.svelte` — pass merchant config
- [ ] `TierCard.svelte` — points_icon instead of "points"
- [ ] `ReferralCard.svelte` — ★ instead of ₹
- [ ] `storefront/types.ts` — new fields
- [ ] `storefront/ui/index.ts` — remove StatGrid export
- [ ] `routes/s/[slug]/+page.svelte` — wire up new props, remove StatGrid
- [ ] `routes/s/[slug]/refer/[code]/+page.svelte` — ★ for reward
- [ ] Visual: verify all 8 states from design doc

## Phase 9C: Frontend Admin (8 areas)
- [ ] Dashboard `+page.svelte` — dual-unit MetricCards
- [ ] Analytics `OverviewCards.svelte` — ★ primary + ₹ sub
- [ ] Analytics `CodMetrics.svelte` — ★ with ₹ hint
- [ ] Analytics `CampaignPerformanceTable.svelte` — dual-unit
- [ ] Transactions `TransactionTable.svelte` — "Points / Cash" column
- [ ] Transactions `BalanceCard.svelte` — 3-col Stars/Cash/Total
- [ ] Customers `CustomerDetail.svelte` — 3-col wallet + tagged buckets
- [ ] Admin transactions `+page.svelte` — detail panel native unit
- [ ] Settings — new Points Configuration section
- [ ] Settings `WalletPolicyForm.svelte` — ★ labels for points buckets
- [ ] Referrals `ReferralProgramForm.svelte` — (₹) → (★)
- [ ] Gift card pages — verify NO changes (all still ₹)
- [ ] Redemption history — verify NO changes (all still ₹)

## Seed Data
- [ ] Update `scripts/seed.sql` with points configs per merchant
- [ ] Re-seed and verify visually

## Verification
- [ ] `cargo check`
- [ ] `cargo test` — all 211+ tests pass
- [ ] `npx svelte-check --threshold error`
- [ ] `make reset-and-seed` works
- [ ] Storefront: test with Chai & Co (rate=0.25) — points display correctly
- [ ] Storefront: test with a 1:1 merchant — backward compatible
- [ ] Admin: dashboard, transactions, customer detail show dual-unit
- [ ] Admin: settings shows points config form
- [ ] Admin: gift card pages unchanged
