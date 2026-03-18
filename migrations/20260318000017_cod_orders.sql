CREATE TABLE cod_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    order_id TEXT NOT NULL,
    wallet_id UUID NOT NULL REFERENCES wallets(id),
    ledger_entry_id UUID NOT NULL REFERENCES ledger_entries(id),
    state TEXT NOT NULL DEFAULT 'pending',
    delivery_confirmed_at TIMESTAMPTZ,
    released_entry_id UUID REFERENCES ledger_entries(id),
    cancelled_entry_id UUID REFERENCES ledger_entries(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, order_id)
);
CREATE INDEX idx_cod_orders_merchant ON cod_orders(merchant_id);
CREATE INDEX idx_cod_orders_state ON cod_orders(state);
CREATE INDEX idx_cod_orders_wallet ON cod_orders(wallet_id);
