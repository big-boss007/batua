# Phase 5: Handlers

## `src/services/campaigns/handler.rs`
- `list_templates` -- GET, returns all festive templates
- `create_from_template` -- POST, creates campaign from festive template via helpers
- `calendar` -- GET, uses reader replica, returns campaign calendar entries
- `performance` -- GET, uses reader replica, returns campaign performance metrics

## `src/services/notifications/handler.rs`
- `send_notification` -- POST, sends notification via helpers
- `create_template` -- POST, creates notification template
- `list_templates` -- GET, lists templates filtered by merchant_id query param
- `update_template` -- PUT by template id
- `create_connector` -- POST, creates connector
- `list_connectors` -- GET, lists connectors filtered by merchant_id
- `list_notification_logs` -- GET, lists logs by merchant_id + optional customer_id
- `list_merchant_notification_logs` -- GET by merchant_id path param, paginated (page/limit)

## `src/services/rules/handler.rs`
- `create_rule` -- POST /rules
- `get_rule` -- GET /rules/{id}, uses reader replica
- `list_rules` -- GET /rules, uses reader replica, defaults rule_type to "reward"
- `update_rule` -- PUT /rules/{id}
- `evaluate` -- POST /rules/evaluate, invokes rule engine
- `get_rule_performance` -- GET /rules/{id}/performance, uses reader replica
- `create_campaign` -- POST /campaigns
- `list_campaigns` -- GET /campaigns, uses reader replica, supports active_only filter

## `src/services/admin/handler.rs`
- `create_merchant` -- POST, returns 201
- `get_merchant` -- GET by id, reader replica
- `list_merchants` -- GET, paginated (page/limit, clamped 1-100), reader replica
- `update_merchant` -- PUT by id
- `get_merchant_by_slug` -- GET /admin/merchants/by-slug/{slug}, reader replica
- `bulk_credit` -- POST, processes bulk credit via helpers
- `process_dispute` -- POST, processes dispute reversal via helpers
- `create_wallet_policy` -- POST, returns 201
- `list_wallet_policies` -- GET by merchant_id path param, reader replica
- `get_geo_policy` / `create_geo_policy` / `list_all_geo_policies` -- Geo policy CRUD
- `dashboard` -- GET, system-wide stats via reader replica
- `get_merchant_stats` -- GET by id, reader replica
- `system_health` -- GET, unprocessed/failed events + expiring credits
- `recent_events` -- GET, limit clamped 1-100, reader replica
- `update_plan` -- PUT by id, updates plan_tier
- `merchant_dashboard` -- GET by merchant_id, reader replica
- `merchant_customers` -- GET by merchant_id, paginated + searchable
- `merchant_transactions` -- GET by merchant_id, paginated + filterable (search, bucket_type, movement_type)
- `merchant_analytics` -- GET by merchant_id, reader replica
- `create_coalition` -- POST, requires >= 2 merchants, returns 201
- `get_merchant_coalitions` -- GET by merchant_id, reader replica
- `coalition_transfer` -- POST, validates positive amount
- `get_coalition_transfers` -- GET by customer_id, reader replica
