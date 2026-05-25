# Phase 1: Scheduler -- SKIPPED

No scheduled tasks were required for Phase 1. Membership expiry is checked lazily on `get_membership_status` rather than via a background job. Birthday bonuses are triggered via an explicit API call (`POST /earn/birthday-bonus`) rather than a cron scheduler.
