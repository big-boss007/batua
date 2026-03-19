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
