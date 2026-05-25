# Phase 5: Storage

## `src/services/campaigns/storage.rs`
- `get_campaign_calendar(pool, merchant_id, from, to)` -- Fetches campaigns overlapping a date range, computes `is_currently_running` flag
- `get_campaign_performance(pool, campaign_id)` -- Joins `campaign_snapshots` to `ledger_entries` for total_entries, total_value, unique_customers

## `src/services/notifications/storage.rs`
- `get_template(pool, merchant_id, name, channel, locale)` -- Finds active template by merchant+name+channel+locale
- `create_template(pool, req)` -- Inserts notification template with default locale "en"
- `list_templates(pool, merchant_id)` -- Lists all templates for a merchant
- `update_template(pool, id, req)` -- Partial update of template_id, body_template, variables, is_active
- `get_connector(pool, merchant_id, capability)` -- Finds active connector by priority, falls back to system defaults (merchant_id IS NULL)
- `create_connector(pool, req)` -- Inserts connector, handles unique violation
- `list_connectors(pool, merchant_id)` -- Lists merchant + system connectors
- `create_notification_log(pool, merchant_id, customer_id, template_id, channel, variables)` -- Creates pending log entry
- `update_notification_status(pool, log_id, status, external_message_id)` -- Updates status, sets sent_at on "sent"
- `list_notification_logs(pool, merchant_id, customer_id)` -- Lists logs, optionally filtered by customer
- `list_notification_logs_paginated(pool, merchant_id, customer_id, page, limit)` -- Paginated version

## `src/services/rules/storage.rs`
- `create_rule(pool, req)` -- Inserts rule with version 1
- `update_rule(pool, id, config)` -- Updates config, increments version
- `get_rule(pool, id)` -- Fetch single rule
- `get_active_rules(pool, merchant_id, rule_type)` -- All active rules of a type for a merchant
- `create_rule_snapshot(pool, rule)` -- Immutable snapshot of rule config at evaluation time
- `get_active_campaigns(pool, merchant_id, at)` -- Active campaigns at a point in time
- `create_campaign_snapshot(pool, campaign)` -- Immutable snapshot for audit trail
- `create_campaign(pool, req)` -- Inserts campaign
- `list_campaigns(pool, merchant_id, active_only)` -- Lists campaigns, optionally filtering to currently active
- `get_rule_performance(pool, rule_id)` -- Aggregates ledger_entries via rule_snapshots

## `src/services/admin/storage.rs`
- **Merchant CRUD:** `create_merchant`, `get_merchant`, `get_merchant_by_external_id`, `get_merchant_by_slug`, `update_merchant`, `list_merchants`, `update_merchant_plan`
- **Wallet Policies:** `create_wallet_policy` (UPSERT on merchant_id+bucket_type), `get_wallet_policies`
- **Geo Policies:** `get_geo_policy`, `create_geo_policy`, `list_geo_policies`
- **Dashboard/Analytics:** `get_dashboard_stats` (system-wide counts), `get_merchant_stats`, `get_system_health` (unprocessed events, failed events, pending COD, expiring credits at 7d/30d), `get_recent_events`, `get_merchant_dashboard` (CTE-based aggregation), `list_merchant_customers` (search by phone/name), `list_merchant_transactions` (filterable by bucket_type, movement_type, phone), `get_merchant_analytics` (earned/redeemed/expired/COD breakdown)
- **Coalition:** `create_coalition`, `add_coalition_member`, `get_coalition_for_merchants` (finds shared active coalition), `get_merchant_coalitions`, `record_coalition_transfer`, `get_coalition_transfers_for_customer`
