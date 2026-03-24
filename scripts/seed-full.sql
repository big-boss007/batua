-- ==========================================================================
-- Batua Full Seed — 50 merchants, 100 customers each, 20 txns each
-- Run after: make reset-db (clean DB with migrations)
-- Usage: psql -d batua -f scripts/seed-full.sql
-- ==========================================================================

BEGIN;

-- ==========================================================================
-- TRUNCATE everything (safe re-run)
-- ==========================================================================
TRUNCATE
    coalition_transfers, coalition_members, coalitions,
    customer_memberships,
    spin_results, spin_wheel_segments, spin_wheel_configs,
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
-- 1. GEO POLICY
-- ==========================================================================
INSERT INTO geo_policies (id, geo_code, name, config) VALUES
('a0000000-0000-0000-0000-000000000001', 'india', 'India',
 '{"currency":"INR","timezone":"Asia/Kolkata","wallet_enabled":true,"gift_card_enabled":true,"cod_enabled":true,"referrals_enabled":true,"max_wallet_balance":10000,"kyc_threshold":10000}');

-- ==========================================================================
-- 2. MERCHANTS (50)
-- ==========================================================================
DO $$
DECLARE
    m_names TEXT[] := ARRAY[
        'Chai & Co', 'Desi Drapes', 'Spice Route', 'Kurta Palace', 'Ghee & Gold',
        'Rang Manch', 'Silk Story', 'Masala Box', 'Jhumka Junction', 'Neem & Tulsi',
        'Papad Wala', 'Khadi Kraft', 'Biryani Bros', 'Henna House', 'Mango Maison',
        'Tikka Town', 'Zari Zeal', 'Saffron & Sage', 'Paratha Point', 'Lotus Luxe',
        'Ajrakh Art', 'Paan Palace', 'Chikankari Club', 'Raita Republic', 'Dhoop Diya',
        'Phulkari Fab', 'Garam Masala Co', 'Bindis & Bangles', 'Cardamom Café', 'Ikat India',
        'Samosa Street', 'Chanderi Charm', 'Pickle Jar', 'Kolhapuri King', 'Attar Avenue',
        'Rogan Josh & Co', 'Patola Prints', 'Lassi Land', 'Mehndi Magic', 'Tussar Tales',
        'Dal Dynasty', 'Lucknowi Lane', 'Nimbu Pani Co', 'Banarasi Bazaar', 'Kulfi Corner',
        'Chutney Collective', 'Leheriya Lab', 'Aam Aadmi Apparel', 'Kesar Kitchen', 'Sherwani Studio'
    ];
    m_slugs TEXT[] := ARRAY[
        'chaiandco', 'desidrapes', 'spiceroute', 'kurtapalace', 'gheeandgold',
        'rangmanch', 'silkstory', 'masalabox', 'jhumkajunction', 'neemtulsi',
        'papadwala', 'khadikraft', 'biryanibros', 'hennahouse', 'mangomaison',
        'tikkatown', 'zarizeal', 'saffronsage', 'parathapoint', 'lotusluxe',
        'ajrakhart', 'paanpalace', 'chikanclub', 'raitarepublic', 'dhoopdiya',
        'phulkarifab', 'garammasalaco', 'bindisbangles', 'cardamomcafe', 'ikatindia',
        'samosastreet', 'chandericharm', 'picklejar', 'kolhapuriking', 'attaravenue',
        'roganjoshco', 'patolaprints', 'lassiland', 'mehndimagic', 'tussartales',
        'daldynasty', 'lucknowilane', 'nimbupani', 'banarasibazaar', 'kulficorner',
        'chutneycollective', 'leheriyalab', 'aamapparel', 'kesarkitchen', 'sherwanistudio'
    ];
    pts_names TEXT[] := ARRAY['Stars','Points','Coins','Gems','Sparks','Perks','Tokens','Rewards','Credits','Jewels'];
    pts_icons TEXT[] := ARRAY['★','pts','🪙','💎','✨','🎁','🔷','🏆','💰','💍'];
    pts_rates DOUBLE PRECISION[] := ARRAY[0.25, 1.0, 0.5, 0.1, 0.25, 1.0, 0.5, 0.25, 1.0, 0.1];
    tiers TEXT[] := ARRAY['free','growth','pro'];
    m_idx INT;
    pts_idx INT;
BEGIN
    FOR m_idx IN 1..50 LOOP
        pts_idx := ((m_idx - 1) % 10) + 1;
        INSERT INTO merchants (
            id, external_id, name, domain, currency, slug, plan_tier,
            geo_policy_id, points_name, points_icon, points_to_currency_rate,
            created_at
        ) VALUES (
            ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid,
            'gid://shopify/Shop/' || (10000 + m_idx),
            m_names[m_idx],
            m_slugs[m_idx] || '.myshopify.com',
            'INR',
            m_slugs[m_idx],
            tiers[((m_idx - 1) % 3) + 1],
            'a0000000-0000-0000-0000-000000000001',
            pts_names[pts_idx],
            pts_icons[pts_idx],
            pts_rates[pts_idx],
            now() - interval '180 days' + (m_idx * interval '1 day')
        );
    END LOOP;
END $$;

-- ==========================================================================
-- 3. CUSTOMERS (5000 = 100 per merchant)
--    Global phone uniqueness: +9190MMCCCCCC  (MM=merchant 01-50, CCCCCC=customer 000001-000100)
-- ==========================================================================
DO $$
DECLARE
    first_names TEXT[] := ARRAY[
        'Aarav','Vivaan','Aditya','Vihaan','Arjun','Reyansh','Sai','Arnav','Dhruv','Kabir',
        'Ananya','Diya','Myra','Sara','Aadhya','Isha','Kiara','Riya','Pari','Anika',
        'Rohan','Karan','Ishaan','Lakshya','Yash','Vedant','Krish','Tanish','Neil','Ayaan',
        'Priya','Nisha','Meera','Kavya','Zara','Sanya','Tara','Nandini','Ira','Avni',
        'Rahul','Amit','Vikram','Suresh','Rajesh','Deepak','Manoj','Nikhil','Gaurav','Pankaj',
        'Pooja','Neha','Swati','Shreya','Ankita','Divya','Sakshi','Komal','Ritika','Preeti',
        'Dev','Om','Shiv','Rudra','Atharv','Parth','Harsh','Jay','Raj','Manav',
        'Aashi','Sia','Navya','Aditi','Mahika','Anvi','Suhana','Ishita','Bhavya','Kyra',
        'Varun','Kunal','Mohit','Sahil','Abhishek','Ajay','Sanjay','Ravi','Ashok','Sunil',
        'Sneha','Pallavi','Tanvi','Kriti','Simran','Jasmine','Radhika','Bhavna','Megha','Archana'
    ];
    last_names TEXT[] := ARRAY[
        'Sharma','Patel','Singh','Kumar','Gupta','Reddy','Joshi','Mehta','Shah','Verma',
        'Iyer','Nair','Rao','Das','Chatterjee','Mukherjee','Kapoor','Malhotra','Bhatia','Chopra',
        'Pillai','Menon','Desai','Thakur','Banerjee','Mishra','Srivastava','Pandey','Chauhan','Yadav',
        'Agarwal','Saxena','Tiwari','Dubey','Jain','Goyal','Arora','Sethi','Khanna','Bajaj',
        'Bhat','Hegde','Kulkarni','Patil','Naik','Shetty','Kamat','Deshpande','Rangan','Mani'
    ];
    m_id UUID;
    c_phone TEXT;
    c_email TEXT;
    c_name TEXT;
    fn_idx INT;
    ln_idx INT;
    bday DATE;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            fn_idx := ((m_idx * 7 + c_idx * 3) % 100) + 1;
            ln_idx := ((m_idx * 3 + c_idx * 7) % 50) + 1;
            c_name := first_names[fn_idx] || ' ' || last_names[ln_idx];
            c_phone := '+9190' || lpad(m_idx::text, 2, '0') || lpad(c_idx::text, 6, '0');
            c_email := lower(first_names[fn_idx]) || '.' || lower(last_names[ln_idx])
                       || (m_idx * 100 + c_idx)::text || '@gmail.com';
            -- ~30% have birthdays
            IF (m_idx + c_idx) % 3 = 0 THEN
                bday := '1985-01-01'::date + ((m_idx * 100 + c_idx) % 10000) * interval '1 day';
            ELSE
                bday := NULL;
            END IF;

            INSERT INTO customers (
                id, phone, email, name, external_id, is_verified, birthday,
                created_at
            ) VALUES (
                ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid,
                c_phone, c_email, c_name,
                'gid://shopify/Customer/' || (m_idx * 1000 + c_idx),
                true, bday,
                now() - interval '170 days' + (c_idx * interval '1 day')
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 4. WALLETS (5000 customer wallets + 200 bearer wallets for gift cards)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_id UUID;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;
            INSERT INTO wallets (id, merchant_id, customer_id, is_bearer, created_at) VALUES (
                ('d' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid,
                m_id, c_id, false,
                now() - interval '170 days' + (c_idx * interval '1 day')
            );
        END LOOP;
    END LOOP;

    -- Bearer wallets for gift cards (200 across first 40 merchants, 5 each)
    FOR m_idx IN 1..40 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR gc_idx IN 1..5 LOOP
            INSERT INTO wallets (id, merchant_id, customer_id, is_bearer, bearer_code, created_at) VALUES (
                ('e' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(gc_idx::text, 12, '0'))::uuid,
                m_id, NULL, true,
                'GC-' || upper(lpad(m_idx::text, 3, '0')) || '-' || lpad(gc_idx::text, 3, '0'),
                now() - interval '90 days' + (gc_idx * interval '5 days')
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 5. WALLET POLICIES (6 bucket types per merchant = 300)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    btypes TEXT[] := ARRAY['earned_credit','cod_pending','gift_card','referral_reward','goodwill_credit','refund_credit'];
    min_reds DOUBLE PRECISION[] := ARRAY[10, 0, 0, 10, 0, 0];
    steps DOUBLE PRECISION[] := ARRAY[1, 1, 1, 1, 1, 1];
    max_pcts DOUBLE PRECISION[] := ARRAY[50, 0, 100, 50, 100, 100];
    max_fixeds DOUBLE PRECISION[] := ARRAY[500, 0, 5000, 200, 1000, 2000];
    expiry_days INT[] := ARRAY[90, 30, 365, 180, 60, 90];
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR b_idx IN 1..6 LOOP
            INSERT INTO wallet_policies (
                merchant_id, bucket_type,
                min_redemption, step_size, max_per_order_pct, max_per_order_fixed,
                stackable_with_discounts, default_expiry_days,
                excluded_payment_methods
            ) VALUES (
                m_id, btypes[b_idx]::bucket_type,
                min_reds[b_idx], steps[b_idx],
                max_pcts[b_idx] + (m_idx % 10),
                max_fixeds[b_idx] + (m_idx * 10),
                b_idx != 2,  -- cod_pending not stackable
                expiry_days[b_idx] + (m_idx % 30),
                CASE WHEN b_idx = 1 THEN ARRAY['cod'] ELSE ARRAY[]::TEXT[] END
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 6. RULES (3 per merchant = 150) + 7. RULE SNAPSHOTS
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    r_id UUID;
    r_config JSONB;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        -- Rule 1: Order reward (percentage)
        r_id := gen_random_uuid();
        r_config := jsonb_build_object(
            'event_type', 'order.completed',
            'conditions', jsonb_build_array(
                jsonb_build_object('field', 'order_amount', 'operator', 'gte', 'value', 300 + (m_idx * 10))
            ),
            'action', jsonb_build_object(
                'bucket_type', 'earned_credit',
                'calculation', 'percentage',
                'value', 5 + (m_idx % 10),
                'max_amount', 200 + (m_idx * 5),
                'expiry_days', 90
            )
        );
        INSERT INTO rules (id, merchant_id, rule_type, name, config, version, is_active, created_at)
        VALUES (r_id, m_id, 'reward', 'Order Reward ' || m_idx, r_config, 1, true,
                now() - interval '160 days');
        INSERT INTO rule_snapshots (rule_id, version, config, created_at)
        VALUES (r_id, 1, r_config, now() - interval '160 days');

        -- Rule 2: Review reward (fixed)
        r_id := gen_random_uuid();
        r_config := jsonb_build_object(
            'event_type', 'review.submitted',
            'conditions', jsonb_build_array(),
            'action', jsonb_build_object(
                'bucket_type', 'earned_credit',
                'calculation', 'fixed',
                'value', 20 + (m_idx % 30),
                'expiry_days', 60
            )
        );
        INSERT INTO rules (id, merchant_id, rule_type, name, config, version, is_active, created_at)
        VALUES (r_id, m_id, 'reward', 'Review Bonus ' || m_idx, r_config, 1, true,
                now() - interval '155 days');
        INSERT INTO rule_snapshots (rule_id, version, config, created_at)
        VALUES (r_id, 1, r_config, now() - interval '155 days');

        -- Rule 3: First order bonus (fixed, first 35 merchants only)
        IF m_idx <= 35 THEN
            r_id := gen_random_uuid();
            r_config := jsonb_build_object(
                'event_type', 'order.completed',
                'conditions', jsonb_build_array(
                    jsonb_build_object('field', 'order_count', 'operator', 'eq', 'value', 1)
                ),
                'action', jsonb_build_object(
                    'bucket_type', 'earned_credit',
                    'calculation', 'fixed',
                    'value', 50 + (m_idx * 5),
                    'expiry_days', 30
                )
            );
            INSERT INTO rules (id, merchant_id, rule_type, name, config, version, is_active, created_at)
            VALUES (r_id, m_id, 'reward', 'First Order Bonus ' || m_idx, r_config, 1,
                    m_idx <= 30,  -- some inactive
                    now() - interval '150 days');
            INSERT INTO rule_snapshots (rule_id, version, config, created_at)
            VALUES (r_id, 1, r_config, now() - interval '150 days');
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- 8. EVENTS (~20 per customer = ~100,000)
-- 9. LEDGER ENTRIES (~20 per customer = ~100,000)
-- These are the big tables — use a single loop for efficiency.
--
-- Distribution per customer (20 entries):
--   12 = earned_credit/in (orders)
--    3 = earned_credit/out (redemptions)
--    2 = cod_pending/held
--    1 = gift_card/in
--    1 = referral_reward/in
--    1 = goodwill_credit/in  (only odd customers)
--      or refund_credit/in   (only even customers)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    w_id UUID;
    c_id UUID;
    ev_id UUID;
    pts_rate DOUBLE PRECISION;
    pts_rates DOUBLE PRECISION[] := ARRAY[0.25, 1.0, 0.5, 0.1, 0.25, 1.0, 0.5, 0.25, 1.0, 0.1];
    order_amt DOUBLE PRECISION;
    earn_amt DOUBLE PRECISION;
    cur_equiv DOUBLE PRECISION;
    ev_ts TIMESTAMPTZ;
    day_offset INT;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        pts_rate := pts_rates[((m_idx - 1) % 10) + 1];

        FOR c_idx IN 1..100 LOOP
            w_id := ('d' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;

            FOR t_idx IN 1..20 LOOP
                day_offset := 170 - (t_idx * 8) - (c_idx % 5);
                IF day_offset < 1 THEN day_offset := 1; END IF;
                ev_ts := now() - (day_offset || ' days')::interval
                         + ((m_idx * 60 + c_idx * 30 + t_idx * 15) % 86400 || ' seconds')::interval;

                -- Entries 1-12: earned_credit/in from orders
                IF t_idx <= 12 THEN
                    order_amt := 500 + ((m_idx * 31 + c_idx * 17 + t_idx * 53) % 4500);
                    earn_amt  := greatest(10, round((order_amt * (5 + (m_idx % 10)) / 100.0)::numeric, 0)::double precision);
                    cur_equiv := earn_amt * pts_rate;
                    ev_id := gen_random_uuid();

                    INSERT INTO events (
                        id, merchant_id, event_type, event_source, external_event_id,
                        payload, state, idempotency_key, created_at, processed_at
                    ) VALUES (
                        ev_id, m_id, 'order.completed', 'shopify',
                        'evt_ord_' || m_idx || '_' || c_idx || '_' || t_idx,
                        jsonb_build_object(
                            'order_id', 'ORD-' || m_idx || '-' || c_idx || '-' || t_idx,
                            'order_amount', order_amt,
                            'customer_id', 'gid://shopify/Customer/' || (m_idx * 1000 + c_idx),
                            'line_items', jsonb_build_array(
                                jsonb_build_object('title', 'Product ' || t_idx, 'quantity', 1 + (t_idx % 3), 'price', order_amt)
                            ),
                            'payment_method', CASE WHEN t_idx % 5 = 0 THEN 'cod' ELSE 'prepaid' END
                        ),
                        'processed', 'seed-evt-' || m_idx || '-' || c_idx || '-' || t_idx,
                        ev_ts, ev_ts + interval '2 seconds'
                    );

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, event_id, actor_type, actor_id,
                        constraints,
                        expires_at, created_at, state
                    ) VALUES (
                        w_id, 'earned_credit', 'in',
                        earn_amt, cur_equiv, pts_rate,
                        'seed-earn-' || m_idx || '-' || c_idx || '-' || t_idx,
                        ev_id, 'system', 'rule-engine',
                        '{"transferable":false,"stackable":true,"excluded_payment_methods":[]}'::jsonb,
                        ev_ts + interval '90 days',
                        ev_ts, 'active'
                    );

                -- Entries 13-15: earned_credit/out (redemptions)
                ELSIF t_idx <= 15 THEN
                    earn_amt := 20 + ((m_idx + c_idx + t_idx) % 80);
                    cur_equiv := earn_amt * pts_rate;

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, actor_type, actor_id,
                        constraints, created_at, state
                    ) VALUES (
                        w_id, 'earned_credit', 'out',
                        earn_amt, cur_equiv, pts_rate,
                        'seed-redeem-' || m_idx || '-' || c_idx || '-' || t_idx,
                        'system', 'redemption-engine',
                        '{"transferable":false,"stackable":true}'::jsonb,
                        ev_ts, 'redeemed'
                    );

                -- Entries 16-17: cod_pending/held
                ELSIF t_idx <= 17 THEN
                    order_amt := 800 + ((m_idx * 41 + c_idx * 23) % 3000);
                    earn_amt  := greatest(10, round((order_amt * 5 / 100.0)::numeric, 0)::double precision);
                    cur_equiv := earn_amt * pts_rate;
                    ev_id := gen_random_uuid();

                    INSERT INTO events (
                        id, merchant_id, event_type, event_source, external_event_id,
                        payload, state, idempotency_key, created_at, processed_at
                    ) VALUES (
                        ev_id, m_id, 'order.completed', 'shopify',
                        'evt_cod_' || m_idx || '_' || c_idx || '_' || t_idx,
                        jsonb_build_object(
                            'order_id', 'COD-' || m_idx || '-' || c_idx || '-' || t_idx,
                            'order_amount', order_amt,
                            'customer_id', 'gid://shopify/Customer/' || (m_idx * 1000 + c_idx),
                            'payment_method', 'cod'
                        ),
                        'processed', 'seed-evt-cod-' || m_idx || '-' || c_idx || '-' || t_idx,
                        ev_ts, ev_ts + interval '2 seconds'
                    );

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, event_id, actor_type, actor_id,
                        constraints, expires_at, created_at, state
                    ) VALUES (
                        w_id, 'cod_pending', 'held',
                        earn_amt, cur_equiv, pts_rate,
                        'seed-cod-' || m_idx || '-' || c_idx || '-' || t_idx,
                        ev_id, 'system', 'cod-engine',
                        '{"transferable":false,"stackable":false,"excluded_payment_methods":["cod"]}'::jsonb,
                        ev_ts + interval '30 days',
                        ev_ts, 'active'
                    );

                -- Entry 18: gift_card/in
                ELSIF t_idx = 18 THEN
                    earn_amt := 100 + ((m_idx * 7 + c_idx * 11) % 400);
                    cur_equiv := earn_amt;  -- gift cards are 1:1

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, actor_type, actor_id,
                        payment_reference, constraints,
                        expires_at, created_at, state
                    ) VALUES (
                        w_id, 'gift_card', 'in',
                        earn_amt, cur_equiv, 1.0,
                        'seed-gc-' || m_idx || '-' || c_idx || '-' || t_idx,
                        'system', 'gift-card-engine',
                        'PAY-GC-' || m_idx || '-' || c_idx,
                        '{"transferable":true,"stackable":true}'::jsonb,
                        ev_ts + interval '365 days',
                        ev_ts, 'active'
                    );

                -- Entry 19: referral_reward/in
                ELSIF t_idx = 19 THEN
                    earn_amt := 50 + (m_idx % 150);
                    cur_equiv := earn_amt * pts_rate;

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, actor_type, actor_id,
                        constraints, expires_at, created_at, state
                    ) VALUES (
                        w_id, 'referral_reward', 'in',
                        earn_amt, cur_equiv, pts_rate,
                        'seed-ref-' || m_idx || '-' || c_idx || '-' || t_idx,
                        'system', 'referral-engine',
                        '{"transferable":false,"stackable":true}'::jsonb,
                        ev_ts + interval '180 days',
                        ev_ts, 'active'
                    );

                -- Entry 20: goodwill (odd) or refund (even)
                ELSIF t_idx = 20 THEN
                    earn_amt := 25 + ((m_idx + c_idx) % 200);
                    cur_equiv := earn_amt * pts_rate;

                    IF c_idx % 2 = 1 THEN
                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, actor_type, actor_id,
                            constraints, expires_at, created_at, state
                        ) VALUES (
                            w_id, 'goodwill_credit', 'in',
                            earn_amt, cur_equiv, pts_rate,
                            'seed-gw-' || m_idx || '-' || c_idx || '-' || t_idx,
                            'human', 'admin-user',
                            '{"transferable":false,"stackable":true}'::jsonb,
                            ev_ts + interval '60 days',
                            ev_ts, 'active'
                        );
                    ELSE
                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, actor_type, actor_id,
                            constraints, expires_at, created_at, state
                        ) VALUES (
                            w_id, 'refund_credit', 'in',
                            earn_amt, cur_equiv, pts_rate,
                            'seed-rf-' || m_idx || '-' || c_idx || '-' || t_idx,
                            'system', 'refund-engine',
                            '{"transferable":false,"stackable":true}'::jsonb,
                            ev_ts + interval '90 days',
                            ev_ts, 'active'
                        );
                    END IF;
                END IF;
            END LOOP;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 10. LOYALTY PROGRAMS (first 30 merchants)
-- 11. LOYALTY TIERS (4 per program = 120)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    lp_id UUID;
    eval_criteria TEXT;
    tier_names TEXT[] := ARRAY['Bronze','Silver','Gold','Platinum'];
    thresholds DOUBLE PRECISION[] := ARRAY[0, 5000, 15000, 50000];
    multipliers DOUBLE PRECISION[] := ARRAY[1.0, 1.25, 1.5, 2.0];
BEGIN
    FOR m_idx IN 1..30 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        lp_id := ('f0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        eval_criteria := CASE WHEN m_idx % 2 = 0 THEN 'points' ELSE 'spend' END;

        INSERT INTO loyalty_programs (id, merchant_id, name, evaluation_criteria, evaluation_period_days, is_active)
        VALUES (lp_id, m_id, 'Loyalty Program', eval_criteria, CASE WHEN m_idx % 3 = 0 THEN 365 ELSE NULL END, true);

        FOR t_idx IN 1..4 LOOP
            INSERT INTO loyalty_tiers (
                id, program_id, name, rank, threshold, earn_rate_multiplier, benefits
            ) VALUES (
                ('f' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(t_idx::text, 12, '0'))::uuid,
                lp_id, tier_names[t_idx], t_idx,
                thresholds[t_idx] + (m_idx * 100),
                multipliers[t_idx],
                jsonb_build_object(
                    'free_shipping', t_idx >= 3,
                    'early_access', t_idx >= 2,
                    'birthday_bonus', t_idx * 50,
                    'exclusive_sales', t_idx = 4
                )
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 12. CUSTOMER TIERS (for first 30 merchants, ~70% Bronze, ~20% Silver, ~8% Gold, ~2% Platinum)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_id UUID;
    tier_rank INT;
    tier_id UUID;
    qual_val DOUBLE PRECISION;
BEGIN
    FOR m_idx IN 1..30 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;
            -- Determine tier rank based on distribution
            IF c_idx <= 2 THEN tier_rank := 4;       -- 2% Platinum
            ELSIF c_idx <= 10 THEN tier_rank := 3;    -- 8% Gold
            ELSIF c_idx <= 30 THEN tier_rank := 2;    -- 20% Silver
            ELSE tier_rank := 1;                       -- 70% Bronze
            END IF;

            tier_id := ('f' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(tier_rank::text, 12, '0'))::uuid;
            qual_val := CASE tier_rank
                WHEN 1 THEN 500 + (c_idx * 40)
                WHEN 2 THEN 5000 + (c_idx * 300)
                WHEN 3 THEN 15000 + (c_idx * 500)
                WHEN 4 THEN 50000 + (c_idx * 1000)
            END;

            INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value, qualified_at)
            VALUES (c_id, m_id, tier_id, qual_val, now() - interval '60 days' + (c_idx * interval '12 hours'));
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 13. GIFT CARDS (200 across first 40 merchants, 5 per merchant)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    bearer_w_id UUID;
    claimed_w_id UUID;
    gc_amt DOUBLE PRECISION;
BEGIN
    FOR m_idx IN 1..40 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR gc_idx IN 1..5 LOOP
            bearer_w_id := ('e' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(gc_idx::text, 12, '0'))::uuid;
            gc_amt := 100 * (1 + (gc_idx % 5)) + (m_idx * 10);

            IF gc_idx <= 3 THEN
                -- Claimed gift cards
                claimed_w_id := ('d' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad((gc_idx * 10)::text, 12, '0'))::uuid;
                INSERT INTO gift_cards (
                    merchant_id, wallet_id, code, initial_amount, current_amount,
                    issued_by, issued_by_id, is_claimed, claimed_by_wallet_id, claimed_at,
                    expires_at, created_at
                ) VALUES (
                    m_id, bearer_w_id,
                    'GC-' || upper(lpad(m_idx::text, 3, '0')) || '-' || lpad(gc_idx::text, 3, '0'),
                    gc_amt, gc_amt * 0.6,
                    'human', 'admin-user',
                    true, claimed_w_id, now() - interval '30 days',
                    now() + interval '365 days',
                    now() - interval '60 days'
                );
            ELSE
                -- Unclaimed gift cards
                INSERT INTO gift_cards (
                    merchant_id, wallet_id, code, initial_amount, current_amount,
                    issued_by, issued_by_id, is_claimed,
                    expires_at, created_at
                ) VALUES (
                    m_id, bearer_w_id,
                    'GC-' || upper(lpad(m_idx::text, 3, '0')) || '-' || lpad(gc_idx::text, 3, '0'),
                    gc_amt, gc_amt,
                    'system', 'bulk-issue',
                    false,
                    now() + interval '365 days',
                    now() - interval '45 days'
                );
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 14. REFERRAL PROGRAMS (first 40 merchants)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
BEGIN
    FOR m_idx IN 1..40 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        INSERT INTO referral_programs (merchant_id, referrer_reward_amount, referee_reward_amount,
            max_referrals_per_customer, is_active)
        VALUES (
            m_id,
            50 + (m_idx * 3),
            25 + (m_idx * 2),
            CASE WHEN m_idx % 3 = 0 THEN NULL ELSE 10 + (m_idx % 5) END,
            m_idx <= 35
        );
    END LOOP;
END $$;

-- ==========================================================================
-- 15. REFERRAL CODES (2 per customer for first 10 merchants = 2000)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_id UUID;
BEGIN
    FOR m_idx IN 1..10 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;

            -- Auto-generated code
            INSERT INTO referral_codes (merchant_id, customer_id, code, is_vanity,
                total_referrals, total_conversions, is_active)
            VALUES (
                m_id, c_id,
                'REF' || lpad(m_idx::text, 2, '0') || lpad(c_idx::text, 3, '0') || 'A',
                false,
                (c_idx % 8), greatest(0, (c_idx % 8) - 2),
                true
            );

            -- Vanity code (every 3rd customer)
            IF c_idx % 3 = 0 THEN
                INSERT INTO referral_codes (merchant_id, customer_id, code, is_vanity,
                    total_referrals, total_conversions, is_active)
                VALUES (
                    m_id, c_id,
                    upper(substr(md5(m_idx::text || c_idx::text), 1, 6)),
                    true,
                    (c_idx % 5), greatest(0, (c_idx % 5) - 1),
                    true
                );
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 16. REFERRAL CONVERSIONS (~500 across first 10 merchants)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    referrer_cid UUID;
    referee_cid UUID;
    rc_id UUID;
BEGIN
    FOR m_idx IN 1..10 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR conv_idx IN 1..50 LOOP
            referrer_cid := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(conv_idx::text, 12, '0'))::uuid;
            referee_cid := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad((conv_idx + 50)::text, 12, '0'))::uuid;

            SELECT id INTO rc_id FROM referral_codes
            WHERE merchant_id = m_id AND customer_id = referrer_cid LIMIT 1;

            IF rc_id IS NOT NULL THEN
                INSERT INTO referral_conversions (
                    merchant_id, referral_code_id, referrer_id, referee_id,
                    order_id, is_suspicious, created_at
                ) VALUES (
                    m_id, rc_id, referrer_cid, referee_cid,
                    'ORD-REF-' || m_idx || '-' || conv_idx,
                    conv_idx % 20 = 0,
                    now() - interval '90 days' + (conv_idx * interval '1 day')
                );
            END IF;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 17. CUSTOMER ORDER STATS (1 per customer per merchant = 5000)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_id UUID;
    total_orders INT;
    total_spend DOUBLE PRECISION;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;
            total_orders := 12 + (c_idx % 8);  -- 12-19 orders (matching ledger events)
            total_spend := total_orders * (800 + (m_idx * 20 + c_idx * 10));

            INSERT INTO customer_order_stats (merchant_id, customer_id, total_orders, total_spend,
                first_order_at, last_order_at)
            VALUES (
                m_id, c_id, total_orders, total_spend,
                now() - interval '170 days' + (c_idx * interval '1 day'),
                now() - interval '10 days' + ((c_idx % 10) * interval '1 day')
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 18. COD ORDERS (~10% of order events — use cod_pending ledger entries)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    w_id UUID;
    le_id UUID;
    cod_state TEXT;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..100 LOOP
            w_id := ('d' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;

            -- Each customer has 2 COD entries (t_idx 16,17)
            FOR t_idx IN 16..17 LOOP
                SELECT id INTO le_id FROM ledger_entries
                WHERE idempotency_key = 'seed-cod-' || m_idx || '-' || c_idx || '-' || t_idx;

                IF le_id IS NOT NULL THEN
                    -- 70% delivered, 20% pending, 10% cancelled
                    IF (c_idx + t_idx) % 10 < 7 THEN cod_state := 'delivered';
                    ELSIF (c_idx + t_idx) % 10 < 9 THEN cod_state := 'pending';
                    ELSE cod_state := 'cancelled';
                    END IF;

                    INSERT INTO cod_orders (
                        merchant_id, order_id, wallet_id, ledger_entry_id,
                        state, delivery_confirmed_at, created_at
                    ) VALUES (
                        m_id,
                        'COD-' || m_idx || '-' || c_idx || '-' || t_idx,
                        w_id, le_id,
                        cod_state,
                        CASE WHEN cod_state = 'delivered' THEN now() - interval '5 days' ELSE NULL END,
                        now() - interval '30 days' + (c_idx * interval '6 hours')
                    );
                END IF;
            END LOOP;
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 19. REDEMPTION REQUESTS (~15% of customers = ~750)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    w_id UUID;
    req_state redemption_state;
    req_amt DOUBLE PRECISION;
    ord_amt DOUBLE PRECISION;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        FOR c_idx IN 1..15 LOOP  -- first 15 customers per merchant
            w_id := ('d' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad(c_idx::text, 12, '0'))::uuid;
            ord_amt := 1000 + ((m_idx * 37 + c_idx * 71) % 4000);
            req_amt := 50 + ((m_idx + c_idx) % 200);

            -- 80% completed, 10% applied, 10% rejected
            IF c_idx <= 12 THEN req_state := 'completed';
            ELSIF c_idx <= 13 THEN req_state := 'applied';
            ELSE req_state := 'rejected';
            END IF;

            INSERT INTO redemption_requests (
                merchant_id, wallet_id, requested_amount, eligible_amount, applied_amount,
                order_id, order_amount, payment_method, state,
                rejection_reason, created_at
            ) VALUES (
                m_id, w_id, req_amt,
                CASE WHEN req_state != 'rejected' THEN req_amt ELSE NULL END,
                CASE WHEN req_state IN ('completed','applied') THEN req_amt ELSE NULL END,
                'ORD-RDM-' || m_idx || '-' || c_idx,
                ord_amt, 'prepaid', req_state,
                CASE WHEN req_state = 'rejected' THEN 'Insufficient balance' ELSE NULL END,
                now() - interval '45 days' + (c_idx * interval '2 days')
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 20. CAMPAIGNS (2-3 per merchant = ~125)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_type TEXT;
    starts TIMESTAMPTZ;
    ends TIMESTAMPTZ;
    camp_id UUID;
    rule_id UUID;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        -- Get first rule for this merchant
        SELECT id INTO rule_id FROM rules WHERE merchant_id = m_id LIMIT 1;

        -- Campaign 1: Active multiplier weekend
        camp_id := gen_random_uuid();
        INSERT INTO campaigns (
            id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
            starts_at, ends_at, is_active
        ) VALUES (
            camp_id, m_id, '2x Weekend Bonanza', 'multiplier',
            '{"description":"Double points every weekend","target_days":["saturday","sunday"]}'::jsonb,
            rule_id, 2.0,
            now() - interval '30 days', now() + interval '60 days', true
        );
        INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
        VALUES (camp_id, '{"description":"Double points every weekend"}'::jsonb, 2.0);

        -- Campaign 2: Past festival bonus
        camp_id := gen_random_uuid();
        INSERT INTO campaigns (
            id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
            starts_at, ends_at, is_active
        ) VALUES (
            camp_id, m_id, 'Diwali Dhamaka', 'festive',
            '{"description":"Festive bonus points","flat_bonus":100}'::jsonb,
            rule_id, 3.0,
            now() - interval '150 days', now() - interval '140 days', false
        );
        INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
        VALUES (camp_id, '{"description":"Festive bonus points","flat_bonus":100}'::jsonb, 3.0);

        -- Campaign 3: (every other merchant) upcoming bonus
        IF m_idx % 2 = 0 THEN
            camp_id := gen_random_uuid();
            INSERT INTO campaigns (
                id, merchant_id, name, campaign_type, config, base_rule_id, multiplier,
                starts_at, ends_at, is_active
            ) VALUES (
                camp_id, m_id, 'Holi Special 3x', 'bonus',
                '{"description":"3x points for Holi week","min_order":500}'::jsonb,
                rule_id, 3.0,
                now() + interval '10 days', now() + interval '17 days', true
            );
            INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
            VALUES (camp_id, '{"description":"3x points for Holi week"}'::jsonb, 3.0);
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- 21. CONNECTORS (1-2 per merchant)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        -- WhatsApp connector (all merchants)
        INSERT INTO connectors (merchant_id, capability, vendor, config, is_active, priority) VALUES (
            m_id, 'whatsapp-bsp',
            CASE WHEN m_idx % 2 = 0 THEN 'interakt' ELSE 'wati' END,
            jsonb_build_object('api_key', 'sk_test_' || md5(m_idx::text), 'sender_id', '+91' || (9800000000 + m_idx)),
            true, 1
        );

        -- SMS connector (first 40 merchants)
        IF m_idx <= 40 THEN
            INSERT INTO connectors (merchant_id, capability, vendor, config, is_active, priority) VALUES (
                m_id, 'sms', 'msg91',
                jsonb_build_object('auth_key', 'msg91_' || md5(m_idx::text || 'sms'), 'sender_id', 'BREEZE'),
                true, 2
            );
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- 22. NOTIFICATION TEMPLATES (3 per merchant = 150)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
BEGIN
    FOR m_idx IN 1..50 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        INSERT INTO notification_templates (merchant_id, name, channel, template_id, body_template, variables) VALUES
        (m_id, 'Order Reward', 'whatsapp', 'tmpl_order_reward_' || m_idx,
         'Hi {{customer_name}}! You earned {{points}} {{points_name}} on your order of ₹{{order_amount}}. Check your wallet: {{storefront_url}}',
         '["customer_name","points","points_name","order_amount","storefront_url"]'::jsonb),
        (m_id, 'Welcome Bonus', 'whatsapp', 'tmpl_welcome_' || m_idx,
         'Welcome to {{merchant_name}}, {{customer_name}}! You''ve received {{points}} {{points_name}} as a welcome gift. 🎉',
         '["merchant_name","customer_name","points","points_name"]'::jsonb),
        (m_id, 'Points Expiry Reminder', 'sms', 'tmpl_expiry_' || m_idx,
         '{{customer_name}}, your {{points}} {{points_name}} at {{merchant_name}} expire in {{days}} days. Use them now!',
         '["customer_name","points","points_name","merchant_name","days"]'::jsonb);
    END LOOP;
END $$;

-- ==========================================================================
-- 23. SPIN WHEEL CONFIGS (first 20 merchants) + 6 SEGMENTS EACH
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    sw_id UUID;
    seg_labels TEXT[] := ARRAY['10 Points','25 Points','50 Points','100 Points','Better Luck','5 Points'];
    seg_amounts DOUBLE PRECISION[] := ARRAY[10, 25, 50, 100, 0, 5];
    seg_probs DOUBLE PRECISION[] := ARRAY[0.30, 0.20, 0.15, 0.05, 0.15, 0.15];
    seg_colors TEXT[] := ARRAY['#7c6aff','#4ade80','#f59e0b','#ef4444','#6b7280','#3b82f6'];
BEGIN
    FOR m_idx IN 1..20 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        sw_id := gen_random_uuid();

        INSERT INTO spin_wheel_configs (id, merchant_id, name, is_active, daily_spin_limit)
        VALUES (sw_id, m_id, 'Lucky Wheel', true, CASE WHEN m_idx % 3 = 0 THEN 2 ELSE 1 END);

        FOR s_idx IN 1..6 LOOP
            INSERT INTO spin_wheel_segments (wheel_id, label, reward_amount, probability, color, position)
            VALUES (sw_id, seg_labels[s_idx], seg_amounts[s_idx], seg_probs[s_idx], seg_colors[s_idx], s_idx);
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 24. CUSTOMER MEMBERSHIPS (~5% of customers for first 15 merchants = ~75)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
    c_id UUID;
    t_id UUID;
BEGIN
    FOR m_idx IN 1..15 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;
        -- Pick the Gold tier (rank 3) for memberships
        t_id := ('f' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-000000000003')::uuid;

        FOR c_idx IN 1..5 LOOP
            c_id := ('c' || lpad(m_idx::text, 4, '0') || '000-0000-0000-0000-' || lpad((c_idx + 90)::text, 12, '0'))::uuid;

            INSERT INTO customer_memberships (
                merchant_id, customer_id, tier_id, status,
                started_at, expires_at, renewed_count
            ) VALUES (
                m_id, c_id, t_id, 'active',
                now() - interval '60 days',
                now() + interval '305 days',
                CASE WHEN c_idx <= 2 THEN 1 ELSE 0 END
            );
        END LOOP;
    END LOOP;
END $$;

-- ==========================================================================
-- 25. MILESTONE CONFIGS (2-3 per merchant, first 25 merchants)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
BEGIN
    FOR m_idx IN 1..25 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount) VALUES
        (m_id, '₹5,000 Spend Club',   'spend',       5000,  100),
        (m_id, '₹10,000 Spend Club',  'spend',      10000,  250);

        IF m_idx <= 20 THEN
            INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount) VALUES
            (m_id, '₹25,000 VIP',     'spend',      25000,  750);
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- 26. STREAK CONFIGS (1-2 per merchant, first 20 merchants)
-- ==========================================================================
DO $$
DECLARE
    m_id UUID;
BEGIN
    FOR m_idx IN 1..20 LOOP
        m_id := ('b0000000-0000-0000-0000-' || lpad(m_idx::text, 12, '0'))::uuid;

        INSERT INTO streak_configs (merchant_id, name, required_orders, window_days, reward_amount) VALUES
        (m_id, '3-in-30 Streak', 3, 30, 75 + (m_idx * 5));

        IF m_idx <= 15 THEN
            INSERT INTO streak_configs (merchant_id, name, required_orders, window_days, reward_amount) VALUES
            (m_id, '5-in-60 Mega Streak', 5, 60, 200 + (m_idx * 10));
        END IF;
    END LOOP;
END $$;

-- ==========================================================================
-- DONE
-- ==========================================================================
COMMIT;

-- Summary counts
DO $$
BEGIN
    RAISE NOTICE '=== Seed Complete ===';
    RAISE NOTICE 'Merchants:           %', (SELECT count(*) FROM merchants);
    RAISE NOTICE 'Customers:           %', (SELECT count(*) FROM customers);
    RAISE NOTICE 'Wallets:             %', (SELECT count(*) FROM wallets);
    RAISE NOTICE 'Wallet Policies:     %', (SELECT count(*) FROM wallet_policies);
    RAISE NOTICE 'Rules:               %', (SELECT count(*) FROM rules);
    RAISE NOTICE 'Rule Snapshots:      %', (SELECT count(*) FROM rule_snapshots);
    RAISE NOTICE 'Events:              %', (SELECT count(*) FROM events);
    RAISE NOTICE 'Ledger Entries:      %', (SELECT count(*) FROM ledger_entries);
    RAISE NOTICE 'Loyalty Programs:    %', (SELECT count(*) FROM loyalty_programs);
    RAISE NOTICE 'Loyalty Tiers:       %', (SELECT count(*) FROM loyalty_tiers);
    RAISE NOTICE 'Customer Tiers:      %', (SELECT count(*) FROM customer_tiers);
    RAISE NOTICE 'Gift Cards:          %', (SELECT count(*) FROM gift_cards);
    RAISE NOTICE 'Referral Programs:   %', (SELECT count(*) FROM referral_programs);
    RAISE NOTICE 'Referral Codes:      %', (SELECT count(*) FROM referral_codes);
    RAISE NOTICE 'Referral Conversions:%', (SELECT count(*) FROM referral_conversions);
    RAISE NOTICE 'Customer Order Stats:%', (SELECT count(*) FROM customer_order_stats);
    RAISE NOTICE 'COD Orders:          %', (SELECT count(*) FROM cod_orders);
    RAISE NOTICE 'Redemption Requests: %', (SELECT count(*) FROM redemption_requests);
    RAISE NOTICE 'Campaigns:           %', (SELECT count(*) FROM campaigns);
    RAISE NOTICE 'Connectors:          %', (SELECT count(*) FROM connectors);
    RAISE NOTICE 'Notif Templates:     %', (SELECT count(*) FROM notification_templates);
    RAISE NOTICE 'Spin Wheel Configs:  %', (SELECT count(*) FROM spin_wheel_configs);
    RAISE NOTICE 'Spin Wheel Segments: %', (SELECT count(*) FROM spin_wheel_segments);
    RAISE NOTICE 'Customer Memberships:%', (SELECT count(*) FROM customer_memberships);
    RAISE NOTICE 'Milestone Configs:   %', (SELECT count(*) FROM milestone_configs);
    RAISE NOTICE 'Streak Configs:      %', (SELECT count(*) FROM streak_configs);
END $$;
