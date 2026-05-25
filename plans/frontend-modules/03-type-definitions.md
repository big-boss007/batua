# Phase 3: Type Definitions

Status: COMPLETED

## Conventions

- All types use the `type` keyword (never `interface`)
- `null` for absence (never `undefined`)
- `import type { ... }` for type-only imports
- Types are hand-written in each module's `types.ts` (no generated types consumed yet)

## foundation/

No `types.ts` file. Types are defined inline in `remote.ts`:

```
APISuccess<T> = { tag: 'success'; data: T; status: number }
APIError = { tag: 'error'; message: string; status: number; body: unknown }
APIResult<T> = APISuccess<T> | APIError
RequestConfig = { method: 'GET'|'POST'|'PUT'|'PATCH'|'DELETE'; path: string; body?: Record<string, unknown> | null; headers?: Record<string, string>; params?: Record<string, string> }
```

Store types defined inline in `store.ts`:
```
Theme = 'light' | 'dark'
SidebarState = { collapsed: boolean }
ToastLevel = 'success' | 'error' | 'warning' | 'info'
Toast = { id: string; message: string; level: ToastLevel; durationMs: number }
ToastInput = { message: string; level: ToastLevel; durationMs?: number }
```

## admin/types.ts

```
DashboardStats
  total_merchants: number
  total_wallets: number
  total_ledger_entries: number
  total_value_in_system: number

Merchant
  id: string
  external_id: string
  name: string
  domain: string | null
  slug: string | null
  plan_tier: string | null
  currency: string
  timezone: string
  is_active: boolean

MerchantDashboard
  merchant_id: string
  active_customers: number
  total_wallets: number
  total_earned: number
  total_redeemed: number
  total_cod_pending: number
  active_credits: number
  total_ledger_entries: number
  redemption_count: number

Breadcrumb
  label: string
  href: string | null

NavItem
  label: string
  href: string
  icon: string
```

Utils type: `MetricType = 'number' | 'currency' | 'percentage'`

## platform/types.ts

```
PlatformMerchant
  id: string
  name: string
  slug: string | null
  domain: string | null
  external_id: string
  currency: string
  timezone: string
  plan_tier: string
  is_active: boolean
  geo_policy_id: string | null
  created_at: string

MerchantStats
  merchant_id: string
  total_wallets: number
  total_customers: number
  total_ledger_entries: number
  active_credits: number
  total_redeemed: number

SystemHealth
  unprocessed_events: number
  failed_events: number
  pending_cod_orders: number
  expiring_7d_count: number
  expiring_7d_value: number
  expiring_30d_count: number
  expiring_30d_value: number

GeoPolicy
  id: string
  geo_code: string
  name: string
  config: Record<string, unknown>
  is_active: boolean
  created_at: string

RecentEvent
  id: string
  merchant_id: string
  merchant_name: string
  event_type: string
  event_source: string
  state: string
  created_at: string

DashboardStats
  total_merchants: number
  total_wallets: number
  total_ledger_entries: number
  total_value_in_system: number

OnboardMerchantForm
  name: string
  external_id: string
  domain: string | null
  slug: string | null
  plan_tier: string
```

## transactions/types.ts

```
LedgerEntry
  id: string
  wallet_id: string
  bucket_type: string
  movement_type: string
  earning_unit: number
  currency_equivalent: number
  conversion_rate: number
  state: string
  event_id: string | null
  actor_type: string
  created_at: string
  expires_at: string | null

LedgerEntryDetail (extended)
  -- all LedgerEntry fields plus:
  idempotency_key: string
  rule_snapshot_id: string | null
  campaign_snapshot_id: string | null
  actor_id: string | null
  payment_reference: string | null
  transfer_id: string | null
  constraints: Record<string, unknown>
  customer_name: string | null
  customer_phone: string | null
  customer_email: string | null
  rule_name: string | null
  campaign_name: string | null
  event_type: string | null
  linked_entry_id: string | null

WalletBalance
  wallet_id: string
  displayed_balance: number
  spendable_balance: number
  buckets: Array<BucketBalance>

BucketBalance
  bucket_type: string
  displayed: number
  spendable: number
  count: number

RedemptionRequest
  id: string
  wallet_id: string
  state: string
  requested_amount: number
  applied_amount: number | null
  order_id: string
  created_at: string

TransactionFilters
  bucket_type: string | null
  movement_type: string | null
  page: number
  limit: number

LedgerResponse
  entries: Array<LedgerEntry>
  total: number
  page: number
  limit: number

RedemptionResponse
  redemptions: Array<RedemptionRequest>
  total: number
  page: number
  limit: number

WalletLookupResult
  wallet_id: string
  customer_id: string
  merchant_id: string
  created_at: string

MerchantTransactionRow
  entry_id: string
  wallet_id: string
  customer_name: string | null
  customer_phone: string
  bucket_type: string
  movement_type: string
  currency_equivalent: number
  state: string
  created_at: string
```

Utils types: `MovementHint = { label: string; color: string }`, `StateHint = { label: string; color: string }`

## customers/types.ts

```
Customer
  id: string
  phone: string
  email: string | null
  name: string | null
  is_verified: boolean
  created_at: string

CustomerDetail
  customer: Customer
  wallet: WalletSummary | null
  tier: CustomerTierInfo | null
  recent_entries: Array<LedgerEntrySummary>

WalletSummary
  id: string
  displayed_balance: number
  spendable_balance: number

CustomerTierInfo
  tier_name: string
  rank: number
  earn_rate_multiplier: number
  progress_to_next: TierProgress | null

TierProgress
  next_tier_name: string
  current_value: number
  threshold: number
  percentage: number

LedgerEntrySummary
  id: string
  bucket_type: string
  movement_type: string
  currency_equivalent: number
  created_at: string

LoyaltyProgram
  id: string
  name: string
  evaluation_criteria: string
  is_active: boolean

LoyaltyTier
  id: string
  name: string
  rank: number
  threshold: number
  earn_rate_multiplier: number
  benefits: Record<string, unknown>

TierDistribution
  tier_name: string
  count: number

MerchantCustomerRow
  customer_id: string
  customer_name: string | null
  customer_phone: string
  customer_email: string | null
  wallet_id: string
  created_at: string
```

## gift-cards/types.ts

```
GiftCard
  id: string
  code: string
  initial_amount: number
  current_amount: number
  is_claimed: boolean
  is_active: boolean
  expires_at: string | null
  created_at: string

IssueGiftCardForm
  merchant_id: string
  amount: number
  expires_at: string | null

BulkIssueForm
  merchant_id: string
  batch_id: string
  cards: Array<{ amount: number; recipient_phone: string | null }>

BulkIssueInput
  cards: Array<{ amount: number; recipient_phone: string | null }>

GiftCardStats
  total_issued: number
  total_outstanding_value: number
  total_redeemed_value: number
  total_expired: number
  total_active: number
  total_claimed: number
```

## referrals/types.ts

```
ReferralProgram
  id: string
  referrer_reward_amount: number
  referee_reward_amount: number
  max_referrals_per_customer: number | null
  is_active: boolean

ReferralCode
  id: string
  code: string
  customer_id: string
  is_vanity: boolean
  is_creator: boolean
  commission_rate: number | null
  total_referrals: number
  total_conversions: number
  is_active: boolean

ReferralAnalytics
  total_codes: number
  total_referrals: number
  total_conversions: number
  total_suspicious: number
  conversion_rate: number

ReferralConversion
  id: string
  referrer_id: string
  referee_id: string
  order_id: string | null
  is_suspicious: boolean
  fraud_signals: Array<string>
  created_at: string
```

## rules/types.ts

```
Rule
  id: string
  merchant_id: string
  rule_type: string
  name: string
  config: RewardRuleConfig
  version: number
  is_active: boolean
  created_at: string

RewardRuleConfig
  event_type: string
  conditions: Array<Condition>
  action: RewardAction

Condition
  field: string
  operator: string
  value: unknown

RewardAction
  bucket_type: string
  calculation: string
  value: number
  max_amount: number | null
  conversion_rate: number | null
  expiry_days: number | null

Campaign
  id: string
  merchant_id: string
  name: string
  campaign_type: string
  multiplier: number | null
  starts_at: string
  ends_at: string
  is_active: boolean

FestiveTemplate
  name: string
  display_name: string
  description: string
  default_multiplier: number
  default_duration_days: number
  category: string

CreateRuleRequest
  merchant_id: string
  rule_type: string
  name: string
  config: RewardRuleConfig

UpdateRuleRequest
  config: RewardRuleConfig
  is_active?: boolean

CreateCampaignFromTemplateRequest
  merchant_id: string
  template_name: string
  base_rule_id: string
  name: string
  multiplier: number
  starts_at: string
  ends_at: string

CampaignCalendarEntry
  id: string
  name: string
  campaign_type: string
  starts_at: string
  ends_at: string
  is_active: boolean

RulePerformance
  rule_id: string
  total_entries: number
  total_value: number
  unique_customers: number
```

## settings/types.ts

```
WalletPolicy
  id: string
  merchant_id: string
  bucket_type: string
  min_redemption: number | null
  step_size: number | null
  max_per_order_pct: number | null
  max_per_order_fixed: number | null
  stackable_with_discounts: boolean
  default_expiry_days: number | null
  is_transferable: boolean

Connector
  id: string
  capability: string
  vendor: string
  config: Record<string, unknown>
  is_active: boolean
  priority: number

NotificationTemplate
  id: string
  name: string
  channel: string
  locale: string
  body_template: string
  is_active: boolean

UpdateWalletPolicyRequest
  min_redemption: number | null
  step_size: number | null
  max_per_order_pct: number | null
  max_per_order_fixed: number | null
  stackable_with_discounts: boolean
  default_expiry_days: number | null
  is_transferable: boolean

CreateConnectorRequest
  capability: string
  vendor: string
  config: Record<string, unknown>
  priority: number

UpdateTemplateRequest
  body_template: string
  is_active: boolean

NotificationLog
  id: string
  customer_id: string
  channel: string
  status: string
  created_at: string
```

## analytics/types.ts

```
CodAnalytics
  total_pending: number
  total_delivered: number
  total_rto: number
  pending_amount: number
  released_amount: number
  cancelled_amount: number

CampaignPerformance
  campaign_id: string
  name: string
  total_entries: number
  total_value: number
  unique_customers: number
  average_reward: number

OverviewMetrics
  total_wallets: number
  total_active_credits: number
  total_redeemed: number
  total_expired: number
  rto_rate_loyalty: number
  rto_rate_non_loyalty: number

DateRange
  from: string
  to: string

MerchantAnalytics
  total_earned: number
  total_redeemed: number
  total_expired: number
  active_credits: number
  cod_pending: number
  cod_delivered: number
  cod_rto: number
  total_orders: number
  prepaid_orders: number
  cod_orders: number
  loyalty_rto_rate: number
  non_loyalty_rto_rate: number
  repeat_purchase_rate: number
```

## storefront/types.ts

```
StorefrontMerchant
  id: string
  name: string
  slug: string | null
  domain: string | null
  currency: string

CustomerBalance
  wallet_id: string
  displayed_balance: number
  spendable_balance: number
  buckets: Array<BucketBalance>

BucketBalance
  bucket_type: string
  displayed: number
  spendable: number
  count: number

CustomerTierInfo
  tier_name: string
  rank: number
  earn_rate_multiplier: number
  progress_to_next: TierProgress | null

TierProgress
  next_tier_name: string
  current_value: number
  threshold: number
  percentage: number

TransactionEntry
  id: string
  bucket_type: string
  movement_type: string
  currency_equivalent: number
  created_at: string
  state: string

GiftCardInfo
  code: string
  initial_amount: number
  current_amount: number
  is_claimed: boolean
  is_active: boolean
  expires_at: string | null
  created_at: string

ReferralCodeInfo
  code: string
  total_referrals: number
  total_conversions: number
  is_creator: boolean
  commission_rate: number | null

ReferralLandingInfo
  code: string
  merchant_name: string
  referee_reward_amount: number

ReferralProgramInfo
  referrer_reward_amount: number
  referee_reward_amount: number
  is_active: boolean

CustomerIdentity
  id: string
  phone: string
  name: string | null

WalletLookup
  id: string
```

## Shared Type Patterns

Several types appear in similar form across modules (not shared, defined independently):

| Concept | admin/ | platform/ | customers/ | storefront/ |
|---|---|---|---|---|
| Merchant | `Merchant` | `PlatformMerchant` (more fields) | -- | `StorefrontMerchant` (fewer fields) |
| Balance | -- | -- | `WalletSummary` | `CustomerBalance` (with buckets) |
| Tier info | -- | -- | `CustomerTierInfo` | `CustomerTierInfo` (same shape) |
| Tier progress | -- | -- | `TierProgress` | `TierProgress` (same shape) |
| Bucket balance | -- | -- | -- | `BucketBalance` (also in transactions) |
| Dashboard stats | `DashboardStats` | `DashboardStats` (same shape) | -- | -- |

This duplication is intentional -- each module owns its own types tailored to its API surface, avoiding coupling between modules.
