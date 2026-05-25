# Phase 7: UI Components

Status: COMPLETED

## Component Inventory

57 Svelte components across 11 module `ui/` directories. Each uses Svelte 5 runes, scoped styles with CSS custom properties, and callbacks-as-props.

---

### admin/ui/ (4 components)

#### Sidebar
Props: `items: NavItem[]`, `collapsed: boolean`, `ontoggle: () => void`
Library components: none (custom build)
Purpose: Left navigation sidebar with collapsible state, active route highlighting via `$app/stores` page store, nav items rendered from `NavItem[]` array. Footer links to platform merchant switcher. Responsive: auto-collapses at 768px.

#### MetricCard
Props: `label: string`, `value: number`, `metricType?: MetricType` (default: 'number'), `icon?: string | null`
Library components: none (custom card)
Purpose: Displays a single dashboard metric with formatted value (number/currency/percentage) and optional icon. Uses `formatMetricValue` from admin utils.

#### MerchantSelector
Props: none (self-contained)
Library components: `Select` from @juspay/svelte-ui-components
Purpose: Dropdown to switch between merchants. Auto-loads merchant list on mount, persists selection via `currentMerchant` store. Uses `Select` component with `{ id, label }` items.

#### QuickActions
Props: `actions?: QuickAction[]` (defaults to 3 hardcoded actions: Create Rule, Issue Gift Card, View Transactions)
Library components: none
Purpose: Grid of action cards linking to key admin pages. Each card has icon, label, and description.

---

### platform/ui/ (8 components)

#### PlatformSidebar
Props: `collapsed: boolean`, `ontoggle: () => void`
Library components: none
Purpose: Platform-specific sidebar with hardcoded nav items (Dashboard, Merchants, Geo Policies, System, Events, Defaults). Similar structure to admin Sidebar.

#### MerchantTable
Props: `merchants: PlatformMerchant[]`
Library components: `Table`, `Pill`, `Input` from @juspay/svelte-ui-components
Purpose: Searchable, sortable table of merchants. Client-side search filtering by name/slug/domain. Plan tier shown as colored Pill, status as Pill. Row click navigates to `/platform/merchants/{id}` via `goto()`.

#### StatsGrid
Props: `stats: DashboardStats`
Library components: none
Purpose: Grid of 4 stat cards displaying total merchants, wallets, entries, and system value.

#### HealthCard
Props: `label: string`, `value: number`, `metricType?: MetricType`, `threshold?: 'green' | 'amber' | 'red'`, `subtitle?: string | null`
Library components: none
Purpose: Health indicator card with colored left border (green/amber/red threshold). Uses `formatMetricValue` for value display.

#### EventsTable
Props: `events: RecentEvent[]`
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of recent system events with event type, source, state pills, and timestamps.

#### OnboardForm
Props: `onSubmit: (form: OnboardMerchantForm) => void`
Library components: `Input`, `Select`, `Button` from @juspay/svelte-ui-components
Purpose: Form for onboarding a new merchant. Fields: name, external_id, domain, slug, plan_tier (select with free/grow/scale/enterprise).

#### GeoPolicyTable
Props: `policies: GeoPolicy[]`
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table listing geo policies with geo code, name, status, and creation date.

#### GeoPolicyForm
Props: `onSubmit: (data: Record<string, unknown>) => void`
Library components: `Input`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating a geo policy with geo code, name, and JSON config textarea.

---

### transactions/ui/ (4 components)

#### TransactionTable
Props: `entries: LedgerEntry[]`, `total?: number`, `page?: number`, `limit?: number`, `onPageChange: (page: number) => void`
Library components: `Table`, `Pill`, `Pagination` from @juspay/svelte-ui-components
Purpose: Paginated table of ledger entries. Local sorting by created_at/bucket_type/movement_type/earning_unit/state. Bucket type, movement type, and state shown as formatted Pill components. Currency formatted with `formatCurrencyINR`.

#### BalanceCard
Props: wallet balance data
Library components: none
Purpose: Displays wallet balance breakdown with spendable and displayed amounts.

#### TransactionFilters
Props: `filters: TransactionFilters`, `onChange: (updated: TransactionFilters) => void`
Library components: `Select` from @juspay/svelte-ui-components
Purpose: Two Select dropdowns for filtering by bucket type and movement type. Resets page to 1 on filter change.

#### RedemptionHistory
Props: redemption data
Library components: `Table` from @juspay/svelte-ui-components
Purpose: Table showing redemption request history with amounts and states.

---

### customers/ui/ (7 components)

#### CustomerSearch
Props: search-related callbacks
Library components: `Input` from @juspay/svelte-ui-components
Purpose: Search input for finding customers by phone or external ID.

#### CustomerDetail
Props: `detail: CustomerDetail`
Library components: various
Purpose: Full customer detail view showing customer info, wallet summary, tier info, and recent entries.

#### TierBadge
Props: `tierName: string`, `rank: number`, `multiplier: number`
Library components: `Pill` from @juspay/svelte-ui-components
Purpose: Colored pill showing tier name and earn rate multiplier. Uses `formatMultiplier` to show "Nx" format.

#### TierProgress
Props: `progress: TierProgress`
Library components: `Progress` from @juspay/svelte-ui-components
Purpose: Progress bar showing advancement toward next tier. Displays current value and threshold with `en-IN` locale formatting.

#### LoyaltyProgramForm
Props: form submission callback
Library components: `Input`, `Select`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating a loyalty program with name and evaluation criteria.

#### TierForm
Props: form submission callback
Library components: `Input`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating a loyalty tier with name, rank, threshold, earn rate multiplier, and benefits.

#### TierDistributionChart
Props: `distribution: Array<TierDistribution>`
Library components: `Progress` from @juspay/svelte-ui-components
Purpose: Horizontal bar chart showing customer distribution across tiers. Uses Progress bars proportional to max count. Shows total customer count. Uses `getTierColor` for tier-specific coloring.

---

### gift-cards/ui/ (4 components)

#### GiftCardsList
Props: gift card array
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of gift cards with code, amounts, status, and dates.

#### IssueGiftCardForm
Props: form submission callback
Library components: `Input`, `Button` from @juspay/svelte-ui-components
Purpose: Form for issuing a single gift card with amount and optional expiry date.

#### BulkIssueForm
Props: form submission callback
Library components: `Input`, `Button` from @juspay/svelte-ui-components
Purpose: Form for bulk issuing gift cards with multiple amount/phone rows.

#### GiftCardDetail
Props: gift card data
Library components: `Pill`, `Progress` from @juspay/svelte-ui-components
Purpose: Detailed view of a single gift card showing code, amounts, status, and usage progress.

---

### referrals/ui/ (5 components)

#### ReferralProgramForm
Props: form submission callback
Library components: `Input`, `Button` from @juspay/svelte-ui-components
Purpose: Form for configuring referral program: referrer reward, referee reward, max referrals per customer.

#### ReferralCodesList
Props: referral codes array
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of referral codes with code, customer, vanity/creator flags, stats, and status.

#### CreateCodeForm
Props: form submission callback
Library components: `Input`, `Button`, `Toggle` from @juspay/svelte-ui-components
Purpose: Form for creating a referral code with customer ID, optional custom code, vanity/creator toggles, and commission rate.

#### ReferralAnalyticsCard
Props: analytics data
Library components: none
Purpose: Summary card showing total codes, referrals, conversions, suspicious count, and conversion rate.

#### ConversionsList
Props: conversions array
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of referral conversions with referrer, referee, order, fraud signals, and timestamps.

---

### rules/ui/ (5 components)

#### RulesList
Props: `rules: Rule[]`, `onEdit: (rule: Rule) => void`, `onToggle: (rule: Rule) => void`
Library components: `Table`, `Pill`, `Toggle`, `Button` from @juspay/svelte-ui-components
Purpose: Table of reward rules with name, type, event type, version, active toggle, and edit button. Toggle fires `onToggle`, edit button fires `onEdit`.

#### RuleForm
Props: form submission callback, optional initial rule for editing
Library components: `Input`, `Select`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating/editing a rule. Fields: name, rule type, event type, conditions, action config (bucket type, calculation, value, max amount, conversion rate, expiry days).

#### CampaignsList
Props: campaigns array
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of campaigns with name, type, multiplier, date range, and status.

#### FestiveTemplateGrid
Props: templates array, onSelect callback
Library components: `Button` from @juspay/svelte-ui-components
Purpose: Grid of festive campaign template cards with name, description, default multiplier, duration, and category.

#### CampaignForm
Props: form submission callback, template data
Library components: `Input`, `Select`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating a campaign from a template with name, base rule, multiplier, start/end dates.

---

### settings/ui/ (5 components)

#### WalletPoliciesList
Props: policies array
Library components: `Table` from @juspay/svelte-ui-components
Purpose: Table of wallet policies by bucket type showing redemption rules, order limits, and feature flags.

#### WalletPolicyForm
Props: policy data, onSubmit callback
Library components: `Input`, `Toggle`, `Button` from @juspay/svelte-ui-components
Purpose: Form for editing a wallet policy: min redemption, step size, order percentage/fixed limits, stackability toggle, expiry days, transferability toggle.

#### ConnectorsList
Props: connectors array
Library components: `Table`, `Pill` from @juspay/svelte-ui-components
Purpose: Table of notification connectors with capability, vendor, priority, and status.

#### ConnectorForm
Props: form submission callback
Library components: `Input`, `Select`, `Button` from @juspay/svelte-ui-components
Purpose: Form for creating a notification connector with capability, vendor, JSON config, and priority.

#### NotificationTemplateEditor
Props: template data, onSubmit callback
Library components: `Button`, `Toggle` from @juspay/svelte-ui-components
Purpose: Editor for notification templates with template name, channel, locale display, body template textarea, and active toggle.

---

### analytics/ui/ (4 components)

#### CodMetrics
Props: `analytics: CodAnalytics`
Library components: none
Purpose: Three-card grid showing COD pending/delivered/RTO counts and amounts. Uses `formatCurrencyINR`.

#### OverviewCards
Props: `metrics: OverviewMetrics`
Library components: none
Purpose: Four-card grid showing total wallets, active credits, redeemed, and expired values.

#### CampaignPerformanceTable
Props: campaign performance data
Library components: `Table` from @juspay/svelte-ui-components
Purpose: Table of campaign performance metrics (entries, value, unique customers, average reward).

#### RtoComparison
Props: `loyaltyRate: number`, `nonLoyaltyRate: number`
Library components: none
Purpose: Side-by-side horizontal bar comparison of RTO rates for loyalty vs non-loyalty customers. Shows percentage reduction badge.

---

### storefront/ui/ (10 components)

#### PhoneInput
Props: `onSubmit: (phone: string) => void`
Library components: `Button` from @juspay/svelte-ui-components
Purpose: Phone number input with +91 prefix, 10-digit validation, submit on Enter key. Entry point for customer-facing storefront.

#### BalanceCard
Props: `balance: CustomerBalance`
Library components: `Progress` from @juspay/svelte-ui-components
Purpose: Displays spendable balance prominently, displayed balance if different, and bucket breakdown with progress bars per bucket type.

#### TierCard
Props: `tier: CustomerTierInfo | null`
Library components: `Pill`, `Progress` from @juspay/svelte-ui-components
Purpose: Tier status card showing tier name pill (color-coded by rank), earn rate multiplier, and progress bar toward next tier.

#### TransactionCard
Props: `entry: TransactionEntry`
Library components: `RelativeTime` from @juspay/svelte-ui-components
Purpose: Single transaction entry card with movement type icon (+/-), bucket label, amount with sign prefix, and relative timestamp.

#### TransactionList
Props: `entries: TransactionEntry[]`
Library components: none (composes TransactionCard)
Purpose: "Recent Activity" section rendering a list of TransactionCard components. Shows empty state message.

#### ReferralCard
Props: `code: ReferralCodeInfo`, `referralReward: number`, `merchantName: string`
Library components: none (composes ShareButtons)
Purpose: Referral card showing referral code, share URL, stats (total referrals/conversions), and share buttons. Generates WhatsApp share text with reward amount.

#### ShareButtons
Props: `text: string`, `url: string`, `onCopy: () => void`
Library components: `Button` from @juspay/svelte-ui-components
Purpose: Two buttons -- "Share on WhatsApp" (opens WhatsApp share URL) and "Copy Link" (copies to clipboard, fires onCopy callback).

#### GiftCardStatus
Props: `card: GiftCardInfo`
Library components: `Pill`, `Progress` from @juspay/svelte-ui-components
Purpose: Gift card status display with remaining amount, usage progress bar, status pill (Active/Claimed/Expired/Used), and expiry date.

#### CampaignBanner
Props: `campaign: { name: string; multiplier: number; ends_at: string }`
Library components: none
Purpose: Promotional banner for active campaigns showing campaign name, multiplier badge, and end date.

#### MerchantHeader
Props: `merchant: StorefrontMerchant`
Library components: none
Purpose: Sticky header with merchant logo (first letter), name, and primary-color brand bar. Max-width 480px (mobile-first storefront).

---

## Library Component Usage Summary

| Library Component | Used In (count) |
|---|---|
| `Table` | 10 components (MerchantTable, TransactionTable, RulesList, CampaignsList, EventsTable, GeoPolicyTable, GiftCardsList, ReferralCodesList, ConversionsList, WalletPoliciesList, ConnectorsList, CampaignPerformanceTable) |
| `Pill` | 10 components (MerchantTable, TransactionTable, RulesList, TierBadge, TierCard, GiftCardStatus, EventsTable, GeoPolicyTable, GiftCardsList, ReferralCodesList, ConversionsList, ConnectorsList, CampaignsList) |
| `Button` | 10 components (ShareButtons, OnboardForm, GeoPolicyForm, RulesList, RuleForm, CampaignForm, IssueGiftCardForm, BulkIssueForm, CreateCodeForm, WalletPolicyForm, ConnectorForm, NotificationTemplateEditor, FestiveTemplateGrid, PhoneInput) |
| `Input` | 8 components (MerchantTable, OnboardForm, GeoPolicyForm, RuleForm, CampaignForm, IssueGiftCardForm, BulkIssueForm, CreateCodeForm, WalletPolicyForm, ConnectorForm) |
| `Select` | 4 components (MerchantSelector, TransactionFilters, RuleForm, OnboardForm) |
| `Progress` | 5 components (TierProgress, TierDistributionChart, BalanceCard (storefront), TierCard, GiftCardStatus) |
| `Toggle` | 3 components (RulesList, CreateCodeForm, WalletPolicyForm, NotificationTemplateEditor) |
| `Pagination` | 1 component (TransactionTable) |
| `ThemeSwitcher` | 2 layouts (admin, platform) |
| `RelativeTime` | 1 component (TransactionCard) |
