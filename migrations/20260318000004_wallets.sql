-- One wallet per customer per merchant (Truth 1)
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID REFERENCES customers(id),  -- NULL for bearer instruments
    is_bearer BOOLEAN NOT NULL DEFAULT false,
    bearer_code TEXT UNIQUE,  -- For gift cards before claim
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)  -- One wallet per customer per merchant
);

CREATE INDEX idx_wallets_merchant ON wallets(merchant_id);
CREATE INDEX idx_wallets_customer ON wallets(customer_id);
CREATE INDEX idx_wallets_bearer_code ON wallets(bearer_code) WHERE bearer_code IS NOT NULL;
