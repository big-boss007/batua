# Phase 3: Gift Cards — Helpers

**Status:** COMPLETED

## Helper Functions (`src/services/gift_cards/helpers.rs`)

All public async functions instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

### Code Generation

| Function | Purpose |
|----------|---------|
| `generate_gift_card_code()` | Generates `BRZE-XXXX-XXXX-XXXX` format codes using UUID v4 bytes mapped to alphanumeric charset (excludes ambiguous chars like 0, O, I, 1) |
| `parse_actor_type(s)` | Converts string to `ActorType` enum |
| `parse_expires_at(s)` | Parses ISO 8601 datetime string |

### Issue Flow

`issue_gift_card(pool, req) -> Result<GiftCardResponse>`
1. Validate amount > 0
2. Parse actor_type and optional expires_at
3. Generate unique code
4. Create bearer wallet via `wallet_storage::create_wallet` (with `is_bearer: true`, `bearer_code: Some(code)`)
5. Create ledger entry (BucketType::GiftCard, MovementType::In) with idempotency key `gc-issue-{merchant_id}-{code}`
6. Create gift_card record linking to bearer wallet
7. Return `GiftCardResponse`

### Bulk Issue Flow

`bulk_issue(pool, req) -> Result<BulkIssueResponse>`
1. Iterate cards with position index
2. Validate each amount > 0
3. Check for existing card at (batch_id, batch_position) -- skip if exists (idempotency)
4. For new cards: generate code, create bearer wallet, create ledger entry with key `gc-bulk-{batch_id}-{position}`, create gift_card record
5. Track `total_issued` vs `total_skipped`
6. Return `BulkIssueResponse`

### Claim Flow

`claim_gift_card(pool, req) -> Result<GiftCardResponse>`
1. Lookup gift card by code
2. Validate: not already claimed, is active, not expired
3. Get or create customer wallet via `wallet_storage::get_or_create_wallet`
4. Create paired ledger entries via `create_across_movement`: OUT from bearer wallet, IN to customer wallet
5. Mark gift card as claimed with customer's wallet_id
6. Return updated `GiftCardResponse`

### Redeem Flow

`redeem_gift_card(pool, req) -> Result<GiftCardResponse>`
1. Validate amount > 0
2. Lookup gift card by code
3. Validate: is active, not expired, sufficient balance
4. Determine wallet: `claimed_by_wallet_id` if claimed, else bearer `wallet_id`
5. Create ledger entry (OUT) with idempotency key `gc-redeem-{card_id}-{order_id}`
6. Update `current_amount` to `current_amount - amount`
7. Return updated `GiftCardResponse`
