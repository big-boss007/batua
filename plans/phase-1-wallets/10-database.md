# Phase 1: Database -- COMPLETED

## Core Enums (`20260318000001_core_enums.sql`)

```sql
movement_type     -- 'in', 'held', 'out', 'across'
actor_type        -- 'system', 'human', 'automation', 'migration'
bucket_type       -- 'earned_credit', 'cod_pending', 'gift_card', 'customer_funded',
                     'referral_reward', 'goodwill_credit', 'membership_benefit', 'refund_credit'
credit_state      -- 'active', 'expired', 'redeemed', 'reversed', 'cancelled'
redemption_state  -- 'initiated', 'validating', 'rejected', 'committed', 'applied',
                     'failed', 'compensated', 'completed'
event_state       -- 'received', 'processing', 'processed', 'failed', 'duplicate'
```

## Wallets (`20260318000004_wallets.sql`)

```sql
wallets (
    id              UUID PK DEFAULT gen_random_uuid(),
    merchant_id     UUID NOT NULL FK merchants,
    customer_id     UUID FK customers,        -- NULL for bearer instruments
    is_bearer       BOOLEAN NOT NULL DEFAULT false,
    bearer_code     TEXT UNIQUE,              -- Gift cards before claim
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)          -- Truth 1
)
```

Indexes: merchant_id, customer_id, bearer_code (partial WHERE NOT NULL).

## Ledger Entries (`20260318000005_ledger_entries.sql`)

```sql
ledger_entries (
    id                    UUID PK DEFAULT gen_random_uuid(),
    wallet_id             UUID NOT NULL FK wallets,
    bucket_type           bucket_type NOT NULL,
    movement_type         movement_type NOT NULL,
    earning_unit          DOUBLE PRECISION NOT NULL,
    currency_equivalent   DOUBLE PRECISION NOT NULL,
    conversion_rate       DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    idempotency_key       TEXT NOT NULL UNIQUE,        -- Truth 8
    event_id              UUID,
    rule_snapshot_id      UUID,
    campaign_snapshot_id  UUID,
    actor_type            actor_type NOT NULL,
    actor_id              TEXT,
    payment_reference     TEXT,
    transfer_id           UUID,                        -- Links across-movement pairs
    constraints           JSONB NOT NULL DEFAULT '{}',
    expires_at            TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    state                 credit_state NOT NULL DEFAULT 'active'
)
```

Indexes: wallet_id, (wallet_id, bucket_type), (wallet_id, state), idempotency_key, transfer_id (partial), event_id (partial), created_at, expires_at (partial WHERE active).

Immutability trigger: `prevent_ledger_mutation()` blocks UPDATE on all columns except `state` and `expires_at`.

## Wallet Policies (`20260318000008_wallet_policies.sql`)

```sql
wallet_policies (
    id                        UUID PK DEFAULT gen_random_uuid(),
    merchant_id               UUID NOT NULL FK merchants,
    bucket_type               bucket_type NOT NULL,
    min_redemption            DOUBLE PRECISION,
    step_size                 DOUBLE PRECISION,
    max_per_order_pct         DOUBLE PRECISION,
    max_per_order_fixed       DOUBLE PRECISION,
    stackable_with_discounts  BOOLEAN NOT NULL DEFAULT true,
    default_conversion_rate   DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    default_expiry_days       INTEGER,
    is_transferable           BOOLEAN NOT NULL DEFAULT false,
    excluded_payment_methods  TEXT[] NOT NULL DEFAULT '{}',
    excluded_collections      TEXT[] NOT NULL DEFAULT '{}',
    is_active                 BOOLEAN NOT NULL DEFAULT true,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, bucket_type)
)
```

## Redemption Requests (`20260318000009_redemption_requests.sql`)

```sql
redemption_requests (
    id                    UUID PK DEFAULT gen_random_uuid(),
    merchant_id           UUID NOT NULL FK merchants,
    wallet_id             UUID NOT NULL FK wallets,
    requested_amount      DOUBLE PRECISION NOT NULL,
    eligible_amount       DOUBLE PRECISION,
    applied_amount        DOUBLE PRECISION,
    order_id              TEXT NOT NULL,
    order_amount          DOUBLE PRECISION NOT NULL,
    payment_method        TEXT,
    state                 redemption_state NOT NULL DEFAULT 'initiated',
    debit_entry_id        UUID FK ledger_entries,
    compensation_entry_id UUID FK ledger_entries,
    shopify_discount_id   TEXT,
    rejection_reason      TEXT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

Indexes: merchant_id, wallet_id, order_id, state.

## COD Orders (`20260318000017_cod_orders.sql`)

```sql
cod_orders (
    id                    UUID PK DEFAULT gen_random_uuid(),
    merchant_id           UUID NOT NULL FK merchants,
    order_id              TEXT NOT NULL,
    wallet_id             UUID NOT NULL FK wallets,
    ledger_entry_id       UUID NOT NULL FK ledger_entries,
    state                 TEXT NOT NULL DEFAULT 'pending',
    delivery_confirmed_at TIMESTAMPTZ,
    released_entry_id     UUID FK ledger_entries,
    cancelled_entry_id    UUID FK ledger_entries,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, order_id)
)
```

Indexes: merchant_id, state, wallet_id.

## Milestones (`20260319000002_milestones.sql`)

```sql
milestone_configs (
    id             UUID PK,
    merchant_id    UUID NOT NULL FK merchants,
    name           TEXT NOT NULL,
    milestone_type TEXT NOT NULL,       -- 'order_count' or 'lifetime_spend'
    threshold      DOUBLE PRECISION NOT NULL,
    reward_amount  DOUBLE PRECISION NOT NULL,
    is_active      BOOLEAN NOT NULL DEFAULT true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
)

milestone_achievements (
    id               UUID PK,
    merchant_id      UUID NOT NULL FK merchants,
    customer_id      UUID NOT NULL FK customers,
    milestone_id     UUID NOT NULL FK milestone_configs,
    ledger_entry_id  UUID NOT NULL FK ledger_entries,
    achieved_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(customer_id, milestone_id)
)
```

## Newsletter Signups (`20260319000003_newsletter_signups.sql`)

```sql
newsletter_signups (
    id               UUID PK,
    merchant_id      UUID NOT NULL FK merchants,
    customer_id      UUID NOT NULL FK customers,
    ledger_entry_id  UUID NOT NULL FK ledger_entries,
    email            TEXT NOT NULL,
    source           TEXT NOT NULL DEFAULT 'webhook',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)
)
```

## Streaks (`20260319000004_streaks.sql`)

```sql
streak_configs (
    id               UUID PK,
    merchant_id      UUID NOT NULL FK merchants,
    name             TEXT NOT NULL,
    required_orders  INTEGER NOT NULL,
    window_days      INTEGER NOT NULL,
    reward_amount    DOUBLE PRECISION NOT NULL,
    is_active        BOOLEAN NOT NULL DEFAULT true,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
)

streak_achievements (
    id               UUID PK,
    merchant_id      UUID NOT NULL FK merchants,
    customer_id      UUID NOT NULL FK customers,
    streak_config_id UUID NOT NULL FK streak_configs,
    ledger_entry_id  UUID NOT NULL FK ledger_entries,
    window_start     TIMESTAMPTZ NOT NULL,
    window_end       TIMESTAMPTZ NOT NULL,
    achieved_at      TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

## Spin Wheel (`20260319000005_spin_wheel.sql`)

```sql
spin_wheel_configs (
    id               UUID PK,
    merchant_id      UUID NOT NULL FK merchants UNIQUE,   -- One wheel per merchant
    name             TEXT NOT NULL DEFAULT 'Lucky Wheel',
    is_active        BOOLEAN NOT NULL DEFAULT true,
    daily_spin_limit INTEGER NOT NULL DEFAULT 1,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
)

spin_wheel_segments (
    id             UUID PK,
    wheel_id       UUID NOT NULL FK spin_wheel_configs,
    label          TEXT NOT NULL,
    reward_amount  DOUBLE PRECISION NOT NULL,
    probability    DOUBLE PRECISION NOT NULL,
    color          TEXT NOT NULL DEFAULT '#7c6aff',
    position       INTEGER NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
)

spin_results (
    id              UUID PK,
    merchant_id     UUID NOT NULL FK merchants,
    customer_id     UUID NOT NULL FK customers,
    segment_id      UUID NOT NULL FK spin_wheel_segments,
    reward_amount   DOUBLE PRECISION NOT NULL,
    ledger_entry_id UUID FK ledger_entries,
    spun_at         TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

## Memberships (`20260319000006_memberships.sql`)

```sql
membership_plans (
    id                    UUID PK,
    merchant_id           UUID NOT NULL FK merchants,
    name                  TEXT NOT NULL,
    plan_type             TEXT NOT NULL,           -- 'monthly' or 'annual'
    price                 DOUBLE PRECISION NOT NULL,
    earn_rate_multiplier  DOUBLE PRECISION NOT NULL DEFAULT 1.5,
    benefits              JSONB NOT NULL DEFAULT '{}',
    is_active             BOOLEAN NOT NULL DEFAULT true,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now()
)

customer_memberships (
    id             UUID PK,
    merchant_id    UUID NOT NULL FK merchants,
    customer_id    UUID NOT NULL FK customers,
    plan_id        UUID NOT NULL FK membership_plans,
    status         TEXT NOT NULL DEFAULT 'active',    -- 'active', 'expired', 'cancelled'
    started_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at     TIMESTAMPTZ NOT NULL,
    renewed_count  INTEGER NOT NULL DEFAULT 0,
    cancelled_at   TIMESTAMPTZ,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id, plan_id)
)
```
