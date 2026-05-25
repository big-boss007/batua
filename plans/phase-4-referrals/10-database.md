# Phase 4: Referrals — Database

**Status:** COMPLETED

## Migration: `20260318000014_referrals.sql`

### Table: `referral_programs`

| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `merchant_id` | UUID | NOT NULL, FK merchants(id), UNIQUE |
| `referrer_reward_amount` | DOUBLE PRECISION | NOT NULL |
| `referee_reward_amount` | DOUBLE PRECISION | NOT NULL |
| `referrer_bucket_type` | bucket_type (enum) | NOT NULL, DEFAULT 'referral_reward' |
| `referee_bucket_type` | bucket_type (enum) | NOT NULL, DEFAULT 'referral_reward' |
| `max_referrals_per_customer` | INTEGER | nullable |
| `is_active` | BOOLEAN | NOT NULL, DEFAULT true |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

One program per merchant enforced by UNIQUE on `merchant_id`.

### Table: `referral_codes`

| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `merchant_id` | UUID | NOT NULL, FK merchants(id) |
| `customer_id` | UUID | NOT NULL, FK customers(id) |
| `code` | TEXT | NOT NULL, UNIQUE |
| `is_vanity` | BOOLEAN | NOT NULL, DEFAULT false |
| `is_creator` | BOOLEAN | NOT NULL, DEFAULT false |
| `commission_rate` | DOUBLE PRECISION | nullable |
| `total_referrals` | INTEGER | NOT NULL, DEFAULT 0 |
| `total_conversions` | INTEGER | NOT NULL, DEFAULT 0 |
| `is_active` | BOOLEAN | NOT NULL, DEFAULT true |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

### Table: `referral_conversions`

| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `merchant_id` | UUID | NOT NULL, FK merchants(id) |
| `referral_code_id` | UUID | NOT NULL, FK referral_codes(id) |
| `referrer_id` | UUID | NOT NULL, FK customers(id) |
| `referee_id` | UUID | NOT NULL, FK customers(id) |
| `order_id` | TEXT | nullable |
| `referrer_entry_id` | UUID | nullable, FK ledger_entries(id) |
| `referee_entry_id` | UUID | nullable, FK ledger_entries(id) |
| `referee_ip` | TEXT | nullable |
| `referee_device_fingerprint` | TEXT | nullable |
| `is_suspicious` | BOOLEAN | NOT NULL, DEFAULT false |
| `fraud_signals` | JSONB | NOT NULL, DEFAULT '[]' |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

### Indexes

| Index | Table | Columns |
|-------|-------|---------|
| `idx_referral_codes_merchant` | referral_codes | merchant_id |
| `idx_referral_codes_customer` | referral_codes | customer_id |
| `idx_referral_codes_code` | referral_codes | code |
| `idx_referral_conversions_merchant` | referral_conversions | merchant_id |
| `idx_referral_conversions_code` | referral_conversions | referral_code_id |

### Design Notes

- `referrer_entry_id` and `referee_entry_id` are nullable because suspicious conversions do not create ledger entries
- `fraud_signals` stored as JSONB array for flexible signal storage
- `total_referrals` and `total_conversions` denormalized on `referral_codes` for fast analytics
- Uses `bucket_type` enum from the ledger service for reward type configuration
