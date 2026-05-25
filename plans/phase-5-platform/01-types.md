# Phase 5: Types

## Backend Types

### `src/services/campaigns/types.rs`
- `FestiveTemplate` -- Name, display_name, description, default_multiplier, default_duration_days, suggested_start, category
- `CreateFromTemplateRequest` -- merchant_id, template_name, base_rule_id, multiplier, starts_at, ends_at, custom_name
- `CampaignCalendarEntry` -- id, name, campaign_type, multiplier, starts_at, ends_at, is_active, is_currently_running
- `CampaignPerformance` -- campaign_id, name, total_entries, total_value, unique_customers, average_reward
- `CampaignCalendarQuery` -- merchant_id, from, to

### `src/services/notifications/types.rs`
- `NotificationTemplate` (sqlx::FromRow) -- id, merchant_id, name, channel, locale, template_id, body_template, variables, is_active, created_at, updated_at
- `NotificationLog` (sqlx::FromRow) -- id, merchant_id, customer_id, template_id, channel, variables, status, external_message_id, sent_at, created_at
- `Connector` (sqlx::FromRow) -- id, merchant_id, capability, vendor, config, is_active, priority, created_at, updated_at
- `SendNotificationRequest` -- merchant_id, customer_id, template_name, variables, channel_hint, locale
- `NotificationResult` -- log_id, channel, status, external_message_id
- `CreateTemplateRequest`, `UpdateTemplateRequest`, `CreateConnectorRequest`, `UpdateConnectorRequest`
- `NotificationTrigger` enum -- EarnCredit, RedeemCredit, CodPending, CodReleased, CodCancelled, Expiry, GiftCardReceived, ReferralReward, TierUpgrade (with `template_name()` and `channel_capability()` methods)
- `MerchantIdQuery`, `NotificationLogQuery`, `NotificationLogPaginatedQuery`

### `src/services/rules/types.rs`
- `Rule` (sqlx::FromRow) -- id, merchant_id, rule_type, name, config (JSONB), version, is_active
- `RuleSnapshot` (sqlx::FromRow) -- id, rule_id, version, config
- `Campaign` (sqlx::FromRow) -- id, merchant_id, name, campaign_type, config, base_rule_id, multiplier, starts_at, ends_at, is_active
- `CampaignSnapshot` (sqlx::FromRow) -- id, campaign_id, config, multiplier
- `RewardRuleConfig` -- event_type, conditions[], action
- `Condition` -- field, operator, value (JSON)
- `RewardAction` -- bucket_type, calculation, value, max_amount, conversion_rate, expiry_days
- `EvaluationContext` -- merchant_id, event_type, event_payload, order_amount, payment_method, is_cod, collections, customer_tags, is_first_order
- `EvaluationResult` -- matched, rule_snapshot_id, campaign_snapshot_id, earning_unit, currency_equivalent, conversion_rate, bucket_type, expiry_days, constraints
- `CreateRuleRequest`, `UpdateRuleRequest`, `EvaluateRequest`, `ListRulesQuery`
- `CreateCampaignRequest`, `ListCampaignsQuery`
- `RulePerformance` -- rule_id, total_entries, total_value, unique_customers

### `src/services/admin/types.rs`
- `Merchant` (sqlx::FromRow) -- id, external_id, name, domain, currency, timezone, is_active, slug, geo_policy_id, plan_tier
- `CreateMerchantRequest`, `UpdateMerchantRequest`
- `BulkCreditRequest` -- merchant_id, customer_ids[], amount, bucket_type, reason, actor_id
- `BulkCreditResult`, `BulkCreditItemResult`
- `DisputeRequest` -- merchant_id, customer_id, ledger_entry_id, reason, actor_id
- `DisputeResult` -- reversal_entry_id, original_amount, reversed
- `GeoPolicy` (sqlx::FromRow) -- id, geo_code, name, config, is_active
- `WalletPolicyRequest` -- merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_expiry_days, is_transferable
- `AdminDashboard` -- total_merchants, total_wallets, total_ledger_entries, total_value_in_system
- `MerchantStats` -- merchant_id, total_wallets, total_customers, total_ledger_entries, active_credits, total_redeemed
- `SystemHealth` -- unprocessed_events, failed_events, pending_cod_orders, expiring_7d_count/value, expiring_30d_count/value
- `RecentEvent` (sqlx::FromRow) -- id, merchant_id, merchant_name, event_type, event_source, state
- `MerchantDashboard` -- merchant_id, active_customers, total_wallets, total_earned, total_redeemed, total_cod_pending, active_credits, total_ledger_entries, redemption_count
- `MerchantCustomer` (sqlx::FromRow), `MerchantTransaction` (sqlx::FromRow)
- `MerchantAnalytics` -- total_earned, total_redeemed, total_expired, active_credits, cod_pending/delivered/rto, total/prepaid/cod_orders, loyalty_rto_rate, non_loyalty_rto_rate, repeat_purchase_rate
- `Coalition` (sqlx::FromRow), `CoalitionMember` (sqlx::FromRow), `CoalitionMemberInfo` (sqlx::FromRow)
- `CreateCoalitionRequest`, `CoalitionTransferRequest`, `CoalitionTransferResult`, `CoalitionInfo`
- `CoalitionTransferRecord` (sqlx::FromRow)
- `PaginationQuery`, `MerchantCustomersQuery`, `MerchantTransactionsQuery`, `RecentEventsQuery`, `UpdatePlanRequest`, `CreateGeoPolicyRequest`

## Frontend Types

### `analytics/types.ts`
- `CodAnalytics`, `CampaignPerformance`, `OverviewMetrics`, `DateRange`, `MerchantAnalytics`

### `rules/types.ts`
- `Rule`, `RewardRuleConfig`, `Condition`, `RewardAction`, `Campaign`, `FestiveTemplate`
- `CreateRuleRequest`, `UpdateRuleRequest`, `CreateCampaignFromTemplateRequest`, `CampaignCalendarEntry`, `RulePerformance`

### `settings/types.ts`
- `WalletPolicy`, `Connector`, `NotificationTemplate`, `NotificationLog`
- `UpdateWalletPolicyRequest`, `CreateConnectorRequest`, `UpdateTemplateRequest`

### `platform/types.ts`
- `PlatformMerchant`, `MerchantStats`, `SystemHealth`, `GeoPolicy`, `RecentEvent`, `DashboardStats`, `OnboardMerchantForm`

### `admin/types.ts`
- `DashboardStats`, `Merchant`, `MerchantDashboard`, `Breadcrumb`, `NavItem`
