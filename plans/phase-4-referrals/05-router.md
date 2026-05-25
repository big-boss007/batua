# Phase 4: Referrals — Router

**Status:** COMPLETED

## Route Configuration (`src/services/referrals/mod.rs`)

```
POST /referrals/programs                                    -> handler::create_program
GET  /referrals/programs/{merchant_id}                      -> handler::get_program
POST /referrals/codes                                       -> handler::create_code
GET  /referrals/codes/{code}                                -> handler::get_code
GET  /referrals/codes/customer/{merchant_id}/{customer_id}  -> handler::get_customer_code
POST /referrals/convert                                     -> handler::convert_referral
GET  /referrals/analytics/{merchant_id}                     -> handler::get_analytics
GET  /referrals/merchant/{merchant_id}/codes                -> handler::list_merchant_codes
GET  /referrals/conversions/{merchant_id}                   -> handler::list_conversions
```

## Module Exports

- `handler` (private)
- `pub mod helpers` (process_referral, check_fraud, generate_referral_code)
- `pub mod storage` (database operations)
- `pub mod types` (shared types)
