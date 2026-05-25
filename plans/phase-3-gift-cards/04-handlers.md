# Phase 3: Gift Cards — Handlers

**Status:** COMPLETED

## Handler Functions (`src/services/gift_cards/handler.rs`)

All handlers instrumented with `#[tracing::instrument(skip(app_state))]`.

| Handler | Method | Path | Purpose |
|---------|--------|------|---------|
| `issue_gift_card` | POST | `/gift-cards/issue` | Issues single gift card; returns 201 |
| `bulk_issue` | POST | `/gift-cards/bulk-issue` | Bulk issues gift cards; returns 201 |
| `claim_gift_card` | POST | `/gift-cards/claim` | Claims gift card for customer |
| `redeem_gift_card` | POST | `/gift-cards/redeem` | Redeems amount from gift card |
| `get_gift_card_by_code` | GET | `/gift-cards/{code}` | Lookup by code; uses db_reader |
| `list_gift_cards_for_merchant` | GET | `/gift-cards/merchant/{merchant_id}` | Paginated list; uses db_reader; defaults page=1, limit=20, max=100 |
| `get_gift_card_stats` | GET | `/gift-cards/merchant/{merchant_id}/stats` | Aggregate stats; uses db_reader |

## Notes

- Write operations use primary `app_state.db`
- Read operations use `db_reader` with fallback to primary
- Pagination uses `PaginationQuery` with clamping: page min 1, limit clamped to 1..100
- Handlers return `impl IntoResponse` with `Ok::<_, AppError>(...)` pattern
