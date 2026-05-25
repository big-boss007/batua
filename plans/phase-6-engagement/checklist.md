# Phase 6: Checklist

## Birthday Bonus
- [x] Migration: `customers.birthday` column added (20260319000001)
- [x] Handler: `birthday_bonus` POST endpoint
- [x] Helper: `process_birthday_bonuses` with SHA-256 idempotency
- [x] Integration: Uses `identity::storage::get_customers_with_birthday_today`

## Milestones
- [x] Migration: `milestone_configs` + `milestone_achievements` tables (20260319000002)
- [x] Types: `MilestoneConfig`, `CreateMilestoneRequest`, `MilestoneCheckResult`, `AchievedMilestone`
- [x] Storage: CRUD for configs, achievement tracking with UNIQUE constraint
- [x] Helper: `check_and_award_milestones` -- order_count/lifetime_spend thresholds
- [x] Handlers: create_milestone, list_milestones, check_milestones, get_customer_milestones
- [x] Routes: 4 endpoints under `/earn/milestones`

## Newsletter Signup
- [x] Migration: `newsletter_signups` table with UNIQUE(merchant_id, customer_id) (20260319000003)
- [x] Types: `NewsletterSignupRequest`, `NewsletterSignupResult`, `NewsletterSignupCount`
- [x] Storage: signup check, recording, count
- [x] Helper: `process_newsletter_signup` with email validation and customer resolution
- [x] Handlers: newsletter_signup, get_newsletter_signup_count
- [x] Routes: 2 endpoints under `/earn/newsletter`

## Profile Completion
- [x] Types: `ProfileCompletionRequest`, `ProfileCompletionResult`
- [x] Helper: `process_profile_completion` -- checks name/email/birthday, awards 30.0 credit
- [x] Handler: profile_completion
- [x] Route: POST `/earn/profile-completion`

## Streaks
- [x] Migration: `streak_configs` + `streak_achievements` tables (20260319000004)
- [x] Types: `StreakConfig`, `CreateStreakConfigRequest`, `StreakCheckResult`, `ActiveStreak`
- [x] Storage: config CRUD, order counting (rolling window), achievement tracking
- [x] Helper: `check_and_award_streaks` -- rolling window with progress tracking
- [x] Handlers: create_streak_config, list_streak_configs, check_streaks
- [x] Routes: 3 endpoints under `/earn/streaks`

## Spin-the-Wheel
- [x] Migration: `spin_wheel_configs` + `spin_wheel_segments` + `spin_results` tables (20260319000005)
- [x] Types: `SpinWheelConfig`, `SpinWheelSegment`, `SpinResult`, `WheelWithSegments`
- [x] Storage: wheel config/segment CRUD, daily spin counting, result recording
- [x] Helper: `create_wheel`, `spin_wheel` -- probability-weighted selection, daily limit
- [x] Handlers: create_wheel_config, get_wheel_config, spin_wheel
- [x] Routes: 3 endpoints under `/earn/spin-wheel`

## Paid Memberships
- [x] Migration: `membership_plans` + `customer_memberships` tables (20260319000006)
- [x] Types: `MembershipPlan`, `CustomerMembership`, `SubscribeResult`, `MembershipStatus`
- [x] Storage: plan CRUD, subscribe/renew/cancel/expire, expiry detection
- [x] Helpers: `subscribe_to_plan`, `renew_membership`, `get_membership_status`, `cancel_membership_by_id`
- [x] Handlers: 6 endpoints (create plan, list plans, subscribe, renew, cancel, status)
- [x] Routes: 6 endpoints under `/earn/memberships`

## Multi-Brand Coalition
- [x] Migration: `coalitions` + `coalition_members` + `coalition_transfers` tables (20260319000007)
- [x] Types: `Coalition`, `CoalitionMember`, `CoalitionTransferRequest`, `CoalitionTransferResult`
- [x] Storage (admin): coalition CRUD, member management, transfer recording, transfer history
- [x] Helper (admin): `transfer_coalition_credits` with conversion rate calculation
- [x] Handlers (admin): 4 endpoints (create, list, transfer, history)
- [x] Routes: 4 endpoints under `/admin/coalitions`

## Integration
- [x] All engagement routes registered in `src/services/earn/mod.rs` (20 routes total)
- [x] Coalition routes registered in `src/services/admin/mod.rs`
- [x] No new service modules required
- [x] All helpers follow `#[tracing::instrument]` convention
- [x] All storage functions follow `#[tracing::instrument(skip(pool), err(Debug))]` convention
- [x] SHA-256 idempotency keys on all one-time rewards
