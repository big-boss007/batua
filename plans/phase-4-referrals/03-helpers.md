# Phase 4: Referrals — Helpers

**Status:** COMPLETED

## Helper Functions (`src/services/referrals/helpers.rs`)

All public async functions instrumented with `#[tracing::instrument(skip(pool), err(Debug))]`.

### Code Generation

| Function | Purpose |
|----------|---------|
| `generate_referral_code(customer_name)` | If name provided and >= 2 usable chars: generates `{NAME_PREFIX}{DIGIT}{DIGIT}` (e.g., "RIYA42"). Otherwise falls back to random 8-char alphanumeric. |
| `generate_random_code()` | 8-char alphanumeric code using UUID bytes mapped to A-Z0-9 charset |

### Fraud Detection

`check_fraud(pool, req, referral_code) -> Result<FraudCheckResult>`

Checks five fraud signals:

| Signal | Condition |
|--------|-----------|
| `self_referral` | referrer customer_id == referee_id |
| `duplicate_ip` | Same IP already used for this referral code |
| `duplicate_device_fingerprint` | Same device fingerprint already used for this code |
| `high_velocity` | 10+ conversions for this code in the last hour |
| `new_account_referee` | Referee account created within 5 minutes |

If any signals are present, `is_suspicious = true` and rewards are withheld.

### Conversion Processing

`process_referral(pool, req) -> Result<ReferralResponse>`

1. Lookup referral code by string; validate is_active and belongs to merchant
2. Run fraud detection
3. Lookup referral program; validate is_active
4. Check per-customer referral limit (only counts non-suspicious conversions)
5. If not suspicious:
   - Get/create referrer's wallet; create ledger entry (BucketType::ReferralReward, MovementType::In) with idempotency key `referral_referrer_{code}_{referee_id}_{order_part}_{wallet_id}`
   - Get/create referee's wallet; create ledger entry with idempotency key `referral_referee_{code}_{referee_id}_{order_part}_{wallet_id}`
6. Create conversion record with fraud_signals and entry references
7. Increment referral stats (referral count always, conversion count only if not suspicious)
8. Return `ReferralResponse` with rewarded flags and signals
