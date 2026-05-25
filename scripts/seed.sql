-- ==========================================================================
-- Batua Seed Data — Full Reset & Load
-- 20 merchants, 100 customers each, 50 txns each, all features
-- Run: psql batua < scripts/seed.sql
-- ==========================================================================

BEGIN;

-- ==========================================================================
-- TRUNCATE everything
-- ==========================================================================
TRUNCATE
    coalition_transfers, coalition_members, coalitions,
    customer_memberships, spin_results, spin_wheel_segments, spin_wheel_configs,
    streak_achievements, streak_configs, newsletter_signups,
    milestone_achievements, milestone_configs, cod_orders,
    referral_conversions, referral_codes, referral_programs,
    gift_cards, customer_tiers, loyalty_tiers, loyalty_programs,
    notification_logs, notification_templates, connectors,
    redemption_requests, wallet_policies,
    campaign_snapshots, campaigns, rule_snapshots, rules,
    events, ledger_entries, wallets, customer_order_stats,
    product_collection_mappings, customers, merchants, geo_policies
CASCADE;

-- ==========================================================================
-- GEO POLICY
-- ==========================================================================
INSERT INTO geo_policies (id, geo_code, name, config) VALUES
('a0000000-0000-0000-0000-000000000001', 'india', 'India', '{"currency":"INR","timezone":"Asia/Kolkata","wallet_enabled":true,"gift_card_enabled":true,"cod_enabled":true,"referrals_enabled":true,"max_wallet_balance":10000,"kyc_threshold":10000}');

-- ==========================================================================
-- 20 MERCHANTS
-- ==========================================================================
INSERT INTO merchants (id, external_id, name, domain, currency, slug, plan_tier, geo_policy_id, points_name, points_icon, points_to_currency_rate) VALUES
('b0000000-0000-0000-0000-000000000001', 'gid://shopify/Shop/10001', 'Chai & Co',           'chaiandco.myshopify.com',         'INR', 'chaiandco',          'growth', 'a0000000-0000-0000-0000-000000000001', 'Stars',    '★',  0.25),
('b0000000-0000-0000-0000-000000000002', 'gid://shopify/Shop/10002', 'Desi Drapes',         'desidrapes.myshopify.com',        'INR', 'desidrapes',         'free',   'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000003', 'gid://shopify/Shop/10003', 'Spice Route',         'spiceroute.myshopify.com',        'INR', 'spiceroute',         'growth', 'a0000000-0000-0000-0000-000000000001', 'Coins',    '🪙', 0.5),
('b0000000-0000-0000-0000-000000000004', 'gid://shopify/Shop/10004', 'Kurta Palace',        'kurtapalace.myshopify.com',       'INR', 'kurtapalace',        'pro',    'a0000000-0000-0000-0000-000000000001', 'Gems',     '💎', 0.1),
('b0000000-0000-0000-0000-000000000005', 'gid://shopify/Shop/10005', 'Ghee & Gold',         'gheeandgold.myshopify.com',       'INR', 'gheeandgold',        'growth', 'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000006', 'gid://shopify/Shop/10006', 'Rangoli Beauty',      'rangolibeauty.myshopify.com',     'INR', 'rangolibeauty',      'pro',    'a0000000-0000-0000-0000-000000000001', 'Glow',     '✨', 0.25),
('b0000000-0000-0000-0000-000000000007', 'gid://shopify/Shop/10007', 'Namaste Nutrition',   'namastenutrition.myshopify.com',  'INR', 'namastenutrition',   'growth', 'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000008', 'gid://shopify/Shop/10008', 'Dhobi Fresh',         'dhobifresh.myshopify.com',        'INR', 'dhobifresh',         'free',   'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000009', 'gid://shopify/Shop/10009', 'Jaipur Gems',         'jaipurgems.myshopify.com',        'INR', 'jaipurgems',         'pro',    'a0000000-0000-0000-0000-000000000001', 'Sparkles', '✦',  0.5),
('b0000000-0000-0000-0000-00000000000a', 'gid://shopify/Shop/10010', 'Bamboo & Bloom',      'bambooandbloom.myshopify.com',    'INR', 'bambooandbloom',     'growth', 'a0000000-0000-0000-0000-000000000001', 'Seeds',    '🌱', 0.25),
('b0000000-0000-0000-0000-00000000000b', 'gid://shopify/Shop/10011', 'Loom & Stitch',       'loomandstitch.myshopify.com',     'INR', 'loomandstitch',      'pro',    'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-00000000000c', 'gid://shopify/Shop/10012', 'Masala Box',          'masalabox.myshopify.com',         'INR', 'masalabox',          'growth', 'a0000000-0000-0000-0000-000000000001', 'Masala',   '🌶', 0.5),
('b0000000-0000-0000-0000-00000000000d', 'gid://shopify/Shop/10013', 'Yoga Vibes',          'yogavibes.myshopify.com',         'INR', 'yogavibes',          'free',   'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-00000000000e', 'gid://shopify/Shop/10014', 'Kerala Basket',       'keralabasket.myshopify.com',      'INR', 'keralabasket',       'growth', 'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-00000000000f', 'gid://shopify/Shop/10015', 'Desi Gadgets',        'desigadgets.myshopify.com',       'INR', 'desigadgets',        'pro',    'a0000000-0000-0000-0000-000000000001', 'Credits',  'cr',  1.0),
('b0000000-0000-0000-0000-000000000010', 'gid://shopify/Shop/10016', 'Tandoori Treats',     'tandooritreats.myshopify.com',    'INR', 'tandooritreats',     'growth', 'a0000000-0000-0000-0000-000000000001', 'Flames',   '🔥', 0.25),
('b0000000-0000-0000-0000-000000000011', 'gid://shopify/Shop/10017', 'Silk & Stone',        'silkandstone.myshopify.com',      'INR', 'silkandstone',       'pro',    'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000012', 'gid://shopify/Shop/10018', 'Fitoor Fitness',      'fitoorfitness.myshopify.com',     'INR', 'fitoorfitness',      'growth', 'a0000000-0000-0000-0000-000000000001', 'Reps',     '💪', 0.5),
('b0000000-0000-0000-0000-000000000013', 'gid://shopify/Shop/10019', 'Chikankari House',    'chikankarihouse.myshopify.com',   'INR', 'chikankarihouse',    'free',   'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0),
('b0000000-0000-0000-0000-000000000014', 'gid://shopify/Shop/10020', 'Himalayan Harvest',   'himalayanharvest.myshopify.com',  'INR', 'himalayanharvest',   'growth', 'a0000000-0000-0000-0000-000000000001', 'Points',   'pts', 1.0);

-- ==========================================================================
-- 500 CUSTOMERS — enough to give each of 20 merchants 100 unique customers
-- ==========================================================================
DO $$
DECLARE
    first_names text[] := ARRAY[
        'Aarav','Aditi','Aisha','Ajay','Akash','Amit','Ananya','Aniket','Anjali','Ankita',
        'Arjun','Aruna','Ashok','Bhavna','Chetan','Deepa','Deepak','Devika','Dhruv','Dinesh',
        'Divya','Farhan','Gaurav','Geeta','Girish','Gouri','Harish','Hema','Ishaan','Jaya',
        'Jyoti','Kailash','Kajal','Kamal','Kamini','Karan','Kartik','Kavitha','Kiran','Komal',
        'Kunal','Lakshmi','Leela','Madhavi','Manoj','Meera','Mira','Mohit','Nandini','Naveen',
        'Neha','Nikhil','Nilesh','Nisha','Om','Padma','Pallavi','Pankaj','Parvathi','Pooja',
        'Prachi','Prakash','Pratibha','Praveen','Preeti','Priya','Radha','Rahul','Rajat','Rajesh',
        'Rakesh','Ravi','Rekha','Renuka','Ritu','Rohit','Sachin','Sahil','Samir','Sandeep',
        'Sanjay','Sarita','Saurabh','Seema','Shanti','Shreya','Siddharth','Sneha','Sonal','Sudhir',
        'Sunita','Suresh','Swati','Tanvi','Tarun','Tushar','Uma','Urvashi','Varun','Vishal'
    ];
    last_names text[] := ARRAY[
        'Agarwal','Arora','Bajaj','Bansal','Bhat','Bhatt','Biswas','Bose','Chauhan','Chopra',
        'Choudhary','Das','Desai','Deshpande','Dubey','Gadkari','Ghosh','Goswami','Gupta','Hegde',
        'Iyer','Jain','Jha','Joshi','Kamath','Kapoor','Kashyap','Kaul','Khanna','Krishnan',
        'Kulkarni','Kumar','Lal','Luthra','Mahajan','Malhotra','Mane','Mehta','Menon','Mishra',
        'Mittal','Mohan','Mukherjee','Murugan','Nair','Nambiar','Nayak','Oberoi','Pandey','Parekh',
        'Parmar','Patel','Pathak','Pillai','Prasad','Purohit','Rajan','Rao','Rawat','Reddy',
        'Sarin','Saxena','Sen','Sethi','Shah','Sharma','Shetty','Shukla','Singh','Soni',
        'Srinivasan','Subramaniam','Sundaram','Tandon','Thakur','Tiwari','Trivedi','Venkatesh','Verma','Vohra',
        'Wagh','Yadav','Ahuja','Bhargava','Chatterjee','Dhawan','Gill','Hegde','Iyer','Johar',
        'Kher','Luthra','Mistry','Nagpal','Puri','Raghavan','Saini','Talwar','Uppal','Walia'
    ];
    full_name text;
    fi int;
    li int;
    cid uuid;
    bday date;
BEGIN
    FOR i IN 1..500 LOOP
        fi := 1 + ((i - 1) % 100);
        li := 1 + (((i - 1) / 100 + (i * 7)) % 100);
        full_name := first_names[fi] || ' ' || last_names[li];
        cid := ('c0000000-0000-0000-0000-' || lpad(i::text, 12, '0'))::uuid;
        bday := CASE WHEN i % 5 = 0 THEN NULL ELSE ('1985-01-01'::date + (i * 73 % 10000 || ' days')::interval)::date END;

        INSERT INTO customers (id, phone, email, name, birthday, is_verified)
        VALUES (
            cid,
            '+91' || (9000000000 + i)::text,
            lower(replace(full_name, ' ', '.')) || i || '@gmail.com',
            full_name,
            bday,
            i % 7 != 0
        );
    END LOOP;
END $$;

-- ==========================================================================
-- WALLETS — exactly 100 customers per merchant (2000 wallets total)
-- Customers are distributed so each appears at 3-6 merchants
-- ==========================================================================
DO $$
DECLARE
    merchant_ids uuid[];
    cid uuid;
    mid uuid;
    wid uuid;
    m_idx int;
    c_start int;
BEGIN
    SELECT array_agg(id ORDER BY id) INTO merchant_ids FROM merchants;

    FOR m_idx IN 1..20 LOOP
        mid := merchant_ids[m_idx];
        -- Each merchant gets customers c_start..c_start+99 (with wrap)
        c_start := ((m_idx - 1) * 25) % 500;

        FOR j IN 0..99 LOOP
            cid := ('c0000000-0000-0000-0000-' || lpad((1 + (c_start + j) % 500)::text, 12, '0'))::uuid;
            wid := gen_random_uuid();
            INSERT INTO wallets (id, merchant_id, customer_id)
            VALUES (wid, mid, cid)
            ON CONFLICT (merchant_id, customer_id) DO NOTHING;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- RULES — 3 per merchant (cashback, first-order bonus, review reward)
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    pct float;
    max_amt float;
BEGIN
    FOR mid IN SELECT id FROM merchants LOOP
        pct := 3 + abs(hashtext(mid::text)::bigint % 8);
        max_amt := 100 + abs(hashtext(mid::text || 'max')::bigint % 400);

        INSERT INTO rules (merchant_id, rule_type, name, config) VALUES
        (mid, 'reward', pct || '% Cashback on All Orders', json_build_object(
            'event_type', 'order.completed',
            'conditions', '[]'::json,
            'action', json_build_object(
                'bucket_type', 'earned_credit', 'calculation', 'percentage',
                'value', pct, 'max_amount', max_amt, 'conversion_rate', 1.0, 'expiry_days', 90
            )
        )::jsonb);

        INSERT INTO rules (merchant_id, rule_type, name, config) VALUES
        (mid, 'reward', 'First Order Bonus ₹50', json_build_object(
            'event_type', 'order.completed',
            'conditions', json_build_array(json_build_object('field','is_first_order','operator','eq','value',true)),
            'action', json_build_object(
                'bucket_type', 'earned_credit', 'calculation', 'fixed',
                'value', 50, 'max_amount', null, 'conversion_rate', 1.0, 'expiry_days', 60
            )
        )::jsonb);

        INSERT INTO rules (merchant_id, rule_type, name, config) VALUES
        (mid, 'reward', 'Review Reward ₹25', json_build_object(
            'event_type', 'review.submitted',
            'conditions', '[]'::json,
            'action', json_build_object(
                'bucket_type', 'earned_credit', 'calculation', 'fixed',
                'value', 25, 'max_amount', null, 'conversion_rate', 1.0, 'expiry_days', 60
            )
        )::jsonb);
    END LOOP;
END $$;

-- ==========================================================================
-- CAMPAIGNS — 12 merchants have active campaigns (mix of types)
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    rid uuid;
    cnt int := 0;
    ctype text;
    cname text;
    mult float;
BEGIN
    FOR mid IN SELECT id FROM merchants ORDER BY id LIMIT 12 LOOP
        SELECT id INTO rid FROM rules WHERE merchant_id = mid AND name LIKE '%Cashback%' LIMIT 1;
        cnt := cnt + 1;
        IF rid IS NOT NULL THEN
            IF cnt <= 4 THEN
                ctype := 'multiplier'; cname := 'Summer Sale 2x'; mult := 2.0;
            ELSIF cnt <= 8 THEN
                ctype := 'multiplier'; cname := 'Holi Festival 3x'; mult := 3.0;
            ELSE
                ctype := 'bonus'; cname := 'Weekend Bonus 1.5x'; mult := 1.5;
            END IF;

            INSERT INTO campaigns (merchant_id, name, campaign_type, config, base_rule_id, multiplier, starts_at, ends_at)
            VALUES (mid, cname, ctype, '{}'::jsonb, rid, mult, now() - interval '3 days', now() + interval '27 days');
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- WALLET POLICIES — every merchant: earned_credit, gift_card, referral, cod
-- ==========================================================================
INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_expiry_days, excluded_payment_methods)
SELECT id, 'earned_credit', 10.0, 1.0, 50.0, 500.0, true, 90, ARRAY['cod'] FROM merchants;

INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, excluded_payment_methods)
SELECT id, 'gift_card', NULL, NULL, 100.0, NULL, true, ARRAY[]::text[] FROM merchants;

INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, excluded_payment_methods)
SELECT id, 'referral_reward', NULL, NULL, 100.0, 300.0, true, ARRAY['cod'] FROM merchants;

INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_expiry_days, excluded_payment_methods)
SELECT id, 'cod_pending', NULL, NULL, 100.0, NULL, false, NULL, ARRAY[]::text[] FROM merchants;

INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_expiry_days, excluded_payment_methods)
SELECT id, 'goodwill_credit', NULL, NULL, 100.0, 500.0, true, 180, ARRAY[]::text[] FROM merchants;

INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, max_per_order_fixed, stackable_with_discounts, default_expiry_days, excluded_payment_methods)
SELECT id, 'refund_credit', NULL, NULL, 100.0, NULL, true, 365, ARRAY[]::text[] FROM merchants;

-- ==========================================================================
-- LOYALTY PROGRAMS + TIERS — every merchant
-- ==========================================================================
INSERT INTO loyalty_programs (id, merchant_id, name, evaluation_criteria, evaluation_period_days)
SELECT
    ('d0000000-0000-0000-0000-' || lpad(row_number() OVER (ORDER BY id)::text, 12, '0'))::uuid,
    id,
    name || ' Rewards',
    CASE (row_number() OVER (ORDER BY id))::int % 3
        WHEN 0 THEN 'spend'
        WHEN 1 THEN 'spend'
        ELSE 'order_count'
    END,
    CASE WHEN (row_number() OVER (ORDER BY id))::int % 5 = 0 THEN NULL ELSE 180 + ((row_number() OVER (ORDER BY id))::int * 30 % 185) END
FROM merchants;

DO $$
DECLARE
    prog record;
BEGIN
    FOR prog IN SELECT id FROM loyalty_programs LOOP
        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits) VALUES
        (prog.id, 'Bronze',   1, 0,     1.0,  '{"perks":["Early access"]}'),
        (prog.id, 'Silver',   2, 2000,  1.25, '{"perks":["Early access","Free shipping > ₹500"]}'),
        (prog.id, 'Gold',     3, 5000,  1.5,  '{"perks":["All Silver","Birthday bonus","Exclusive products"]}'),
        (prog.id, 'Platinum', 4, 15000, 2.0,  '{"perks":["All Gold","Personal concierge","Annual gift"]}');
    END LOOP;
END $$;

-- ==========================================================================
-- EVENTS — 30 per merchant (600 total) covering various event types
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    m_cnt int := 0;
    evt_types text[] := ARRAY['order.completed','order.completed','order.completed','order.refunded','order.cancelled','cod.delivered','review.submitted','newsletter.signup','cart.abandoned','customer.created'];
    etype text;
    i int;
BEGIN
    FOR mid IN SELECT id FROM merchants ORDER BY id LOOP
        m_cnt := m_cnt + 1;
        FOR i IN 1..30 LOOP
            etype := evt_types[1 + ((m_cnt + i) % array_length(evt_types, 1))];
            INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
            VALUES (
                mid, etype, 'shopify',
                'evt_' || m_cnt || '_' || i,
                json_build_object('order_id', 'SHOP-' || m_cnt || '-' || i, 'amount', 500 + (i * 37 % 4500))::jsonb,
                CASE WHEN i <= 27 THEN 'processed'::event_state ELSE 'received'::event_state END,
                'evt:' || mid || ':' || i,
                now() - ((i * 3) || ' days')::interval,
                CASE WHEN i <= 27 THEN now() - ((i * 3) || ' days')::interval + interval '2 seconds' ELSE NULL END
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- LEDGER ENTRIES + ORDER STATS — 50 transactions per wallet
-- Mixed bucket types: ~35 earned_credit, 5 redemptions, 3 gift_card,
-- 2 referral_reward, 2 goodwill_credit, 1 refund_credit, 2 cod_pending
-- ==========================================================================
DO $$
DECLARE
    w record;
    order_amt float;
    earn_amt float;
    total_spend float;
    total_orders int;
    days_ago int;
    i int;
    hash_val bigint;
    conv_rate float;
BEGIN
    FOR w IN
        SELECT wal.id AS wallet_id, wal.merchant_id, wal.customer_id,
               m.points_to_currency_rate AS rate
        FROM wallets wal
        JOIN merchants m ON m.id = wal.merchant_id
        WHERE wal.is_bearer = false AND wal.customer_id IS NOT NULL
        ORDER BY wal.merchant_id, wal.customer_id
    LOOP
        total_spend := 0;
        total_orders := 0;
        conv_rate := COALESCE(w.rate, 1.0);

        -- 35 earned_credit IN entries (order cashback)
        FOR i IN 1..35 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            days_ago := 1 + (hash_val % 180);
            order_amt := 300 + (hash_val % 4700);
            earn_amt := round((order_amt * (3 + hash_val % 8) / 100.0)::numeric, 2);
            total_spend := total_spend + order_amt;
            total_orders := total_orders + 1;

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, payment_reference, idempotency_key, expires_at, created_at)
            VALUES (w.wallet_id, 'earned_credit', 'in',
                    CASE WHEN conv_rate != 1.0 THEN round((earn_amt / conv_rate)::numeric, 2) ELSE earn_amt END,
                    earn_amt, conv_rate, 'system',
                    'order:SHOP-' || abs(hash_val),
                    'earn:' || w.wallet_id || ':' || i,
                    now() + interval '90 days',
                    now() - (days_ago || ' days')::interval);
        END LOOP;

        -- 5 earned_credit OUT entries (redemptions)
        FOR i IN 36..40 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            days_ago := 1 + (hash_val % 90);
            earn_amt := 20 + (hash_val % 150);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, payment_reference, idempotency_key, created_at)
            VALUES (w.wallet_id, 'earned_credit', 'out',
                    CASE WHEN conv_rate != 1.0 THEN round((earn_amt / conv_rate)::numeric, 2) ELSE earn_amt END,
                    earn_amt, conv_rate, 'system', 'redemption:auto',
                    'order:RED-' || abs(hash_val),
                    'redeem:' || w.wallet_id || ':' || (i - 35),
                    now() - (days_ago || ' days')::interval);
        END LOOP;

        -- 3 gift_card IN entries
        FOR i IN 41..43 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            earn_amt := 100 + (hash_val % 400);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, expires_at, created_at)
            VALUES (w.wallet_id, 'gift_card', 'in', earn_amt, earn_amt, 1.0, 'system', 'gc-claim',
                    'gc:' || w.wallet_id || ':' || (i - 40),
                    now() + interval '365 days',
                    now() - ((hash_val % 60) || ' days')::interval);
        END LOOP;

        -- 2 referral_reward IN entries
        FOR i IN 44..45 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            earn_amt := 25 + (hash_val % 100);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, created_at)
            VALUES (w.wallet_id, 'referral_reward', 'in', earn_amt, earn_amt, 1.0, 'system', 'referral:auto',
                    'ref:' || w.wallet_id || ':' || (i - 43),
                    now() - ((hash_val % 90) || ' days')::interval);
        END LOOP;

        -- 2 goodwill_credit IN entries
        FOR i IN 46..47 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            earn_amt := 25 + (hash_val % 150);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, payment_reference, idempotency_key, created_at)
            VALUES (w.wallet_id, 'goodwill_credit', 'in', earn_amt, earn_amt, 1.0, 'human', 'admin:support',
                    'late delivery compensation',
                    'goodwill:' || w.wallet_id || ':' || (i - 45),
                    now() - ((hash_val % 45) || ' days')::interval);
        END LOOP;

        -- 1 refund_credit IN entry
        hash_val := abs(hashtext(w.wallet_id::text || '48'));
        earn_amt := 100 + (hash_val % 500);

        INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, payment_reference, idempotency_key, expires_at, created_at)
        VALUES (w.wallet_id, 'refund_credit', 'in', earn_amt, earn_amt, 1.0, 'system',
                'order:REFUND-' || abs(hash_val),
                'refund:' || w.wallet_id,
                now() + interval '365 days',
                now() - ((hash_val % 30) || ' days')::interval);

        -- 2 cod_pending HELD entries
        FOR i IN 49..50 LOOP
            hash_val := abs(hashtext(w.wallet_id::text || i::text));
            earn_amt := 15 + (hash_val % 80);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, payment_reference, idempotency_key, created_at)
            VALUES (w.wallet_id, 'cod_pending', 'held', earn_amt, earn_amt, 1.0, 'system',
                    'order:COD-' || abs(hash_val),
                    'cod:' || w.wallet_id || ':' || (i - 48),
                    now() - ((hash_val % 14) || ' days')::interval);
        END LOOP;

        -- Order stats
        INSERT INTO customer_order_stats (merchant_id, customer_id, total_orders, total_spend, first_order_at, last_order_at)
        VALUES (w.merchant_id, w.customer_id, total_orders, total_spend,
                now() - interval '180 days', now() - interval '1 day')
        ON CONFLICT (merchant_id, customer_id) DO UPDATE SET
            total_orders = EXCLUDED.total_orders,
            total_spend = EXCLUDED.total_spend;
    END LOOP;
END $$;

-- ==========================================================================
-- CUSTOMER TIERS — assign based on spend
-- ==========================================================================
DO $$
DECLARE
    cos record;
    tier_id uuid;
    prog_id uuid;
BEGIN
    FOR cos IN SELECT merchant_id, customer_id, total_spend FROM customer_order_stats LOOP
        SELECT lp.id INTO prog_id FROM loyalty_programs lp WHERE lp.merchant_id = cos.merchant_id LIMIT 1;
        IF prog_id IS NULL THEN CONTINUE; END IF;

        SELECT lt.id INTO tier_id
        FROM loyalty_tiers lt
        WHERE lt.program_id = prog_id AND lt.threshold <= cos.total_spend
        ORDER BY lt.threshold DESC LIMIT 1;

        IF tier_id IS NOT NULL THEN
            INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value)
            VALUES (cos.customer_id, cos.merchant_id, tier_id, cos.total_spend)
            ON CONFLICT (customer_id, merchant_id) DO UPDATE SET tier_id = EXCLUDED.tier_id, qualifying_value = EXCLUDED.qualifying_value;
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- COD ORDERS — ~200 across merchants (from cod_pending ledger entries)
-- ==========================================================================
DO $$
DECLARE
    le record;
    cnt int := 0;
BEGIN
    FOR le IN
        SELECT l.id AS entry_id, l.wallet_id, w.merchant_id, l.payment_reference
        FROM ledger_entries l
        JOIN wallets w ON w.id = l.wallet_id
        WHERE l.bucket_type = 'cod_pending' AND l.movement_type = 'held'
        ORDER BY l.created_at
        LIMIT 200
    LOOP
        cnt := cnt + 1;
        INSERT INTO cod_orders (merchant_id, order_id, wallet_id, ledger_entry_id, state)
        VALUES (le.merchant_id, 'SHOP-COD-' || cnt, le.wallet_id, le.entry_id,
                CASE WHEN cnt % 4 = 0 THEN 'delivered' WHEN cnt % 7 = 0 THEN 'released' ELSE 'pending' END)
        ON CONFLICT (merchant_id, order_id) DO NOTHING;
    END LOOP;
END $$;

-- ==========================================================================
-- GIFT CARDS — 100 cards across all 20 merchants (5 per merchant)
-- Mix of claimed/unclaimed, with batch IDs
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    mids uuid[];
    bearer_wid uuid;
    claim_wid uuid;
    gc_amt float;
    code text;
    batch uuid;
    i int;
    m_idx int;
    chars text := 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';
    seg text;
    j int;
BEGIN
    SELECT array_agg(id ORDER BY id) INTO mids FROM merchants;

    FOR m_idx IN 1..20 LOOP
        mid := mids[m_idx];
        batch := gen_random_uuid();

        FOR i IN 1..5 LOOP
            gc_amt := 200 + ((m_idx * 5 + i) * 47 % 800);

            seg := '';
            FOR j IN 1..12 LOOP
                seg := seg || substr(chars, 1 + (random() * 30)::int, 1);
                IF j IN (4, 8) THEN seg := seg || '-'; END IF;
            END LOOP;
            code := 'BRZE-' || seg;

            bearer_wid := gen_random_uuid();
            INSERT INTO wallets (id, merchant_id, is_bearer, bearer_code)
            VALUES (bearer_wid, mid, true, code);

            INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key)
            VALUES (bearer_wid, 'gift_card', 'in', gc_amt, gc_amt, 1.0, 'system', 'gc-issue', 'gc-issue:' || m_idx || ':' || i);

            IF i <= 2 THEN
                -- Claimed (first 2 per merchant)
                SELECT w.id INTO claim_wid FROM wallets w WHERE w.merchant_id = mid AND w.is_bearer = false LIMIT 1;
                IF claim_wid IS NOT NULL THEN
                    INSERT INTO gift_cards (merchant_id, wallet_id, code, initial_amount, current_amount, issued_by, batch_id, batch_position, is_claimed, claimed_by_wallet_id, claimed_at, expires_at)
                    VALUES (mid, bearer_wid, code, gc_amt, gc_amt * 0.6, 'system', batch, i, true, claim_wid, now() - interval '5 days', now() + interval '180 days');

                    INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key)
                    VALUES (claim_wid, 'gift_card', 'in', gc_amt, gc_amt, 1.0, 'system', 'gc-claim:' || code, 'gc-claim-in:' || m_idx || ':' || i);
                ELSE
                    INSERT INTO gift_cards (merchant_id, wallet_id, code, initial_amount, current_amount, issued_by, batch_id, batch_position, expires_at)
                    VALUES (mid, bearer_wid, code, gc_amt, gc_amt, 'system', batch, i, now() + interval '365 days');
                END IF;
            ELSIF i = 5 THEN
                -- Expired
                INSERT INTO gift_cards (merchant_id, wallet_id, code, initial_amount, current_amount, issued_by, batch_id, batch_position, expires_at, is_active)
                VALUES (mid, bearer_wid, code, gc_amt, gc_amt, 'system', batch, i, now() - interval '10 days', false);
            ELSE
                -- Unclaimed
                INSERT INTO gift_cards (merchant_id, wallet_id, code, initial_amount, current_amount, issued_by, batch_id, batch_position, expires_at)
                VALUES (mid, bearer_wid, code, gc_amt, gc_amt, 'system', batch, i, now() + interval '365 days');
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- REFERRAL PROGRAMS + CODES + CONVERSIONS
-- ==========================================================================
INSERT INTO referral_programs (merchant_id, referrer_reward_amount, referee_reward_amount, max_referrals_per_customer)
SELECT id, 50 + (row_number() OVER (ORDER BY id) * 10 % 100), 25 + (row_number() OVER (ORDER BY id) * 7 % 75), 15 + (row_number() OVER (ORDER BY id)::int % 10)
FROM merchants;

DO $$
DECLARE
    mid uuid;
    cid uuid;
    code_text text;
    rc_id uuid;
    referee_id uuid;
    ref_wid uuid;
    ree_wid uuid;
    cnt int := 0;
    cname text;
BEGIN
    FOR mid IN SELECT id FROM merchants ORDER BY id LOOP
        -- 5 referral codes per merchant
        FOR cid IN SELECT customer_id FROM wallets WHERE merchant_id = mid AND is_bearer = false ORDER BY customer_id LIMIT 5 LOOP
            cnt := cnt + 1;
            SELECT name INTO cname FROM customers WHERE id = cid;
            code_text := upper(split_part(cname, ' ', 1)) || cnt;
            rc_id := gen_random_uuid();

            INSERT INTO referral_codes (id, merchant_id, customer_id, code, is_vanity, is_creator, total_referrals, total_conversions)
            VALUES (rc_id, mid, cid, code_text,
                    cnt % 5 = 0,  -- every 5th is vanity
                    cnt % 10 = 0, -- every 10th is a creator
                    2 + cnt % 8, cnt % 4)
            ON CONFLICT DO NOTHING;

            -- Create conversions for every 2nd code
            IF cnt % 2 = 0 THEN
                SELECT customer_id INTO referee_id FROM wallets WHERE merchant_id = mid AND is_bearer = false AND customer_id != cid ORDER BY customer_id DESC LIMIT 1;
                IF referee_id IS NOT NULL THEN
                    SELECT id INTO ref_wid FROM wallets WHERE merchant_id = mid AND customer_id = cid LIMIT 1;
                    SELECT id INTO ree_wid FROM wallets WHERE merchant_id = mid AND customer_id = referee_id LIMIT 1;

                    IF ref_wid IS NOT NULL AND ree_wid IS NOT NULL THEN
                        INSERT INTO referral_conversions (merchant_id, referral_code_id, referrer_id, referee_id, order_id)
                        VALUES (mid, rc_id, cid, referee_id, 'order:ref-' || cnt);
                    END IF;
                END IF;
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- MILESTONES — 3 per merchant + achievements
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    m_id uuid;
    ms_id uuid;
    cos_rec record;
    entry_id uuid;
BEGIN
    FOR mid IN SELECT id FROM merchants LOOP
        INSERT INTO milestone_configs (id, merchant_id, name, milestone_type, threshold, reward_amount)
        VALUES
        (gen_random_uuid(), mid, '5th Order Bonus', 'order_count', 5, 100),
        (gen_random_uuid(), mid, '₹10K Lifetime Spend', 'lifetime_spend', 10000, 500),
        (gen_random_uuid(), mid, '10th Order VIP', 'order_count', 10, 250);
    END LOOP;

    -- Milestone achievements for qualifying customers
    FOR cos_rec IN
        SELECT cos.merchant_id, cos.customer_id, cos.total_orders, cos.total_spend
        FROM customer_order_stats cos
        WHERE cos.total_orders >= 10
        LIMIT 300
    LOOP
        SELECT mc.id INTO ms_id FROM milestone_configs mc
        WHERE mc.merchant_id = cos_rec.merchant_id AND mc.milestone_type = 'order_count' AND mc.threshold <= cos_rec.total_orders
        ORDER BY mc.threshold DESC LIMIT 1;

        IF ms_id IS NOT NULL THEN
            SELECT w.id INTO m_id FROM wallets w WHERE w.merchant_id = cos_rec.merchant_id AND w.customer_id = cos_rec.customer_id LIMIT 1;
            IF m_id IS NOT NULL THEN
                entry_id := gen_random_uuid();
                INSERT INTO ledger_entries (id, wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, created_at)
                VALUES (entry_id, m_id, 'earned_credit', 'in', 100, 100, 1.0, 'system', 'milestone', 'ms:' || cos_rec.customer_id || ':' || ms_id, now() - interval '10 days');

                INSERT INTO milestone_achievements (merchant_id, customer_id, milestone_id, ledger_entry_id)
                VALUES (cos_rec.merchant_id, cos_rec.customer_id, ms_id, entry_id)
                ON CONFLICT (customer_id, milestone_id) DO NOTHING;
            END IF;
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- STREAK CONFIGS + ACHIEVEMENTS
-- ==========================================================================
INSERT INTO streak_configs (id, merchant_id, name, required_orders, window_days, reward_amount)
SELECT gen_random_uuid(), id, 'Weekly Regular', 2, 7, 25 FROM merchants;

INSERT INTO streak_configs (id, merchant_id, name, required_orders, window_days, reward_amount)
SELECT gen_random_uuid(), id, 'Monthly Loyal', 4, 30, 75 FROM merchants;

DO $$
DECLARE
    sc record;
    cos_rec record;
    wid uuid;
    entry_id uuid;
    cnt int := 0;
BEGIN
    FOR sc IN SELECT id, merchant_id, required_orders FROM streak_configs WHERE name = 'Weekly Regular' LOOP
        FOR cos_rec IN
            SELECT customer_id FROM customer_order_stats
            WHERE merchant_id = sc.merchant_id AND total_orders >= sc.required_orders
            LIMIT 15
        LOOP
            SELECT w.id INTO wid FROM wallets w WHERE w.merchant_id = sc.merchant_id AND w.customer_id = cos_rec.customer_id LIMIT 1;
            IF wid IS NOT NULL THEN
                cnt := cnt + 1;
                entry_id := gen_random_uuid();
                INSERT INTO ledger_entries (id, wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, created_at)
                VALUES (entry_id, wid, 'earned_credit', 'in', 25, 25, 1.0, 'system', 'streak', 'streak:' || cnt, now() - interval '5 days');

                INSERT INTO streak_achievements (merchant_id, customer_id, streak_config_id, ledger_entry_id, window_start, window_end)
                VALUES (sc.merchant_id, cos_rec.customer_id, sc.id, entry_id, now() - interval '12 days', now() - interval '5 days');
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- SPIN WHEELS — every merchant with 6 segments
-- ==========================================================================
INSERT INTO spin_wheel_configs (id, merchant_id, name, daily_spin_limit)
SELECT
    ('e0000000-0000-0000-0000-' || lpad(row_number() OVER (ORDER BY id)::text, 12, '0'))::uuid,
    id,
    name || ' Lucky Wheel',
    1 + (row_number() OVER (ORDER BY id)::int % 3)
FROM merchants;

DO $$
DECLARE
    whl record;
BEGIN
    FOR whl IN SELECT id FROM spin_wheel_configs LOOP
        INSERT INTO spin_wheel_segments (wheel_id, label, reward_amount, probability, color, position) VALUES
        (whl.id, '₹5 Off',      5.0,   25.0, '#10b981', 0),
        (whl.id, '₹10 Off',     10.0,  20.0, '#6366f1', 1),
        (whl.id, '₹25 Off',     25.0,  15.0, '#f59e0b', 2),
        (whl.id, '₹50 Off',     50.0,  8.0,  '#ef4444', 3),
        (whl.id, '₹100 Off',    100.0, 2.0,  '#8b5cf6', 4),
        (whl.id, 'Better Luck', 0.0,   30.0, '#94a3b8', 5);
    END LOOP;
END $$;

-- Spin results — ~200 across merchants
DO $$
DECLARE
    whl record;
    seg_id uuid;
    cid uuid;
    wid uuid;
    entry_id uuid;
    cnt int := 0;
BEGIN
    FOR whl IN
        SELECT swc.id AS wheel_id, swc.merchant_id
        FROM spin_wheel_configs swc
        ORDER BY swc.merchant_id LIMIT 15
    LOOP
        FOR cid IN
            SELECT customer_id FROM wallets WHERE merchant_id = whl.merchant_id AND is_bearer = false
            ORDER BY customer_id LIMIT 10
        LOOP
            cnt := cnt + 1;
            SELECT id INTO seg_id FROM spin_wheel_segments WHERE wheel_id = whl.wheel_id ORDER BY random() LIMIT 1;
            SELECT w.id INTO wid FROM wallets w WHERE w.merchant_id = whl.merchant_id AND w.customer_id = cid LIMIT 1;

            IF seg_id IS NOT NULL AND wid IS NOT NULL THEN
                entry_id := NULL;
                -- Credit reward for non-zero segments
                IF (SELECT reward_amount FROM spin_wheel_segments WHERE id = seg_id) > 0 THEN
                    entry_id := gen_random_uuid();
                    INSERT INTO ledger_entries (id, wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, created_at)
                    VALUES (entry_id, wid, 'earned_credit', 'in',
                            (SELECT reward_amount FROM spin_wheel_segments WHERE id = seg_id),
                            (SELECT reward_amount FROM spin_wheel_segments WHERE id = seg_id),
                            1.0, 'system', 'spin-wheel', 'spin:' || cnt, now() - ((cnt % 30) || ' days')::interval);
                END IF;

                INSERT INTO spin_results (merchant_id, customer_id, segment_id, reward_amount, ledger_entry_id, spun_at)
                VALUES (whl.merchant_id, cid, seg_id,
                        (SELECT reward_amount FROM spin_wheel_segments WHERE id = seg_id),
                        entry_id, now() - ((cnt % 30) || ' days')::interval);
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- MEMBERSHIPS — ~80 customers with paid tier memberships
-- ==========================================================================
DO $$
DECLARE
    r record;
    cnt int := 0;
BEGIN
    FOR r IN
        SELECT cti.customer_id, cti.merchant_id, cti.tier_id
        FROM customer_tiers cti
        JOIN loyalty_tiers lt ON lt.id = cti.tier_id
        WHERE lt.rank >= 3
        ORDER BY cti.qualifying_value DESC
        LIMIT 80
    LOOP
        cnt := cnt + 1;
        INSERT INTO customer_memberships (merchant_id, customer_id, tier_id, status, expires_at, renewed_count)
        VALUES (r.merchant_id, r.customer_id, r.tier_id,
                CASE WHEN cnt % 10 = 0 THEN 'expired' WHEN cnt % 15 = 0 THEN 'cancelled' ELSE 'active' END,
                CASE WHEN cnt % 10 = 0 THEN now() - interval '15 days' ELSE now() + interval '330 days' END,
                CASE WHEN cnt % 3 = 0 THEN 1 WHEN cnt % 5 = 0 THEN 2 ELSE 0 END)
        ON CONFLICT (merchant_id, customer_id, tier_id) DO NOTHING;
    END LOOP;
END $$;

-- ==========================================================================
-- REDEMPTION REQUESTS — ~150 across various states
-- ==========================================================================
DO $$
DECLARE
    w record;
    states text[] := ARRAY['initiated','validating','completed','completed','completed','rejected','applied','applied','compensated','failed'];
    cnt int := 0;
    st text;
    req_amt float;
    ord_amt float;
BEGIN
    FOR w IN
        SELECT wal.id AS wallet_id, wal.merchant_id
        FROM wallets wal
        WHERE wal.is_bearer = false AND wal.customer_id IS NOT NULL
        AND abs(hashtext(wal.id::text)) % 13 = 0
        ORDER BY wal.id LIMIT 150
    LOOP
        cnt := cnt + 1;
        st := states[1 + (cnt % array_length(states, 1))];
        req_amt := 30 + (cnt * 17 % 300);
        ord_amt := 400 + (cnt * 41 % 4000);

        INSERT INTO redemption_requests (merchant_id, wallet_id, requested_amount, eligible_amount, applied_amount, order_amount, order_id, payment_method, state, rejection_reason)
        VALUES (w.merchant_id, w.wallet_id, req_amt,
                CASE WHEN st NOT IN ('initiated','validating') THEN req_amt ELSE NULL END,
                CASE WHEN st IN ('completed','applied','compensated') THEN req_amt ELSE NULL END,
                ord_amt,
                'order:REDM-' || cnt,
                CASE WHEN cnt % 3 = 0 THEN 'upi' WHEN cnt % 3 = 1 THEN 'card' ELSE 'cod' END,
                st::redemption_state,
                CASE WHEN st = 'rejected' THEN 'Insufficient balance' WHEN st = 'failed' THEN 'Shopify discount creation failed' ELSE NULL END);
    END LOOP;
END $$;

-- ==========================================================================
-- NEWSLETTER SIGNUPS — ~100 across merchants
-- ==========================================================================
DO $$
DECLARE
    w record;
    entry_id uuid;
    cnt int := 0;
    cemail text;
BEGIN
    FOR w IN
        SELECT wal.id AS wallet_id, wal.merchant_id, wal.customer_id
        FROM wallets wal
        WHERE wal.is_bearer = false AND wal.customer_id IS NOT NULL
        AND abs(hashtext(wal.id::text)) % 20 = 0
        ORDER BY wal.id LIMIT 100
    LOOP
        cnt := cnt + 1;
        SELECT email INTO cemail FROM customers WHERE id = w.customer_id;
        entry_id := gen_random_uuid();

        INSERT INTO ledger_entries (id, wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, actor_id, idempotency_key, created_at)
        VALUES (entry_id, w.wallet_id, 'earned_credit', 'in', 10, 10, 1.0, 'system', 'newsletter', 'newsletter:' || w.customer_id || ':' || w.merchant_id, now() - ((cnt * 3) || ' days')::interval);

        INSERT INTO newsletter_signups (merchant_id, customer_id, ledger_entry_id, email)
        VALUES (w.merchant_id, w.customer_id, entry_id, cemail)
        ON CONFLICT (merchant_id, customer_id) DO NOTHING;
    END LOOP;
END $$;

-- ==========================================================================
-- CONNECTORS — WhatsApp + Email + SMS for all merchants
-- ==========================================================================
INSERT INTO connectors (merchant_id, capability, vendor, config, priority)
SELECT id, 'whatsapp-bsp', 'interakt', jsonb_build_object('api_key','ik_test_' || slug, 'phone_number_id','91' || (1000000000 + row_number() OVER (ORDER BY id))), 0
FROM merchants;

INSERT INTO connectors (merchant_id, capability, vendor, config, priority)
SELECT id, 'email', 'smtp', jsonb_build_object('host','smtp.gmail.com','port',587,'from','rewards@' || slug || '.com'), 0
FROM merchants;

INSERT INTO connectors (merchant_id, capability, vendor, config, priority)
SELECT id, 'sms', 'twilio', jsonb_build_object('account_sid','AC_test_' || slug, 'from','+91' || (8000000000 + row_number() OVER (ORDER BY id))), 0
FROM merchants ORDER BY id LIMIT 12;

-- ==========================================================================
-- NOTIFICATION TEMPLATES — 4 per merchant
-- ==========================================================================
INSERT INTO notification_templates (merchant_id, name, channel, template_id, body_template, variables)
SELECT id, 'Points Earned', 'whatsapp', 'points_earned_v1',
       'Hey {{customer_name}}! You earned ₹{{amount}} credits on your order. Balance: ₹{{balance}}.',
       '[{"name":"customer_name"},{"name":"amount"},{"name":"balance"}]'::jsonb
FROM merchants;

INSERT INTO notification_templates (merchant_id, name, channel, body_template, variables)
SELECT id, 'Tier Upgrade', 'email',
       'Congratulations {{customer_name}}! You''ve been upgraded to {{tier_name}}. New perks: {{benefits}}',
       '[{"name":"customer_name"},{"name":"tier_name"},{"name":"benefits"}]'::jsonb
FROM merchants;

INSERT INTO notification_templates (merchant_id, name, channel, template_id, body_template, variables)
SELECT id, 'Redemption Applied', 'whatsapp', 'redemption_v1',
       '₹{{amount}} credits applied to order {{order_id}}. Enjoy!',
       '[{"name":"amount"},{"name":"order_id"}]'::jsonb
FROM merchants;

INSERT INTO notification_templates (merchant_id, name, channel, body_template, variables)
SELECT id, 'Gift Card Received', 'email',
       'Hi {{customer_name}}! You received a ₹{{amount}} gift card. Code: {{code}}. Redeem at {{store_url}}.',
       '[{"name":"customer_name"},{"name":"amount"},{"name":"code"},{"name":"store_url"}]'::jsonb
FROM merchants;

-- ==========================================================================
-- COALITIONS — 2 coalitions with cross-merchant groups
-- ==========================================================================
INSERT INTO coalitions (id, name) VALUES
('f0000000-0000-0000-0000-000000000001', 'Fashion Alliance'),
('f0000000-0000-0000-0000-000000000002', 'Food & Wellness Circle');

-- Fashion Alliance: merchants 2, 4, 6, 9, 11
INSERT INTO coalition_members (coalition_id, merchant_id, conversion_rate) VALUES
('f0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000002', 1.0),
('f0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000004', 0.5),
('f0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000006', 1.0),
('f0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000009', 0.8),
('f0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-00000000000b', 1.0);

-- Food & Wellness Circle: merchants 1, 3, 5, 7, 12
INSERT INTO coalition_members (coalition_id, merchant_id, conversion_rate) VALUES
('f0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-000000000001', 1.0),
('f0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-000000000003', 0.8),
('f0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-000000000005', 1.0),
('f0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-000000000007', 1.0),
('f0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-00000000000c', 0.6);

-- Coalition transfers — 20 cross-merchant transfers
DO $$
DECLARE
    cm record;
    from_cm record;
    to_cm record;
    cid uuid;
    from_wid uuid;
    to_wid uuid;
    amt float;
    tid uuid;
    cnt int := 0;
BEGIN
    FOR cm IN SELECT id, name FROM coalitions LOOP
        FOR from_cm IN SELECT merchant_id, conversion_rate FROM coalition_members WHERE coalition_id = cm.id ORDER BY merchant_id LIMIT 3 LOOP
            FOR to_cm IN SELECT merchant_id, conversion_rate FROM coalition_members WHERE coalition_id = cm.id AND merchant_id != from_cm.merchant_id ORDER BY merchant_id LIMIT 2 LOOP
                -- Find a customer who has wallets at both merchants
                SELECT w1.customer_id INTO cid
                FROM wallets w1
                JOIN wallets w2 ON w2.customer_id = w1.customer_id AND w2.merchant_id = to_cm.merchant_id
                WHERE w1.merchant_id = from_cm.merchant_id AND w1.is_bearer = false
                LIMIT 1;

                IF cid IS NOT NULL THEN
                    SELECT id INTO from_wid FROM wallets WHERE merchant_id = from_cm.merchant_id AND customer_id = cid;
                    SELECT id INTO to_wid FROM wallets WHERE merchant_id = to_cm.merchant_id AND customer_id = cid;

                    IF from_wid IS NOT NULL AND to_wid IS NOT NULL THEN
                        cnt := cnt + 1;
                        amt := 50 + (cnt * 23 % 200);
                        tid := gen_random_uuid();

                        INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, transfer_id, idempotency_key, created_at)
                        VALUES (from_wid, 'earned_credit', 'out', amt, amt, 1.0, 'system', tid, 'coal-out:' || cnt, now() - ((cnt * 5) || ' days')::interval);

                        INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent, conversion_rate, actor_type, transfer_id, idempotency_key, created_at)
                        VALUES (to_wid, 'earned_credit', 'in', round((amt * to_cm.conversion_rate / from_cm.conversion_rate)::numeric, 2),
                                round((amt * to_cm.conversion_rate / from_cm.conversion_rate)::numeric, 2),
                                1.0, 'system', tid, 'coal-in:' || cnt, now() - ((cnt * 5) || ' days')::interval);

                        INSERT INTO coalition_transfers (coalition_id, customer_id, from_merchant_id, to_merchant_id, from_wallet_id, to_wallet_id, amount, converted_amount, conversion_rate, transfer_id)
                        VALUES (cm.id, cid, from_cm.merchant_id, to_cm.merchant_id, from_wid, to_wid, amt,
                                round((amt * to_cm.conversion_rate / from_cm.conversion_rate)::numeric, 2),
                                round((to_cm.conversion_rate / from_cm.conversion_rate)::numeric, 4), tid);
                    END IF;
                END IF;
            END LOOP;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- PRODUCT COLLECTION MAPPINGS — sample Shopify data
-- ==========================================================================
DO $$
DECLARE
    mid uuid;
    collections text[] := ARRAY['Sale','New Arrivals','Bestsellers','Premium','Clearance'];
    m_cnt int := 0;
    p int;
BEGIN
    FOR mid IN SELECT id FROM merchants ORDER BY id LIMIT 10 LOOP
        m_cnt := m_cnt + 1;
        FOR p IN 1..20 LOOP
            INSERT INTO product_collection_mappings (merchant_id, product_id, collection_id, collection_name)
            VALUES (
                mid,
                'gid://shopify/Product/' || (m_cnt * 1000 + p),
                'gid://shopify/Collection/' || (m_cnt * 100 + (p % 5) + 1),
                collections[1 + (p % 5)]
            );
        END LOOP;
    END LOOP;
END $$;

COMMIT;

-- ==========================================================================
-- Summary
-- ==========================================================================
\echo ''
\echo '=== Seed Complete ==='
SELECT 'merchants'              AS entity, count(*) FROM merchants
UNION ALL SELECT 'customers',              count(*) FROM customers
UNION ALL SELECT 'wallets',                count(*) FROM wallets
UNION ALL SELECT 'ledger_entries',         count(*) FROM ledger_entries
UNION ALL SELECT 'events',                 count(*) FROM events
UNION ALL SELECT 'rules',                  count(*) FROM rules
UNION ALL SELECT 'campaigns',              count(*) FROM campaigns
UNION ALL SELECT 'wallet_policies',        count(*) FROM wallet_policies
UNION ALL SELECT 'loyalty_programs',       count(*) FROM loyalty_programs
UNION ALL SELECT 'loyalty_tiers',          count(*) FROM loyalty_tiers
UNION ALL SELECT 'customer_tiers',         count(*) FROM customer_tiers
UNION ALL SELECT 'customer_memberships',   count(*) FROM customer_memberships
UNION ALL SELECT 'gift_cards',             count(*) FROM gift_cards
UNION ALL SELECT 'referral_programs',      count(*) FROM referral_programs
UNION ALL SELECT 'referral_codes',         count(*) FROM referral_codes
UNION ALL SELECT 'referral_conversions',   count(*) FROM referral_conversions
UNION ALL SELECT 'cod_orders',             count(*) FROM cod_orders
UNION ALL SELECT 'milestone_configs',      count(*) FROM milestone_configs
UNION ALL SELECT 'milestone_achievements', count(*) FROM milestone_achievements
UNION ALL SELECT 'streak_configs',         count(*) FROM streak_configs
UNION ALL SELECT 'streak_achievements',    count(*) FROM streak_achievements
UNION ALL SELECT 'spin_wheel_configs',     count(*) FROM spin_wheel_configs
UNION ALL SELECT 'spin_wheel_segments',    count(*) FROM spin_wheel_segments
UNION ALL SELECT 'spin_results',           count(*) FROM spin_results
UNION ALL SELECT 'newsletter_signups',     count(*) FROM newsletter_signups
UNION ALL SELECT 'connectors',             count(*) FROM connectors
UNION ALL SELECT 'notification_templates', count(*) FROM notification_templates
UNION ALL SELECT 'coalitions',             count(*) FROM coalitions
UNION ALL SELECT 'coalition_members',      count(*) FROM coalition_members
UNION ALL SELECT 'coalition_transfers',    count(*) FROM coalition_transfers
UNION ALL SELECT 'product_collection_mappings', count(*) FROM product_collection_mappings
UNION ALL SELECT 'customer_order_stats',   count(*) FROM customer_order_stats
UNION ALL SELECT 'redemption_requests',    count(*) FROM redemption_requests
ORDER BY 1;
