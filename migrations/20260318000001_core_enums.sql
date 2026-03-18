-- Movement types for ledger entries
CREATE TYPE movement_type AS ENUM ('in', 'held', 'out', 'across');

-- Actor types for cause tracing
CREATE TYPE actor_type AS ENUM ('system', 'human', 'automation', 'migration');

-- Bucket types that determine value origin and constraints
CREATE TYPE bucket_type AS ENUM (
    'earned_credit',
    'cod_pending',
    'gift_card',
    'customer_funded',
    'referral_reward',
    'goodwill_credit',
    'membership_benefit',
    'refund_credit'
);

-- Credit lifecycle states
CREATE TYPE credit_state AS ENUM (
    'active',
    'expired',
    'redeemed',
    'reversed',
    'cancelled'
);

-- Redemption request states
CREATE TYPE redemption_state AS ENUM (
    'initiated',
    'validating',
    'rejected',
    'committed',
    'applied',
    'failed',
    'compensated',
    'completed'
);

-- Event processing states
CREATE TYPE event_state AS ENUM (
    'received',
    'processing',
    'processed',
    'failed',
    'duplicate'
);
