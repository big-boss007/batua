# Customer Storefront — Specification

Customer-facing pages accessible via `/s/{merchant_slug}`. These pages are public (no auth) but require phone verification via OTP for sensitive actions. Designed mobile-first since 80%+ of Indian D2C traffic is mobile.

## Design Principles

- **Mobile-first** — 375px primary breakpoint, scales up
- **Phone is identity** — Customer enters phone to see their data
- **No login wall** — Balance check and referral sharing work without OTP
- **OTP for sensitive** — Redemption, wallet top-up, gift card claim require OTP (stubbed for now)
- **Merchant-branded** — Each merchant gets their own URL slug, colors come from merchant config
- **Fast** — No heavy frameworks on storefront, minimal JS, SSR everything

## URL Structure

```
/s/{merchant_slug}                    → Loyalty Hub (landing)
/s/{merchant_slug}/balance            → Quick balance check
/s/{merchant_slug}/gift-cards/check   → Gift card balance check
/s/{merchant_slug}/refer/{code}       → Referral landing page
/s/{merchant_slug}/refer              → My referral page (after phone entry)
```

## Pages

### 1. Loyalty Hub (`/s/{merchant_slug}`)

The main customer-facing page. Shows everything about their loyalty status.

**Flow:**
1. Landing shows merchant branding + phone input
2. Customer enters phone number
3. Page loads: balance card, tier status, recent transactions, active campaigns, referral code

**Sections:**
- **Hero** — Merchant name, "Your Rewards" heading
- **Phone input** — "Enter your phone to view rewards"
- **Balance card** — Displayed balance (big), spendable balance, per-bucket breakdown
- **Tier card** — Current tier name + badge, progress bar to next tier, tier benefits list
- **Recent transactions** — Last 5-10 entries (earn/redeem/held), mobile-friendly card layout
- **Active campaigns** — "Diwali 3x cashback! Ends in 5 days" style cards
- **Referral section** — "Share your code, earn ₹50" with code display + share buttons
- **Quick actions** — Links to gift card check, referral page

### 2. Balance Check (`/s/{merchant_slug}/balance`)

Lightweight single-purpose page for quick balance lookup.

**Flow:**
1. Phone input
2. Shows: spendable balance, displayed balance, bucket breakdown
3. Link to full Loyalty Hub

### 3. Gift Card Balance Check (`/s/{merchant_slug}/gift-cards/check`)

Check remaining balance on a gift card by code.

**Flow:**
1. Code input (format: BRZE-XXXX-XXXX-XXXX)
2. Shows: initial amount, remaining amount, usage percentage, expiry date, status (active/claimed/expired)
3. If not yet claimed: "Claim to your wallet" CTA (requires phone + OTP, stubbed)

### 4. Referral Page (`/s/{merchant_slug}/refer/{code}`)

Landing page when someone clicks a referral link.

**Flow:**
1. Shows referrer's name, the reward amount ("Get ₹25 on your first order")
2. CTA to shop at the merchant's store
3. Automatically applies referral code when customer makes first purchase

**My Referrals (`/s/{merchant_slug}/refer`):**
1. Phone input to view own referral status
2. Shows: personal referral code, copy button, WhatsApp share, total referrals, total earnings
3. If creator: additional stats (commission earned, payout info)

## API Endpoints Used

| Page | Endpoints |
|------|-----------|
| Loyalty Hub | GET /identity/customers?phone, GET /wallets/lookup, GET /wallets/{id}/balance, GET /wallets/{id}/entries, GET /loyalty/customers/{mid}/{cid} |
| Balance Check | GET /identity/customers?phone, GET /wallets/lookup, GET /wallets/{id}/balance |
| Gift Card Check | GET /gift-cards/{code} |
| Referral Landing | GET /referrals/codes/{code} |
| My Referrals | GET /identity/customers?phone, GET /referrals/codes (by customer) |

## Merchant Slug Resolution

Need a new backend endpoint or lookup:
- `GET /admin/merchants/by-slug/{slug}` — returns merchant ID from slug

Alternatively, use the merchant's `external_id` or `domain` as the slug. For MVP, use the merchant's `external_id` (e.g., `shop_desi_threads` → `/s/shop_desi_threads`).

## Mobile-First Design

- Cards stack vertically
- Large tap targets (48px min)
- Phone input with auto-focus and numeric keyboard (`inputmode="numeric"`)
- Balance in large, bold font
- Tier progress as horizontal bar
- Transactions as simple card list (no table)
- Share buttons: WhatsApp (primary), Copy Link, SMS

## Tech Notes

- All pages are SvelteKit routes under `src/routes/s/[slug]/`
- Use `+page.ts` load functions with the slug param
- No auth required — phone lookup is sufficient for read-only data
- OTP verification stubbed with a `verifyOtp(phone, code) → boolean` function that always returns true in dev
- Merchant theming: load merchant config in layout, set CSS variables for brand color
