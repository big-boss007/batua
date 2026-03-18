CREATE TABLE gift_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    wallet_id UUID NOT NULL REFERENCES wallets(id),  -- Bearer wallet initially
    code TEXT NOT NULL UNIQUE,  -- The bearer code
    initial_amount DOUBLE PRECISION NOT NULL,
    current_amount DOUBLE PRECISION NOT NULL,
    currency TEXT NOT NULL DEFAULT 'INR',

    -- Issuance info
    issued_by actor_type NOT NULL,
    issued_by_id TEXT,
    payment_reference TEXT,  -- If purchased
    batch_id UUID,  -- For bulk issuance
    batch_position INTEGER,  -- Position in batch (for idempotency)

    -- State
    is_claimed BOOLEAN NOT NULL DEFAULT false,
    claimed_by_wallet_id UUID REFERENCES wallets(id),
    claimed_at TIMESTAMPTZ,

    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_gift_cards_merchant ON gift_cards(merchant_id);
CREATE INDEX idx_gift_cards_code ON gift_cards(code);
CREATE INDEX idx_gift_cards_wallet ON gift_cards(wallet_id);
CREATE INDEX idx_gift_cards_batch ON gift_cards(batch_id) WHERE batch_id IS NOT NULL;
