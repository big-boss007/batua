# Phase 3: Gift Cards — Integration

**Status:** COMPLETED

## Backend Integration

### Inbound Dependencies (what gift_cards uses)
- **Wallets service**: `create_wallet` for bearer wallets, `get_or_create_wallet` for customer wallets at claim time
- **Ledger service**: `create_entry` for issue/redeem movements, `create_across_movement` for claim (paired OUT/IN)
- **Ledger types**: `ActorType`, `BucketType::GiftCard`, `MovementType`, `NewLedgerEntry`

### Outbound Dependencies (what uses gift_cards)
- Storefront APIs can call issue/claim/redeem endpoints
- Admin dashboard uses list and stats endpoints

### Idempotency Keys
| Operation | Key Format |
|-----------|-----------|
| Single issue | `gc-issue-{merchant_id}-{code}` |
| Bulk issue | `gc-bulk-{batch_id}-{position}` |
| Claim OUT | `gc-claim-out-{card_id}` |
| Claim IN | `gc-claim-in-{card_id}` |
| Redeem | `gc-redeem-{card_id}-{order_id}` |

## Frontend Integration

### Module: `gift-cards/`
- **remote.ts**: API calls for issue, bulk-issue, fetch, lookup by code, claim, redeem, stats
- **store.ts**: `giftCards` store with set, add, addMany, clear operations

### UI Components
| Component | Purpose |
|-----------|---------|
| `GiftCardsList` | Table with code, amounts, status pills (active/claimed/expired/inactive) |
| `IssueGiftCardForm` | Amount input + optional expiry date picker |
| `BulkIssueForm` | CSV file upload with preview table showing parsed rows |
| `GiftCardDetail` | Code display, amount grid, usage progress bar, metadata |

### Component Library Usage
- `Table`, `Pill` for list view
- `Progress` for usage visualization
- `Input`, `Button` for forms
