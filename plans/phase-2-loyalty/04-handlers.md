# Phase 2: Loyalty — Handlers

**Status:** COMPLETED

## Handler Functions (`src/services/loyalty/handler.rs`)

All handlers instrumented with `#[tracing::instrument(skip(app_state))]`.

| Handler | Method | Path | Purpose |
|---------|--------|------|---------|
| `create_program` | POST | `/loyalty/programs` | Creates loyalty program; returns 201 |
| `get_program` | GET | `/loyalty/programs/{merchant_id}` | Reads program; uses db_reader |
| `create_tier` | POST | `/loyalty/tiers` | Creates tier; returns 201 |
| `get_tiers` | GET | `/loyalty/programs/{program_id}/tiers` | Lists tiers for program; uses db_reader |
| `get_customer_tier_info` | GET | `/loyalty/customers/{merchant_id}/{customer_id}` | Returns customer's tier info with progress; uses db_reader |
| `evaluate_tier` | POST | `/loyalty/evaluate/{merchant_id}/{customer_id}` | Evaluates single customer's tier |
| `evaluate_all_tiers` | POST | `/loyalty/programs/{merchant_id}/evaluate` | Evaluates all customers for a merchant; returns `{ evaluated: N }` |
| `get_tier_distribution` | GET | `/loyalty/distribution/{merchant_id}` | Returns tier distribution analytics; uses db_reader |

## Notes

- Read operations use `db_reader` (replica) when available, falling back to primary
- `evaluate_all_tiers` queries distinct customer_ids from wallets table and evaluates each sequentially
- Error responses use standard `AppError` variants (NotFound, Conflict, BadRequest)
