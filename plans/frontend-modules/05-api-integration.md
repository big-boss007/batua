# Phase 5: API Integration

Status: COMPLETED

## API Caller Pattern

All API communication flows through the `APICaller` singleton in `foundation/remote.ts`.

### Request Flow

```
Component/load function
  -> module remote.ts function (e.g., fetchGiftCards)
    -> apiCaller.get/post/put/patch/delete
      -> request() internal function
        -> buildUrl() constructs full URL
        -> fetch() with JSON body and headers
        -> response handling (ok -> decoder, !ok -> error extraction)
      -> returns APIResult<T>
```

### Result Handling Pattern

All callers check the discriminated union tag:
```ts
const result = await fetchSomething(id);
if (result.tag === 'success') {
  // use result.data (typed as T)
} else {
  // handle result.message, result.status
}
```

### Decoder Pattern

Each `remote.ts` defines decoder functions that transform `unknown` API responses into typed values. Two approaches are used:

1. **Field-by-field decoding** (safer, used in most modules):
```ts
function decodeMerchant(raw: unknown): Merchant {
  const r = raw as Record<string, unknown>;
  return {
    id: (r['id'] as string) ?? '',
    name: (r['name'] as string) ?? '',
    // ...
  };
}
```

2. **Direct cast** (used in rules, settings where types match API exactly):
```ts
function decodeRule(raw: unknown): Rule {
  return raw as Rule;
}
```

List decoders handle both array responses and wrapped object responses:
```ts
function decodeList(raw: unknown): Array<T> {
  if (Array.isArray(raw)) return raw.map(decodeItem);
  const r = raw as Record<string, unknown>;
  const items = (r['items_key'] ?? r['data'] ?? []) as Array<unknown>;
  if (!Array.isArray(items)) return [];
  return items.map(decodeItem);
}
```

## remote.ts Files -- API Endpoint Map

### admin/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchDashboardStats` | GET | `/admin/dashboard` | `DashboardStats` |
| `fetchMerchant` | GET | `/admin/merchants/{id}` | `Merchant` |
| `fetchMerchants` | GET | `/admin/merchants?page=&limit=` | `Merchant[]` |
| `fetchMerchantDashboard` | GET | `/admin/merchants/{id}/dashboard` | `MerchantDashboard` |

### platform/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchDashboardStats` | GET | `/admin/dashboard` | `DashboardStats` |
| `fetchMerchants` | GET | `/admin/merchants?page=&limit=` | `PlatformMerchant[]` |
| `fetchMerchant` | GET | `/admin/merchants/{id}` | `PlatformMerchant` |
| `fetchMerchantStats` | GET | `/admin/merchants/{id}/stats` | `MerchantStats` |
| `createMerchant` | POST | `/admin/merchants` | `PlatformMerchant` |
| `updateMerchant` | PUT | `/admin/merchants/{id}` | `PlatformMerchant` |
| `updateMerchantPlan` | PUT | `/admin/merchants/{id}/plan` | `PlatformMerchant` |
| `fetchGeoPolicies` | GET | `/admin/geo-policies` | `GeoPolicy[]` |
| `createGeoPolicy` | POST | `/admin/geo-policies` | `GeoPolicy` |
| `fetchSystemHealth` | GET | `/admin/system/health` | `SystemHealth` |
| `fetchRecentEvents` | GET | `/admin/events/recent?limit=` | `RecentEvent[]` |

### transactions/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchEntries` | GET | `/wallets/{id}/entries?page=&limit=&bucket_type=&movement_type=` | `LedgerResponse` |
| `fetchBalance` | GET | `/wallets/{id}/balance` | `WalletBalance` |
| `fetchRedemptions` | GET | `/redemptions?page=&limit=` | `RedemptionResponse` |
| `lookupWallet` | GET | `/wallets/lookup?merchant_id=&customer_id=` | `WalletLookupResult` |
| `fetchMerchantTransactions` | GET | `/admin/merchants/{id}/transactions?search=&bucket_type=&movement_type=&page=&limit=` | `MerchantTransactionRow[]` |
| `fetchEntryDetail` | GET | `/entries/{id}` | `LedgerEntryDetail` |

### customers/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `searchCustomers` | GET | `/identity/customers?phone=` or `?external_id=` | `Customer[]` |
| `getCustomerDetail` | GET | Orchestrated: `/identity/customers/{id}` + `/wallets/lookup` + `/wallets/{id}/balance` + `/wallets/{id}/entries?limit=5` | `CustomerDetail` |
| `fetchMerchantCustomers` | GET | `/admin/merchants/{id}/customers?search=&page=&limit=` | `MerchantCustomerRow[]` |
| `fetchLoyaltyProgram` | GET | `/loyalty/programs/{merchantId}` | `LoyaltyProgram` |
| `fetchTiers` | GET | `/loyalty/programs/{programId}/tiers` | `LoyaltyTier[]` |
| `fetchTierDistribution` | GET | `/loyalty/distribution/{merchantId}` | `TierDistribution[]` |
| `createProgram` | POST | `/loyalty/programs` | `LoyaltyProgram` |
| `createTier` | POST | `/loyalty/tiers` | `LoyaltyTier` |
| `evaluateTier` | POST | `/loyalty/programs/{merchantId}/evaluate` | `{ evaluated: number }` |

Note: `getCustomerDetail` is the most complex -- it orchestrates 4 sequential API calls to assemble a `CustomerDetail` object from separate endpoints.

### gift-cards/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `issueGiftCard` | POST | `/gift-cards/issue` | `GiftCard` |
| `bulkIssue` | POST | `/gift-cards/bulk-issue` | `GiftCard[]` |
| `fetchGiftCards` | GET | `/gift-cards/merchant/{merchantId}` | `GiftCard[]` |
| `getGiftCardByCode` | GET | `/gift-cards/{code}` | `GiftCard` |
| `claimGiftCard` | POST | `/gift-cards/claim` | `GiftCard` |
| `redeemGiftCard` | POST | `/gift-cards/redeem` | `GiftCard` |
| `fetchGiftCardStats` | GET | `/gift-cards/merchant/{merchantId}/stats` | `GiftCardStats` |

### referrals/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchProgram` | GET | `/referrals/programs/{merchantId}` | `ReferralProgram` |
| `createProgram` | POST | `/referrals/programs` | `ReferralProgram` |
| `createCode` | POST | `/referrals/codes` | `ReferralCode` |
| `fetchCodeByCode` | GET | `/referrals/codes/{code}` | `ReferralCode` |
| `processConversion` | POST | `/referrals/convert` | `ReferralConversion` |
| `fetchAnalytics` | GET | `/referrals/analytics/{merchantId}` | `ReferralAnalytics` |
| `fetchConversions` | GET | `/referrals/conversions/{merchantId}` | `ReferralConversion[]` |
| `fetchMerchantCodes` | GET | `/referrals/merchant/{merchantId}/codes?page=&limit=` | `ReferralCode[]` |

### rules/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchRules` | GET | `/rules?merchant_id=` | `Rule[]` |
| `createRule` | POST | `/rules` | `Rule` |
| `updateRule` | PUT | `/rules/{id}` | `Rule` |
| `fetchCampaigns` | GET | `/campaigns/calendar?merchant_id=` | `Campaign[]` |
| `createCampaignFromTemplate` | POST | `/campaigns/from-template` | `Campaign` |
| `fetchFestiveTemplates` | GET | `/campaigns/templates` | `FestiveTemplate[]` |
| `fetchCampaignCalendar` | GET | `/campaigns/calendar?merchant_id=&from=&to=` | `CampaignCalendarEntry[]` |
| `fetchRulePerformance` | GET | `/rules/{id}/performance` | `RulePerformance` |

### settings/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchWalletPolicies` | GET | `/admin/wallet-policies/{merchantId}` | `WalletPolicy[]` |
| `updateWalletPolicy` | PUT | `/wallets/policies/{policyId}` | `WalletPolicy` |
| `fetchConnectors` | GET | `/notifications/connectors?merchant_id=` | `Connector[]` |
| `createConnector` | POST | `/notifications/connectors` | `Connector` |
| `fetchTemplates` | GET | `/notifications/templates?merchant_id=` | `NotificationTemplate[]` |
| `updateTemplate` | PUT | `/notifications/templates/{templateId}` | `NotificationTemplate` |
| `updateMerchantProfile` | PUT | `/admin/merchants/{id}` | `Merchant` |
| `fetchNotificationLogs` | GET | `/notifications/logs?merchant_id=&page=&limit=` | `NotificationLog[]` |

### analytics/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchMerchantAnalytics` | GET | `/admin/merchants/{id}/analytics` | `MerchantAnalytics` |
| `fetchCodAnalytics` | GET | `/cod/analytics/{merchantId}?from=&to=` | `CodAnalytics` |
| `fetchCampaignPerformance` | GET | `/admin/dashboard?from=&to=` | `CampaignPerformance[]` |
| `fetchOverviewMetrics` | GET | `/admin/dashboard?from=&to=` | `OverviewMetrics` |

### storefront/remote.ts

| Function | Method | Endpoint | Returns |
|---|---|---|---|
| `fetchMerchantBySlug` | GET | `/admin/merchants/by-slug/{slug}` | `StorefrontMerchant` |
| `lookupCustomer` | GET | `/identity/customers?phone=` | `CustomerIdentity[]` |
| `lookupWallet` | GET | `/wallets/lookup?merchant_id=&customer_id=` | `WalletLookup` |
| `fetchBalance` | GET | `/wallets/{id}/balance` | `CustomerBalance` |
| `fetchEntries` | GET | `/wallets/{id}/entries?limit=` | `TransactionEntry[]` |
| `fetchCustomerTier` | GET | `/loyalty/customers/{merchantId}/{customerId}` | `CustomerTierInfo` |
| `fetchGiftCard` | GET | `/gift-cards/{code}` | `GiftCardInfo` |
| `fetchReferralCode` | GET | `/referrals/codes/{code}` | `ReferralCodeInfo` |
| `fetchCustomerReferralCode` | GET | `/referrals/codes/customer/{merchantId}/{customerId}` | `ReferralCodeInfo` |
| `fetchReferralProgram` | GET | `/referrals/programs/{merchantId}` | `ReferralProgramInfo` |

## API Base URL Prefixes

| Prefix | Service Area |
|---|---|
| `/admin/*` | Admin/merchant management |
| `/wallets/*` | Wallet operations |
| `/identity/*` | Customer identity |
| `/loyalty/*` | Loyalty programs and tiers |
| `/gift-cards/*` | Gift card operations |
| `/referrals/*` | Referral system |
| `/rules` | Reward rules |
| `/campaigns/*` | Campaign management |
| `/cod/*` | COD analytics |
| `/notifications/*` | Notification connectors, templates, logs |
| `/redemptions` | Redemption requests |
| `/entries/*` | Ledger entry detail |

## Total API Functions: ~60

Breakdown: admin (4), platform (11), transactions (6), customers (9), gift-cards (7), referrals (8), rules (8), settings (8), analytics (4), storefront (10)
