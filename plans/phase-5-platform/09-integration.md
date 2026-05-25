# Phase 5: Integration

## Router Registration (`src/main.rs`)
All four services merged into the application router:
- `.merge(services::rules::router())`
- `.merge(services::notifications::router())`
- `.merge(services::campaigns::router())`
- `.merge(services::admin::router())`

## Module Declaration (`src/services/mod.rs`)
- `pub mod admin`
- `pub mod campaigns`
- `pub mod notifications`
- `pub mod rules`

## Cross-Service Dependencies

### Rules -> Earn
- `earn/helpers.rs` calls `rules::helpers::evaluate_rules(pool, context)` during order processing
- Rule evaluation results drive ledger entry creation (bucket_type, earning_unit, expiry_days)

### Campaigns -> Rules
- `campaigns/helpers.rs` calls `rules::storage::create_campaign` to persist campaigns
- Campaign multipliers are applied during rule evaluation via `apply_campaign_multiplier`
- Campaigns link to rules via `base_rule_id`

### Notifications -> Connectors
- Notification sending resolves connectors by capability (whatsapp-bsp, sms, email)
- Connector fallback: merchant-specific -> system default (merchant_id IS NULL)

### Admin -> Ledger/Wallets
- Bulk credit uses `ledger::storage::create_entry` and `wallets::storage::get_or_create_wallet`
- Dispute uses `ledger::storage::create_entry` for reversal
- Coalition transfer uses `ledger::storage::create_across_movement`

### Admin -> Identity
- Merchant customer listing joins `customers` table from identity service

### Reader Replica Pattern
All read-heavy handlers use: `let pool = app_state.db_reader.as_ref().unwrap_or(&app_state.db)`

## Frontend Module Integration

### Admin Module
- `admin/store.ts` manages current merchant state with `localStorage` persistence via `utils.ts`
- `admin/utils.ts` provides `formatMetricValue` for number/currency/percentage formatting (INR locale)
- Derived store `currentMerchantId` reads from either store state or localStorage

### Settings Module
- References `admin.Merchant` type for merchant profile updates
- Fetches from both `/admin/` and `/notifications/` API paths

### Platform Module
- Independent from admin module (separate merchant list, no localStorage persistence)
- Used exclusively on `/platform/*` routes
