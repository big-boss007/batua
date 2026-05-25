# Phase 1: Handlers -- COMPLETED

## Wallets (`src/services/wallets/handler.rs`)

| Handler | Method | Input | Output | Status Code |
|---------|--------|-------|--------|-------------|
| `create_wallet` | POST | Json<CreateWalletRequest> | Json<WalletResponse> | 200 |
| `get_wallet` | GET | Path<Uuid> | Json<WalletResponse> | 200 |
| `lookup_wallet` | GET | Query<WalletLookupQuery> | Json<WalletResponse> | 200 |
| `get_or_create_wallet` | POST | Json<GetOrCreateRequest> | Json<WalletResponse> | 200 |
| `list_wallets_for_merchant` | GET | Path<merchant_id>, Query<PaginationQuery> | Json<Vec<Wallet>> | 200 |

## Earn (`src/services/earn/handler.rs`)

| Handler | Method | Input | Output | Status Code |
|---------|--------|-------|--------|-------------|
| `process_earn` | POST | Json<ProcessEarnRequest> | Json<EarnResult> | 200 |
| `manual_credit` | POST | Json<ManualCreditRequest> | Json<ManualCreditResult> | 201 |
| `birthday_bonus` | POST | Json<ProcessBirthdayBonusRequest> | Json<BirthdayBonusResult> | 200 |
| `create_milestone` | POST | Json<CreateMilestoneRequest> | Json<MilestoneConfig> | 201 |
| `list_milestones` | GET | Path<merchant_id> | Json<Vec<MilestoneConfig>> | 200 |
| `check_milestones` | POST | Json<CheckMilestonesRequest> | Json<MilestoneCheckResult> | 200 |
| `get_customer_milestones` | GET | Path<(merchant_id, customer_id)> | Json<Vec<AchievedMilestone>> | 200 |
| `newsletter_signup` | POST | Json<NewsletterSignupRequest> | Json<NewsletterSignupResult> | 201 (or 200 if already subscribed) |
| `get_newsletter_signup_count` | GET | Path<merchant_id> | Json<NewsletterSignupCount> | 200 |
| `profile_completion` | POST | Json<ProfileCompletionRequest> | Json<ProfileCompletionResult> | 201 (or 200 if already rewarded / incomplete) |
| `create_streak_config` | POST | Json<CreateStreakConfigRequest> | Json<StreakConfig> | 201 |
| `list_streak_configs` | GET | Path<merchant_id> | Json<Vec<StreakConfig>> | 200 |
| `check_streaks` | POST | Json<CheckStreakRequest> | Json<StreakCheckResult> | 200 |
| `create_wheel_config` | POST | Json<CreateWheelRequest> | Json<WheelWithSegments> | 201 |
| `get_wheel_config` | GET | Path<merchant_id> | Json<WheelWithSegments> | 200 |
| `spin_wheel` | POST | Json<SpinRequest> | Json<SpinResult> | 200 |
| `create_membership_plan` | POST | Json<CreateMembershipPlanRequest> | Json<MembershipPlan> | 201 |
| `list_membership_plans` | GET | Path<merchant_id> | Json<Vec<MembershipPlan>> | 200 |
| `subscribe_membership` | POST | Json<SubscribeRequest> | Json<SubscribeResult> | 201 (or 200 if already subscribed) |
| `renew_membership` | POST | Json<RenewRequest> | Json<SubscribeResult> | 200 |
| `cancel_membership` | POST | Path<membership_id> | Json<CustomerMembership> | 200 |
| `membership_status` | GET | Path<(merchant_id, customer_id)> | Json<MembershipStatus> | 200 |

## Redemption (`src/services/redemption/handler.rs`)

| Handler | Method | Input | Output | Status Code |
|---------|--------|-------|--------|-------------|
| `initiate_redemption` | POST | Json<InitiateRedemptionRequest> | Json<RedemptionResponse> | 201 |
| `get_redemption` | GET | Path<id> | Json<RedemptionRequest> | 200 |
| `compensate_redemption` | POST | Path<id> | Json<RedemptionRequest> | 200 |
| `check_eligibility` | GET | Path<wallet_id>, Query<EligibilityQuery> | Json<RedemptionEligibility> | 200 |

Read endpoints (`get_redemption`, `check_eligibility`) use `db_reader` when available.

## COD (`src/services/cod/handler.rs`)

| Handler | Method | Input | Output | Status Code |
|---------|--------|-------|--------|-------------|
| `delivery_webhook` | POST | Json<DeliveryWebhookPayload> | Json (status + order_id) | 200 |
| `cod_to_prepaid` | POST | Json<CodToPrepaidRequest> | Json<CodToPrepaidResponse> | 200 |
| `list_cod_orders` | GET | Path<merchant_id>, Query<CodOrdersQuery> | Json<Vec<CodOrderResponse>> | 200 |
| `cod_analytics` | GET | Path<merchant_id> | Json<CodAnalytics> | 200 |

`delivery_webhook` dispatches to `process_delivery` for "delivered" status and `process_rto` for "rto"/"cancelled" status.

Read endpoints (`list_cod_orders`, `cod_analytics`) use `db_reader` when available.
