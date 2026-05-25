# Phase 1: Wallets -- COMPLETED

## Goal

Implement the core wallet operations for Batua: wallet lifecycle, credit earning, redemption, and COD hold/release flows. All value movements pass through an immutable ledger governed by the 8 Foundational Truths.

## Scope

### 8 Foundational Truths of the Ledger

1. **Merchant-scoped wallets** -- One wallet per customer per merchant. `UNIQUE(merchant_id, customer_id)` enforced at the database level. Bearer wallets (gift cards before claim) use `bearer_code` instead of `customer_id`.
2. **Atomic triple** -- Every ledger entry records `(wallet_id, bucket_type, earning_unit)` along with `currency_equivalent` and `conversion_rate`. All three value fields are immutable once written.
3. **Four movements** -- `In` (credit), `Out` (debit), `Held` (pending/reserved), `Across` (move between buckets, e.g. COD pending to earned). Enforced by the `movement_type` enum.
4. **Traceable cause** -- Every entry links to `event_id`, `rule_snapshot_id`, `campaign_snapshot_id`, `actor_type`, and `actor_id`. No value appears without a recorded cause.
5. **Double-entry** -- Across movements create paired out+in entries via `transfer_id`. Redemptions create debit entries; compensations reverse them with credit entries.
6. **Non-fungible value (bucket types)** -- 8 bucket types with distinct constraints: `earned_credit`, `cod_pending`, `gift_card`, `customer_funded`, `referral_reward`, `goodwill_credit`, `membership_benefit`, `refund_credit`. Per-bucket policies control redemption caps, step sizes, and payment-method exclusions.
7. **Time first-class (expiry)** -- `expires_at` on every ledger entry. `created_at` immutable. The `credit_state` lifecycle (`active` -> `expired` / `redeemed` / `reversed` / `cancelled`) is the only mutable column. A database trigger (`prevent_ledger_mutation`) blocks mutation of all other columns.
8. **Idempotency** -- Every entry has a `UNIQUE idempotency_key`. Duplicate attempts are rejected at the database level. Keys are deterministic hashes of (event, rule, context) so replays are safe.

### Services Built

| Service | Path | Purpose |
|---------|------|---------|
| wallets | `src/services/wallets/` | Wallet CRUD, lookup, get-or-create |
| earn | `src/services/earn/` | Order cashback, manual credit, birthday bonus, milestones, streaks, newsletter signup, profile completion, spin wheel, memberships |
| redemption | `src/services/redemption/` | Eligibility check, initiate/execute redemption, compensation |
| cod | `src/services/cod/` | COD hold on earn, delivery release, RTO cancellation, COD-to-prepaid incentive |

## Success Criteria

- [x] Wallets are merchant-scoped with unique constraint
- [x] Ledger entries are immutable (DB trigger enforced)
- [x] All four movement types implemented and used
- [x] Every entry has a traceable cause (event/rule/actor)
- [x] All 8 bucket types defined and policy-configurable
- [x] Expiry tracked on entries, credit state lifecycle works
- [x] Idempotency keys prevent duplicate credits
- [x] COD hold/release/RTO cycle works end-to-end
- [x] Redemption state machine (initiated -> validating -> committed -> applied -> completed) with compensation path
- [x] Earn service handles order cashback, manual credit, birthday bonus, milestones, streaks, newsletter signup, profile completion, spin wheel, memberships
