# Phase 1: Business Rule Extraction

## Objective
Extract every business rule from the untested helpers.rs files to drive test generation.

## Business Rules Extracted

### BR-001: Bucket Eligibility (redemption/helpers.rs:58-98)
- Spendable <= 0 returns 0 eligible
- No policy → full spendable amount eligible
- Inactive policy → 0 eligible
- Excluded payment method → 0 eligible (case-insensitive match)
- max_per_order_pct caps at `order_amount * pct / 100`
- max_per_order_fixed caps at fixed amount
- Both caps applied: min of both
- Final result floored at 0

### BR-002: Validate Constraints (redemption/helpers.rs:118-176)
- Requested amount <= 0 → error
- Requested > total eligible → error
- Discount codes present + any policy not stackable → error
- Amount < min_redemption → error
- Step size: amount snapped down to nearest step
- Validated amount <= 0 after step snap → error

### BR-003: Compensation Guard (redemption/helpers.rs:407-415)
- Only states Failed, Applied, Committed can be compensated
- Other states → error

### BR-004: Gift Card Issuance (gift_cards/helpers.rs:66-132)
- Amount <= 0 → error
- Code generated in BRZE-XXXX-XXXX-XXXX format
- Bearer wallet created (no customer_id)
- Ledger entry created with GiftCard bucket, In movement

### BR-005: Gift Card Claim (gift_cards/helpers.rs:223-300)
- Already claimed → error
- Not active → error
- Expired → error
- Across movement: Out from bearer wallet, In to customer wallet
- Gift card marked as claimed

### BR-006: Gift Card Redeem (gift_cards/helpers.rs:302-365)
- Amount <= 0 → error
- Not active → error
- Expired → error
- Amount > current_amount → error (insufficient balance)
- Out entry created, gift card amount decremented

### BR-007: Code Generation (gift_cards/helpers.rs:22-44)
- Format: BRZE-XXXX-XXXX-XXXX
- Alphabet: A-H, J-N, P-Z, 2-9 (no I, O, 0, 1 to avoid ambiguity)
- 3 segments of 4 characters each

### BR-008: Earn — Bucket Type Parsing (earn/helpers.rs:703-717)
- 8 valid bucket types with snake_case and PascalCase variants
- Unknown type → error

### BR-009: Earn — Phone Extraction (earn/helpers.rs:434-453)
- Tries customer.phone first, then order.phone
- Empty strings skipped
- No phone found → error

### BR-010: Earn — Multiplier Selection (earn/helpers.rs:100-112)
- Takes max of loyalty_mult and membership_mult
- Only applied when > 1.0
- Applied to earning_unit and currency_equivalent

### BR-011: Loyalty — Tier Finding (loyalty/helpers.rs:133-139)
- Tiers searched in reverse order (highest threshold first)
- First tier where qualifying_value >= threshold wins
- No match → None

### BR-012: Loyalty — Tier Direction (loyalty/helpers.rs:210-225)
- None→Some = upgrade
- Some→None = downgrade
- rank comparison determines upgrade vs downgrade

### BR-013: Spin Wheel (earn/helpers.rs:839-935)
- Inactive wheel → error
- Daily spin limit enforced
- No segments → error
- Zero total weight → error
- Weighted random selection
- Zero reward amount → no ledger entry created
- Spins remaining calculated correctly

## Edge Cases Identified Per Rule

See phase 02 for test generation from these rules.
