# Phase 2: Loyalty — Database

**Status:** COMPLETED

## Migration: `20260318000012_loyalty_programs.sql`

### Tables

#### `loyalty_programs`
| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `merchant_id` | UUID | NOT NULL, FK merchants(id), UNIQUE |
| `name` | TEXT | NOT NULL |
| `evaluation_criteria` | TEXT | NOT NULL, DEFAULT 'spend' |
| `evaluation_period_days` | INTEGER | nullable (NULL = lifetime) |
| `is_active` | BOOLEAN | NOT NULL, DEFAULT true |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

One program per merchant enforced by UNIQUE on `merchant_id`.

#### `loyalty_tiers`
| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `program_id` | UUID | NOT NULL, FK loyalty_programs(id) |
| `name` | TEXT | NOT NULL |
| `rank` | INTEGER | NOT NULL |
| `threshold` | DOUBLE PRECISION | NOT NULL |
| `earn_rate_multiplier` | DOUBLE PRECISION | NOT NULL, DEFAULT 1.0 |
| `benefits` | JSONB | NOT NULL, DEFAULT '{}' |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

UNIQUE on `(program_id, rank)`. Index on `program_id`.

#### `customer_tiers`
| Column | Type | Constraints |
|--------|------|-------------|
| `id` | UUID | PK, DEFAULT gen_random_uuid() |
| `customer_id` | UUID | NOT NULL, FK customers(id) |
| `merchant_id` | UUID | NOT NULL, FK merchants(id) |
| `tier_id` | UUID | NOT NULL, FK loyalty_tiers(id) |
| `qualifying_value` | DOUBLE PRECISION | NOT NULL, DEFAULT 0 |
| `qualified_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |
| `expires_at` | TIMESTAMPTZ | nullable |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT now() |

UNIQUE on `(customer_id, merchant_id)`. Indexes on `customer_id` and `merchant_id`.
