CREATE TABLE newsletter_signups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    ledger_entry_id UUID NOT NULL REFERENCES ledger_entries(id),
    email TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'webhook',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(merchant_id, customer_id)
);
CREATE INDEX idx_newsletter_merchant ON newsletter_signups(merchant_id);
