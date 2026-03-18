CREATE TABLE loyalty_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) UNIQUE,
    name TEXT NOT NULL,
    evaluation_criteria TEXT NOT NULL DEFAULT 'spend',  -- 'spend', 'points', 'order_count'
    evaluation_period_days INTEGER,  -- NULL = lifetime
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE loyalty_tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    program_id UUID NOT NULL REFERENCES loyalty_programs(id),
    name TEXT NOT NULL,
    rank INTEGER NOT NULL,  -- Lower = more basic, higher = more premium
    threshold DOUBLE PRECISION NOT NULL,  -- Minimum to qualify
    earn_rate_multiplier DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    benefits JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(program_id, rank)
);

CREATE INDEX idx_loyalty_tiers_program ON loyalty_tiers(program_id);

CREATE TABLE customer_tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    tier_id UUID NOT NULL REFERENCES loyalty_tiers(id),
    qualifying_value DOUBLE PRECISION NOT NULL DEFAULT 0,
    qualified_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(customer_id, merchant_id)
);

CREATE INDEX idx_customer_tiers_customer ON customer_tiers(customer_id);
CREATE INDEX idx_customer_tiers_merchant ON customer_tiers(merchant_id);
