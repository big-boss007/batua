# Phase 4: State Management

Status: COMPLETED

## State Tiers

The frontend follows a three-tier state model:

| Tier | Mechanism | Lifecycle | Example |
|---|---|---|---|
| URL state | Query params | Survives refresh, shareable | Transaction filters, search queries, pagination, tab selection |
| Store state | Svelte stores (`writable`/`derived`) | App-level, shared across components | `currentMerchant`, `toastStore`, `sidebarStore`, `analyticsStore` |
| Component state | `$state()`/`$derived()` | Local to component, dies with unmount | Form inputs, sort direction, search query in MerchantTable |

## URL State

URL state is managed through `+page.ts` load functions and `$app/stores` page store. Modules that use URL state:

- **transactions/** -- `TransactionFilters` (bucket_type, movement_type, page, limit) flow through store but are initialized from defaults
- **routes** -- Tab selection, merchant ID in URL params (`/platform/merchants/[id]`, `/s/[slug]`)
- **storefront** -- Customer phone lookup, gift card code check via route params

## Store State -- All Stores by Module

### foundation/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `themeStore` | `Theme` ('light'\|'dark') | localStorage (`batua-theme`) + `data-theme` attr | `toggle()`, `setTheme(theme)` |
| `sidebarStore` | `SidebarState` | None (memory only) | `toggle()`, `collapse()`, `expand()` |
| `toastStore` | `Toast[]` | None | `push(input)` -> id, `dismiss(id)`, `clear()` |

### admin/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `currentMerchant` | `Merchant \| null` | localStorage (`batua_merchant_id`) via utils | `set(merchant)`, `clear()` |
| `currentMerchantId` | `string \| null` (derived) | Reads from `currentMerchant` or localStorage fallback | (read-only derived) |
| `breadcrumbs` | `Breadcrumb[]` | None | `set(crumbs)`, `clear()` |

### platform/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `merchantsList` | `PlatformMerchant[]` | None | `set(merchants)`, `clear()` |
| `selectedMerchant` | `PlatformMerchant \| null` | None | `set(merchant)`, `clear()` |

### transactions/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `transactionFilters` | `TransactionFilters` | None | Standard writable |
| `allEntries` | `LedgerEntry[]` | None | Standard writable |
| `filteredEntries` | `LedgerEntry[]` (derived) | -- | Derived from `allEntries` + `transactionFilters` |

Also exports: `resetFilters()` function, `DEFAULT_FILTERS` constant.

The `filteredEntries` store applies client-side filtering on `bucket_type` and `movement_type` from the filters store.

### customers/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `customerSearchStore` | `{ results, loading, query }` | None | `setQuery(q)`, `setLoading(b)`, `setResults(r)`, `clear()` |
| `customerDetailStore` | `{ detail, loading }` | None | `setDetail(d)`, `setLoading()`, `clear()` |
| `loyaltyStore` | `{ program, tiers, distribution, loading }` | None | `setProgram(p)`, `setTiers(t)`, `setDistribution(d)`, `setLoading(b)`, `clear()` |

### gift-cards/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `giftCards` | `GiftCard[]` | None | `set(cards)`, `add(card)`, `addMany(cards)`, `clear()` |

### referrals/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `referralProgram` | `ReferralProgram \| null` | None | `set(program)`, `clear()` |
| `referralCodes` | `ReferralCode[]` | None | `set(codes)`, `add(code)`, `clear()` |

### rules/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `rulesStore` | `Rule[]` | None | `set`, `addRule(r)`, `updateRule(r)`, `toggleRule(id)` |
| `campaignsStore` | `Campaign[]` | None | `set`, `addCampaign(c)` |
| `selectedRuleStore` | `Rule \| null` | None | `select(rule)`, `clear()` |

### settings/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `walletPoliciesStore` | `WalletPolicy[]` | None | `set`, `updatePolicy(p)` |
| `connectorsStore` | `Connector[]` | None | `set`, `addConnector(c)`, `updateConnector(c)` |
| `templatesStore` | `NotificationTemplate[]` | None | `set`, `updateTemplate(t)` |

### analytics/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `analyticsStore` | `{ cod, campaigns, overview, loading }` | None | `setCod(c)`, `setCampaigns(c)`, `setOverview(o)`, `setLoading(b)`, `clear()` |
| `dateRangeStore` | `DateRange` | None | `set(range)` -- initialized to last 30 days |

### storefront/store.ts

| Store | Type | Persistence | Methods |
|---|---|---|---|
| `customerPhone` | `string \| null` | sessionStorage (`batua-storefront-phone`) | `set(phone)`, `clear()` |
| `merchantContext` | `StorefrontMerchant \| null` | None | `set(merchant)`, `clear()` |

## Persistence Summary

| Key | Storage | Module | Purpose |
|---|---|---|---|
| `batua-theme` | localStorage | foundation | Theme preference |
| `batua_merchant_id` | localStorage | admin | Currently selected merchant |
| `batua-storefront-phone` | sessionStorage | storefront | Customer phone for session |

## Store Factory Pattern

All stores use a consistent factory pattern:

```ts
function createXxxStore() {
  const { subscribe, set, update } = writable<Type>(initialValue);
  return {
    subscribe,
    // domain-specific methods that call set/update
  };
}
export const xxxStore = createXxxStore();
```

This encapsulates the writable's `set`/`update` behind semantic method names (e.g., `toggle()`, `dismiss(id)`, `addRule(r)`) while exposing `subscribe` for the Svelte store contract.

## Component State Examples

- `MerchantTable`: `searchQuery` ($state), `filtered` ($derived) -- local search/filter
- `TransactionTable`: `sortField`, `sortDirection` ($state) -- local sort
- `MerchantSelector`: `merchants` ($state), `loaded` ($state), `merchantItems` ($derived)
- `TierCard`: `tierColorClass`, `earnLabel` ($derived.by) -- conditional rendering logic
- `PhoneInput`: `phoneValue` ($state), `isValid` ($derived) -- form validation
