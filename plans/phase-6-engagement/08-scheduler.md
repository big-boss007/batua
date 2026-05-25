# Phase 6: Scheduler

No dedicated scheduler process, but infrastructure is in place for scheduled tasks:

## Membership Expiry
- `storage::get_expired_memberships(pool)` -- Returns all memberships with status='active' and expires_at < now(). Designed to be called by a scheduled job.
- `storage::expire_membership(pool, membership_id)` -- Sets status to 'expired'.
- `helpers::get_membership_status` also performs lazy expiry: if a membership is active but past expires_at, it is expired on read.

## Birthday Bonus
- `POST /earn/birthday-bonus` is designed to be called once daily by an external scheduler or cron job. Idempotency via SHA-256 hash prevents double-crediting.

## Streak Window Resets
- Streaks are checked on-demand via `POST /earn/check-streaks`. The window is computed dynamically from now() - window_days. No explicit reset scheduler is needed since `count_recent_orders` queries ledger_entries with a rolling time window.
