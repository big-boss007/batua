# Phase 6: Database Migrations

## `20260319000001_customer_birthday.sql`
Adds `birthday DATE` column to the `customers` table.

## `20260319000002_milestones.sql`
```sql
CREATE TABLE milestone_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    name TEXT NOT NULL,
    milestone_type TEXT NOT NULL,  -- 'order_count', 'lifetime_spend'
    threshold DOUBLE PRECISION NOT NULL,
    reward_amount DOUBLE PRECISION NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_milestones_merchant ON milestone_configs(merchant_id);

CREATE TABLE milestone_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    milestone_id UUID NOT NULL REFERENCES milestone_configs(id),
    ledger_entry_id UUID NOT NULL REFERENCES ledger_entries(id),
    achieved_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(customer_id, milestone_id)
);
CREATE INDEX idx_milestone_achievements_customer ON milestone_achievements(customer_id);
```

## `20260319000003_newsletter_signups.sql`
```sql
CREATE TABLE newsletter_signups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    ledger_entry_id UUID NOT NULL REFERENCES ledger_entries(id),
    email TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'webhook',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)
);
CREATE INDEX idx_newsletter_merchant ON newsletter_signups(merchant_id);
```

## `20260319000004_streaks.sql`
```sql
CREATE TABLE streak_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    name TEXT NOT NULL,
    required_orders INTEGER NOT NULL,
    window_days INTEGER NOT NULL,
    reward_amount DOUBLE PRECISION NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_streak_configs_merchant ON streak_configs(merchant_id);

CREATE TABLE streak_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    streak_config_id UUID NOT NULL REFERENCES streak_configs(id),
    ledger_entry_id UUID NOT NULL REFERENCES ledger_entries(id),
    window_start TIMESTAMPTZ NOT NULL,
    window_end TIMESTAMPTZ NOT NULL,
    achieved_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_streak_achievements_customer ON streak_achievements(customer_id);
CREATE INDEX idx_streak_achievements_config ON streak_achievements(streak_config_id);
```

## `20260319000005_spin_wheel.sql`
```sql
CREATE TABLE spin_wheel_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) UNIQUE,
    name TEXT NOT NULL DEFAULT 'Lucky Wheel',
    is_active BOOLEAN NOT NULL DEFAULT true,
    daily_spin_limit INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE spin_wheel_segments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wheel_id UUID NOT NULL REFERENCES spin_wheel_configs(id),
    label TEXT NOT NULL,
    reward_amount DOUBLE PRECISION NOT NULL,
    probability DOUBLE PRECISION NOT NULL,
    color TEXT NOT NULL DEFAULT '#7c6aff',
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_spin_segments_wheel ON spin_wheel_segments(wheel_id);

CREATE TABLE spin_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    segment_id UUID NOT NULL REFERENCES spin_wheel_segments(id),
    reward_amount DOUBLE PRECISION NOT NULL,
    ledger_entry_id UUID REFERENCES ledger_entries(id),
    spun_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_spin_results_customer ON spin_results(customer_id);
CREATE INDEX idx_spin_results_merchant ON spin_results(merchant_id);
```

## `20260319000006_memberships.sql`
```sql
CREATE TABLE membership_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    name TEXT NOT NULL,
    plan_type TEXT NOT NULL,  -- 'monthly', 'annual'
    price DOUBLE PRECISION NOT NULL,
    earn_rate_multiplier DOUBLE PRECISION NOT NULL DEFAULT 1.5,
    benefits JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_membership_plans_merchant ON membership_plans(merchant_id);

CREATE TABLE customer_memberships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    plan_id UUID NOT NULL REFERENCES membership_plans(id),
    status TEXT NOT NULL DEFAULT 'active',  -- 'active', 'expired', 'cancelled'
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    renewed_count INTEGER NOT NULL DEFAULT 0,
    cancelled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id, plan_id)
);
CREATE INDEX idx_customer_memberships_customer ON customer_memberships(customer_id);
CREATE INDEX idx_customer_memberships_merchant ON customer_memberships(merchant_id);
CREATE INDEX idx_customer_memberships_status ON customer_memberships(status);
```

## `20260319000007_coalition.sql`
```sql
CREATE TABLE coalitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE coalition_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coalition_id UUID NOT NULL REFERENCES coalitions(id),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    conversion_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(coalition_id, merchant_id)
);
CREATE INDEX idx_coalition_members_merchant ON coalition_members(merchant_id);
CREATE INDEX idx_coalition_members_coalition ON coalition_members(coalition_id);

CREATE TABLE coalition_transfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coalition_id UUID NOT NULL REFERENCES coalitions(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    from_merchant_id UUID NOT NULL REFERENCES merchants(id),
    to_merchant_id UUID NOT NULL REFERENCES merchants(id),
    from_wallet_id UUID NOT NULL REFERENCES wallets(id),
    to_wallet_id UUID NOT NULL REFERENCES wallets(id),
    amount DOUBLE PRECISION NOT NULL,
    converted_amount DOUBLE PRECISION NOT NULL,
    conversion_rate DOUBLE PRECISION NOT NULL,
    transfer_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_coalition_transfers_customer ON coalition_transfers(customer_id);
```
