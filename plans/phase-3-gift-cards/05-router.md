# Phase 3: Gift Cards — Router

**Status:** COMPLETED

## Route Configuration (`src/services/gift_cards/mod.rs`)

```
POST /gift-cards/issue                       -> handler::issue_gift_card
POST /gift-cards/bulk-issue                  -> handler::bulk_issue
POST /gift-cards/claim                       -> handler::claim_gift_card
POST /gift-cards/redeem                      -> handler::redeem_gift_card
GET  /gift-cards/{code}                      -> handler::get_gift_card_by_code
GET  /gift-cards/merchant/{merchant_id}      -> handler::list_gift_cards_for_merchant
GET  /gift-cards/merchant/{merchant_id}/stats -> handler::get_gift_card_stats
```

## Module Exports

- `handler` (private)
- `pub mod helpers` (issue, bulk_issue, claim, redeem functions)
- `pub mod storage` (database operations)
- `pub mod types` (shared types)
