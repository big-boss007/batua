# Phase 4: Referrals — Overview

**Status:** COMPLETED

## Goal

Build a referral system that lets merchants run referral programs with configurable rewards for both referrers and referees, supports vanity and creator/influencer codes, includes fraud detection, and provides conversion analytics.

## Scope

### Backend (`src/services/referrals/`)
- Referral program CRUD (one per merchant, configurable reward amounts)
- Referral code generation (auto-generated, vanity, or creator codes with commission rates)
- Referral conversion processing with dual-reward distribution
- Fraud detection: self-referral, duplicate IP, duplicate device fingerprint, high velocity, new account referee
- Analytics: total codes, referrals, conversions, suspicious count, conversion rate
- Per-customer referral limits
- Paginated listing of codes and conversions

### Frontend (`frontend/src/lib/client/modules/referrals/`)
- Referral program creation form with reward amounts and limits
- Code creation form with vanity/creator toggles
- Referral codes list with type badges and stats
- Conversions list with fraud signal pills
- Analytics card with key metrics

### Database (`20260318000014_referrals.sql`)
- `referral_programs` table (one per merchant)
- `referral_codes` table (per customer, with vanity/creator flags)
- `referral_conversions` table (with fraud signals and ledger entry references)

## Success Criteria

- [x] Merchants can create referral programs with configurable rewards
- [x] Customers get unique referral codes (auto, vanity, or creator)
- [x] Referral conversions reward both referrer and referee via ledger entries
- [x] Fraud detection flags suspicious conversions without rewarding
- [x] Per-customer referral limits enforced
- [x] Analytics provide conversion rates and suspicious counts
- [x] Frontend provides full management UI for programs, codes, and conversions

## Dependencies

- Phase 0: Foundation (AppState, error handling)
- Phase 1: Wallets (get_or_create_wallet for reward distribution) and Ledger (create_entry with BucketType::ReferralReward)
- Identity service (customer lookup for vanity code generation, customer age for fraud checks)
