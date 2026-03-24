-- seed-large.sql
-- Populates the Batua database with 10 merchants, 500 customers, and realistic transaction data.
-- Usage: psql -U chirag -d batua -f scripts/seed-large.sql

BEGIN;

DO $$
DECLARE
    geo_india_id UUID;

    m_ids UUID[10];
    tmp_id UUID;
    m_slugs TEXT[] := ARRAY[
        'desi-threads', 'glowup-beauty', 'spice-route', 'techbazaar', 'fitindia',
        'petpals-india', 'kidzone', 'homevibes', 'bookworm', 'jewelcraft'
    ];
    m_names TEXT[] := ARRAY[
        'Desi Threads', 'GlowUp Beauty', 'Spice Route', 'TechBazaar', 'FitIndia',
        'PetPals India', 'KidZone', 'HomeVibes', 'BookWorm', 'JewelCraft'
    ];
    m_domains TEXT[] := ARRAY[
        'desithreads.myshopify.com', 'glowupbeauty.myshopify.com', 'spiceroute.myshopify.com',
        'techbazaar.myshopify.com', 'fitindia.myshopify.com', 'petpalsindia.myshopify.com',
        'kidzone.myshopify.com', 'homevibes.myshopify.com', 'bookworm.myshopify.com',
        'jewelcraft.myshopify.com'
    ];
    m_tiers TEXT[] := ARRAY[
        'scale', 'grow', 'enterprise', 'scale', 'grow',
        'free', 'grow', 'scale', 'free', 'enterprise'
    ];

    m_prepaid_pct DOUBLE PRECISION[] := ARRAY[5.0, 7.0, 3.0, 4.0, 5.0, 3.0, 6.0, 4.0, 3.0, 7.0];
    m_prepaid_min DOUBLE PRECISION[] := ARRAY[400, 300, 300, 500, 400, 300, 350, 400, 300, 500];
    m_cod_pct DOUBLE PRECISION[] := ARRAY[2.0, 3.0, 1.0, 2.0, 2.0, 1.0, 3.0, 2.0, 1.0, 4.0];
    m_first_bonus DOUBLE PRECISION[] := ARRAY[50, 75, 25, 50, 40, 30, 50, 35, 25, 75];

    m_order_min DOUBLE PRECISION[] := ARRAY[600, 500, 400, 1500, 800, 500, 700, 1000, 400, 2000];
    m_order_max DOUBLE PRECISION[] := ARRAY[4000, 3500, 2000, 8000, 5000, 3000, 4000, 6000, 2500, 8000];

    m_wp_min DOUBLE PRECISION[] := ARRAY[15, 10, 10, 25, 15, 10, 15, 20, 10, 25];
    m_wp_max_pct DOUBLE PRECISION[] := ARRAY[40, 35, 30, 50, 40, 30, 35, 45, 30, 50];

    m_silver DOUBLE PRECISION[] := ARRAY[2500, 2000, 2000, 3000, 2500, 2000, 2500, 3000, 2000, 3000];
    m_gold   DOUBLE PRECISION[] := ARRAY[6000, 5000, 5000, 8000, 6000, 5000, 6000, 7000, 5000, 8000];
    m_plat   DOUBLE PRECISION[] := ARRAY[18000, 15000, 15000, 20000, 18000, 15000, 16000, 18000, 15000, 20000];

    m_ref_er DOUBLE PRECISION[] := ARRAY[50, 75, 30, 50, 40, 30, 50, 40, 30, 75];
    m_ref_ee DOUBLE PRECISION[] := ARRAY[25, 40, 15, 25, 20, 15, 25, 20, 15, 40];

    c_ids UUID[500];
    c_first TEXT[];
    c_last TEXT[];

    male_first TEXT[] := ARRAY[
        'Aarav','Vivaan','Aditya','Vihaan','Arjun','Reyansh','Sai','Arnav','Ayaan','Krishna',
        'Ishaan','Shaurya','Atharva','Advait','Dhruv','Kabir','Ritvik','Aaryan','Kian','Darsh',
        'Rohan','Nikhil','Rahul','Amit','Vikram','Suresh','Rajesh','Manoj','Deepak','Abhishek',
        'Gaurav','Sanjay','Aakash','Prateek','Kunal','Manish','Naveen','Ankit','Varun','Harsh',
        'Tushar','Sahil','Mohit','Ankur','Piyush','Rishi','Tarun','Ashish','Karan','Pranav',
        'Yash','Dev','Jai','Om','Laksh','Parth','Neel','Samar','Rudra','Veer',
        'Mihir','Chirag','Tanmay','Siddharth','Akshay','Ishan','Krish','Manan','Arnab','Shreyas',
        'Anand','Vinay','Rakesh','Prashant','Hemant','Dinesh','Mukesh','Satish','Ramesh','Ganesh',
        'Bharat','Chetan','Girish','Hitesh','Jatin','Lalit','Mayank','Parag','Raghav','Sachin',
        'Umesh','Vijay','Yogesh','Zayan','Faiz','Imran','Kabeer','Naeem','Salman','Wasim'
    ];
    female_first TEXT[] := ARRAY[
        'Aanya','Aadhya','Ananya','Saanvi','Myra','Aarohi','Diya','Pari','Isha','Navya',
        'Anika','Sara','Kiara','Riya','Avni','Zara','Siya','Prisha','Aditi','Kavya',
        'Pooja','Neha','Priya','Swati','Divya','Anjali','Preeti','Meera','Nisha','Shruti',
        'Tanvi','Radhika','Sakshi','Pallavi','Megha','Jyoti','Sonal','Mansi','Komal','Rashmi',
        'Deepika','Sunita','Rekha','Geeta','Seema','Usha','Lata','Anita','Kavita','Shobha',
        'Tara','Dia','Mira','Sia','Alia','Ryka','Zoya','Pihu','Naina','Trisha',
        'Ridhi','Lavanya','Charvi','Vanya','Mishka','Aahana','Ivaan','Shanaya','Mahika','Arya',
        'Bhavna','Chitra','Darshana','Ekta','Falguni','Garima','Harini','Ira','Jhanvi','Keerthi',
        'Latika','Madhuri','Nimisha','Oviya','Padma','Roshni','Smriti','Tanya','Urvi','Vidya',
        'Yamini','Zahra','Farida','Gulnaz','Hina','Inara','Jasmin','Khadija','Naaz','Saba'
    ];
    last_names TEXT[] := ARRAY[
        'Sharma','Patel','Singh','Kumar','Gupta','Reddy','Nair','Joshi','Mehta','Iyer',
        'Das','Verma','Shah','Agarwal','Rao','Pillai','Menon','Chopra','Banerjee','Mukherjee',
        'Chauhan','Malhotra','Kapoor','Srinivasan','Thakur','Pandey','Mishra','Tiwari','Saxena','Bhat',
        'Kulkarni','Deshmukh','Patil','Kaur','Gill','Dhawan','Bajaj','Choudhury','Dasgupta','Ghosh',
        'Hegde','Iyengar','Jain','Khanna','Lal','Mathur','Narayan','Oberoi','Prasad','Rajan'
    ];

    i INTEGER;
    j INTEGER;
    k INTEGER;
    ci INTEGER;  -- customer index in wallet loop
    merchant_count_for_cust INTEGER;
    assigned_merchants INTEGER[];
    rand_m INTEGER;
    already_assigned BOOLEAN;

    -- Rule/snapshot IDs: flat arrays indexed (merchant-1)*3 + {1,2,3}
    rule_ids UUID[];
    snap_ids UUID[];
    cur_rule_id UUID;
    cur_snap_id UUID;
    cur_config JSONB;

    w_id UUID;
    e_id UUID;
    order_num INTEGER;
    order_count INTEGER;
    order_amount DOUBLE PRECISION;
    is_prepaid BOOLEAN;
    cashback_amount DOUBLE PRECISION;
    ts TIMESTAMPTZ;
    base_ts TIMESTAMPTZ := now() - interval '90 days';
    idem_key TEXT;
    first_order_done BOOLEAN;

    lp_id UUID;
    tier_ids UUID[4];
    rp_id UUID;
    tmp_uuid UUID;
    xfer_id UUID;
    coal_id UUID;

    -- Wallet tracking: for each customer, which merchant indices they have wallets for
    -- We store wallet UUIDs in a 2D lookup: wallet_lookup[customer_idx][merchant_idx]
    -- But PL/pgSQL doesn't support 2D arrays well, so we use a temp table approach.

    total_wallets INTEGER := 0;
    total_events INTEGER := 0;
    total_ledger INTEGER := 0;
BEGIN

    -- =========================================================================
    -- 0. TEMP TABLE for wallet-customer-merchant mapping
    -- =========================================================================
    CREATE TEMP TABLE _seed_wallets (
        cust_idx INTEGER NOT NULL,
        merch_idx INTEGER NOT NULL,
        wallet_id UUID NOT NULL,
        customer_id UUID NOT NULL,
        merchant_id UUID NOT NULL,
        PRIMARY KEY (cust_idx, merch_idx)
    ) ON COMMIT DROP;

    -- =========================================================================
    -- 1. GEO POLICY
    -- =========================================================================
    INSERT INTO geo_policies (geo_code, name, config)
    VALUES (
        'india', 'India',
        '{"cod_enabled": true, "default_currency": "INR", "whatsapp_default": true, "upi_topup_enabled": true}'::jsonb
    )
    ON CONFLICT (geo_code) DO UPDATE SET name = EXCLUDED.name
    RETURNING id INTO geo_india_id;

    -- =========================================================================
    -- 2. MERCHANTS
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO merchants (external_id, name, domain, currency, timezone, is_active, geo_policy_id, slug, plan_tier)
        VALUES (
            'shop_' || lpad(i::text, 6, '0'),
            m_names[i], m_domains[i], 'INR', 'Asia/Kolkata', true,
            geo_india_id, m_slugs[i], m_tiers[i]
        )
        RETURNING id INTO tmp_id;
        m_ids[i] := tmp_id;
    END LOOP;
    RAISE NOTICE 'Created 10 merchants';

    -- =========================================================================
    -- 3. CUSTOMERS (500 unique)
    -- =========================================================================
    c_first := ARRAY[]::TEXT[];
    c_last := ARRAY[]::TEXT[];

    FOR i IN 1..500 LOOP
        IF i % 2 = 0 THEN
            c_first := c_first || male_first[((i / 2 - 1) % 100) + 1];
        ELSE
            c_first := c_first || female_first[((i / 2) % 100) + 1];
        END IF;
        c_last := c_last || last_names[((i - 1) % 50) + 1];
    END LOOP;

    FOR i IN 1..500 LOOP
        INSERT INTO customers (phone, email, name, external_id, is_verified, birthday)
        VALUES (
            '+91' || lpad(((6000000000::bigint + i * 17 + (i * i) % 9000000))::text, 10, '0'),
            lower(c_first[i]) || '.' || lower(c_last[i]) || i::text || '@example.com',
            c_first[i] || ' ' || c_last[i],
            'cust_' || lpad(i::text, 6, '0'),
            (i % 10 > 2),  -- ~70% verified
            '1985-01-01'::date + ((i * 37 + (i * i) % 365) % 6209)::integer
        )
        RETURNING id INTO tmp_id;
        c_ids[i] := tmp_id;
    END LOOP;
    RAISE NOTICE 'Created 500 customers';

    -- =========================================================================
    -- 4. WALLETS (with overlap logic)
    -- =========================================================================
    FOR i IN 1..500 LOOP
        IF i <= 200 THEN
            merchant_count_for_cust := 1;
        ELSIF i <= 350 THEN
            merchant_count_for_cust := 2 + (i % 2);
        ELSIF i <= 450 THEN
            merchant_count_for_cust := 3 + (i % 3);
        ELSE
            merchant_count_for_cust := 5 + (i % 4);
        END IF;

        assigned_merchants := ARRAY[((i - 1) % 10) + 1];

        WHILE array_length(assigned_merchants, 1) < merchant_count_for_cust LOOP
            rand_m := ((i * 7 + array_length(assigned_merchants, 1) * 13 +
                        (i * array_length(assigned_merchants, 1)) % 97) % 10) + 1;
            already_assigned := false;
            FOR j IN 1..array_length(assigned_merchants, 1) LOOP
                IF assigned_merchants[j] = rand_m THEN
                    already_assigned := true;
                    EXIT;
                END IF;
            END LOOP;
            IF NOT already_assigned THEN
                assigned_merchants := assigned_merchants || rand_m;
            ELSE
                FOR k IN 1..10 LOOP
                    rand_m := (((i + k + array_length(assigned_merchants, 1)) % 10) + 1);
                    already_assigned := false;
                    FOR j IN 1..array_length(assigned_merchants, 1) LOOP
                        IF assigned_merchants[j] = rand_m THEN
                            already_assigned := true;
                            EXIT;
                        END IF;
                    END LOOP;
                    IF NOT already_assigned THEN
                        assigned_merchants := assigned_merchants || rand_m;
                        EXIT;
                    END IF;
                END LOOP;
            END IF;
        END LOOP;

        FOR j IN 1..array_length(assigned_merchants, 1) LOOP
            INSERT INTO wallets (merchant_id, customer_id)
            VALUES (m_ids[assigned_merchants[j]], c_ids[i])
            ON CONFLICT (merchant_id, customer_id) DO NOTHING
            RETURNING id INTO w_id;

            IF w_id IS NOT NULL THEN
                INSERT INTO _seed_wallets (cust_idx, merch_idx, wallet_id, customer_id, merchant_id)
                VALUES (i, assigned_merchants[j], w_id, c_ids[i], m_ids[assigned_merchants[j]]);
                total_wallets := total_wallets + 1;
            END IF;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created % wallets', total_wallets;

    -- =========================================================================
    -- 5. RULES + SNAPSHOTS (per merchant)
    -- =========================================================================
    rule_ids := ARRAY[]::UUID[];
    snap_ids := ARRAY[]::UUID[];

    FOR i IN 1..10 LOOP
        -- Rule 1: Prepaid cashback
        cur_config := json_build_object(
            'trigger', 'order.completed',
            'condition', json_build_object('payment_method', 'prepaid', 'min_order_amount', m_prepaid_min[i]),
            'action', json_build_object('type', 'percentage', 'value', m_prepaid_pct[i], 'bucket', 'earned_credit', 'max_reward', 500),
            'description', 'Earn ' || m_prepaid_pct[i] || '% cashback on prepaid orders above Rs.' || m_prepaid_min[i]
        )::jsonb;

        INSERT INTO rules (merchant_id, rule_type, name, config, version, is_active)
        VALUES (m_ids[i], 'reward', 'Prepaid Cashback', cur_config, 1, true)
        RETURNING id INTO cur_rule_id;
        rule_ids := rule_ids || cur_rule_id;

        INSERT INTO rule_snapshots (rule_id, version, config)
        VALUES (cur_rule_id, 1, cur_config)
        RETURNING id INTO cur_snap_id;
        snap_ids := snap_ids || cur_snap_id;

        -- Rule 2: COD cashback
        cur_config := json_build_object(
            'trigger', 'order.delivered',
            'condition', json_build_object('payment_method', 'cod', 'min_order_amount', 300),
            'action', json_build_object('type', 'percentage', 'value', m_cod_pct[i], 'bucket', 'cod_pending', 'max_reward', 200),
            'description', 'Earn ' || m_cod_pct[i] || '% cashback on COD orders upon delivery'
        )::jsonb;

        INSERT INTO rules (merchant_id, rule_type, name, config, version, is_active)
        VALUES (m_ids[i], 'reward', 'COD Cashback', cur_config, 1, true)
        RETURNING id INTO cur_rule_id;
        rule_ids := rule_ids || cur_rule_id;

        INSERT INTO rule_snapshots (rule_id, version, config)
        VALUES (cur_rule_id, 1, cur_config)
        RETURNING id INTO cur_snap_id;
        snap_ids := snap_ids || cur_snap_id;

        -- Rule 3: First order bonus
        cur_config := json_build_object(
            'trigger', 'order.completed',
            'condition', json_build_object('is_first_order', true),
            'action', json_build_object('type', 'fixed', 'value', m_first_bonus[i], 'bucket', 'earned_credit'),
            'description', 'Earn Rs.' || m_first_bonus[i] || ' bonus on your first order'
        )::jsonb;

        INSERT INTO rules (merchant_id, rule_type, name, config, version, is_active)
        VALUES (m_ids[i], 'reward', 'First Order Bonus', cur_config, 1, true)
        RETURNING id INTO cur_rule_id;
        rule_ids := rule_ids || cur_rule_id;

        INSERT INTO rule_snapshots (rule_id, version, config)
        VALUES (cur_rule_id, 1, cur_config)
        RETURNING id INTO cur_snap_id;
        snap_ids := snap_ids || cur_snap_id;
    END LOOP;
    RAISE NOTICE 'Created 30 rules with 30 snapshots';

    -- =========================================================================
    -- 6. EVENTS + LEDGER ENTRIES (per wallet)
    -- =========================================================================
    FOR i IN 1..10 LOOP
        FOR w_id, ci IN
            SELECT sw.wallet_id, sw.cust_idx
            FROM _seed_wallets sw
            WHERE sw.merch_idx = i
            ORDER BY sw.cust_idx
        LOOP
            order_count := 2 + ((ci * 3 + i * 7) % 7);
            first_order_done := false;

            FOR order_num IN 1..order_count LOOP
                ts := base_ts
                    + ((order_num::double precision / order_count) * interval '85 days')
                    + (((ci * order_num) % 1440) * interval '1 minute');

                order_amount := m_order_min[i] +
                    (((ci * 31 + order_num * 97 + i * 53) % 1000)::double precision / 1000.0)
                    * (m_order_max[i] - m_order_min[i]);
                order_amount := round(order_amount::numeric, 2)::double precision;

                is_prepaid := ((ci + order_num + i) % 5) < 3;

                idem_key := 'seed:' || m_ids[i]::text || ':' || c_ids[ci]::text || ':order:' || order_num;

                IF is_prepaid THEN
                    -- ---- PREPAID ORDER ----

                    INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
                    VALUES (
                        m_ids[i], 'order.completed', 'shopify',
                        'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                        json_build_object(
                            'order_id', 'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                            'customer_id', 'cust_' || lpad(ci::text, 6, '0'),
                            'total_price', order_amount,
                            'currency', 'INR',
                            'payment_method', 'prepaid',
                            'line_items', json_build_array(
                                json_build_object('title', 'Product ' || order_num, 'quantity', 1, 'price', order_amount)
                            ),
                            'created_at', ts
                        )::jsonb,
                        'processed', idem_key || ':evt', ts, ts + interval '2 seconds'
                    )
                    RETURNING id INTO e_id;
                    total_events := total_events + 1;

                    IF order_amount >= m_prepaid_min[i] THEN
                        cashback_amount := least(
                            round((order_amount * m_prepaid_pct[i] / 100.0)::numeric, 2)::double precision,
                            500.0
                        );
                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, constraints, expires_at, created_at, state
                        ) VALUES (
                            w_id, 'earned_credit', 'in',
                            cashback_amount, cashback_amount, 1.0,
                            idem_key || ':prepaid_cb', e_id, snap_ids[(i - 1) * 3 + 1],
                            'system', 'reward_engine',
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '180 days', ts + interval '2 seconds', 'active'
                        );
                        total_ledger := total_ledger + 1;
                    END IF;

                    IF NOT first_order_done THEN
                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, constraints, expires_at, created_at, state
                        ) VALUES (
                            w_id, 'earned_credit', 'in',
                            m_first_bonus[i], m_first_bonus[i], 1.0,
                            idem_key || ':first_bonus', e_id, snap_ids[(i - 1) * 3 + 3],
                            'system', 'reward_engine',
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '90 days', ts + interval '3 seconds', 'active'
                        );
                        total_ledger := total_ledger + 1;
                        first_order_done := true;
                    END IF;

                ELSE
                    -- ---- COD ORDER ----

                    INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
                    VALUES (
                        m_ids[i], 'order.completed', 'shopify',
                        'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                        json_build_object(
                            'order_id', 'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                            'customer_id', 'cust_' || lpad(ci::text, 6, '0'),
                            'total_price', order_amount,
                            'currency', 'INR',
                            'payment_method', 'cod',
                            'line_items', json_build_array(
                                json_build_object('title', 'Product ' || order_num, 'quantity', 1, 'price', order_amount)
                            ),
                            'created_at', ts
                        )::jsonb,
                        'processed', idem_key || ':evt', ts, ts + interval '2 seconds'
                    )
                    RETURNING id INTO e_id;
                    total_events := total_events + 1;

                    cashback_amount := least(
                        round((order_amount * m_cod_pct[i] / 100.0)::numeric, 2)::double precision,
                        200.0
                    );

                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, event_id, rule_snapshot_id,
                        actor_type, actor_id, constraints, expires_at, created_at, state
                    ) VALUES (
                        w_id, 'cod_pending', 'held',
                        cashback_amount, cashback_amount, 1.0,
                        idem_key || ':cod_held', e_id, snap_ids[(i - 1) * 3 + 2],
                        'system', 'reward_engine',
                        '{"stackable": true, "transferable": false}'::jsonb,
                        NULL, ts + interval '2 seconds', 'active'
                    );
                    total_ledger := total_ledger + 1;

                    IF NOT first_order_done THEN
                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, constraints, expires_at, created_at, state
                        ) VALUES (
                            w_id, 'earned_credit', 'in',
                            m_first_bonus[i], m_first_bonus[i], 1.0,
                            idem_key || ':first_bonus', e_id, snap_ids[(i - 1) * 3 + 3],
                            'system', 'reward_engine',
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '90 days', ts + interval '3 seconds', 'active'
                        );
                        total_ledger := total_ledger + 1;
                        first_order_done := true;
                    END IF;

                    -- 60% of COD delivered
                    IF ((ci + order_num) % 5) < 3 THEN
                        INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
                        VALUES (
                            m_ids[i], 'order.delivered', 'shiprocket',
                            'dlv_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                            json_build_object(
                                'order_id', 'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                                'customer_id', 'cust_' || lpad(ci::text, 6, '0'),
                                'delivered_at', ts + interval '4 days',
                                'awb_number', 'AWB' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0')
                            )::jsonb,
                            'processed', idem_key || ':dlv_evt',
                            ts + interval '4 days', ts + interval '4 days 2 seconds'
                        )
                        RETURNING id INTO tmp_uuid;
                        total_events := total_events + 1;

                        xfer_id := gen_random_uuid();

                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, transfer_id, constraints, created_at, state
                        ) VALUES (
                            w_id, 'cod_pending', 'across',
                            -cashback_amount, -cashback_amount, 1.0,
                            idem_key || ':cod_across_out', tmp_uuid, snap_ids[(i - 1) * 3 + 2],
                            'system', 'cod_settlement', xfer_id,
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '4 days 2 seconds', 'active'
                        );
                        total_ledger := total_ledger + 1;

                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, transfer_id, constraints,
                            expires_at, created_at, state
                        ) VALUES (
                            w_id, 'earned_credit', 'across',
                            cashback_amount, cashback_amount, 1.0,
                            idem_key || ':cod_across_in', tmp_uuid, snap_ids[(i - 1) * 3 + 2],
                            'system', 'cod_settlement', xfer_id,
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '184 days', ts + interval '4 days 2 seconds', 'active'
                        );
                        total_ledger := total_ledger + 1;

                    -- 15% RTO
                    ELSIF ((ci + order_num) % 5) = 3 THEN
                        INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
                        VALUES (
                            m_ids[i], 'order.rto', 'shiprocket',
                            'rto_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                            json_build_object(
                                'order_id', 'ord_' || lpad(((i - 1) * 100000 + ci * 20 + order_num)::text, 12, '0'),
                                'customer_id', 'cust_' || lpad(ci::text, 6, '0'),
                                'rto_at', ts + interval '6 days',
                                'reason', 'customer_refused'
                            )::jsonb,
                            'processed', idem_key || ':rto_evt',
                            ts + interval '6 days', ts + interval '6 days 2 seconds'
                        )
                        RETURNING id INTO tmp_uuid;
                        total_events := total_events + 1;

                        INSERT INTO ledger_entries (
                            wallet_id, bucket_type, movement_type,
                            earning_unit, currency_equivalent, conversion_rate,
                            idempotency_key, event_id, rule_snapshot_id,
                            actor_type, actor_id, constraints, created_at, state
                        ) VALUES (
                            w_id, 'cod_pending', 'out',
                            -cashback_amount, -cashback_amount, 1.0,
                            idem_key || ':cod_rto_cancel', tmp_uuid, snap_ids[(i - 1) * 3 + 2],
                            'system', 'rto_handler',
                            '{"stackable": true, "transferable": false}'::jsonb,
                            ts + interval '6 days 2 seconds', 'cancelled'
                        );
                        total_ledger := total_ledger + 1;
                    END IF;
                    -- Remaining 25% stay in held/pending
                END IF;
            END LOOP;

            -- ~30% of wallets get a redemption
            IF ((ci + i) % 10) < 3 THEN
                ts := base_ts + interval '60 days'
                     + (((ci * 11 + i * 7) % 1440) * interval '1 minute');

                INSERT INTO events (merchant_id, event_type, event_source, external_event_id, payload, state, idempotency_key, created_at, processed_at)
                VALUES (
                    m_ids[i], 'redemption.applied', 'batua',
                    'rdm_' || lpad(((i - 1) * 100000 + ci)::text, 12, '0'),
                    json_build_object(
                        'customer_id', 'cust_' || lpad(ci::text, 6, '0'),
                        'amount', least(100.0, m_first_bonus[i] * 0.8),
                        'order_id', 'ord_rdm_' || lpad(((i - 1) * 100000 + ci)::text, 12, '0')
                    )::jsonb,
                    'processed',
                    'seed:' || m_ids[i]::text || ':' || c_ids[ci]::text || ':redeem:1:evt',
                    ts, ts + interval '1 second'
                )
                RETURNING id INTO e_id;
                total_events := total_events + 1;

                INSERT INTO ledger_entries (
                    wallet_id, bucket_type, movement_type,
                    earning_unit, currency_equivalent, conversion_rate,
                    idempotency_key, event_id,
                    actor_type, actor_id, constraints, created_at, state
                ) VALUES (
                    w_id, 'earned_credit', 'out',
                    -least(100.0, m_first_bonus[i] * 0.8),
                    -least(100.0, m_first_bonus[i] * 0.8),
                    1.0,
                    'seed:' || m_ids[i]::text || ':' || c_ids[ci]::text || ':redeem:1',
                    e_id,
                    'system', 'redemption_engine',
                    '{"stackable": true}'::jsonb,
                    ts + interval '1 second', 'redeemed'
                );
                total_ledger := total_ledger + 1;
            END IF;

        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created % events and % ledger entries', total_events, total_ledger;

    -- =========================================================================
    -- 7. LOYALTY PROGRAMS + TIERS
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO loyalty_programs (merchant_id, name, evaluation_criteria, evaluation_period_days, is_active)
        VALUES (m_ids[i], m_names[i] || ' Rewards Club', 'spend', 365, true)
        RETURNING id INTO lp_id;

        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
        VALUES (lp_id, 'Bronze', 1, 0, 1.0, '{"perks": ["Basic rewards"]}'::jsonb)
        RETURNING id INTO tmp_id;
        tier_ids[1] := tmp_id;

        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
        VALUES (lp_id, 'Silver', 2, m_silver[i], 1.25, '{"perks": ["1.25x earn rate", "Early access to sales"]}'::jsonb)
        RETURNING id INTO tmp_id;
        tier_ids[2] := tmp_id;

        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
        VALUES (lp_id, 'Gold', 3, m_gold[i], 1.5, '{"perks": ["1.5x earn rate", "Free shipping", "Priority support"]}'::jsonb)
        RETURNING id INTO tmp_id;
        tier_ids[3] := tmp_id;

        INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
        VALUES (lp_id, 'Platinum', 4, m_plat[i], 2.0, '{"perks": ["2x earn rate", "Free shipping", "Priority support", "Exclusive products", "Birthday bonus"]}'::jsonb)
        RETURNING id INTO tmp_id;
        tier_ids[4] := tmp_id;

        FOR w_id, ci IN
            SELECT sw.wallet_id, sw.cust_idx
            FROM _seed_wallets sw
            WHERE sw.merch_idx = i
        LOOP
            IF (ci % 20) < 10 THEN
                INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value)
                VALUES (c_ids[ci], m_ids[i], tier_ids[1], ((ci * 37 + i * 13) % 2000)::double precision)
                ON CONFLICT (customer_id, merchant_id) DO NOTHING;
            ELSIF (ci % 20) < 15 THEN
                INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value)
                VALUES (c_ids[ci], m_ids[i], tier_ids[2], m_silver[i] + ((ci * 23 + i * 11) % 3000)::double precision)
                ON CONFLICT (customer_id, merchant_id) DO NOTHING;
            ELSIF (ci % 20) < 18 THEN
                INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value)
                VALUES (c_ids[ci], m_ids[i], tier_ids[3], m_gold[i] + ((ci * 19 + i * 7) % 5000)::double precision)
                ON CONFLICT (customer_id, merchant_id) DO NOTHING;
            ELSE
                INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value)
                VALUES (c_ids[ci], m_ids[i], tier_ids[4], m_plat[i] + ((ci * 41 + i * 17) % 8000)::double precision)
                ON CONFLICT (customer_id, merchant_id) DO NOTHING;
            END IF;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created 10 loyalty programs with 40 tiers and customer tier assignments';

    -- =========================================================================
    -- 8. REFERRAL PROGRAMS + CODES
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO referral_programs (
            merchant_id, referrer_reward_amount, referee_reward_amount,
            referrer_bucket_type, referee_bucket_type, max_referrals_per_customer, is_active
        )
        VALUES (m_ids[i], m_ref_er[i], m_ref_ee[i], 'referral_reward', 'referral_reward', 20, true);

        FOR w_id, ci IN
            SELECT sw.wallet_id, sw.cust_idx
            FROM _seed_wallets sw
            WHERE sw.merch_idx = i
            ORDER BY sw.cust_idx
            LIMIT 50
        LOOP
            IF (ci % 5) = 0 THEN
                INSERT INTO referral_codes (merchant_id, customer_id, code, is_vanity, total_referrals, total_conversions, is_active)
                VALUES (
                    m_ids[i], c_ids[ci],
                    upper(left(c_first[ci], 4)) || ci || '-' || m_slugs[i],
                    (ci % 3 = 0),
                    (ci % 7),
                    greatest(0, (ci % 7) - 2),
                    true
                )
                ON CONFLICT (code) DO NOTHING;
            END IF;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created 10 referral programs with referral codes';

    -- =========================================================================
    -- 9. WALLET POLICIES
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO wallet_policies (
            merchant_id, bucket_type,
            min_redemption, max_per_order_pct, stackable_with_discounts,
            default_expiry_days
        ) VALUES (
            m_ids[i], 'earned_credit',
            m_wp_min[i], m_wp_max_pct[i], true, 180
        );

        INSERT INTO wallet_policies (
            merchant_id, bucket_type,
            min_redemption, max_per_order_pct, stackable_with_discounts,
            default_expiry_days
        ) VALUES (
            m_ids[i], 'cod_pending',
            NULL, NULL, false, NULL
        );
    END LOOP;
    RAISE NOTICE 'Created 20 wallet policies';

    -- =========================================================================
    -- 10. NOTIFICATION TEMPLATES + CONNECTORS
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO notification_templates (merchant_id, name, channel, locale, template_id, body_template, variables, is_active)
        VALUES (
            m_ids[i], 'earn_credit', 'whatsapp', 'en',
            'tpl_earn_' || m_slugs[i],
            'Hey {{customer_name}}! You just earned Rs.{{amount}} cashback on your order at ' || m_names[i] || '. Your wallet balance is now Rs.{{balance}}.',
            '["customer_name", "amount", "balance"]'::jsonb, true
        );

        INSERT INTO notification_templates (merchant_id, name, channel, locale, template_id, body_template, variables, is_active)
        VALUES (
            m_ids[i], 'redemption_success', 'whatsapp', 'en',
            'tpl_redeem_' || m_slugs[i],
            'Hi {{customer_name}}, Rs.{{amount}} has been applied from your ' || m_names[i] || ' wallet on order {{order_id}}. Remaining balance: Rs.{{balance}}.',
            '["customer_name", "amount", "order_id", "balance"]'::jsonb, true
        );

        INSERT INTO notification_templates (merchant_id, name, channel, locale, template_id, body_template, variables, is_active)
        VALUES (
            m_ids[i], 'cod_delivered', 'sms', 'en',
            'tpl_cod_' || m_slugs[i],
            'Your COD order from ' || m_names[i] || ' is delivered! Rs.{{amount}} cashback has been credited to your wallet.',
            '["amount"]'::jsonb, true
        );

        INSERT INTO connectors (merchant_id, capability, vendor, config, is_active, priority)
        VALUES (
            m_ids[i], 'whatsapp-bsp', 'interakt',
            json_build_object(
                'api_key', 'iak_' || m_slugs[i] || '_seed',
                'phone_number_id', '+91' || lpad(((9800000000::bigint + i * 111))::text, 10, '0'),
                'business_id', 'biz_' || m_slugs[i]
            )::jsonb, true, 0
        );

        INSERT INTO connectors (merchant_id, capability, vendor, config, is_active, priority)
        VALUES (
            m_ids[i], 'sms', 'msg91',
            json_build_object(
                'auth_key', 'msg91_' || m_slugs[i] || '_seed',
                'sender_id', upper(left(replace(m_slugs[i], '-', ''), 6)),
                'route', '4'
            )::jsonb, true, 0
        );
    END LOOP;
    RAISE NOTICE 'Created 30 notification templates and 20 connectors';

    -- =========================================================================
    -- 11. COALITIONS
    -- =========================================================================

    -- Coalition 1: Fashion & Beauty Alliance
    INSERT INTO coalitions (name, is_active) VALUES ('Fashion & Beauty Alliance', true)
    RETURNING id INTO coal_id;

    INSERT INTO coalition_members (coalition_id, merchant_id, conversion_rate) VALUES
        (coal_id, m_ids[1], 1.0),
        (coal_id, m_ids[2], 1.0),
        (coal_id, m_ids[10], 1.0);

    -- Coalition 2: Lifestyle Club
    INSERT INTO coalitions (name, is_active) VALUES ('Lifestyle Club', true)
    RETURNING id INTO coal_id;

    INSERT INTO coalition_members (coalition_id, merchant_id, conversion_rate) VALUES
        (coal_id, m_ids[8], 1.0),
        (coal_id, m_ids[5], 1.0),
        (coal_id, m_ids[6], 1.0);

    RAISE NOTICE 'Created 2 coalitions with 6 member merchants';

    -- =========================================================================
    -- SUMMARY
    -- =========================================================================
    RAISE NOTICE '========================================';
    RAISE NOTICE 'SEED DATA SUMMARY';
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Merchants:        10';
    RAISE NOTICE 'Customers:        500';
    RAISE NOTICE 'Wallets:          %', total_wallets;
    RAISE NOTICE 'Events:           %', total_events;
    RAISE NOTICE 'Ledger Entries:   %', total_ledger;
    RAISE NOTICE 'Rules:            30 (3 per merchant)';
    RAISE NOTICE 'Rule Snapshots:   30';
    RAISE NOTICE 'Loyalty Programs: 10 (40 tiers)';
    RAISE NOTICE 'Referral Progs:   10';
    RAISE NOTICE 'Wallet Policies:  20';
    RAISE NOTICE 'Templates:        30';
    RAISE NOTICE 'Connectors:       20';
    RAISE NOTICE 'Coalitions:       2 (6 members)';
    RAISE NOTICE '========================================';

END;
$$;

-- Final verification query (runs after the DO block)
SELECT
    (SELECT count(*) FROM merchants)              AS merchants,
    (SELECT count(*) FROM customers)              AS customers,
    (SELECT count(*) FROM wallets)                AS wallets,
    (SELECT count(*) FROM events)                 AS events,
    (SELECT count(*) FROM ledger_entries)         AS ledger_entries,
    (SELECT count(*) FROM rules)                  AS rules,
    (SELECT count(*) FROM rule_snapshots)         AS rule_snapshots,
    (SELECT count(*) FROM loyalty_programs)       AS loyalty_programs,
    (SELECT count(*) FROM loyalty_tiers)          AS loyalty_tiers,
    (SELECT count(*) FROM customer_tiers)         AS customer_tiers,
    (SELECT count(*) FROM referral_programs)      AS referral_programs,
    (SELECT count(*) FROM referral_codes)         AS referral_codes,
    (SELECT count(*) FROM wallet_policies)        AS wallet_policies,
    (SELECT count(*) FROM notification_templates) AS notification_templates,
    (SELECT count(*) FROM connectors)             AS connectors,
    (SELECT count(*) FROM coalitions)             AS coalitions,
    (SELECT count(*) FROM coalition_members)      AS coalition_members;

COMMIT;
