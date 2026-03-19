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
