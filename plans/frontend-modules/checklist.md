# Frontend Modules Checklist

## Phase 0: Overview
- [x] Inventory all 11 modules with purpose and file counts
- [x] Document module dependency graph
- [x] Map routes to modules
- [x] Count all types, stores, remote functions, components

## Phase 1: Planning & Architecture
- [x] Document module structure pattern (index.ts, types.ts, store.ts, remote.ts, utils.ts, ui/)
- [x] Document barrel export design and grouping convention
- [x] Document API client architecture (APICaller, APIResult tagged union, decoder pattern)
- [x] Document CSS custom properties theming strategy (design tokens, dark theme, data-theme attribute)
- [x] Document @juspay/svelte-ui-components integration (all themed components, CSS variable mapping)
- [x] Document component design patterns (props, callbacks, $derived, snippets, scoped styles)
- [x] Document Svelte 5 runes adoption ($state, $derived, $props, $bindable)

## Phase 2: Setup & Configuration
- [x] Document SvelteKit config (adapter-auto, vitePreprocess, aliases)
- [x] Document Vite config (sveltekit plugin)
- [x] Document path aliases ($lib, $generated, $app/*, $env/*)
- [x] Document root layout (app.css import)
- [x] Document app.css structure (tokens, reset, library overrides, variants, dark theme)
- [x] Document environment variables (PUBLIC_API_BASE_URL)
- [x] Document generated types placeholder

## Phase 3: Type Definitions
- [x] Document foundation types (APIResult, APISuccess, APIError, RequestConfig, Theme, Toast)
- [x] Document admin types (DashboardStats, Merchant, MerchantDashboard, Breadcrumb, NavItem)
- [x] Document platform types (PlatformMerchant, MerchantStats, SystemHealth, GeoPolicy, RecentEvent, OnboardMerchantForm)
- [x] Document transactions types (LedgerEntry, LedgerEntryDetail, WalletBalance, BucketBalance, RedemptionRequest, TransactionFilters, MerchantTransactionRow)
- [x] Document customers types (Customer, CustomerDetail, WalletSummary, CustomerTierInfo, TierProgress, LoyaltyProgram, LoyaltyTier, TierDistribution, MerchantCustomerRow)
- [x] Document gift-cards types (GiftCard, GiftCardStats, IssueGiftCardForm, BulkIssueForm, BulkIssueInput)
- [x] Document referrals types (ReferralProgram, ReferralCode, ReferralAnalytics, ReferralConversion)
- [x] Document rules types (Rule, RewardRuleConfig, Condition, RewardAction, Campaign, FestiveTemplate, CreateRuleRequest, UpdateRuleRequest, CreateCampaignFromTemplateRequest, CampaignCalendarEntry, RulePerformance)
- [x] Document settings types (WalletPolicy, Connector, NotificationTemplate, NotificationLog, request types)
- [x] Document analytics types (CodAnalytics, CampaignPerformance, OverviewMetrics, DateRange, MerchantAnalytics)
- [x] Document storefront types (StorefrontMerchant, CustomerBalance, CustomerTierInfo, TransactionEntry, GiftCardInfo, ReferralCodeInfo, ReferralLandingInfo, ReferralProgramInfo, CustomerIdentity, WalletLookup)
- [x] Document shared type patterns and intentional duplication

## Phase 4: State Management
- [x] Document three-tier state model (URL, store, component)
- [x] Document all stores by module with types, persistence, and methods
- [x] Document persistence strategy (localStorage for theme and merchant ID, sessionStorage for storefront phone)
- [x] Document store factory pattern (createXxxStore with subscribe + domain methods)
- [x] Document component state examples ($state/$derived usage)

## Phase 5: API Integration
- [x] Document API caller request flow (component -> remote -> apiCaller -> fetch)
- [x] Document result handling pattern (tag-based discriminated union)
- [x] Document decoder pattern (field-by-field vs direct cast)
- [x] Document all API endpoints by module (admin: 4, platform: 11, transactions: 6, customers: 9, gift-cards: 7, referrals: 8, rules: 8, settings: 8, analytics: 4, storefront: 10)
- [x] Document API base URL prefix organization
- [x] Document orchestrated fetch pattern (getCustomerDetail)

## Phase 6: Utilities
- [x] Document foundation utils (formatCurrencyINR, formatDate, formatDateTime, normalizePhoneE164)
- [x] Document admin utils (formatMetricValue, getCurrentMerchantId, setCurrentMerchantId)
- [x] Document transactions utils (BUCKET_LABELS, formatBucketType, formatMovementType, formatState)
- [x] Document customers utils (getTierColor, formatMultiplier, sortTiersByRank)
- [x] Document storefront utils (generateWhatsAppShareUrl, copyToClipboard, formatBucketLabel, formatMovementLabel, getMovementPrefix, maskPhone)
- [x] Document shared BUCKET_LABELS duplication across 3 modules

## Phase 7: UI Components
- [x] Document all 57 components with props, library component usage, and purpose
- [x] Document admin/ui (4): Sidebar, MetricCard, MerchantSelector, QuickActions
- [x] Document platform/ui (8): PlatformSidebar, MerchantTable, StatsGrid, HealthCard, EventsTable, OnboardForm, GeoPolicyTable, GeoPolicyForm
- [x] Document transactions/ui (4): TransactionTable, BalanceCard, TransactionFilters, RedemptionHistory
- [x] Document customers/ui (7): CustomerSearch, CustomerDetail, TierBadge, TierProgress, LoyaltyProgramForm, TierForm, TierDistributionChart
- [x] Document gift-cards/ui (4): GiftCardsList, IssueGiftCardForm, BulkIssueForm, GiftCardDetail
- [x] Document referrals/ui (5): ReferralProgramForm, ReferralCodesList, CreateCodeForm, ReferralAnalyticsCard, ConversionsList
- [x] Document rules/ui (5): RulesList, RuleForm, CampaignsList, FestiveTemplateGrid, CampaignForm
- [x] Document settings/ui (5): WalletPoliciesList, WalletPolicyForm, ConnectorsList, ConnectorForm, NotificationTemplateEditor
- [x] Document analytics/ui (4): CodMetrics, OverviewCards, CampaignPerformanceTable, RtoComparison
- [x] Document storefront/ui (10): PhoneInput, BalanceCard, TierCard, TransactionCard, TransactionList, ReferralCard, ShareButtons, GiftCardStatus, CampaignBanner, MerchantHeader
- [x] Document library component usage summary (Table, Pill, Button, Input, Select, Progress, Toggle, Pagination, ThemeSwitcher, RelativeTime)

## Phase 8: Integration
- [x] Document complete route structure (admin, platform, storefront)
- [x] Document layout hierarchy (root, admin, platform, storefront)
- [x] Document data loading patterns (layout-level, page-level, component-level, orchestrated)
- [x] Document navigation patterns (sidebar links, programmatic goto, cross-zone links)
