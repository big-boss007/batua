# Phase 2: Loyalty — Router

**Status:** COMPLETED

## Route Configuration (`src/services/loyalty/mod.rs`)

```
POST /loyalty/programs                              -> handler::create_program
GET  /loyalty/programs/{merchant_id}                -> handler::get_program
POST /loyalty/tiers                                 -> handler::create_tier
GET  /loyalty/programs/{program_id}/tiers           -> handler::get_tiers
GET  /loyalty/customers/{merchant_id}/{customer_id} -> handler::get_customer_tier_info
POST /loyalty/evaluate/{merchant_id}/{customer_id}  -> handler::evaluate_tier
POST /loyalty/programs/{merchant_id}/evaluate       -> handler::evaluate_all_tiers
GET  /loyalty/distribution/{merchant_id}            -> handler::get_tier_distribution
```

## Module Exports

- `handler` (private)
- `pub mod helpers` (used by other services for earn multiplier)
- `pub mod storage` (used by helpers)
- `pub mod types` (used across the service)
