# Phase 3: Gift Cards — Checklist

**Status:** COMPLETED

## Backend

- [x] Define types in `src/services/gift_cards/types.rs`
  - [x] `GiftCard` with sqlx::FromRow and ActorType from ledger
  - [x] `IssueGiftCardRequest`, `BulkIssueRequest`, `BulkCardItem`
  - [x] `ClaimGiftCardRequest`, `RedeemGiftCardRequest`, `TransferGiftCardRequest`
  - [x] `GiftCardResponse` with `to_response()` impl
  - [x] `BulkIssueResponse` with issued/skipped counts
  - [x] `GiftCardStats` aggregate type
  - [x] `PaginationQuery`

- [x] Implement storage in `src/services/gift_cards/storage.rs`
  - [x] `create_gift_card` with unique violation handling
  - [x] `get_gift_card_by_code` and `get_gift_card`
  - [x] `update_gift_card_amount`
  - [x] `claim_gift_card` (set claimed state)
  - [x] `list_gift_cards` paginated
  - [x] `get_batch_gift_cards` and `get_gift_card_by_batch_position`
  - [x] `get_gift_card_stats` with FILTER aggregation

- [x] Implement helpers in `src/services/gift_cards/helpers.rs`
  - [x] `generate_gift_card_code` (BRZE-XXXX-XXXX-XXXX format)
  - [x] `parse_actor_type` and `parse_expires_at`
  - [x] `issue_gift_card` with bearer wallet + ledger entry
  - [x] `bulk_issue` with batch idempotency
  - [x] `claim_gift_card` with across-movement ledger entries
  - [x] `redeem_gift_card` with balance validation

- [x] Implement handlers in `src/services/gift_cards/handler.rs`
  - [x] `issue_gift_card` POST
  - [x] `bulk_issue` POST
  - [x] `claim_gift_card` POST
  - [x] `redeem_gift_card` POST
  - [x] `get_gift_card_by_code` GET with db_reader
  - [x] `list_gift_cards_for_merchant` GET paginated with db_reader
  - [x] `get_gift_card_stats` GET with db_reader

- [x] Configure router in `src/services/gift_cards/mod.rs`
- [x] All functions have `#[tracing::instrument]`

## Database

- [x] Migration `20260318000013_gift_cards.sql`
  - [x] `gift_cards` table with full schema
  - [x] Indexes on merchant_id, code, wallet_id, batch_id (partial)

## Frontend

- [x] Types defined in `gift-cards/types.ts`
  - [x] `GiftCard`, `GiftCardStats`
  - [x] `IssueGiftCardForm`, `BulkIssueForm`, `BulkIssueInput`
- [x] API calls in `gift-cards/remote.ts`
  - [x] `issueGiftCard`, `bulkIssue`
  - [x] `fetchGiftCards`, `getGiftCardByCode`
  - [x] `claimGiftCard`, `redeemGiftCard`
  - [x] `fetchGiftCardStats`
- [x] Store in `gift-cards/store.ts`
  - [x] `giftCards` store with set, add, addMany, clear
- [x] Barrel exports in `gift-cards/index.ts`
- [x] UI components
  - [x] `GiftCardsList` with Table and status Pill
  - [x] `IssueGiftCardForm` with amount and expiry
  - [x] `BulkIssueForm` with CSV upload and preview
  - [x] `GiftCardDetail` with usage progress bar
