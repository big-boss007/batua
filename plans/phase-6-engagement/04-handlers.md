# Phase 6: Handlers

All in `src/services/earn/handler.rs` unless noted.

## Birthday
- `birthday_bonus` -- POST `/earn/birthday-bonus`, processes birthday bonuses for a merchant

## Milestones
- `create_milestone` -- POST `/earn/milestones`, creates milestone config (201)
- `list_milestones` -- GET `/earn/milestones/{merchant_id}`, lists active configs
- `check_milestones` -- POST `/earn/check-milestones`, checks and awards for a customer
- `get_customer_milestones` -- GET `/earn/milestones/{merchant_id}/{customer_id}`, lists achieved milestones

## Newsletter
- `newsletter_signup` -- POST `/earn/newsletter-signup`, processes signup. Returns 201 if new, 200 if already subscribed.
- `get_newsletter_signup_count` -- GET `/earn/newsletter-signups/{merchant_id}`, returns count

## Profile Completion
- `profile_completion` -- POST `/earn/profile-completion`, checks profile and awards credit

## Streaks
- `create_streak_config` -- POST `/earn/streaks`, creates streak config (201)
- `list_streak_configs` -- GET `/earn/streaks/{merchant_id}`, lists active configs
- `check_streaks` -- POST `/earn/check-streaks`, checks and awards streak rewards

## Spin Wheel
- `create_wheel_config` -- POST `/earn/spin-wheel/config`, creates wheel with segments (201)
- `get_wheel_config` -- GET `/earn/spin-wheel/{merchant_id}`, returns wheel + segments
- `spin_wheel` -- POST `/earn/spin-wheel/spin`, executes spin

## Memberships
- `create_membership_plan` -- POST `/earn/memberships/plans`, creates plan (201)
- `list_membership_plans` -- GET `/earn/memberships/plans/{merchant_id}`, lists active plans
- `subscribe_membership` -- POST `/earn/memberships/subscribe`, subscribes customer (201 if new, 200 if existing)
- `renew_membership` -- POST `/earn/memberships/renew`, renews membership
- `cancel_membership` -- POST `/earn/memberships/cancel/{membership_id}`, cancels membership
- `membership_status` -- GET `/earn/memberships/status/{merchant_id}/{customer_id}`, returns status

## Coalition (in `src/services/admin/handler.rs`)
- `create_coalition` -- POST `/admin/coalitions`, requires >= 2 merchants (201)
- `get_merchant_coalitions` -- GET `/admin/coalitions/{merchant_id}`, uses reader replica
- `coalition_transfer` -- POST `/admin/coalitions/transfer`, validates positive amount
- `get_coalition_transfers` -- GET `/admin/coalitions/transfers/{customer_id}`, uses reader replica
