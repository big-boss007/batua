# Phase 1: Routes -- COMPLETED

## Wallets (`src/services/wallets/mod.rs`)

```
POST /wallets                              -> create_wallet
GET  /wallets/{id}                         -> get_wallet
GET  /wallets/lookup                       -> lookup_wallet
POST /wallets/get-or-create                -> get_or_create_wallet
GET  /merchants/{merchant_id}/wallets      -> list_wallets_for_merchant
```

## Earn (`src/services/earn/mod.rs`)

```
POST /earn/process                                     -> process_earn
POST /earn/manual-credit                               -> manual_credit
POST /earn/birthday-bonus                              -> birthday_bonus
POST /earn/milestones                                  -> create_milestone
GET  /earn/milestones/{merchant_id}                    -> list_milestones
POST /earn/check-milestones                            -> check_milestones
GET  /earn/milestones/{merchant_id}/{customer_id}      -> get_customer_milestones
POST /earn/newsletter-signup                           -> newsletter_signup
GET  /earn/newsletter-signups/{merchant_id}            -> get_newsletter_signup_count
POST /earn/profile-completion                          -> profile_completion
POST /earn/streaks                                     -> create_streak_config
GET  /earn/streaks/{merchant_id}                       -> list_streak_configs
POST /earn/check-streaks                               -> check_streaks
POST /earn/spin-wheel/config                           -> create_wheel_config
POST /earn/spin-wheel/spin                             -> spin_wheel
GET  /earn/spin-wheel/{merchant_id}                    -> get_wheel_config
POST /earn/memberships/plans                           -> create_membership_plan
GET  /earn/memberships/plans/{merchant_id}             -> list_membership_plans
POST /earn/memberships/subscribe                       -> subscribe_membership
POST /earn/memberships/renew                           -> renew_membership
POST /earn/memberships/cancel/{membership_id}          -> cancel_membership
GET  /earn/memberships/status/{merchant_id}/{customer_id} -> membership_status
```

## Redemption (`src/services/redemption/mod.rs`)

```
POST /redemptions                          -> initiate_redemption
GET  /redemptions/{id}                     -> get_redemption
POST /redemptions/{id}/compensate          -> compensate_redemption
GET  /wallets/{wallet_id}/eligibility      -> check_eligibility
```

## COD (`src/services/cod/mod.rs`)

```
POST /cod/webhook/delivery                 -> delivery_webhook
POST /cod/incentive                        -> cod_to_prepaid
GET  /cod/orders/{merchant_id}             -> list_cod_orders
GET  /cod/analytics/{merchant_id}          -> cod_analytics
```
