# Phase 8: Integration -- Routes, Layouts, Data Loading

Status: COMPLETED

## Route Structure

```
src/routes/
  +layout.svelte            -- root: imports app.css, renders children
  +page.svelte              -- landing page / redirect
  +page.ts                  -- root load

  admin/
    +layout.svelte          -- admin shell: Sidebar + top bar + ThemeSwitcher
    +layout.ts              -- empty load (returns {})
    +page.svelte            -- dashboard (MetricCard grid, QuickActions)
    +page.ts                -- loads dashboard stats

    transactions/
      +page.svelte          -- TransactionTable, TransactionFilters, BalanceCard
      +page.ts              -- loads merchant transactions

    customers/
      +page.svelte          -- CustomerSearch, customer list table
      +page.ts              -- loads merchant customers

    loyalty/
      +page.svelte          -- LoyaltyProgramForm, TierForm, TierDistributionChart
      +page.ts              -- loads loyalty program and tiers

    rules/
      +page.svelte          -- RulesList, RuleForm
      +page.ts              -- loads merchant rules

    campaigns/
      +page.svelte          -- CampaignsList, FestiveTemplateGrid, CampaignForm
      +page.ts              -- loads campaigns and templates

    gift-cards/
      +page.svelte          -- GiftCardsList, IssueGiftCardForm, BulkIssueForm
      +page.ts              -- loads merchant gift cards

    referrals/
      +page.svelte          -- ReferralProgramForm, ReferralCodesList, CreateCodeForm, ConversionsList
      +page.ts              -- loads referral program and codes

    analytics/
      +page.svelte          -- OverviewCards, CodMetrics, RtoComparison, CampaignPerformanceTable
      +page.ts              -- loads analytics data

    settings/
      +page.svelte          -- WalletPoliciesList, ConnectorsList, NotificationTemplateEditor
      +page.ts              -- loads wallet policies, connectors, templates

    notifications/
      +page.svelte          -- notification logs view
      +page.ts              -- loads notification logs

    setup/
      +page.svelte          -- merchant setup/onboarding
      +page.ts              -- loads merchant config

  platform/
    +layout.svelte          -- platform shell: PlatformSidebar + accent bar + ThemeSwitcher
    +layout.ts              -- empty load (returns {})
    +page.svelte            -- platform dashboard (StatsGrid, HealthCard grid)
    +page.ts                -- loads platform dashboard stats

    merchants/
      +page.svelte          -- MerchantTable, OnboardForm
      +page.ts              -- loads all merchants

      [id]/
        +page.svelte        -- merchant detail (stats, edit)
        +page.ts            -- loads single merchant + stats

    geo-policies/
      +page.svelte          -- GeoPolicyTable, GeoPolicyForm
      +page.ts              -- loads geo policies

    system/
      +page.svelte          -- SystemHealth cards, EventsTable
      +page.ts              -- loads system health + recent events

    defaults/
      +page.svelte          -- platform default settings

  s/[slug]/
    +layout.svelte          -- storefront shell: MerchantHeader, 480px max-width, error state
    +layout.ts              -- loads merchant by slug (fetchMerchantBySlug)
    +page.svelte            -- phone input -> balance/tier/transactions view
    +page.ts                -- passes layout data through

    balance/
      +page.svelte          -- BalanceCard (storefront), TransactionList
      +page.ts              -- loads wallet balance + entries

    gift-cards/check/
      +page.svelte          -- GiftCardStatus check by code
      +page.ts              -- loads gift card by code

    refer/
      +page.svelte          -- ReferralCard, ShareButtons (for existing customers)
      +page.ts              -- loads customer referral code + program info

      [code]/
        +page.svelte        -- referral landing page (for new customers clicking shared link)
        +page.ts            -- loads referral code info
```

## Layout Hierarchy

### Root Layout (`+layout.svelte`)
- Imports `app.css` (global styles and design tokens)
- Renders children with no wrapper

### Admin Layout (`admin/+layout.svelte`)
- Uses `Sidebar` component (from admin/ui) with navigation items
- Top bar with mobile toggle, merchant name badge, `ThemeSwitcher`
- Calls `ensureMerchantSelected()` on mount to restore or auto-select merchant
- Subscribes to `sidebarStore` and `currentMerchant` for reactive state
- Sticky top bar with `z-index: var(--z-sticky)`

### Platform Layout (`platform/+layout.svelte`)
- Uses `PlatformSidebar` component with platform-specific nav items
- Accent bar ("Breeze Platform") with primary color background
- Top bar with mobile toggle and `ThemeSwitcher`
- Same sidebar/topbar pattern as admin but distinct visual identity

### Storefront Layout (`s/[slug]/+layout.svelte`)
- Loads merchant by slug in `+layout.ts` via `fetchMerchantBySlug`
- If merchant found: shows `MerchantHeader` + content area
- If not found: shows error state with "Store not found" message
- Max-width 480px, centered (mobile-first design)
- Syncs merchant to `merchantContext` store via `$derived.by()`
- Auto-detects system dark/light preference and applies theme

## Data Loading Patterns

### Pattern 1: Layout-Level Load (Storefront)

`s/[slug]/+layout.ts` fetches the merchant, passes it to all child routes:
```ts
export const load: LayoutLoad = async ({ params }) => {
  const result = await fetchMerchantBySlug(params.slug);
  if (result.tag === 'success') {
    return { merchant: result.data };
  }
  return { merchant: null };
};
```

### Pattern 2: Page-Level Load (Most Admin/Platform Pages)

Each `+page.ts` calls the relevant remote functions and returns data:
```ts
export const load: PageLoad = async () => {
  const result = await fetchSomething(merchantId);
  return { data: result.tag === 'success' ? result.data : fallback };
};
```

### Pattern 3: Component-Level Fetch (MerchantSelector)

Some components fetch their own data on mount (e.g., `MerchantSelector` calls `fetchMerchants()` directly). This is used for components that need to self-initialize regardless of the route.

### Pattern 4: Orchestrated Fetch (getCustomerDetail)

`customers/remote.ts` `getCustomerDetail` chains multiple API calls sequentially:
1. Fetch customer by ID
2. Lookup wallet by merchant + customer
3. Fetch wallet balance
4. Fetch recent entries (limit 5)
5. Assemble and return `CustomerDetail`

## Route Protection

No authentication middleware is currently implemented at the route level. The three route groups (admin, platform, storefront) are structurally separated but not access-controlled in the frontend.

## Navigation

- **Admin sidebar**: Links to all admin routes via `NavItem[]` array
- **Platform sidebar**: Links to platform routes (dashboard, merchants, geo-policies, system, events, defaults)
- **Storefront**: Tab-based navigation within the 480px shell
- **Cross-zone**: Admin sidebar footer has "Switch Merchant" link to `/platform/merchants`
- **Programmatic**: `goto()` from `$app/navigation` used in MerchantTable row clicks
