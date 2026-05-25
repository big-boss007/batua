# Phase 4: Referrals — Handlers

**Status:** COMPLETED

## Handler Functions (`src/services/referrals/handler.rs`)

All handlers instrumented with `#[tracing::instrument(skip(app_state))]`.

| Handler | Method | Path | Purpose |
|---------|--------|------|---------|
| `create_program` | POST | `/referrals/programs` | Creates referral program; returns 201 |
| `get_program` | GET | `/referrals/programs/{merchant_id}` | Reads program; returns NotFound if missing |
| `create_code` | POST | `/referrals/codes` | Creates referral code; auto-generates or uses vanity code. For vanity codes, looks up customer name from identity service |
| `get_code` | GET | `/referrals/codes/{code}` | Lookup code by string |
| `get_customer_code` | GET | `/referrals/codes/customer/{merchant_id}/{customer_id}` | Lookup customer's code; uses db_reader |
| `convert_referral` | POST | `/referrals/convert` | Processes referral conversion with fraud detection and rewards |
| `get_analytics` | GET | `/referrals/analytics/{merchant_id}` | Referral analytics summary |
| `list_merchant_codes` | GET | `/referrals/merchant/{merchant_id}/codes` | Paginated code list; uses db_reader; defaults page=1, limit=20, max=100 |
| `list_conversions` | GET | `/referrals/conversions/{merchant_id}` | Paginated conversions; defaults page=1, limit=20, max=100 |

## Code Generation Logic in `create_code`

1. If `req.code` is provided: use it (uppercased)
2. Else if `is_vanity`: lookup customer from identity service, use name-based code generation
3. Else: generate random code

## Notes

- Read operations use `db_reader` where available
- Pagination clamped: page min 1, limit 1..100
- `convert_referral` returns 200 (not 201) since it may not create rewards if suspicious
