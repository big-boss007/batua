# Phase 6: Engagement Features

**Status:** COMPLETED

## Goal

Add engagement and gamification features that extend the earn service: birthday bonuses, milestone rewards, newsletter signup credits, order streaks, spin-the-wheel, paid memberships, and multi-brand coalition transfers.

## Scope

All features are extensions to existing services rather than new services:

### Earn Service Extensions (`src/services/earn/`)
- Birthday bonus -- Credits customers whose birthday matches today
- Milestones -- Configurable thresholds (order_count, lifetime_spend) that award one-time credits
- Newsletter signup -- One-time credit for newsletter subscription
- Profile completion -- Credit for filling all profile fields (name, email, birthday)
- Order streaks -- Configurable order-count-in-window rewards
- Spin-the-wheel -- Probability-weighted gamification with daily spin limits
- Paid memberships -- Plans with earn_rate_multiplier and benefits, subscribe/renew/cancel lifecycle

### Admin Service Extensions (`src/services/admin/`)
- Coalition management -- Multi-brand coalition creation, membership, cross-merchant credit transfers with conversion rates

### Database Migrations
7 new migrations (20260319000001 through 20260319000007) adding tables for each feature.

## Success Criteria
- [x] Birthday bonus processes all matching customers idempotently
- [x] Milestones trigger on order_count or lifetime_spend thresholds, one-time per customer per milestone
- [x] Newsletter signup rewards are idempotent (one per merchant+customer)
- [x] Streak configs define required_orders within window_days; achievements tracked per window
- [x] Spin wheel uses probability-weighted selection with daily limit enforcement
- [x] Membership plans support monthly/annual types with auto-expiry detection
- [x] Coalition transfers validate shared coalition, check balance, apply conversion rate

## Dependencies
- Phase 0: Foundation (ledger, wallets)
- Phase 1: Wallets (get_or_create_wallet)
- Phase 3: Identity (customer resolution, birthday field)
- Phase 5: Admin service (coalition routes integrated into admin router)
