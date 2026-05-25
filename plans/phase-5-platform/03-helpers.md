# Phase 5: Helpers

## `src/services/campaigns/helpers.rs`
- `get_festive_templates()` -- Returns hardcoded list of 10 Indian festive templates: Diwali (3x/10d), Navratri (2x/9d), Holi (2x/3d), Republic Day (1.5x/3d), Independence Day (1.5x/3d), Eid (2x/3d), Christmas (2x/7d), New Year (2x/5d), Raksha Bandhan (1.5x/3d), Valentine's Day (1.5x/3d). Categories: religious, national, international.
- `create_from_template(pool, req)` -- Validates template name, applies multiplier override, calls `rules::storage::create_campaign`
- `get_calendar(pool, query)` -- Defaults date range to now..+90d, delegates to storage
- `parse_datetime(s)` -- Parses RFC3339, YYYY-MM-DDTHH:MM:SS, or YYYY-MM-DD formats

## `src/services/notifications/helpers.rs`
- `send_notification(pool, req)` -- Resolves template by name/channel/locale, finds connector by capability, renders body template, creates log entry, stubs delivery, marks as sent
- `render_template(body, variables)` -- Replaces `{{key}}` placeholders with variable values
- `trigger_notification(pool, trigger, merchant_id, customer_id, variables)` -- Convenience wrapper that maps `NotificationTrigger` to template name, gracefully skips if no template configured

## `src/services/rules/helpers.rs`
- `evaluate_rules(pool, context)` -- Core rule engine: fetches active reward rules, matches event_type, checks all conditions, creates rule snapshots, calculates reward, applies campaign multipliers if campaign is linked to the rule
- `check_conditions(conditions, context)` -- AND logic: all conditions must pass
- `check_condition(condition, context)` -- Operators: eq, neq, gt, gte, lt, lte, in, not_in
- `calculate_reward(action, context)` -- Supports "percentage" (with max cap) and "fixed" calculations; returns (earning_unit, currency_equivalent, conversion_rate)
- `apply_campaign_multiplier(result, campaign)` -- Multiplies earning_unit and currency_equivalent by campaign multiplier (defaults to 1.0)
- Private helpers: `extract_field_value` (maps field names to context values, falls back to event_payload), `values_equal`, `compare_numeric`, `value_in_list`
- **Tests:** 20 unit tests covering all condition operators, percentage/fixed rewards, max cap, campaign multiplier, event_payload fallback

## `src/services/admin/helpers.rs`
- `process_bulk_credit(pool, req)` -- Iterates customer_ids, creates wallet if needed, inserts ledger entries with ActorType::Human, returns per-customer success/failure
- `process_dispute(pool, req)` -- Fetches original entry, prevents double-reversal, creates reversal OUT entry, marks original as "reversed"
- `get_system_dashboard(pool)` -- Delegates to `storage::get_dashboard_stats`
- `transfer_coalition_credits(pool, req)` -- Validates both merchants are in same coalition, checks sender balance, calculates conversion rate (to_rate / from_rate), creates paired ledger entries via `create_across_movement`, records transfer
- `parse_bucket_type(s)` -- Maps string to BucketType enum (8 variants including membership_benefit, refund_credit)
