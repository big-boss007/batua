# Phase 3: Gift Cards — Storage

**Status:** COMPLETED

## Storage Functions (`src/services/gift_cards/storage.rs`)

All functions instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

| Function | Signature | Purpose |
|----------|-----------|---------|
| `create_gift_card` | `(pool, merchant_id, wallet_id, code, initial_amount, issued_by, issued_by_id, payment_reference, batch_id, batch_position, expires_at) -> Result<GiftCard>` | INSERT with unique violation on code |
| `get_gift_card_by_code` | `(pool, code) -> Result<GiftCard>` | Lookup by code, returns NotFound error if missing |
| `get_gift_card` | `(pool, id) -> Result<GiftCard>` | Lookup by UUID |
| `update_gift_card_amount` | `(pool, id, new_amount) -> Result<GiftCard>` | Updates `current_amount` and `updated_at` |
| `claim_gift_card` | `(pool, id, claimed_by_wallet_id) -> Result<GiftCard>` | Sets `is_claimed=true`, `claimed_by_wallet_id`, `claimed_at=now()` |
| `list_gift_cards` | `(pool, merchant_id, page, limit) -> Result<Vec<GiftCard>>` | Paginated list ordered by `created_at DESC` |
| `get_batch_gift_cards` | `(pool, batch_id) -> Result<Vec<GiftCard>>` | All cards in a batch ordered by `batch_position` |
| `get_gift_card_by_batch_position` | `(pool, batch_id, batch_position) -> Result<Option<GiftCard>>` | Single card by batch+position (for idempotency checks) |
| `get_gift_card_stats` | `(pool, merchant_id) -> Result<GiftCardStats>` | Aggregate stats with FILTER expressions for active/expired/claimed counts |

## Key SQL Patterns

- `RETURNING *` used throughout for consistency
- Stats query uses `COUNT(*) FILTER (WHERE ...)` for conditional aggregation in a single pass
- Pagination via `LIMIT $2 OFFSET $3` with offset computed from page/limit
