# Phase 6: Storage

All in `src/services/earn/storage.rs` unless noted.

## Order Stats
- `get_customer_order_stats(pool, merchant_id, customer_id)` -- Reads from customer_order_stats (total_orders, total_spend, first/last_order_at)
- `update_order_stats(pool, merchant_id, customer_id, order_amount)` -- UPSERT: increments order count, adds to spend, updates last_order_at

## Milestones
- `create_milestone_config(pool, req)` -- INSERT into milestone_configs
- `get_active_milestones(pool, merchant_id)` -- Active milestones ordered by threshold ASC
- `has_achieved_milestone(pool, customer_id, milestone_id)` -- Existence check in milestone_achievements
- `record_milestone_achievement(pool, merchant_id, customer_id, milestone_id, ledger_entry_id)` -- INSERT with ON CONFLICT DO NOTHING
- `get_customer_milestones(pool, merchant_id, customer_id)` -- JOIN milestone_configs + milestone_achievements for achieved milestones

## Newsletter
- `has_newsletter_signup(pool, merchant_id, customer_id)` -- Existence check in newsletter_signups
- `record_newsletter_signup(pool, merchant_id, customer_id, ledger_entry_id, email, source)` -- INSERT with ON CONFLICT DO NOTHING
- `get_newsletter_signup_count(pool, merchant_id)` -- COUNT(*) for a merchant

## Streaks
- `create_streak_config(pool, req)` -- INSERT into streak_configs
- `get_active_streak_configs(pool, merchant_id)` -- Active configs ordered by required_orders ASC
- `count_recent_orders(pool, merchant_id, customer_id, days)` -- COUNT DISTINCT payment_reference from ledger_entries within window (earned_credit, movement_type=in, payment_reference LIKE 'order:%')
- `has_streak_achievement_in_window(pool, customer_id, streak_config_id, window_start)` -- Existence check for same window
- `record_streak_achievement(pool, merchant_id, customer_id, streak_config_id, ledger_entry_id, window_start, window_end)` -- INSERT into streak_achievements

## Spin Wheel
- `create_wheel_config(pool, merchant_id, name, daily_spin_limit)` -- INSERT into spin_wheel_configs (one per merchant via UNIQUE)
- `create_wheel_segment(pool, wheel_id, label, reward_amount, probability, color, position)` -- INSERT into spin_wheel_segments
- `get_wheel_config(pool, merchant_id)` -- Fetch spin_wheel_configs for merchant
- `get_wheel_segments(pool, wheel_id)` -- Segments ordered by position ASC
- `count_spins_today(pool, merchant_id, customer_id)` -- COUNT from spin_results where spun_at >= CURRENT_DATE
- `record_spin_result(pool, merchant_id, customer_id, segment_id, reward_amount, ledger_entry_id)` -- INSERT into spin_results

## Memberships
- `create_membership_plan(pool, req)` -- INSERT into membership_plans (defaults: earn_rate_multiplier=1.5, benefits={})
- `get_membership_plans(pool, merchant_id)` -- Active plans ordered by price ASC
- `get_membership_plan(pool, id)` -- Single plan by ID
- `subscribe_customer(pool, merchant_id, customer_id, plan_id, expires_at)` -- INSERT into customer_memberships with status='active'
- `get_customer_membership(pool, merchant_id, customer_id)` -- Latest membership by created_at
- `get_customer_membership_by_id(pool, membership_id)` -- By ID
- `renew_membership(pool, membership_id, new_expires_at)` -- UPDATE: extends expires_at, increments renewed_count, sets status='active'
- `cancel_membership(pool, membership_id)` -- UPDATE: status='cancelled', cancelled_at=now()
- `expire_membership(pool, membership_id)` -- UPDATE: status='expired'
- `get_expired_memberships(pool)` -- Active memberships where expires_at < now() (for scheduled expiry)

## Coalition (in `src/services/admin/storage.rs`)
- `create_coalition(pool, name)` -- INSERT into coalitions
- `add_coalition_member(pool, coalition_id, merchant_id, conversion_rate)` -- INSERT with unique violation handling
- `get_coalition_for_merchants(pool, merchant_a, merchant_b)` -- Finds active coalition where both merchants are active members (complex JOIN query with inline struct)
- `get_merchant_coalitions(pool, merchant_id)` -- All active coalitions for a merchant, with member details
- `record_coalition_transfer(pool, ...)` -- INSERT into coalition_transfers
- `get_coalition_transfers_for_customer(pool, customer_id)` -- Transfer history ordered by created_at DESC
