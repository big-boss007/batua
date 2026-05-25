# Phase 0: Overview

Status: COMPLETED

## Module Inventory

The SvelteKit frontend contains 11 client modules at `frontend/src/lib/client/modules/`. Each module follows the standard structure: `index.ts` (barrel), `types.ts`, `store.ts`, `remote.ts`, optional `utils.ts`, and `ui/` directory with component barrel.

### 1. foundation/

Purpose: Shared infrastructure -- API client, theming, sidebar state, toast notifications, and formatting utilities.

Files: `index.ts`, `store.ts`, `remote.ts`, `utils.ts`, `ui/index.ts` (empty barrel)

Exports:
- `apiCaller` (APICaller class), `buildUrl` -- HTTP layer with tagged-union `APIResult<T>`
- `themeStore` -- light/dark, persisted to localStorage, sets `data-theme` attribute
- `sidebarStore` -- collapsed/expanded state
- `toastStore` -- push/dismiss/clear toast queue
- `formatCurrencyINR`, `formatDate`, `formatDateTime`, `normalizePhoneE164`

Depended on by: every other module (all `remote.ts` files import `apiCaller` and `APIResult`)

### 2. admin/

Purpose: Merchant admin context -- merchant selector, dashboard stats, navigation, breadcrumbs.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `utils.ts`, `ui/index.ts`

Types: `DashboardStats`, `Merchant`, `MerchantDashboard`, `Breadcrumb`, `NavItem`

Stores: `currentMerchant` (writable), `currentMerchantId` (derived), `breadcrumbs`

API: `fetchDashboardStats`, `fetchMerchant`, `fetchMerchants`, `fetchMerchantDashboard`

Utilities: `formatMetricValue` (number/currency/percentage), `getCurrentMerchantId`, `setCurrentMerchantId` (localStorage)

UI Components: `Sidebar`, `MetricCard`, `QuickActions`, `MerchantSelector`

### 3. platform/

Purpose: Super-admin panel -- merchant management, onboarding, geo policies, system health, event monitoring.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `PlatformMerchant`, `MerchantStats`, `SystemHealth`, `GeoPolicy`, `RecentEvent`, `DashboardStats`, `OnboardMerchantForm`

Stores: `merchantsList`, `selectedMerchant`

API: `fetchDashboardStats`, `fetchMerchants`, `fetchMerchant`, `fetchMerchantStats`, `createMerchant`, `updateMerchant`, `updateMerchantPlan`, `fetchGeoPolicies`, `createGeoPolicy`, `fetchSystemHealth`, `fetchRecentEvents`

UI Components: `PlatformSidebar`, `MerchantTable`, `StatsGrid`, `HealthCard`, `EventsTable`, `OnboardForm`, `GeoPolicyTable`, `GeoPolicyForm`

### 4. transactions/

Purpose: Ledger entry browsing, wallet balance display, redemption history, merchant transaction views.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `utils.ts`, `ui/index.ts`

Types: `LedgerEntry`, `LedgerEntryDetail`, `WalletBalance`, `BucketBalance`, `RedemptionRequest`, `TransactionFilters`, `LedgerResponse`, `RedemptionResponse`, `WalletLookupResult`, `MerchantTransactionRow`

Stores: `transactionFilters` (writable), `allEntries` (writable), `filteredEntries` (derived), `resetFilters`, `DEFAULT_FILTERS`

API: `fetchEntries`, `fetchBalance`, `fetchRedemptions`, `lookupWallet`, `fetchMerchantTransactions`, `fetchEntryDetail`

Utilities: `formatBucketType` (with `BUCKET_LABELS` map), `formatMovementType` (returns `MovementHint` with label + color), `formatState` (returns `StateHint`)

UI Components: `TransactionTable`, `BalanceCard`, `TransactionFilters`, `RedemptionHistory`

### 5. customers/

Purpose: Customer search, detail views, loyalty program management (programs, tiers, distribution).

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `utils.ts`, `ui/index.ts`

Types: `Customer`, `CustomerDetail`, `WalletSummary`, `CustomerTierInfo`, `TierProgress`, `LedgerEntrySummary`, `LoyaltyProgram`, `LoyaltyTier`, `TierDistribution`, `MerchantCustomerRow`

Stores: `customerSearchStore` (query, results, loading), `customerDetailStore` (detail, loading), `loyaltyStore` (program, tiers, distribution, loading)

API: `searchCustomers`, `getCustomerDetail` (orchestrates customer + wallet + entries), `fetchMerchantCustomers`, `fetchLoyaltyProgram`, `fetchTiers`, `fetchTierDistribution`, `createProgram`, `createTier`, `evaluateTier`

Utilities: `getTierColor` (rank-to-CSS-var map), `formatMultiplier`, `sortTiersByRank`, `formatMovementType`, `formatBucketType`

UI Components: `CustomerSearch`, `CustomerDetail`, `TierBadge`, `TierProgress`, `LoyaltyProgramForm`, `TierForm`, `TierDistributionChart`

### 6. gift-cards/

Purpose: Gift card lifecycle -- issuing, bulk issuing, lookup, claiming, redeeming, stats.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `GiftCard`, `GiftCardStats`, `IssueGiftCardForm`, `BulkIssueForm`, `BulkIssueInput`

Stores: `giftCards` (with `add`, `addMany`, `clear`)

API: `issueGiftCard`, `bulkIssue`, `fetchGiftCards`, `getGiftCardByCode`, `claimGiftCard`, `redeemGiftCard`, `fetchGiftCardStats`

UI Components: `GiftCardsList`, `IssueGiftCardForm`, `BulkIssueForm`, `GiftCardDetail`

### 7. referrals/

Purpose: Referral program configuration, code management, conversion tracking, analytics.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `ReferralProgram`, `ReferralCode`, `ReferralAnalytics`, `ReferralConversion`

Stores: `referralProgram`, `referralCodes` (with `add`)

API: `fetchProgram`, `createProgram`, `createCode`, `fetchCodeByCode`, `processConversion`, `fetchAnalytics`, `fetchConversions`, `fetchMerchantCodes`

UI Components: `ReferralProgramForm`, `ReferralCodesList`, `CreateCodeForm`, `ReferralAnalyticsCard`, `ConversionsList`

### 8. rules/

Purpose: Reward rule CRUD, campaign management, festive templates, performance tracking.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `Rule`, `RulePerformance`, `RewardRuleConfig`, `Condition`, `RewardAction`, `Campaign`, `FestiveTemplate`, `CampaignCalendarEntry`, `CreateRuleRequest`, `UpdateRuleRequest`, `CreateCampaignFromTemplateRequest`

Stores: `rulesStore` (with `addRule`, `updateRule`, `toggleRule`), `campaignsStore` (with `addCampaign`), `selectedRuleStore`

API: `fetchRules`, `createRule`, `updateRule`, `fetchCampaigns`, `createCampaignFromTemplate`, `fetchFestiveTemplates`, `fetchCampaignCalendar`, `fetchRulePerformance`

UI Components: `RulesList`, `RuleForm`, `CampaignsList`, `FestiveTemplateGrid`, `CampaignForm`

### 9. settings/

Purpose: Wallet policies, notification connectors, notification templates, merchant profile updates.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `WalletPolicy`, `Connector`, `NotificationTemplate`, `NotificationLog`, `UpdateWalletPolicyRequest`, `CreateConnectorRequest`, `UpdateTemplateRequest`

Stores: `walletPoliciesStore` (with `updatePolicy`), `connectorsStore` (with `addConnector`, `updateConnector`), `templatesStore` (with `updateTemplate`)

API: `fetchWalletPolicies`, `updateWalletPolicy`, `fetchConnectors`, `createConnector`, `fetchTemplates`, `updateTemplate`, `updateMerchantProfile`, `fetchNotificationLogs`

UI Components: `WalletPolicyForm`, `WalletPoliciesList`, `ConnectorsList`, `ConnectorForm`, `NotificationTemplateEditor`

### 10. analytics/

Purpose: Dashboard metrics -- COD analytics, campaign performance, overview metrics, RTO comparison.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `ui/index.ts`

Types: `CodAnalytics`, `CampaignPerformance`, `OverviewMetrics`, `DateRange`, `MerchantAnalytics`

Stores: `analyticsStore` (cod, campaigns, overview, loading), `dateRangeStore` (defaults to last 30 days)

API: `fetchMerchantAnalytics`, `fetchCodAnalytics`, `fetchCampaignPerformance`, `fetchOverviewMetrics`

UI Components: `CodMetrics`, `RtoComparison`, `CampaignPerformanceTable`, `OverviewCards`

### 11. storefront/

Purpose: Customer-facing UI -- balance display, tier card, transaction history, referral sharing, gift card status check.

Files: `index.ts`, `types.ts`, `store.ts`, `remote.ts`, `utils.ts`, `ui/index.ts`

Types: `StorefrontMerchant`, `CustomerBalance`, `BucketBalance`, `CustomerTierInfo`, `TierProgress`, `TransactionEntry`, `GiftCardInfo`, `ReferralCodeInfo`, `ReferralLandingInfo`, `ReferralProgramInfo`, `CustomerIdentity`, `WalletLookup`

Stores: `customerPhone` (sessionStorage-backed), `merchantContext`

API: `fetchMerchantBySlug`, `lookupCustomer`, `lookupWallet`, `fetchBalance`, `fetchEntries`, `fetchCustomerTier`, `fetchGiftCard`, `fetchReferralCode`, `fetchCustomerReferralCode`, `fetchReferralProgram`

Utilities: `generateWhatsAppShareUrl`, `copyToClipboard`, `formatBucketLabel`, `formatMovementLabel`, `getMovementPrefix`, `maskPhone`

UI Components: `PhoneInput`, `BalanceCard`, `TierCard`, `TransactionCard`, `TransactionList`, `ReferralCard`, `GiftCardStatus`, `CampaignBanner`, `MerchantHeader`, `ShareButtons`

## Module Dependency Graph

```
foundation  <-- every other module
admin       <-- platform (uses formatMetricValue via admin utils)
                settings (uses Merchant type from admin)
                analytics (uses formatCurrencyINR via foundation)
storefront  -- standalone consumer (no other module depends on it)
```

## Route-to-Module Mapping

| Route prefix | Layout | Primary modules |
|---|---|---|
| `/admin/*` | admin/+layout.svelte (Sidebar + top bar) | admin, transactions, customers, rules, gift-cards, referrals, settings, analytics |
| `/platform/*` | platform/+layout.svelte (PlatformSidebar + accent bar) | platform, admin (for formatMetricValue) |
| `/s/[slug]/*` | storefront/+layout.svelte (MerchantHeader, max-480px shell) | storefront |
| `/` | Root redirect | -- |

## Total Counts

- **11** modules
- **57** UI components (Svelte files in `ui/` directories)
- **11** store files
- **11** remote files
- **5** utils files
- **~55** exported types
- **~60** exported API functions
