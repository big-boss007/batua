# Phase 4: Referrals — Checklist

**Status:** COMPLETED

## Backend

- [x] Define types in `src/services/referrals/types.rs`
  - [x] `ReferralProgram` with BucketType from ledger
  - [x] `ReferralCode` with vanity/creator flags and stats
  - [x] `ReferralConversion` with fraud signals and ledger entry references
  - [x] `CreateProgramRequest`, `CreateCodeRequest`, `ProcessReferralRequest`
  - [x] `ReferralResponse` with rewarded flags
  - [x] `FraudCheckResult` with signals vector
  - [x] `ReferralAnalytics`
  - [x] `PaginationQuery`

- [x] Implement storage in `src/services/referrals/storage.rs`
  - [x] `create_program` with unique violation handling
  - [x] `get_program` by merchant_id
  - [x] `create_referral_code` with unique violation on code
  - [x] `get_referral_code` by code string
  - [x] `get_customer_referral_code` by merchant+customer
  - [x] `increment_referral_stats` (referrals always, conversions conditionally)
  - [x] `create_conversion` with full fraud/entry fields
  - [x] `get_conversions` paginated
  - [x] `get_referral_analytics` with dual-query aggregation
  - [x] `count_customer_referrals` excluding suspicious
  - [x] `list_merchant_referral_codes` paginated, ordered by performance

- [x] Implement helpers in `src/services/referrals/helpers.rs`
  - [x] `generate_referral_code` with name-based prefix
  - [x] `generate_random_code` 8-char alphanumeric
  - [x] `check_fraud` with 5 signal types
    - [x] self_referral detection
    - [x] duplicate_ip detection
    - [x] duplicate_device_fingerprint detection
    - [x] high_velocity detection (10+ per hour)
    - [x] new_account_referee detection (< 5 min old)
  - [x] `process_referral` with fraud check, reward distribution, conversion recording

- [x] Implement handlers in `src/services/referrals/handler.rs`
  - [x] `create_program` POST
  - [x] `get_program` GET
  - [x] `create_code` POST with vanity/auto generation logic
  - [x] `get_code` GET
  - [x] `get_customer_code` GET with db_reader
  - [x] `convert_referral` POST
  - [x] `get_analytics` GET
  - [x] `list_merchant_codes` GET paginated with db_reader
  - [x] `list_conversions` GET paginated

- [x] Configure router in `src/services/referrals/mod.rs`
- [x] All functions have `#[tracing::instrument]`

## Database

- [x] Migration `20260318000014_referrals.sql`
  - [x] `referral_programs` table with UNIQUE merchant_id
  - [x] `referral_codes` table with UNIQUE code
  - [x] `referral_conversions` table with fraud signal fields
  - [x] Indexes on merchant_id, customer_id, code, referral_code_id

## Frontend

- [x] Types defined in `referrals/types.ts`
  - [x] `ReferralProgram`, `ReferralCode`, `ReferralConversion`, `ReferralAnalytics`
- [x] API calls in `referrals/remote.ts`
  - [x] `fetchProgram`, `createProgram`
  - [x] `createCode`, `fetchCodeByCode`
  - [x] `processConversion`
  - [x] `fetchAnalytics`, `fetchConversions`
  - [x] `fetchMerchantCodes`
- [x] Stores in `referrals/store.ts`
  - [x] `referralProgram` store
  - [x] `referralCodes` store with add/clear
- [x] Barrel exports in `referrals/index.ts`
- [x] UI components
  - [x] `ReferralProgramForm` with reward inputs and limit toggle
  - [x] `CreateCodeForm` with vanity/creator toggles
  - [x] `ReferralCodesList` with type badges and stats table
  - [x] `ReferralAnalyticsCard` with metric grid
  - [x] `ConversionsList` with fraud signal pills and suspicious row highlighting
