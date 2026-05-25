# Phase 2: Loyalty — Storage

**Status:** COMPLETED

## Storage Functions (`src/services/loyalty/storage.rs`)

All functions are instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

| Function | Signature | Purpose |
|----------|-----------|---------|
| `create_program` | `(&PgPool, &CreateProgramRequest) -> Result<LoyaltyProgram>` | INSERT with unique violation handling (one program per merchant) |
| `get_program` | `(&PgPool, merchant_id) -> Result<Option<LoyaltyProgram>>` | Fetch program by merchant_id |
| `get_program_by_id` | `(&PgPool, id) -> Result<Option<LoyaltyProgram>>` | Fetch program by its own UUID |
| `update_program` | `(&PgPool, id, name, criteria, period, is_active) -> Result<LoyaltyProgram>` | Full update with `updated_at = now()` |
| `create_tier` | `(&PgPool, &CreateTierRequest) -> Result<LoyaltyTier>` | INSERT with unique violation on (program_id, rank) |
| `get_tiers` | `(&PgPool, program_id) -> Result<Vec<LoyaltyTier>>` | All tiers for a program, ordered by rank ASC |
| `get_tier_by_id` | `(&PgPool, tier_id) -> Result<Option<LoyaltyTier>>` | Single tier lookup |
| `get_customer_tier` | `(&PgPool, customer_id, merchant_id) -> Result<Option<CustomerTier>>` | Current tier for a customer-merchant pair |
| `upsert_customer_tier` | `(&PgPool, customer_id, merchant_id, tier_id, qualifying_value) -> Result<CustomerTier>` | INSERT or UPDATE via ON CONFLICT |
| `get_tier_distribution` | `(&PgPool, merchant_id) -> Result<Vec<(String, i64)>>` | LEFT JOIN tiers to customer_tiers, grouped by tier name |

## Key SQL Patterns

- Numeric columns cast to `::float8` for Rust f64 compatibility
- `ON CONFLICT (customer_id, merchant_id) DO UPDATE` for tier upsert
- Distribution query uses LEFT JOIN to include tiers with zero customers
