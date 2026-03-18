CREATE TABLE redemption_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    wallet_id UUID NOT NULL REFERENCES wallets(id),

    -- Request details
    requested_amount DOUBLE PRECISION NOT NULL,
    eligible_amount DOUBLE PRECISION,  -- Calculated after validation
    applied_amount DOUBLE PRECISION,  -- Actually applied

    -- Order context
    order_id TEXT NOT NULL,
    order_amount DOUBLE PRECISION NOT NULL,
    payment_method TEXT,

    -- State machine
    state redemption_state NOT NULL DEFAULT 'initiated',

    -- Linked ledger entries
    debit_entry_id UUID REFERENCES ledger_entries(id),
    compensation_entry_id UUID REFERENCES ledger_entries(id),

    -- Shopify integration
    shopify_discount_id TEXT,

    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_redemptions_merchant ON redemption_requests(merchant_id);
CREATE INDEX idx_redemptions_wallet ON redemption_requests(wallet_id);
CREATE INDEX idx_redemptions_order ON redemption_requests(order_id);
CREATE INDEX idx_redemptions_state ON redemption_requests(state);
