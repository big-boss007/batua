# Phase 5: Checklist

## Backend -- Campaigns Service
- [x] `campaigns/types.rs` -- FestiveTemplate, CreateFromTemplateRequest, CampaignCalendarEntry, CampaignPerformance, CampaignCalendarQuery
- [x] `campaigns/storage.rs` -- get_campaign_calendar, get_campaign_performance
- [x] `campaigns/helpers.rs` -- 10 festive templates, create_from_template, get_calendar, parse_datetime
- [x] `campaigns/handler.rs` -- list_templates, create_from_template, calendar, performance
- [x] `campaigns/mod.rs` -- Router with 4 routes

## Backend -- Notifications Service
- [x] `notifications/types.rs` -- NotificationTemplate, NotificationLog, Connector, SendNotificationRequest, NotificationResult, NotificationTrigger enum (9 variants), query types
- [x] `notifications/storage.rs` -- Template CRUD, connector CRUD with fallback, notification log CRUD with pagination
- [x] `notifications/helpers.rs` -- send_notification, render_template (mustache-style), trigger_notification
- [x] `notifications/handler.rs` -- 8 handlers covering send, template CRUD, connector CRUD, log listing
- [x] `notifications/mod.rs` -- Router with 7 route definitions

## Backend -- Rules Service
- [x] `rules/types.rs` -- Rule, RuleSnapshot, Campaign, CampaignSnapshot, RewardRuleConfig, Condition, RewardAction, EvaluationContext, EvaluationResult, request/query types
- [x] `rules/storage.rs` -- Rule CRUD, snapshot creation, campaign CRUD, rule_performance aggregation
- [x] `rules/helpers.rs` -- Rule engine (evaluate_rules, check_conditions, calculate_reward, apply_campaign_multiplier) + 20 unit tests
- [x] `rules/handler.rs` -- 8 handlers for rules+campaigns
- [x] `rules/mod.rs` -- Router with 7 route definitions

## Backend -- Admin Service
- [x] `admin/types.rs` -- Merchant, GeoPolicy, AdminDashboard, MerchantStats, SystemHealth, MerchantDashboard, MerchantAnalytics, Coalition types, request/query types
- [x] `admin/storage.rs` -- Merchant CRUD, wallet policy UPSERT, geo policy CRUD, dashboard stats, system health, recent events, merchant analytics, coalition CRUD, coalition transfer recording
- [x] `admin/helpers.rs` -- Bulk credit, dispute processing, system dashboard, coalition credit transfer
- [x] `admin/handler.rs` -- 21 handlers covering full admin API
- [x] `admin/mod.rs` -- Router with 22 route definitions

## Database Migrations
- [x] `20260318000007_rules_and_snapshots.sql` -- rules, rule_snapshots, campaigns, campaign_snapshots
- [x] `20260318000010_connectors.sql` -- connectors with unique constraint
- [x] `20260318000011_notifications.sql` -- notification_templates, notification_logs
- [x] `20260318000016_geo_policies.sql` -- geo_policies + merchants.geo_policy_id
- [x] `20260318000018_merchant_slugs.sql` -- merchants.slug
- [x] `20260318000019_plan_tier.sql` -- merchants.plan_tier

## Frontend -- Analytics Module
- [x] `analytics/types.ts` -- CodAnalytics, CampaignPerformance, OverviewMetrics, DateRange, MerchantAnalytics
- [x] `analytics/remote.ts` -- 4 API callers with decoders
- [x] `analytics/store.ts` -- analyticsStore (cod, campaigns, overview, loading), dateRangeStore (default 30d)
- [x] `analytics/index.ts` -- Barrel exports
- [x] `analytics/ui/` -- CampaignPerformanceTable, CodMetrics, OverviewCards, RtoComparison

## Frontend -- Rules Module
- [x] `rules/types.ts` -- Rule, RewardRuleConfig, Condition, RewardAction, Campaign, FestiveTemplate, request types
- [x] `rules/remote.ts` -- 8 API callers (rules CRUD, campaigns, templates, calendar, performance)
- [x] `rules/store.ts` -- rulesStore (CRUD ops), campaignsStore, selectedRuleStore
- [x] `rules/index.ts` -- Barrel exports
- [x] `rules/ui/` -- RuleForm, RulesList, CampaignForm, CampaignsList, FestiveTemplateGrid

## Frontend -- Settings Module
- [x] `settings/types.ts` -- WalletPolicy, Connector, NotificationTemplate, NotificationLog, request types
- [x] `settings/remote.ts` -- 8 API callers (policies, connectors, templates, merchant profile, notification logs)
- [x] `settings/store.ts` -- walletPoliciesStore, connectorsStore, templatesStore
- [x] `settings/index.ts` -- Barrel exports
- [x] `settings/ui/` -- WalletPoliciesList, WalletPolicyForm, ConnectorForm, ConnectorsList, NotificationTemplateEditor

## Frontend -- Platform Module
- [x] `platform/types.ts` -- PlatformMerchant, MerchantStats, SystemHealth, GeoPolicy, RecentEvent, DashboardStats, OnboardMerchantForm
- [x] `platform/remote.ts` -- 11 API callers (dashboard, merchants, stats, geo-policies, system health, events)
- [x] `platform/store.ts` -- merchantsList, selectedMerchant
- [x] `platform/index.ts` -- Barrel exports
- [x] `platform/ui/` -- MerchantTable, StatsGrid, HealthCard, EventsTable, GeoPolicyTable, GeoPolicyForm, OnboardForm, PlatformSidebar

## Frontend -- Admin Module
- [x] `admin/types.ts` -- DashboardStats, Merchant, MerchantDashboard, Breadcrumb, NavItem
- [x] `admin/remote.ts` -- 4 API callers (dashboard, merchant, merchants, merchantDashboard)
- [x] `admin/store.ts` -- currentMerchant (localStorage-backed), currentMerchantId (derived), breadcrumbs
- [x] `admin/utils.ts` -- formatMetricValue (number/currency/percentage), localStorage merchant ID persistence
- [x] `admin/index.ts` -- Barrel exports
- [x] `admin/ui/` -- MerchantSelector, MetricCard, QuickActions, Sidebar

## Frontend Routes
- [x] `/admin/*` -- 12 sub-routes with layout
- [x] `/platform/*` -- 4 sub-routes with layout

## Router Integration
- [x] All 4 services registered in `src/main.rs` via `.merge()`
- [x] All 4 service modules declared in `src/services/mod.rs`
