# Customer Storefront — Implementation Plan

## Pre-work

### Backend: Merchant slug lookup endpoint
- Add `slug` column to merchants table (migration)
- Add `GET /admin/merchants/by-slug/{slug}` endpoint
- Seed script sets slug to `desi-threads` for the demo merchant

### Backend: Customer referral code lookup by customer
- Add `GET /referrals/codes/customer/{merchant_id}/{customer_id}` endpoint
- Returns the customer's referral code (if any)

## Frontend Structure

```
frontend/src/routes/s/[slug]/
├── +layout.svelte          # Merchant-branded layout (loads merchant, sets theme)
├── +layout.ts              # Resolves slug → merchant
├── +page.svelte            # Loyalty Hub
├── +page.ts                # Loads customer data after phone input
├── balance/
│   ├── +page.svelte        # Quick balance check
│   └── +page.ts
├── gift-cards/
│   └── check/
│       ├── +page.svelte    # Gift card balance check
│       └── +page.ts
└── refer/
    ├── +page.svelte        # My referrals (phone required)
    ├── +page.ts
    └── [code]/
        ├── +page.svelte    # Referral landing (public)
        └── +page.ts
```

### Frontend Module

```
frontend/src/lib/client/modules/storefront/
├── index.ts          # Barrel
├── types.ts          # Customer-facing types
├── remote.ts         # API calls for storefront pages
├── store.ts          # Phone number, merchant context
├── utils.ts          # Share helpers (WhatsApp URL, copy to clipboard)
└── ui/
    ├── index.ts
    ├── PhoneInput.svelte       # Phone entry with numeric keyboard
    ├── BalanceCard.svelte       # Big balance display with buckets
    ├── TierCard.svelte          # Tier badge + progress + benefits
    ├── TransactionCard.svelte   # Single transaction (mobile card)
    ├── TransactionList.svelte   # Recent transactions list
    ├── ReferralCard.svelte      # Code + share buttons
    ├── GiftCardStatus.svelte    # Gift card balance display
    ├── CampaignBanner.svelte    # Active campaign card
    ├── MerchantHeader.svelte    # Merchant branding header
    └── ShareButtons.svelte      # WhatsApp, copy, SMS share
```

## Implementation Order

1. **Backend: slug + referral endpoint** (quick, 2 endpoints)
2. **Storefront module** (types, remote, store, utils)
3. **Storefront layout** (merchant resolution, branding)
4. **Loyalty Hub page** (the main page — most complex)
5. **Balance Check page** (subset of Loyalty Hub)
6. **Gift Card Check page** (standalone)
7. **Referral pages** (landing + my referrals)
8. **Verify all pages in browser with devtools**

## Components from @juspay/svelte-ui-components

| Component | Used For |
|-----------|----------|
| Input | Phone entry, gift card code entry |
| Button | CTAs, share actions |
| Progress | Tier progress bar |
| Pill | Tier badge, transaction type, gift card status |
| Avatar | Customer initial |
| Shimmer | Loading placeholders |
| Toast | Copied to clipboard, errors |

## Acceptance Criteria

- [ ] `/s/desi-threads` loads merchant branding, shows phone input
- [ ] Enter Priya's phone → see ₹365 spendable, tier progress, recent transactions
- [ ] `/s/desi-threads/balance` → quick balance lookup works
- [ ] `/s/desi-threads/gift-cards/check` → enter code, see balance
- [ ] `/s/desi-threads/refer/PRIYA10` → shows referral landing with reward info
- [ ] `/s/desi-threads/refer` → enter phone, see own referral code + stats
- [ ] All pages mobile-responsive (375px)
- [ ] WhatsApp share button generates correct link
