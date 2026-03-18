-- Rule definitions (current, versioned)
CREATE TABLE rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    rule_type TEXT NOT NULL,  -- 'reward', 'wallet_policy', 'notification_flow', etc.
    name TEXT NOT NULL,
    config JSONB NOT NULL,  -- The DSL YAML parsed to JSON
    version INTEGER NOT NULL DEFAULT 1,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_rules_merchant ON rules(merchant_id);
CREATE INDEX idx_rules_type ON rules(merchant_id, rule_type);
CREATE INDEX idx_rules_active ON rules(merchant_id, is_active) WHERE is_active = true;

-- Immutable rule snapshots (Truth 4)
CREATE TABLE rule_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES rules(id),
    version INTEGER NOT NULL,
    config JSONB NOT NULL,  -- Frozen config at snapshot time
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_rule_snapshots_rule ON rule_snapshots(rule_id);

-- Campaign definitions
CREATE TABLE campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    name TEXT NOT NULL,
    campaign_type TEXT NOT NULL,  -- 'multiplier', 'bonus', 'festive'
    config JSONB NOT NULL,
    base_rule_id UUID REFERENCES rules(id),  -- The rule this campaign overlays
    multiplier DOUBLE PRECISION,
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_campaigns_merchant ON campaigns(merchant_id);
CREATE INDEX idx_campaigns_active ON campaigns(merchant_id, is_active, starts_at, ends_at);

-- Immutable campaign snapshots (Truth 4)
CREATE TABLE campaign_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    config JSONB NOT NULL,
    multiplier DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_campaign_snapshots_campaign ON campaign_snapshots(campaign_id);
