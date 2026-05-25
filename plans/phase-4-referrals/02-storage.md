# Phase 4: Referrals — Storage

**Status:** COMPLETED

## Storage Functions (`src/services/referrals/storage.rs`)

All functions instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

| Function | Signature | Purpose |
|----------|-----------|---------|
| `create_program` | `(pool, &CreateProgramRequest) -> Result<ReferralProgram>` | INSERT with unique violation (one per merchant) |
| `get_program` | `(pool, merchant_id) -> Result<Option<ReferralProgram>>` | Fetch program by merchant_id |
| `create_referral_code` | `(pool, &CreateCodeRequest, generated_code) -> Result<ReferralCode>` | INSERT code with unique violation on code string |
| `get_referral_code` | `(pool, code) -> Result<ReferralCode>` | Lookup by code string, returns NotFound if missing |
| `get_customer_referral_code` | `(pool, merchant_id, customer_id) -> Result<Option<ReferralCode>>` | Lookup by customer-merchant pair |
| `increment_referral_stats` | `(pool, code_id, is_conversion) -> Result<()>` | Increments total_referrals (always) and total_conversions (if conversion) |
| `create_conversion` | `(pool, merchant_id, referral_code_id, referrer_id, referee_id, order_id, referrer_entry_id, referee_entry_id, referee_ip, referee_device_fingerprint, is_suspicious, fraud_signals) -> Result<ReferralConversion>` | Full conversion record INSERT |
| `get_conversions` | `(pool, merchant_id, page, limit) -> Result<Vec<ReferralConversion>>` | Paginated, ordered by created_at DESC |
| `get_referral_analytics` | `(pool, merchant_id) -> Result<ReferralAnalytics>` | Two queries: code stats (SUM of referrals/conversions) + suspicious count |
| `count_customer_referrals` | `(pool, merchant_id, customer_id) -> Result<i64>` | Count non-suspicious conversions for a customer (for limit enforcement) |
| `list_merchant_referral_codes` | `(pool, merchant_id, page, limit) -> Result<Vec<ReferralCode>>` | Paginated, ordered by total_conversions DESC then created_at DESC |

### Helper Types in Storage

| Type | Purpose |
|------|---------|
| `AnalyticsRow` | `total_codes`, `total_referrals`, `total_conversions` (all Option<i64>) for aggregation query |

## Key SQL Patterns

- Numeric columns cast to `::float8` for Rust f64 compatibility
- `count_customer_referrals` filters out suspicious conversions when checking limits
- Analytics uses separate queries for code stats and suspicious count
- Codes ordered by performance (total_conversions DESC) in merchant listing
