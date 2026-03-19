CREATE TABLE milestone_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    name TEXT NOT NULL,
    milestone_type TEXT NOT NULL,
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
