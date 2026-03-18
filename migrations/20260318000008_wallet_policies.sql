CREATE TABLE wallet_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    bucket_type bucket_type NOT NULL,

    -- Redemption constraints (burn side)
    min_redemption DOUBLE PRECISION,  -- Minimum per transaction
    step_size DOUBLE PRECISION,  -- Must redeem in multiples of X
    max_per_order_pct DOUBLE PRECISION,  -- % of order value cap
    max_per_order_fixed DOUBLE PRECISION,  -- Fixed rupee cap
    stackable_with_discounts BOOLEAN NOT NULL DEFAULT true,

    -- Earn constraints
    default_conversion_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    default_expiry_days INTEGER,

    -- Movement constraints (Truth 6)
    is_transferable BOOLEAN NOT NULL DEFAULT false,
    excluded_payment_methods TEXT[] NOT NULL DEFAULT '{}',
    excluded_collections TEXT[] NOT NULL DEFAULT '{}',

    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(merchant_id, bucket_type)
);

CREATE INDEX idx_wallet_policies_merchant ON wallet_policies(merchant_id);
