# Phase 6: Routes

All routes added to existing routers (earn and admin services).

## Earn Service Routes (`src/services/earn/mod.rs`)

| Method | Path | Handler | Feature |
|--------|------|---------|---------|
| POST | `/earn/birthday-bonus` | `birthday_bonus` | Birthday |
| POST | `/earn/milestones` | `create_milestone` | Milestones |
| GET | `/earn/milestones/{merchant_id}` | `list_milestones` | Milestones |
| POST | `/earn/check-milestones` | `check_milestones` | Milestones |
| GET | `/earn/milestones/{merchant_id}/{customer_id}` | `get_customer_milestones` | Milestones |
| POST | `/earn/newsletter-signup` | `newsletter_signup` | Newsletter |
| GET | `/earn/newsletter-signups/{merchant_id}` | `get_newsletter_signup_count` | Newsletter |
| POST | `/earn/profile-completion` | `profile_completion` | Profile |
| POST | `/earn/streaks` | `create_streak_config` | Streaks |
| GET | `/earn/streaks/{merchant_id}` | `list_streak_configs` | Streaks |
| POST | `/earn/check-streaks` | `check_streaks` | Streaks |
| POST | `/earn/spin-wheel/config` | `create_wheel_config` | Spin Wheel |
| POST | `/earn/spin-wheel/spin` | `spin_wheel` | Spin Wheel |
| GET | `/earn/spin-wheel/{merchant_id}` | `get_wheel_config` | Spin Wheel |
| POST | `/earn/memberships/plans` | `create_membership_plan` | Memberships |
| GET | `/earn/memberships/plans/{merchant_id}` | `list_membership_plans` | Memberships |
| POST | `/earn/memberships/subscribe` | `subscribe_membership` | Memberships |
| POST | `/earn/memberships/renew` | `renew_membership` | Memberships |
| POST | `/earn/memberships/cancel/{membership_id}` | `cancel_membership` | Memberships |
| GET | `/earn/memberships/status/{merchant_id}/{customer_id}` | `membership_status` | Memberships |

## Admin Service Routes (added to `src/services/admin/mod.rs`)

| Method | Path | Handler | Feature |
|--------|------|---------|---------|
| POST | `/admin/coalitions` | `create_coalition` | Coalition |
| GET | `/admin/coalitions/{merchant_id}` | `get_merchant_coalitions` | Coalition |
| POST | `/admin/coalitions/transfer` | `coalition_transfer` | Coalition |
| GET | `/admin/coalitions/transfers/{customer_id}` | `get_coalition_transfers` | Coalition |
