# Phase 0: Scheduler -- SKIPPED

No scheduled/cron jobs in the foundation phase. Storage functions exist for future scheduler use:
- `events::storage::get_pending_events` -- For a future event processing worker
- `earn::storage::get_expired_memberships` -- For a future membership expiry job

These will be wired to scheduled tasks in a later phase.
