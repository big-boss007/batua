# Phase 1: Type Definitions

## Objective
Update backend and frontend types to support dual-unit display.

## Backend (Rust) Changes

### `src/services/ledger/types.rs`
- `BucketBalance`: add `earning_unit_total: f64` alongside existing `displayed`/`spendable` (which will now represent currency_equivalent)
- `WalletBalance`: add `points_balance: f64`, `cash_balance: f64`, `total_redeemable: f64`

### `src/services/admin/types.rs` (or wherever MerchantDashboard lives)
- Dashboard metrics: add points equivalents for `active_credits`, `total_earned`, `total_redeemed`, `total_cod_pending`

### Merchant response type
- Include `points_name`, `points_icon`, `points_to_currency_rate` in merchant API responses (already in DB after migration)

### New: Bucket classification constant
In `src/services/ledger/types.rs`:
```rust
impl BucketType {
    pub fn is_points_bucket(&self) -> bool {
        matches!(self,
            BucketType::EarnedCredit | BucketType::CodPending |
            BucketType::ReferralReward | BucketType::GoodwillCredit |
            BucketType::MembershipBenefit
        )
    }
}
```

### `src/services/ledger/types.rs` — new response for expiring entries
```rust
pub struct ExpiringSoon {
    pub amount: f64,      // earning_unit
    pub currency: f64,    // currency_equivalent
    pub days: i64,        // days until nearest expiry
    pub count: i64,       // number of expiring entries
}
```

## Frontend (TypeScript) Changes

### `frontend/src/lib/client/modules/foundation/utils.ts`
- New function: `formatPoints(value: number, icon: string): string` — "1,400 ★"
- New constant: `POINTS_BUCKETS` and `CASH_BUCKETS` sets
- New helper: `isPointsBucket(bucketType: string): boolean`

### `frontend/src/lib/client/modules/storefront/types.ts`
- `CustomerBalance`: add `points_balance`, `cash_balance`, `total_redeemable`
- `BucketBalance`: add `earning_unit_total` field
- New type: `ExpiringSoon { amount: number, currency: number, days: number, count: number }`
- `StorefrontMerchant`: add `points_name`, `points_icon`, `points_to_currency_rate`

### `frontend/src/lib/client/modules/transactions/types.ts`
- `WalletBalance`: add `points_balance`, `cash_balance`, `total_redeemable`
- `BucketBalance`: add `earning_unit_total`

### `frontend/src/lib/client/modules/customers/types.ts`
- `WalletSummary`: add `points_balance`, `cash_balance`

### `frontend/src/lib/client/modules/admin/types.ts`
- `MerchantDashboard`: add points-denominated fields for each metric
- `Merchant`: add `points_name`, `points_icon`, `points_to_currency_rate`

## Validation
- `cargo check` passes
- `npx svelte-check --threshold error` passes
