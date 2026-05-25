# Phase 4: Referrals — Types

**Status:** COMPLETED

## Backend Types (`src/services/referrals/types.rs`)

### Domain Models

| Type | Derives | Purpose |
|------|---------|---------|
| `ReferralProgram` | `Debug, Clone, Serialize, sqlx::FromRow` | Program config with reward amounts and bucket types |
| `ReferralCode` | `Debug, Clone, Serialize, sqlx::FromRow` | Individual code with vanity/creator flags and stats |
| `ReferralConversion` | `Debug, Clone, Serialize, sqlx::FromRow` | Conversion record with fraud signals and ledger references |

Key `ReferralProgram` fields: `referrer_reward_amount`, `referee_reward_amount`, `referrer_bucket_type`, `referee_bucket_type` (both use `BucketType` from ledger), `max_referrals_per_customer`.

Key `ReferralCode` fields: `is_vanity`, `is_creator`, `commission_rate`, `total_referrals`, `total_conversions`.

Key `ReferralConversion` fields: `referrer_entry_id`, `referee_entry_id` (nullable UUIDs linking to ledger), `referee_ip`, `referee_device_fingerprint`, `is_suspicious`, `fraud_signals` (JSONB).

### Request Types

| Type | Purpose |
|------|---------|
| `CreateProgramRequest` | `merchant_id`, `referrer_reward_amount`, `referee_reward_amount`, `max_referrals_per_customer` |
| `CreateCodeRequest` | `merchant_id`, `customer_id`, `code` (optional vanity), `is_vanity`, `is_creator`, `commission_rate` |
| `ProcessReferralRequest` | `merchant_id`, `referral_code`, `referee_id`, `order_id`, `referee_ip`, `referee_device_fingerprint` |
| `PaginationQuery` | `page`, `limit` (both optional i32) |

### Response Types

| Type | Purpose |
|------|---------|
| `ReferralResponse` | `conversion_id`, `referrer_rewarded`, `referee_rewarded`, `fraud_signals` |
| `FraudCheckResult` | `is_suspicious`, `signals: Vec<String>` |
| `ReferralAnalytics` | `total_codes`, `total_referrals`, `total_conversions`, `total_suspicious`, `conversion_rate` |

## Frontend Types (`frontend/src/lib/client/modules/referrals/types.ts`)

| Type | Purpose |
|------|---------|
| `ReferralProgram` | Program config summary |
| `ReferralCode` | Code with vanity/creator flags and conversion stats |
| `ReferralAnalytics` | Five aggregate metrics |
| `ReferralConversion` | Conversion with fraud signals array |
