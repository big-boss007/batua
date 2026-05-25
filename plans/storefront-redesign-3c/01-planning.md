# Phase 1: Planning

## Status: COMPLETE

Design concept finalized as Concept 3C (Summary Dashboard). See `plans/storefront-redesign/concept-3c-dashboard.pdf` for the visual reference.

## Design Decisions

1. **No new API endpoints** — derive lifetime savings from transaction history, expiring soon from bucket data
2. **Customer name** — already in `CustomerIdentity.name` from `lookupCustomer()`
3. **Running balance** — calculate client-side by iterating transactions in reverse
4. **Date grouping** — group by calendar date using `created_at` field
5. **Segmented progress** — 6 segments, filled proportionally to tier percentage
6. **Divider-based layout** — replace card backgrounds/shadows with horizontal lines
