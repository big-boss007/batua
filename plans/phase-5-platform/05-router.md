# Phase 5: Routes

## Backend Routes

### Campaigns (`src/services/campaigns/mod.rs`)
| Method | Path | Handler |
|--------|------|---------|
| GET | `/campaigns/templates` | `list_templates` |
| POST | `/campaigns/from-template` | `create_from_template` |
| GET | `/campaigns/calendar` | `calendar` |
| GET | `/campaigns/{id}/performance` | `performance` |

### Notifications (`src/services/notifications/mod.rs`)
| Method | Path | Handler |
|--------|------|---------|
| POST | `/notifications/send` | `send_notification` |
| POST | `/notifications/templates` | `create_template` |
| GET | `/notifications/templates` | `list_templates` |
| PUT | `/notifications/templates/{id}` | `update_template` |
| POST | `/notifications/connectors` | `create_connector` |
| GET | `/notifications/connectors` | `list_connectors` |
| GET | `/notifications/logs` | `list_notification_logs` |
| GET | `/notifications/logs/{merchant_id}` | `list_merchant_notification_logs` |

### Rules (`src/services/rules/mod.rs`)
| Method | Path | Handler |
|--------|------|---------|
| POST | `/rules` | `create_rule` |
| GET | `/rules` | `list_rules` |
| GET | `/rules/{id}` | `get_rule` |
| PUT | `/rules/{id}` | `update_rule` |
| POST | `/rules/evaluate` | `evaluate` |
| GET | `/rules/{id}/performance` | `get_rule_performance` |
| POST | `/campaigns` | `create_campaign` |
| GET | `/campaigns` | `list_campaigns` |

### Admin (`src/services/admin/mod.rs`)
| Method | Path | Handler |
|--------|------|---------|
| POST | `/admin/merchants` | `create_merchant` |
| GET | `/admin/merchants` | `list_merchants` |
| GET | `/admin/merchants/{id}` | `get_merchant` |
| PUT | `/admin/merchants/{id}` | `update_merchant` |
| GET | `/admin/merchants/by-slug/{slug}` | `get_merchant_by_slug` |
| POST | `/admin/bulk-credit` | `bulk_credit` |
| POST | `/admin/disputes` | `process_dispute` |
| POST | `/admin/wallet-policies` | `create_wallet_policy` |
| GET | `/admin/wallet-policies/{merchant_id}` | `list_wallet_policies` |
| GET | `/admin/geo-policies/{geo_code}` | `get_geo_policy` |
| POST | `/admin/geo-policies` | `create_geo_policy` |
| GET | `/admin/geo-policies` | `list_all_geo_policies` |
| GET | `/admin/dashboard` | `dashboard` |
| GET | `/admin/merchants/{id}/stats` | `get_merchant_stats` |
| GET | `/admin/system/health` | `system_health` |
| GET | `/admin/events/recent` | `recent_events` |
| PUT | `/admin/merchants/{id}/plan` | `update_plan` |
| GET | `/admin/merchants/{merchant_id}/dashboard` | `merchant_dashboard` |
| GET | `/admin/merchants/{merchant_id}/customers` | `merchant_customers` |
| GET | `/admin/merchants/{merchant_id}/transactions` | `merchant_transactions` |
| GET | `/admin/merchants/{merchant_id}/analytics` | `merchant_analytics` |
| POST | `/admin/coalitions` | `create_coalition` |
| GET | `/admin/coalitions/{merchant_id}` | `get_merchant_coalitions` |
| POST | `/admin/coalitions/transfer` | `coalition_transfer` |
| GET | `/admin/coalitions/transfers/{customer_id}` | `get_coalition_transfers` |

## Frontend Routes

### Merchant Admin (`/admin/`)
- `/admin/` -- Dashboard
- `/admin/analytics/` -- Analytics
- `/admin/campaigns/` -- Campaigns
- `/admin/rules/` -- Rules
- `/admin/customers/` -- Customers
- `/admin/transactions/` -- Transactions
- `/admin/settings/` -- Settings
- `/admin/notifications/` -- Notifications
- `/admin/gift-cards/` -- Gift Cards
- `/admin/loyalty/` -- Loyalty
- `/admin/referrals/` -- Referrals
- `/admin/setup/` -- Setup

### Super-Admin (`/platform/`)
- `/platform/` -- Platform dashboard
- `/platform/merchants/` -- Merchant management
- `/platform/geo-policies/` -- Geo policy management
- `/platform/system/` -- System health
- `/platform/defaults/` -- Default settings
