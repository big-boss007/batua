# Phase 1: Wallets -- Checklist

## Core Infrastructure

- [x] Define Postgres enums: movement_type, actor_type, bucket_type, credit_state, redemption_state
- [x] Create wallets table with UNIQUE(merchant_id, customer_id) constraint
- [x] Create ledger_entries table with idempotency_key UNIQUE constraint
- [x] Add immutability trigger (prevent_ledger_mutation) on ledger_entries
- [x] Create wallet_policies table with per-bucket merchant constraints
- [x] Create redemption_requests table with state machine
- [x] Create cod_orders table

## Wallets Service

- [x] types.rs: Wallet, CreateWalletRequest, WalletResponse, WalletBalanceSummary, WalletLookupQuery, GetOrCreateRequest, PaginationQuery
- [x] storage.rs: create_wallet, get_wallet, get_wallet_by_merchant_customer, get_or_create_wallet, get_bearer_wallet, list_wallets_for_merchant
- [x] helpers.rs: generate_bearer_code
- [x] handler.rs: create_wallet, get_wallet, lookup_wallet, get_or_create_wallet, list_wallets_for_merchant
- [x] mod.rs: Router with 5 routes

## Earn Service

- [x] types.rs: ProcessEarnRequest, EarnResult, EarnEntry, ManualCreditRequest/Result
- [x] types.rs: BirthdayBonusRequest/Result/Entry
- [x] types.rs: MilestoneConfig, CreateMilestoneRequest, CheckMilestonesRequest, MilestoneCheckResult
- [x] types.rs: NewsletterSignupRequest/Result/Count
- [x] types.rs: ProfileCompletionRequest/Result
- [x] types.rs: StreakConfig, CreateStreakConfigRequest, CheckStreakRequest, StreakCheckResult, ActiveStreak
- [x] types.rs: SpinWheelConfig, SpinWheelSegment, CreateWheelRequest, SpinRequest, SpinResult
- [x] types.rs: MembershipPlan, CustomerMembership, CreateMembershipPlanRequest, SubscribeRequest/Result, MembershipStatus
- [x] storage.rs: Order stats (get/update)
- [x] storage.rs: Milestone CRUD and achievements
- [x] storage.rs: Newsletter signup tracking
- [x] storage.rs: Streak configs and achievements
- [x] storage.rs: Spin wheel configs, segments, results
- [x] storage.rs: Membership plans, subscriptions, renewal, cancellation, expiry
- [x] helpers.rs: process_earn (order cashback with rule evaluation)
- [x] helpers.rs: process_manual_credit
- [x] helpers.rs: process_birthday_bonuses
- [x] helpers.rs: process_newsletter_signup
- [x] helpers.rs: process_profile_completion
- [x] helpers.rs: check_and_award_milestones
- [x] helpers.rs: check_and_award_streaks
- [x] helpers.rs: create_wheel, spin_wheel
- [x] helpers.rs: subscribe_to_plan, renew_membership, get_membership_status, cancel_membership_by_id
- [x] handler.rs: 22 handlers
- [x] mod.rs: Router with 22 routes

## Redemption Service

- [x] types.rs: RedemptionState, RedemptionRequest, WalletPolicy, InitiateRedemptionRequest, BucketEligibility, RedemptionEligibility, RedemptionResponse, OrderContext
- [x] storage.rs: create_redemption, update_redemption_state, get_redemption, get_wallet_policy, get_wallet_policies
- [x] helpers.rs: evaluate_eligibility (per-bucket with policy constraints)
- [x] helpers.rs: validate_constraints (min_redemption, step_size, stackability)
- [x] helpers.rs: execute_redemption (full state machine)
- [x] helpers.rs: compensate (reverse failed redemptions)
- [x] handler.rs: initiate_redemption, get_redemption, compensate_redemption, check_eligibility
- [x] mod.rs: Router with 4 routes

## COD Service

- [x] types.rs: CodOrderState, CodOrder, DeliveryWebhookPayload, CodToPrepaidRequest/Response, CodAnalytics
- [x] storage.rs: create_cod_order, get_cod_order_by_order_id, update_cod_state, get_pending_cod_orders, get_cod_analytics
- [x] helpers.rs: process_delivery (Held -> Across -> EarnedCredit)
- [x] helpers.rs: process_rto (Held -> Out, value destroyed)
- [x] helpers.rs: process_cod_to_prepaid (incentive for switching payment method)
- [x] handler.rs: delivery_webhook, cod_to_prepaid, list_cod_orders, cod_analytics
- [x] mod.rs: Router with 4 routes

## Earn Sub-Features (Database)

- [x] Migration: milestone_configs + milestone_achievements
- [x] Migration: newsletter_signups
- [x] Migration: streak_configs + streak_achievements
- [x] Migration: spin_wheel_configs + spin_wheel_segments + spin_results
- [x] Migration: membership_plans + customer_memberships

## Integration

- [x] Earn -> Wallets: get_or_create_wallet for all credit flows
- [x] Earn -> Ledger: create_entry for all credit types
- [x] Earn -> COD: create_cod_order for COD orders
- [x] COD -> Ledger: create_across_movement for delivery release
- [x] COD -> Ledger: create_entry (Out) for RTO cancellation
- [x] Redemption -> Ledger: get_balance for eligibility
- [x] Redemption -> Ledger: create_entry (Out) for debits
- [x] Redemption -> Ledger: create_entry (In) for compensation
- [x] All services use deterministic idempotency keys where applicable
