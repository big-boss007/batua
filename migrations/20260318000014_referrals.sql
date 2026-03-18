CREATE TABLE referral_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) UNIQUE,
    referrer_reward_amount DOUBLE PRECISION NOT NULL,
    referee_reward_amount DOUBLE PRECISION NOT NULL,
    referrer_bucket_type bucket_type NOT NULL DEFAULT 'referral_reward',
    referee_bucket_type bucket_type NOT NULL DEFAULT 'referral_reward',
    max_referrals_per_customer INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE referral_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    customer_id UUID NOT NULL REFERENCES customers(id),
    code TEXT NOT NULL UNIQUE,  -- Vanity code like "RIYA10" or auto-generated
    is_vanity BOOLEAN NOT NULL DEFAULT false,
    is_creator BOOLEAN NOT NULL DEFAULT false,  -- Creator/influencer mode
    commission_rate DOUBLE PRECISION,  -- For creators
    total_referrals INTEGER NOT NULL DEFAULT 0,
    total_conversions INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_referral_codes_merchant ON referral_codes(merchant_id);
CREATE INDEX idx_referral_codes_customer ON referral_codes(customer_id);
CREATE INDEX idx_referral_codes_code ON referral_codes(code);

CREATE TABLE referral_conversions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id),
    referral_code_id UUID NOT NULL REFERENCES referral_codes(id),
    referrer_id UUID NOT NULL REFERENCES customers(id),
    referee_id UUID NOT NULL REFERENCES customers(id),
    order_id TEXT,
    referrer_entry_id UUID REFERENCES ledger_entries(id),
    referee_entry_id UUID REFERENCES ledger_entries(id),

    -- Anti-fraud signals
    referee_ip TEXT,
    referee_device_fingerprint TEXT,
    is_suspicious BOOLEAN NOT NULL DEFAULT false,
    fraud_signals JSONB NOT NULL DEFAULT '[]',

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_referral_conversions_merchant ON referral_conversions(merchant_id);
CREATE INDEX idx_referral_conversions_code ON referral_conversions(referral_code_id);
