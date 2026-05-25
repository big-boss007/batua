# Phase 3: Gift Cards — Overview

**Status:** COMPLETED

## Goal

Build a gift card system that lets merchants issue, bulk-issue, claim, and redeem gift cards. Gift cards use bearer wallets and the ledger for all balance operations, ensuring full auditability and idempotency.

## Scope

### Backend (`src/services/gift_cards/`)
- Single gift card issuance with auto-generated BRZE-XXXX-XXXX-XXXX codes
- Bulk issuance with batch_id for idempotency and CSV-style batches
- Claim flow: transfers balance from bearer wallet to customer wallet
- Redeem flow: deducts from gift card balance against an order
- Lookup by code, list by merchant (paginated), merchant-level stats
- Integration with wallets service (bearer wallets) and ledger service (all movements)

### Frontend (`frontend/src/lib/client/modules/gift-cards/`)
- Gift card list view with status pills
- Single issue form with amount and optional expiry
- Bulk issue form with CSV upload and preview table
- Gift card detail view with usage progress bar
- Gift card stats display

### Database (`20260318000013_gift_cards.sql`)
- `gift_cards` table with issuance tracking, claim state, batch support

## Success Criteria

- [x] Single gift cards can be issued with auto-generated codes
- [x] Bulk issuance supports batch_id idempotency (skip already-issued positions)
- [x] Gift cards can be claimed by customers (transfers to customer wallet)
- [x] Gift cards can be redeemed against orders with balance validation
- [x] Expiry and active-state checks on claim and redeem
- [x] Paginated listing and merchant-level stats
- [x] All movements recorded in the ledger with idempotency keys
- [x] Frontend provides issue, bulk-issue, list, and detail views

## Dependencies

- Phase 0: Foundation (AppState, error handling)
- Phase 1: Wallets (bearer wallet creation, get_or_create_wallet) and Ledger (create_entry, create_across_movement for claim)
