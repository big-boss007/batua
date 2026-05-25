# Phase 0: Helpers -- COMPLETED

Business logic functions that orchestrate across storage layers.

## Ledger Helpers

`src/services/ledger/helpers.rs`:
- `generate_idempotency_key` -- SHA256 hash of merchant_id + optional event_id + optional rule_id
- `validate_double_entry` -- Verifies In+Held equals Out within epsilon (double-entry integrity check)

## Wallet Helpers

`src/services/wallets/helpers.rs`:
- `generate_bearer_code` -- Generates 16-character alphanumeric code using UUID bytes as entropy source

## Identity Helpers

`src/services/identity/helpers.rs`:
- `normalize_phone` -- India-first phone normalization: handles 10-digit, 0-prefixed, 91-prefixed, 091-prefixed formats; outputs +91XXXXXXXXXX (E.164); rejects invalid lengths
- `validate_email` -- Basic structural validation (local@domain.tld format)
- Unit tests for all phone normalization variants and email edge cases

## Events Helpers

`src/services/events/helpers.rs`:
- `generate_event_idempotency_key` -- SHA256 of merchant_id:source:external_id
- `parse_shopify_webhook` -- Deserializes raw JSON into ShopifyOrderPayload
- `extract_payment_method` -- Extracts from payment_gateway_names (first) or gateway
- `is_cod_order` -- Checks gateway and payment_gateway_names for "cod"/"cash on delivery"/"cash_on_delivery"

## Earn Helpers

`src/services/earn/helpers.rs` -- The largest helper module, orchestrating the full earn lifecycle:

### Order-Based Earn
- `process_earn` -- Top-level: validates event state, transitions to Processing, calls do_process_earn, marks Processed/Failed
- `do_process_earn` -- Core flow: parse order -> extract customer phone/email/name -> resolve identity -> get/create wallet -> fetch order stats -> build evaluation context -> evaluate rules -> create ledger entries (Held+CodPending for COD, In+EarnedCredit for prepaid) -> track COD orders -> update order stats
- `parse_order_payload`, `extract_customer_phone`, `extract_customer_email`, `extract_customer_name` -- Shopify payload extraction
- `build_evaluation_context` -- Constructs EvaluationContext for rules engine
- `generate_earn_idempotency_key` -- "earn:{event_id}:{rule_snapshot_id}"
- `parse_bucket_type` -- String to BucketType enum mapping

### Manual Credit
- `process_manual_credit` -- Get/create wallet, create In entry with Human actor

### Birthday Bonus
- `process_birthday_bonuses` -- Queries customers with today's birthday via identity storage, creates idempotent credits (SHA256 of merchant+customer+date)

### Newsletter Signup
- `process_newsletter_signup` -- Validates email, resolves customer (by ID, phone, or email), checks for existing signup, creates credit, records signup
- `resolve_newsletter_customer` -- Multi-strategy customer resolution

### Profile Completion
- `process_profile_completion` -- Checks name/email/birthday completeness, awards 30-unit credit when 100% complete, idempotent via SHA256 key

### Milestones
- `check_and_award_milestones` -- Checks order_count and lifetime_spend thresholds, awards rewards for newly crossed milestones

### Streaks
- `check_and_award_streaks` -- Counts recent orders in window, awards streaks when threshold met, deduplicates by window_start

### Spin Wheel
- `create_wheel` -- Creates config + segments
- `spin_wheel` -- Enforces daily limit, weighted random selection, creates credit for non-zero wins, records result

### Memberships
- `subscribe_to_plan` -- Validates plan, checks existing, creates subscription with 30/365-day expiry
- `renew_membership` -- Extends from max(expires_at, now)
- `get_membership_status` -- Auto-expires if past due
- `cancel_membership_by_id` -- Sets status to cancelled

## Redemption Helpers

`src/services/redemption/helpers.rs`:
- `evaluate_eligibility` -- Per-bucket: checks policy active, excluded payment methods, applies max_per_order_pct and max_per_order_fixed caps
- `evaluate_bucket_eligibility` -- Single-bucket cap calculation
- `build_constraint_summary` -- Serializes policy constraints for API response
- `validate_constraints` -- Checks positive amount, not exceeding eligible, stackability with discount codes, minimum redemption, step size rounding
- `execute_redemption` -- Full state machine: Initiated -> Validating -> evaluate eligibility -> validate constraints (or Reject) -> Committed -> create debit entries -> Applied -> apply Shopify discount (stub) -> Completed (or Failed + compensate)
- `create_debit_entries` -- Creates Out entries across eligible buckets until requested amount is covered
- `apply_shopify_discount` -- Stub returning placeholder discount ID
- `compensate` -- Reverses debits by creating In entries with requires_review flag

## COD Helpers

`src/services/cod/helpers.rs`:
- `process_delivery` -- Validates pending state, finds held entry, creates Across movement (Out from CodPending + In to EarnedCredit via create_across_movement), updates COD order state
- `process_rto` -- Validates pending state, finds held entry, creates Out entry to cancel, updates state to rto
- `calculate_cod_incentive` -- max(2% of order, 75 flat), capped at order amount
- `process_cod_to_prepaid` -- UPI-only, creates EarnedCredit entry with calculated incentive
