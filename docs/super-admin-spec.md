# Super Admin (Platform Provider) — Specification

The Breeze super-admin is the platform provider's control panel. This is where the Breeze team manages merchants, monitors the system, configures geo policies, and sets platform-wide defaults. Separate from the per-merchant admin.

## URL Structure

```
/platform                           → System dashboard
/platform/merchants                 → Merchant list + onboarding
/platform/merchants/{id}            → Single merchant detail + config
/platform/geo-policies              → Geo policy management
/platform/defaults                  → Platform-wide default rules
/platform/system                    → System health + diagnostics
```

## Pages

### 1. System Dashboard (`/platform`)

The 30,000-foot view of the Breeze platform.

**Metrics cards:**
- Total merchants (active / inactive)
- Total customers across all merchants
- Total wallets
- Total ledger entries
- Total value in system (sum of all active credits)
- Total value redeemed (all time)

**Charts/visual:**
- Merchant growth over time (placeholder — needs time-series data later)
- Top 5 merchants by active credits
- System-wide bucket distribution (how much in earned vs gift card vs COD pending)

**Recent activity:**
- Last 10 merchants onboarded
- Last 10 events processed (across all merchants)

### 2. Merchant List (`/platform/merchants`)

Browse, search, and manage all merchants on the platform.

**Table columns:**
- Name
- Slug
- Domain
- Geo policy (India / none)
- Plan tier (Free / Grow / Scale / Enterprise) — new field, default "Free"
- Status (Active / Inactive) — Pill
- Wallets count
- Total credits
- Created date

**Actions:**
- Search by name / domain / slug
- Filter by status, geo policy, plan tier
- Click row → merchant detail
- "+ Onboard Merchant" button

**Onboard merchant form (Modal):**
- Name (required)
- External ID / Shopify shop ID
- Domain
- Slug (auto-generated from name, editable)
- Geo policy (dropdown: India, or none)
- Plan tier (dropdown)

### 3. Merchant Detail (`/platform/merchants/{id}`)

Deep view into a single merchant. Everything the platform provider needs to manage one merchant.

**Sections:**

**Info card:**
- Name, slug, domain, external ID
- Geo policy
- Plan tier (editable dropdown)
- Status toggle (active/inactive)
- Created date, last updated

**Stats row:**
- Total wallets
- Total customers
- Total ledger entries
- Active credits value
- Total redeemed value

**Configuration:**
- Reward rules count (link to merchant's rules)
- Wallet policies configured (Y/N per bucket)
- Loyalty program status (active/not set up)
- Referral program status (active/not set up)
- Connectors configured (list: WhatsApp, SMS, etc.)
- Notification templates count

**Recent orders:**
- Last 10 events for this merchant

**Actions:**
- "Open Merchant Admin" — link to `/admin` with this merchant pre-selected
- "Deactivate Merchant"
- "Reset Merchant Data" (dangerous, with confirmation)

### 4. Geo Policies (`/platform/geo-policies`)

Manage geography-specific configuration layers.

**List view:**
- Geo code (india, indonesia, etc.)
- Name
- Status (active/inactive)
- Merchants using this policy (count)
- Config summary (COD enabled, WhatsApp default, etc.)

**Create/Edit form:**
- Geo code (lowercase, unique)
- Name
- Config JSON editor with known fields:
  - `cod_enabled: boolean`
  - `default_currency: string`
  - `whatsapp_default: boolean`
  - `upi_topup_enabled: boolean`
  - `default_timezone: string`

### 5. Platform Defaults (`/platform/defaults`)

Manage the `breeze-defaults` layer that all merchants inherit.

**Sections:**

**Default wallet policies:**
- For each bucket type, show the platform default constraints
- Merchants can override these, but this is the fallback

**Default notification templates:**
- System-level templates (merchant_id = NULL in connectors/templates tables)
- earn_credit, redemption_success, cod_delivered, tier_upgrade, etc.

**Default connectors:**
- System-level connector configs (fallback when merchant has no connector for a capability)

### 6. System Health (`/platform/system`)

Operational diagnostics for the Breeze team.

**Sections:**

**Database:**
- Connection pool status
- Table row counts
- Largest tables by size

**Processing:**
- Events in "received" state (unprocessed queue depth)
- Events in "failed" state (need attention)
- COD orders in "pending" state (awaiting delivery signal)

**Expiry:**
- Credits expiring in next 7 days (count + value)
- Credits expiring in next 30 days

## Backend Endpoints Needed

Most of these already exist. New ones needed:

| Endpoint | Status |
|----------|--------|
| `GET /admin/dashboard` | Exists — system-wide stats |
| `GET /admin/merchants` | Exists — paginated list |
| `GET /admin/merchants/{id}` | Exists |
| `POST /admin/merchants` | Exists |
| `PUT /admin/merchants/{id}` | Exists |
| `GET /admin/merchants/by-slug/{slug}` | Exists |
| `GET /admin/geo-policies/{geo_code}` | Exists |
| `POST /admin/geo-policies` | Exists |
| **`GET /admin/geo-policies`** | **NEW — list all geo policies** |
| **`GET /admin/merchants/{id}/stats`** | **NEW — per-merchant stats (wallets, credits, entries)** |
| **`GET /admin/system/health`** | **NEW — DB pool, queue depths, expiry counts** |
| **`GET /admin/events/recent`** | **NEW — last N events across all merchants** |
| **`PUT /admin/merchants/{id}/plan`** | **NEW — update plan tier** |

## Data Model Changes

### Merchants table
Add `plan_tier` column:
```sql
ALTER TABLE merchants ADD COLUMN plan_tier TEXT NOT NULL DEFAULT 'free';
-- Values: 'free', 'grow', 'scale', 'enterprise'
```

## Auth (Future)

For MVP: no auth on super-admin — it's localhost only.

Future: API key or session-based auth. Super-admin endpoints should be behind a middleware that checks for a platform API key. This is NOT the merchant's auth — it's the Breeze team's internal access.

## Design

- **NOT mobile-first** — this is a desktop tool for the Breeze team
- Full-width layout (no 480px constraint)
- Same design tokens as merchant admin (CSS custom properties from app.css)
- Use @juspay/svelte-ui-components: Table, Pill, Button, Modal, Tabs, Select, Toggle, Pagination
- Separate sidebar from merchant admin — platform nav is different

## Sidebar Navigation

```
Platform
├── Dashboard        (/platform)
├── Merchants        (/platform/merchants)
├── Geo Policies     (/platform/geo-policies)
├── Defaults         (/platform/defaults)
└── System           (/platform/system)
```

## Relationship to Merchant Admin

| Aspect | Super Admin (`/platform`) | Merchant Admin (`/admin`) |
|--------|---------------------------|---------------------------|
| Audience | Breeze team | Merchant's team |
| Scope | All merchants | One merchant |
| Merchant selector | N/A (sees all) | Dropdown to pick one |
| Features | Onboarding, plan management, system health | Rules, customers, campaigns, etc. |
| Auth (future) | Platform API key | Merchant session/OAuth |
