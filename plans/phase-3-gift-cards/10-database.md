# Phase 3: Gift Cards — Database

**Status:** COMPLETED

## Migration: `20260318000013_gift_cards.sql`

### Table: `gift_cards`

| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `merchant_id` | UUID | NOT NULL, FK merchants(id) |
| `wallet_id` | UUID | NOT NULL, FK wallets(id) |
| `code` | TEXT | NOT NULL, UNIQUE |
| `initial_amount` | DOUBLE PRECISION | NOT NULL |
| `current_amount` | DOUBLE PRECISION | NOT NULL |
| `currency` | TEXT | NOT NULL, DEFAULT 'INR' |
| `issued_by` | actor_type (enum) | NOT NULL |
| `issued_by_id` | TEXT | nullable |
| `payment_reference` | TEXT | nullable |
| `batch_id` | UUID | nullable |
| `batch_position` | INTEGER | nullable |
| `is_claimed` | BOOLEAN | NOT NULL, DEFAULT false |
| `claimed_by_wallet_id` | UUID | nullable, FK wallets(id) |
| `claimed_at` | TIMESTAMPTZ | nullable |
| `expires_at` | TIMESTAMPTZ | nullable |
| `is_active` | BOOLEAN | NOT NULL, DEFAULT true |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

### Indexes

| Index | Columns | Notes |
|-------|---------|-------|
| `idx_gift_cards_merchant` | `merchant_id` | |
| `idx_gift_cards_code` | `code` | For code lookups |
| `idx_gift_cards_wallet` | `wallet_id` | |
| `idx_gift_cards_batch` | `batch_id` | Partial: WHERE batch_id IS NOT NULL |

### Design Notes

- `wallet_id` references the bearer wallet created at issuance time
- `claimed_by_wallet_id` is set when a customer claims the card
- `batch_id` + `batch_position` enable idempotent bulk issuance
- Uses `actor_type` enum from the ledger service for `issued_by`
- `current_amount` is updated on redeem; `initial_amount` is immutable
