# Phase 3: Gift Cards — Types

**Status:** COMPLETED

## Backend Types (`src/services/gift_cards/types.rs`)

### Domain Models

| Type | Derives | Purpose |
|------|---------|---------|
| `GiftCard` | `Debug, Clone, Serialize, sqlx::FromRow` | Full gift card record with issuance, claim state, and amounts |

Key fields: `code`, `initial_amount`, `current_amount`, `issued_by` (ActorType enum from ledger), `batch_id`/`batch_position` for bulk, `is_claimed`, `claimed_by_wallet_id`, `expires_at`.

### Request Types

| Type | Purpose |
|------|---------|
| `IssueGiftCardRequest` | `merchant_id`, `amount`, `expires_at`, `payment_reference`, `actor_type`, `actor_id` |
| `BulkIssueRequest` | `merchant_id`, `batch_id`, `cards: Vec<BulkCardItem>` |
| `BulkCardItem` | `amount`, `recipient_phone`, `recipient_email` |
| `ClaimGiftCardRequest` | `code`, `customer_id` |
| `RedeemGiftCardRequest` | `code`, `amount`, `order_id` |
| `TransferGiftCardRequest` | `code`, `from_wallet_id`, `to_customer_id` |
| `PaginationQuery` | `page`, `limit` (both optional i32) |

### Response Types

| Type | Purpose |
|------|---------|
| `GiftCardResponse` | Subset: `id`, `code`, `initial_amount`, `current_amount`, `is_claimed`, `is_active`, `expires_at`, `created_at` |
| `BulkIssueResponse` | `batch_id`, `total_issued`, `total_skipped`, `cards: Vec<GiftCardResponse>` |
| `GiftCardStats` | Aggregates: `total_issued`, `total_outstanding_value`, `total_redeemed_value`, `total_expired`, `total_active`, `total_claimed` |

### Impl Blocks

- `GiftCard::to_response()` converts full model to `GiftCardResponse`

## Frontend Types (`frontend/src/lib/client/modules/gift-cards/types.ts`)

| Type | Purpose |
|------|---------|
| `GiftCard` | Card display: id, code, amounts, status, dates |
| `IssueGiftCardForm` | merchant_id, amount, expires_at |
| `BulkIssueForm` | merchant_id, batch_id, cards array |
| `BulkIssueInput` | cards array only (merchant/batch added at call site) |
| `GiftCardStats` | Six aggregate metrics |
