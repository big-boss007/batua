# Phase 6: Types

All types in `src/services/earn/types.rs` unless noted.

## Birthday Bonus
- `ProcessBirthdayBonusRequest` -- merchant_id, amount
- `BirthdayBonusResult` -- merchant_id, processed, credited, skipped, entries[]
- `BirthdayBonusEntry` -- customer_id, customer_name, amount, ledger_entry_id

## Milestones
- `MilestoneConfig` (sqlx::FromRow) -- id, merchant_id, name, milestone_type, threshold, reward_amount, is_active, created_at
- `CreateMilestoneRequest` -- merchant_id, name, milestone_type, threshold, reward_amount
- `CheckMilestonesRequest` -- merchant_id, customer_id
- `MilestoneCheckResult` -- customer_id, milestones_achieved[]
- `MilestoneAchievementEntry` -- milestone_name, reward_amount, ledger_entry_id
- `AchievedMilestone` (sqlx::FromRow) -- id, merchant_id, name, milestone_type, threshold, reward_amount, achieved_at

## Newsletter Signup
- `NewsletterSignupRequest` -- merchant_id, email, phone (optional), customer_id (optional), amount
- `NewsletterSignupResult` -- customer_id, email, rewarded, already_subscribed, ledger_entry_id, amount
- `NewsletterSignupCount` -- merchant_id, count

## Profile Completion
- `ProfileCompletionRequest` -- merchant_id, customer_id
- `ProfileCompletionResult` -- customer_id, fields_complete[], fields_missing[], completion_pct, already_rewarded, rewarded, amount, ledger_entry_id

## Streaks
- `StreakConfig` (sqlx::FromRow) -- id, merchant_id, name, required_orders, window_days, reward_amount, is_active, created_at
- `CreateStreakConfigRequest` -- merchant_id, name, required_orders, window_days, reward_amount
- `CheckStreakRequest` -- merchant_id, customer_id
- `StreakCheckResult` -- customer_id, streaks_achieved[], active_streaks[]
- `StreakAchievementEntry` -- streak_name, reward_amount, ledger_entry_id
- `ActiveStreak` -- streak_name, required_orders, orders_in_window, window_days, progress_pct

## Spin-the-Wheel
- `SpinWheelConfig` (sqlx::FromRow) -- id, merchant_id, name, is_active, daily_spin_limit, created_at
- `SpinWheelSegment` (sqlx::FromRow) -- id, wheel_id, label, reward_amount, probability, color, position, created_at
- `CreateWheelRequest` -- merchant_id, name, daily_spin_limit, segments[]
- `CreateSegmentRequest` -- label, reward_amount, probability, color
- `SpinRequest` -- merchant_id, customer_id
- `SpinResult` -- segment, reward_amount, ledger_entry_id, spins_remaining_today
- `WheelWithSegments` -- config, segments[]

## Paid Memberships
- `MembershipPlan` (sqlx::FromRow) -- id, merchant_id, name, plan_type, price, earn_rate_multiplier, benefits (JSONB), is_active, created_at
- `CustomerMembership` (sqlx::FromRow) -- id, merchant_id, customer_id, plan_id, status, started_at, expires_at, renewed_count, cancelled_at, created_at
- `CreateMembershipPlanRequest` -- merchant_id, name, plan_type, price, earn_rate_multiplier, benefits
- `SubscribeRequest` -- merchant_id, customer_id, plan_id
- `SubscribeResult` -- membership, plan, is_new, message
- `RenewRequest` -- membership_id
- `MembershipStatus` -- membership, plan, is_active, days_remaining
- `CancelMembershipRequest` -- membership_id

## Coalition (in `src/services/admin/types.rs`)
- `Coalition` (sqlx::FromRow) -- id, name, is_active, created_at
- `CoalitionMember` (sqlx::FromRow) -- id, coalition_id, merchant_id, conversion_rate, is_active, joined_at
- `CoalitionMemberInfo` (sqlx::FromRow) -- merchant_id, merchant_name, conversion_rate
- `CreateCoalitionRequest` -- name, merchant_ids[]
- `CoalitionTransferRequest` -- customer_id, from_merchant_id, to_merchant_id, amount
- `CoalitionTransferResult` -- transfer_id, from_amount, to_amount, conversion_rate, from_balance_after, to_balance_after
- `CoalitionInfo` -- coalition, members[]
- `CoalitionTransferRecord` (sqlx::FromRow) -- id, coalition_id, customer_id, from/to_merchant_id, from/to_wallet_id, amount, converted_amount, conversion_rate, transfer_id, created_at
