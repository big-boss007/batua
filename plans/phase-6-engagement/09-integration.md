# Phase 6: Integration

## No New Services
Phase 6 adds no new service modules. All features are extensions within:
- `src/services/earn/` -- Birthday, milestones, newsletter, profile completion, streaks, spin wheel, memberships
- `src/services/admin/` -- Coalition management and transfers

## Cross-Service Dependencies

### Earn -> Identity
- Birthday bonus calls `identity::storage::get_customers_with_birthday_today` (requires `customers.birthday` column from migration 20260319000001)
- Newsletter signup calls `identity::helpers::validate_email` and `identity::storage::resolve_or_create`/`resolve_by_email`
- Profile completion reads customer fields (name, email, birthday) via `identity::storage::get_customer`

### Earn -> Wallets
- All engagement rewards call `wallets::storage::get_or_create_wallet` before creating ledger entries

### Earn -> Ledger
- All rewards create entries via `ledger::storage::create_entry` with BucketType::EarnedCredit
- Profile completion uses `ledger::storage::entry_exists_by_idempotency_key` to check for prior reward

### Earn -> Rules
- `process_earn` (order processing) calls `rules::helpers::evaluate_rules` which drives the core earn flow. Engagement features are additive -- they do not modify the rule evaluation pipeline.

### Admin -> Ledger
- Coalition transfers use `ledger::storage::create_across_movement` for atomic paired entries
- Coalition transfers check `ledger::storage::get_balance` for sufficient spendable_balance

### Admin -> Wallets
- Coalition transfers look up wallets via `wallets::storage::get_wallet_by_merchant_customer`

## Idempotency Pattern
All engagement features use SHA-256 hashed idempotency keys to prevent duplicate rewards:
- Birthday: `birthday:{sha256(merchant_id + customer_id + "birthday" + date)}`
- Milestone: `milestone:{sha256(merchant_id + customer_id + milestone_id)}`
- Newsletter: `newsletter:{sha256(merchant_id + customer_id + "newsletter_signup")}`
- Profile: `profile_complete:{sha256(merchant_id + customer_id + "profile_complete")}`
- Streak: `streak:{sha256(merchant_id + customer_id + config_id + window_start_date)}`
- Spin: `spin:{customer_id}:{uuid_v4}` (not idempotent by design -- each spin is unique)

## Bucket Types
All engagement rewards use `BucketType::EarnedCredit` except:
- Membership benefits could use `BucketType::MembershipBenefit` (supported in parse_bucket_type)
