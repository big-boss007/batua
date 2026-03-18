-- The immutable append-only ledger (Truths 2, 3, 4, 7, 8)
CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Wallet and bucket (Truth 1, 6)
    wallet_id UUID NOT NULL REFERENCES wallets(id),
    bucket_type bucket_type NOT NULL,

    -- Movement (Truth 3)
    movement_type movement_type NOT NULL,

    -- Atomic triple (Truth 2) — immutable once written
    earning_unit DOUBLE PRECISION NOT NULL,
    currency_equivalent DOUBLE PRECISION NOT NULL,
    conversion_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0,

    -- Structural idempotency (Truth 8)
    idempotency_key TEXT NOT NULL UNIQUE,

    -- Full traceable cause (Truth 4)
    event_id UUID,
    rule_snapshot_id UUID,
    campaign_snapshot_id UUID,
    actor_type actor_type NOT NULL,
    actor_id TEXT,  -- human user ID, system process name, etc.
    payment_reference TEXT,  -- For gift card purchases

    -- Cross-movement linking (Truth 3 — across movements)
    transfer_id UUID,  -- Links the out+in pair in across movements

    -- Constraints locked at earn time (Truth 6)
    constraints JSONB NOT NULL DEFAULT '{}',
    -- Example: {"transferable": false, "stackable": true, "excluded_payment_methods": ["cod"], "excluded_collections": []}

    -- Expiry
    expires_at TIMESTAMPTZ,

    -- Time is first-class (Truth 7)
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- State tracking
    state credit_state NOT NULL DEFAULT 'active'
);

-- Critical indexes
CREATE INDEX idx_ledger_wallet ON ledger_entries(wallet_id);
CREATE INDEX idx_ledger_wallet_bucket ON ledger_entries(wallet_id, bucket_type);
CREATE INDEX idx_ledger_wallet_state ON ledger_entries(wallet_id, state);
CREATE INDEX idx_ledger_idempotency ON ledger_entries(idempotency_key);
CREATE INDEX idx_ledger_transfer ON ledger_entries(transfer_id) WHERE transfer_id IS NOT NULL;
CREATE INDEX idx_ledger_event ON ledger_entries(event_id) WHERE event_id IS NOT NULL;
CREATE INDEX idx_ledger_created ON ledger_entries(created_at);
CREATE INDEX idx_ledger_expires ON ledger_entries(expires_at) WHERE expires_at IS NOT NULL AND state = 'active';

-- Prevent mutation (Truth 7) — this trigger prevents UPDATE on critical columns
CREATE OR REPLACE FUNCTION prevent_ledger_mutation() RETURNS TRIGGER AS $$
BEGIN
    IF OLD.earning_unit != NEW.earning_unit
        OR OLD.currency_equivalent != NEW.currency_equivalent
        OR OLD.conversion_rate != NEW.conversion_rate
        OR OLD.idempotency_key != NEW.idempotency_key
        OR OLD.event_id IS DISTINCT FROM NEW.event_id
        OR OLD.rule_snapshot_id IS DISTINCT FROM NEW.rule_snapshot_id
        OR OLD.campaign_snapshot_id IS DISTINCT FROM NEW.campaign_snapshot_id
        OR OLD.actor_type != NEW.actor_type
        OR OLD.actor_id IS DISTINCT FROM NEW.actor_id
        OR OLD.payment_reference IS DISTINCT FROM NEW.payment_reference
        OR OLD.transfer_id IS DISTINCT FROM NEW.transfer_id
        OR OLD.wallet_id != NEW.wallet_id
        OR OLD.bucket_type != NEW.bucket_type
        OR OLD.movement_type != NEW.movement_type
        OR OLD.created_at != NEW.created_at
    THEN
        RAISE EXCEPTION 'Ledger entries are immutable. Only state and expires_at may be updated.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER enforce_ledger_immutability
    BEFORE UPDATE ON ledger_entries
    FOR EACH ROW
    EXECUTE FUNCTION prevent_ledger_mutation();
