CREATE TABLE IF NOT EXISTS spin_wheel_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) UNIQUE,
    name TEXT NOT NULL DEFAULT 'Lucky Wheel',
    is_active BOOLEAN NOT NULL DEFAULT true,
    daily_spin_limit INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS spin_wheel_segments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wheel_id UUID NOT NULL REFERENCES spin_wheel_configs(id),
    label TEXT NOT NULL,
    reward_amount DOUBLE PRECISION NOT NULL,
    probability DOUBLE PRECISION NOT NULL,
    color TEXT NOT NULL DEFAULT '#7c6aff',
    position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_spin_segments_wheel ON spin_wheel_segments(wheel_id);

CREATE TABLE IF NOT EXISTS spin_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    segment_id UUID NOT NULL REFERENCES spin_wheel_segments(id),
    reward_amount DOUBLE PRECISION NOT NULL,
    ledger_entry_id UUID REFERENCES ledger_entries(id),
    spun_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_spin_results_customer ON spin_results(customer_id);
CREATE INDEX IF NOT EXISTS idx_spin_results_merchant ON spin_results(merchant_id);
