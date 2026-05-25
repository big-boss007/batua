# Phase 5: Platform (Campaigns, Notifications, Analytics, Rules, Settings)

**Status:** COMPLETED

## Goal

Build the platform layer that gives merchants campaign management, notification delivery, a DSL-based rule engine, analytics dashboards, and settings management. Provide a super-admin platform view for multi-tenant oversight.

## Scope

### Backend Services
- `src/services/campaigns/` -- Festive campaign templates, calendar, performance metrics
- `src/services/notifications/` -- Template CRUD, connector abstraction, delivery logs, trigger-based sending
- `src/services/rules/` -- DSL rule engine with conditions/actions, versioned snapshots, campaign overlays
- `src/services/admin/` -- Merchant CRUD, bulk credit, disputes, wallet policies, geo-policies, dashboards, analytics, coalition transfers

### Frontend Modules
- `frontend/src/lib/client/modules/analytics/` -- COD analytics, campaign performance, overview metrics
- `frontend/src/lib/client/modules/rules/` -- Rule CRUD, campaign creation from festive templates, calendar
- `frontend/src/lib/client/modules/settings/` -- Wallet policies, connectors, notification templates, merchant profile
- `frontend/src/lib/client/modules/platform/` -- Super-admin: merchant management, geo-policies, system health, events
- `frontend/src/lib/client/modules/admin/` -- Merchant admin: dashboard stats, merchant selector, breadcrumbs, metric formatting

### Routes
- `/admin/*` -- Merchant admin panel (dashboard, analytics, campaigns, rules, customers, transactions, settings, notifications, gift-cards, loyalty, referrals)
- `/platform/*` -- Super-admin panel (merchants, geo-policies, system health, defaults)

## Success Criteria
- [x] Rule engine evaluates conditions (eq, neq, gt, gte, lt, lte, in, not_in) against event context
- [x] Campaigns overlay rules with time-bound multipliers
- [x] Notifications route through connector abstraction with template rendering
- [x] Admin dashboard aggregates merchant-level and system-level analytics
- [x] Geo-policies enable region-specific behavior
- [x] Frontend modules provide full CRUD for rules, campaigns, settings, and platform management

## Dependencies
- Phase 0: Foundation (AppState, error handling, ledger)
- Phase 1: Wallets (wallet_policies, balances)
- Phase 2: Loyalty (tiers, for tier upgrade notifications)
- Phase 3: Gift Cards
- Phase 4: Referrals
