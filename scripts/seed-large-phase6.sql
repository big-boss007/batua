-- seed-large-phase6.sql
-- Supplementary seed: fills all empty tables left after seed-large.sql.
-- Requires seed-large.sql to have run first.
-- Usage: psql -U chirag -d batua -f scripts/seed-large-phase6.sql

BEGIN;

DO $$
DECLARE
    m_ids UUID[];
    tmp_id UUID;
    tmp_id2 UUID;
    tmp_le_id UUID;
    tmp_evt_id UUID;
    base_ts TIMESTAMPTZ := now() - interval '90 days';
    ts TIMESTAMPTZ;
    i INTEGER;
    j INTEGER;
    k INTEGER;

    -- Counters
    cnt_campaigns INTEGER := 0;
    cnt_campaign_snapshots INTEGER := 0;
    cnt_gift_cards INTEGER := 0;
    cnt_gift_card_wallets INTEGER := 0;
    cnt_gift_card_ledger INTEGER := 0;
    cnt_referral_conversions INTEGER := 0;
    cnt_cod_orders INTEGER := 0;
    cnt_redemption_requests INTEGER := 0;
    cnt_notification_logs INTEGER := 0;
    cnt_coalition_transfers INTEGER := 0;
    cnt_milestone_configs INTEGER := 0;
    cnt_milestone_achievements INTEGER := 0;
    cnt_streak_configs INTEGER := 0;
    cnt_streak_achievements INTEGER := 0;
    cnt_spin_wheels INTEGER := 0;
    cnt_spin_segments INTEGER := 0;
    cnt_spin_results INTEGER := 0;
    cnt_membership_plans INTEGER := 0;
    cnt_memberships INTEGER := 0;
    cnt_newsletter_signups INTEGER := 0;
    cnt_customer_order_stats INTEGER := 0;

    -- Working vars
    first_rule_id UUID;
    camp_id UUID;
    camp_snap_id UUID;
    w_id UUID;
    c_id UUID;
    m_id UUID;
    le_id UUID;
    gc_amount DOUBLE PRECISION;
    gc_amounts DOUBLE PRECISION[] := ARRAY[500.0, 1000.0, 2000.0];
    gc_code TEXT;
    bearer_w_id UUID;
    claimer_w_id UUID;

    ref_code_rec RECORD;
    referee_rec RECORD;
    cod_rec RECORD;

    wallet_rec RECORD;
    template_rec RECORD;

    coal_rec RECORD;
    cust_rec RECORD;
    from_w UUID;
    to_w UUID;
    xfer_uuid UUID;

    ms_id UUID;
    milestone_ids UUID[];
    streak_id UUID;
    streak_ids UUID[];
    wheel_id UUID;
    seg_ids UUID[];
    seg_id UUID;
    p_id UUID;
    plan_ids UUID[];

    seg_labels TEXT[] := ARRAY['10 coins', '25 coins', '50 coins', '100 coins', 'Better Luck', '5 coins'];
    seg_rewards DOUBLE PRECISION[] := ARRAY[10.0, 25.0, 50.0, 100.0, 0.0, 5.0];
    seg_probs DOUBLE PRECISION[] := ARRAY[0.30, 0.25, 0.15, 0.05, 0.10, 0.15];
    seg_colors TEXT[] := ARRAY['#7c6aff', '#ff6b6b', '#ffd93d', '#6bcb77', '#cccccc', '#4ecdc4'];

    rand_val DOUBLE PRECISION;
BEGIN

    -- =========================================================================
    -- 0. LOAD MERCHANT IDS (ordered by external_id to match seed-large order)
    -- =========================================================================
    SELECT array_agg(id ORDER BY external_id)
    INTO m_ids
    FROM merchants;

    IF array_length(m_ids, 1) IS NULL OR array_length(m_ids, 1) < 10 THEN
        RAISE EXCEPTION 'Expected at least 10 merchants from seed-large.sql, found %',
            coalesce(array_length(m_ids, 1), 0);
    END IF;
    RAISE NOTICE 'Loaded % merchants', array_length(m_ids, 1);

    -- =========================================================================
    -- 1. CAMPAIGNS (2 per merchant = 20 total)
    -- =========================================================================
    FOR i IN 1..10 LOOP
        SELECT id INTO first_rule_id
        FROM rules
        WHERE merchant_id = m_ids[i]
        ORDER BY created_at
        LIMIT 1;

        -- Diwali 3x campaign
        INSERT INTO campaigns (
            merchant_id, name, campaign_type, config, base_rule_id,
            multiplier, starts_at, ends_at, is_active
        ) VALUES (
            m_ids[i], 'Diwali 3x Points', 'multiplier',
            '{"description": "Triple rewards during Diwali week", "applies_to": "all_orders"}'::jsonb,
            first_rule_id, 3.0,
            '2025-10-01 00:00:00+05:30'::timestamptz,
            '2025-10-15 23:59:59+05:30'::timestamptz,
            true
        )
        ON CONFLICT DO NOTHING
        RETURNING id INTO camp_id;

        IF camp_id IS NOT NULL THEN
            INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
            VALUES (
                camp_id,
                '{"description": "Triple rewards during Diwali week", "applies_to": "all_orders"}'::jsonb,
                3.0
            );
            cnt_campaigns := cnt_campaigns + 1;
            cnt_campaign_snapshots := cnt_campaign_snapshots + 1;
        END IF;

        -- New Year 2x campaign
        INSERT INTO campaigns (
            merchant_id, name, campaign_type, config, base_rule_id,
            multiplier, starts_at, ends_at, is_active
        ) VALUES (
            m_ids[i], 'New Year 2x Bonus', 'multiplier',
            '{"description": "Double rewards for New Year celebrations", "applies_to": "all_orders"}'::jsonb,
            first_rule_id, 2.0,
            '2025-12-31 00:00:00+05:30'::timestamptz,
            '2026-01-05 23:59:59+05:30'::timestamptz,
            true
        )
        ON CONFLICT DO NOTHING
        RETURNING id INTO camp_id;

        IF camp_id IS NOT NULL THEN
            INSERT INTO campaign_snapshots (campaign_id, config, multiplier)
            VALUES (
                camp_id,
                '{"description": "Double rewards for New Year celebrations", "applies_to": "all_orders"}'::jsonb,
                2.0
            );
            cnt_campaigns := cnt_campaigns + 1;
            cnt_campaign_snapshots := cnt_campaign_snapshots + 1;
        END IF;
    END LOOP;
    RAISE NOTICE 'Created % campaigns with % snapshots', cnt_campaigns, cnt_campaign_snapshots;

    -- =========================================================================
    -- 2. GIFT CARDS (50 total, spread across merchants)
    -- =========================================================================
    FOR i IN 1..50 LOOP
        m_id := m_ids[((i - 1) % 10) + 1];
        gc_amount := gc_amounts[((i - 1) % 3) + 1];
        gc_code := 'BRZE-' || upper(substr(md5(random()::text), 1, 4))
                   || '-' || upper(substr(md5(random()::text), 1, 4))
                   || '-' || upper(substr(md5(random()::text), 1, 4));
        ts := base_ts + (((i * 37) % 80) * interval '1 day') + ((i * 73 % 1440) * interval '1 minute');

        -- Create bearer wallet
        INSERT INTO wallets (merchant_id, is_bearer, bearer_code)
        VALUES (m_id, true, gc_code)
        ON CONFLICT DO NOTHING
        RETURNING id INTO bearer_w_id;

        IF bearer_w_id IS NULL THEN
            CONTINUE;
        END IF;

        cnt_gift_card_wallets := cnt_gift_card_wallets + 1;

        -- Create ledger entry for the gift card load
        INSERT INTO ledger_entries (
            wallet_id, bucket_type, movement_type,
            earning_unit, currency_equivalent, conversion_rate,
            idempotency_key,
            actor_type, actor_id, constraints,
            expires_at, created_at, state
        ) VALUES (
            bearer_w_id, 'gift_card', 'in',
            gc_amount, gc_amount, 1.0,
            'seed:gc:' || gc_code || ':load',
            'system', 'gift_card_issuer',
            '{"stackable": false, "transferable": true}'::jsonb,
            ts + interval '365 days', ts, 'active'
        )
        RETURNING id INTO le_id;
        cnt_gift_card_ledger := cnt_gift_card_ledger + 1;

        -- ~50% claimed
        IF (i % 2) = 0 THEN
            SELECT w.id INTO claimer_w_id
            FROM wallets w
            WHERE w.merchant_id = m_id
              AND w.customer_id IS NOT NULL
              AND w.is_bearer = false
            ORDER BY w.created_at
            OFFSET ((i * 7) % 30)
            LIMIT 1;

            INSERT INTO gift_cards (
                merchant_id, wallet_id, code, initial_amount, current_amount,
                currency, issued_by, is_claimed, claimed_by_wallet_id, claimed_at,
                expires_at, is_active, created_at
            ) VALUES (
                m_id, bearer_w_id, gc_code, gc_amount, gc_amount,
                'INR', 'system', true, claimer_w_id, ts + interval '3 days',
                ts + interval '365 days', true, ts
            )
            ON CONFLICT DO NOTHING;
        ELSE
            INSERT INTO gift_cards (
                merchant_id, wallet_id, code, initial_amount, current_amount,
                currency, issued_by, is_claimed,
                expires_at, is_active, created_at
            ) VALUES (
                m_id, bearer_w_id, gc_code, gc_amount, gc_amount,
                'INR', 'system', false,
                ts + interval '365 days', true, ts
            )
            ON CONFLICT DO NOTHING;
        END IF;
        cnt_gift_cards := cnt_gift_cards + 1;
    END LOOP;
    RAISE NOTICE 'Created % gift cards (% bearer wallets, % ledger entries)', cnt_gift_cards, cnt_gift_card_wallets, cnt_gift_card_ledger;

    -- =========================================================================
    -- 3. REFERRAL CONVERSIONS (200 total)
    -- =========================================================================
    FOR i IN 1..200 LOOP
        -- Pick a random referral code
        SELECT rc.id AS code_id, rc.merchant_id, rc.customer_id AS referrer_cust_id
        INTO ref_code_rec
        FROM referral_codes rc
        WHERE rc.is_active = true
        ORDER BY rc.id
        OFFSET (i % (SELECT count(*) FROM referral_codes WHERE is_active = true))
        LIMIT 1;

        IF ref_code_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Pick a different customer who has a wallet at the same merchant
        SELECT w.customer_id
        INTO referee_rec
        FROM wallets w
        WHERE w.merchant_id = ref_code_rec.merchant_id
          AND w.customer_id IS NOT NULL
          AND w.customer_id != ref_code_rec.referrer_cust_id
          AND w.is_bearer = false
        ORDER BY w.id
        OFFSET ((i * 13) % greatest(1, (SELECT count(*) FROM wallets
                                          WHERE merchant_id = ref_code_rec.merchant_id
                                            AND customer_id IS NOT NULL
                                            AND customer_id != ref_code_rec.referrer_cust_id
                                            AND is_bearer = false)))
        LIMIT 1;

        IF referee_rec IS NULL THEN
            CONTINUE;
        END IF;

        ts := base_ts + (((i * 43) % 85) * interval '1 day') + ((i * 61 % 1440) * interval '1 minute');

        INSERT INTO referral_conversions (
            merchant_id, referral_code_id, referrer_id, referee_id,
            order_id, referee_ip, referee_device_fingerprint,
            is_suspicious, fraud_signals, created_at
        ) VALUES (
            ref_code_rec.merchant_id,
            ref_code_rec.code_id,
            ref_code_rec.referrer_cust_id,
            referee_rec.customer_id,
            'ord_ref_' || lpad(i::text, 8, '0'),
            '103.' || ((i % 255) + 1)::text || '.' || (((i * 3) % 255) + 1)::text || '.' || (((i * 7) % 255) + 1)::text,
            'fp_' || md5(i::text || 'device'),
            (i % 10) = 0,
            CASE WHEN (i % 10) = 0
                THEN '["same_ip_cluster", "rapid_conversion"]'::jsonb
                ELSE '[]'::jsonb
            END,
            ts
        )
        ON CONFLICT DO NOTHING;
        cnt_referral_conversions := cnt_referral_conversions + 1;
    END LOOP;
    RAISE NOTICE 'Created % referral conversions', cnt_referral_conversions;

    -- =========================================================================
    -- 4. COD ORDERS (from existing cod_pending/held ledger entries)
    -- =========================================================================
    FOR cod_rec IN
        SELECT le.id AS ledger_entry_id,
               le.wallet_id,
               w.merchant_id,
               le.idempotency_key,
               le.created_at AS le_created_at
        FROM ledger_entries le
        JOIN wallets w ON w.id = le.wallet_id
        WHERE le.bucket_type = 'cod_pending'
          AND le.movement_type = 'held'
          AND NOT EXISTS (
              SELECT 1 FROM cod_orders co WHERE co.ledger_entry_id = le.id
          )
    LOOP
        -- Derive order_id from idempotency_key (format: seed:merchant:customer:order:N:cod_held)
        -- Use a hash-based order_id
        ts := cod_rec.le_created_at;

        -- Determine state: 60% delivered, 20% pending, 20% rto
        IF (cnt_cod_orders % 5) < 3 THEN
            INSERT INTO cod_orders (
                merchant_id, order_id, wallet_id, ledger_entry_id,
                state, delivery_confirmed_at, created_at, updated_at
            ) VALUES (
                cod_rec.merchant_id,
                'cod_ord_' || substr(md5(cod_rec.ledger_entry_id::text), 1, 12),
                cod_rec.wallet_id,
                cod_rec.ledger_entry_id,
                'delivered',
                ts + interval '4 days',
                ts, ts + interval '4 days'
            )
            ON CONFLICT (merchant_id, order_id) DO NOTHING;
        ELSIF (cnt_cod_orders % 5) = 3 THEN
            INSERT INTO cod_orders (
                merchant_id, order_id, wallet_id, ledger_entry_id,
                state, created_at, updated_at
            ) VALUES (
                cod_rec.merchant_id,
                'cod_ord_' || substr(md5(cod_rec.ledger_entry_id::text), 1, 12),
                cod_rec.wallet_id,
                cod_rec.ledger_entry_id,
                'pending',
                ts, ts
            )
            ON CONFLICT (merchant_id, order_id) DO NOTHING;
        ELSE
            INSERT INTO cod_orders (
                merchant_id, order_id, wallet_id, ledger_entry_id,
                state, created_at, updated_at
            ) VALUES (
                cod_rec.merchant_id,
                'cod_ord_' || substr(md5(cod_rec.ledger_entry_id::text), 1, 12),
                cod_rec.wallet_id,
                cod_rec.ledger_entry_id,
                'rto',
                ts, ts + interval '6 days'
            )
            ON CONFLICT (merchant_id, order_id) DO NOTHING;
        END IF;
        cnt_cod_orders := cnt_cod_orders + 1;
    END LOOP;
    RAISE NOTICE 'Created % cod_orders', cnt_cod_orders;

    -- =========================================================================
    -- 5. REDEMPTION REQUESTS (300 total)
    -- =========================================================================
    FOR i IN 1..300 LOOP
        m_id := m_ids[((i - 1) % 10) + 1];
        ts := base_ts + (((i * 29) % 85) * interval '1 day') + ((i * 53 % 1440) * interval '1 minute');

        -- Pick a wallet that has earned_credit/out ledger entries (redeemers)
        SELECT w.id, w.customer_id
        INTO wallet_rec
        FROM wallets w
        WHERE w.merchant_id = m_id
          AND w.customer_id IS NOT NULL
          AND w.is_bearer = false
        ORDER BY w.id
        OFFSET ((i * 11) % greatest(1, (SELECT count(*) FROM wallets
                                          WHERE merchant_id = m_id
                                            AND customer_id IS NOT NULL
                                            AND is_bearer = false)))
        LIMIT 1;

        IF wallet_rec IS NULL THEN
            CONTINUE;
        END IF;

        rand_val := ((i * 37 + 13) % 100)::double precision;

        IF rand_val < 70 THEN
            -- completed (70%)
            INSERT INTO redemption_requests (
                merchant_id, wallet_id, requested_amount, eligible_amount, applied_amount,
                order_id, order_amount, payment_method, state,
                shopify_discount_id, created_at, updated_at
            ) VALUES (
                m_id, wallet_rec.id,
                (50 + (i * 7 % 200))::double precision,
                (50 + (i * 7 % 200))::double precision,
                (50 + (i * 7 % 200))::double precision,
                'ord_rdm_p6_' || lpad(i::text, 8, '0'),
                (800 + (i * 31 % 3000))::double precision,
                CASE WHEN (i % 3) = 0 THEN 'prepaid' ELSE 'cod' END,
                'completed',
                'disc_' || substr(md5(i::text), 1, 10),
                ts, ts + interval '5 seconds'
            )
            ON CONFLICT DO NOTHING;
        ELSIF rand_val < 85 THEN
            -- rejected (15%)
            INSERT INTO redemption_requests (
                merchant_id, wallet_id, requested_amount,
                order_id, order_amount, payment_method, state,
                rejection_reason, created_at, updated_at
            ) VALUES (
                m_id, wallet_rec.id,
                (50 + (i * 7 % 200))::double precision,
                'ord_rdm_p6_' || lpad(i::text, 8, '0'),
                (800 + (i * 31 % 3000))::double precision,
                'prepaid',
                'rejected',
                CASE
                    WHEN (i % 3) = 0 THEN 'Insufficient balance'
                    WHEN (i % 3) = 1 THEN 'Below minimum redemption threshold'
                    ELSE 'Order amount too low for wallet usage'
                END,
                ts, ts + interval '2 seconds'
            )
            ON CONFLICT DO NOTHING;
        ELSIF rand_val < 95 THEN
            -- failed (10%)
            INSERT INTO redemption_requests (
                merchant_id, wallet_id, requested_amount,
                order_id, order_amount, payment_method, state,
                rejection_reason, created_at, updated_at
            ) VALUES (
                m_id, wallet_rec.id,
                (50 + (i * 7 % 200))::double precision,
                'ord_rdm_p6_' || lpad(i::text, 8, '0'),
                (800 + (i * 31 % 3000))::double precision,
                'cod',
                'failed',
                'Shopify discount API timeout',
                ts, ts + interval '3 seconds'
            )
            ON CONFLICT DO NOTHING;
        ELSE
            -- compensated (5%)
            INSERT INTO redemption_requests (
                merchant_id, wallet_id, requested_amount, eligible_amount, applied_amount,
                order_id, order_amount, payment_method, state,
                created_at, updated_at
            ) VALUES (
                m_id, wallet_rec.id,
                (50 + (i * 7 % 200))::double precision,
                (50 + (i * 7 % 200))::double precision,
                0,
                'ord_rdm_p6_' || lpad(i::text, 8, '0'),
                (800 + (i * 31 % 3000))::double precision,
                'prepaid',
                'compensated',
                ts, ts + interval '10 seconds'
            )
            ON CONFLICT DO NOTHING;
        END IF;
        cnt_redemption_requests := cnt_redemption_requests + 1;
    END LOOP;
    RAISE NOTICE 'Created % redemption requests', cnt_redemption_requests;

    -- =========================================================================
    -- 6. NOTIFICATION LOGS (500 total)
    -- =========================================================================
    FOR i IN 1..500 LOOP
        m_id := m_ids[((i - 1) % 10) + 1];
        ts := base_ts + (((i * 19) % 85) * interval '1 day') + ((i * 41 % 1440) * interval '1 minute');

        -- Pick a customer wallet for this merchant
        SELECT w.customer_id
        INTO wallet_rec
        FROM wallets w
        WHERE w.merchant_id = m_id
          AND w.customer_id IS NOT NULL
          AND w.is_bearer = false
        ORDER BY w.id
        OFFSET ((i * 7) % greatest(1, (SELECT count(*) FROM wallets
                                         WHERE merchant_id = m_id
                                           AND customer_id IS NOT NULL
                                           AND is_bearer = false)))
        LIMIT 1;

        IF wallet_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Determine channel: 70% whatsapp, 20% sms, 10% email
        -- Pick matching template
        IF (i % 10) < 7 THEN
            -- whatsapp
            SELECT id INTO template_rec
            FROM notification_templates
            WHERE merchant_id = m_id AND channel = 'whatsapp'
            ORDER BY id
            LIMIT 1;

            INSERT INTO notification_logs (
                merchant_id, customer_id, template_id, channel,
                variables, status, external_message_id, sent_at, created_at
            ) VALUES (
                m_id, wallet_rec.customer_id, template_rec.id, 'whatsapp',
                json_build_object('customer_name', 'Customer ' || i, 'amount', (10 + i % 200)::text, 'balance', (100 + i % 500)::text)::jsonb,
                CASE
                    WHEN (i % 20) < 16 THEN 'sent'
                    WHEN (i % 20) < 18 THEN 'delivered'
                    WHEN (i % 20) < 19 THEN 'failed'
                    ELSE 'pending'
                END,
                CASE WHEN (i % 20) < 18 THEN 'wamid.' || md5(i::text || 'wa') ELSE NULL END,
                CASE WHEN (i % 20) < 18 THEN ts + interval '1 second' ELSE NULL END,
                ts
            )
            ON CONFLICT DO NOTHING;
        ELSIF (i % 10) < 9 THEN
            -- sms
            SELECT id INTO template_rec
            FROM notification_templates
            WHERE merchant_id = m_id AND channel = 'sms'
            ORDER BY id
            LIMIT 1;

            INSERT INTO notification_logs (
                merchant_id, customer_id, template_id, channel,
                variables, status, external_message_id, sent_at, created_at
            ) VALUES (
                m_id, wallet_rec.customer_id, template_rec.id, 'sms',
                json_build_object('amount', (10 + i % 200)::text)::jsonb,
                CASE
                    WHEN (i % 20) < 16 THEN 'sent'
                    WHEN (i % 20) < 18 THEN 'delivered'
                    WHEN (i % 20) < 19 THEN 'failed'
                    ELSE 'pending'
                END,
                CASE WHEN (i % 20) < 18 THEN 'msg91_' || md5(i::text || 'sms') ELSE NULL END,
                CASE WHEN (i % 20) < 18 THEN ts + interval '2 seconds' ELSE NULL END,
                ts
            )
            ON CONFLICT DO NOTHING;
        ELSE
            -- email (use whatsapp template as fallback since no email templates exist)
            SELECT id INTO template_rec
            FROM notification_templates
            WHERE merchant_id = m_id
            ORDER BY id
            LIMIT 1;

            INSERT INTO notification_logs (
                merchant_id, customer_id, template_id, channel,
                variables, status, external_message_id, sent_at, created_at
            ) VALUES (
                m_id, wallet_rec.customer_id, template_rec.id, 'email',
                json_build_object('customer_name', 'Customer ' || i, 'amount', (10 + i % 200)::text)::jsonb,
                CASE
                    WHEN (i % 20) < 16 THEN 'sent'
                    WHEN (i % 20) < 18 THEN 'delivered'
                    WHEN (i % 20) < 19 THEN 'failed'
                    ELSE 'pending'
                END,
                CASE WHEN (i % 20) < 18 THEN 'ses_' || md5(i::text || 'email') ELSE NULL END,
                CASE WHEN (i % 20) < 18 THEN ts + interval '3 seconds' ELSE NULL END,
                ts
            )
            ON CONFLICT DO NOTHING;
        END IF;
        cnt_notification_logs := cnt_notification_logs + 1;
    END LOOP;
    RAISE NOTICE 'Created % notification logs', cnt_notification_logs;

    -- =========================================================================
    -- 7. COALITION TRANSFERS (30 total)
    -- =========================================================================
    -- Coalition 1: merchants 1, 2, 10 (Fashion & Beauty Alliance)
    -- Coalition 2: merchants 8, 5, 6 (Lifestyle Club)
    -- We need customers who have wallets at multiple coalition merchants

    -- Coalition 1 transfers (15)
    FOR i IN 1..15 LOOP
        SELECT co.id INTO coal_rec
        FROM coalitions co
        WHERE co.name = 'Fashion & Beauty Alliance'
        LIMIT 1;

        IF coal_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Find a customer with wallets at merchants 1 and 2
        SELECT w1.customer_id, w1.id AS from_wallet, w2.id AS to_wallet
        INTO cust_rec
        FROM wallets w1
        JOIN wallets w2 ON w2.customer_id = w1.customer_id
                       AND w2.merchant_id = m_ids[2]
                       AND w2.is_bearer = false
        WHERE w1.merchant_id = m_ids[1]
          AND w1.customer_id IS NOT NULL
          AND w1.is_bearer = false
        ORDER BY w1.id
        OFFSET ((i * 3) % greatest(1, (SELECT count(*)
                                         FROM wallets w1
                                         JOIN wallets w2 ON w2.customer_id = w1.customer_id
                                                        AND w2.merchant_id = m_ids[2]
                                                        AND w2.is_bearer = false
                                         WHERE w1.merchant_id = m_ids[1]
                                           AND w1.customer_id IS NOT NULL
                                           AND w1.is_bearer = false)))
        LIMIT 1;

        IF cust_rec IS NULL THEN
            CONTINUE;
        END IF;

        ts := base_ts + (((i * 51) % 80) * interval '1 day') + ((i * 97 % 1440) * interval '1 minute');
        xfer_uuid := gen_random_uuid();

        INSERT INTO coalition_transfers (
            coalition_id, customer_id, from_merchant_id, to_merchant_id,
            from_wallet_id, to_wallet_id, amount, converted_amount,
            conversion_rate, transfer_id, created_at
        ) VALUES (
            coal_rec.id, cust_rec.customer_id,
            m_ids[1], m_ids[2],
            cust_rec.from_wallet, cust_rec.to_wallet,
            (20 + (i * 11 % 80))::double precision,
            (20 + (i * 11 % 80))::double precision,
            1.0, xfer_uuid, ts
        )
        ON CONFLICT DO NOTHING;
        cnt_coalition_transfers := cnt_coalition_transfers + 1;
    END LOOP;

    -- Coalition 2 transfers (15)
    FOR i IN 1..15 LOOP
        SELECT co.id INTO coal_rec
        FROM coalitions co
        WHERE co.name = 'Lifestyle Club'
        LIMIT 1;

        IF coal_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Find a customer with wallets at merchants 8 and 5
        SELECT w1.customer_id, w1.id AS from_wallet, w2.id AS to_wallet
        INTO cust_rec
        FROM wallets w1
        JOIN wallets w2 ON w2.customer_id = w1.customer_id
                       AND w2.merchant_id = m_ids[5]
                       AND w2.is_bearer = false
        WHERE w1.merchant_id = m_ids[8]
          AND w1.customer_id IS NOT NULL
          AND w1.is_bearer = false
        ORDER BY w1.id
        OFFSET ((i * 5) % greatest(1, (SELECT count(*)
                                         FROM wallets w1
                                         JOIN wallets w2 ON w2.customer_id = w1.customer_id
                                                        AND w2.merchant_id = m_ids[5]
                                                        AND w2.is_bearer = false
                                         WHERE w1.merchant_id = m_ids[8]
                                           AND w1.customer_id IS NOT NULL
                                           AND w1.is_bearer = false)))
        LIMIT 1;

        IF cust_rec IS NULL THEN
            CONTINUE;
        END IF;

        ts := base_ts + (((i * 47 + 30) % 80) * interval '1 day') + ((i * 83 % 1440) * interval '1 minute');
        xfer_uuid := gen_random_uuid();

        INSERT INTO coalition_transfers (
            coalition_id, customer_id, from_merchant_id, to_merchant_id,
            from_wallet_id, to_wallet_id, amount, converted_amount,
            conversion_rate, transfer_id, created_at
        ) VALUES (
            coal_rec.id, cust_rec.customer_id,
            m_ids[8], m_ids[5],
            cust_rec.from_wallet, cust_rec.to_wallet,
            (15 + (i * 9 % 60))::double precision,
            (15 + (i * 9 % 60))::double precision,
            1.0, xfer_uuid, ts
        )
        ON CONFLICT DO NOTHING;
        cnt_coalition_transfers := cnt_coalition_transfers + 1;
    END LOOP;
    RAISE NOTICE 'Created % coalition transfers', cnt_coalition_transfers;

    -- =========================================================================
    -- 8. MILESTONE CONFIGS + ACHIEVEMENTS (3 per merchant = 30 configs)
    -- =========================================================================
    milestone_ids := ARRAY[]::UUID[];

    FOR i IN 1..10 LOOP
        -- Milestone 1: 3rd order = Rs.50
        INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount, is_active)
        VALUES (m_ids[i], '3rd Order Bonus', 'order_count', 3, 50.0, true)
        RETURNING id INTO ms_id;
        milestone_ids := milestone_ids || ms_id;
        cnt_milestone_configs := cnt_milestone_configs + 1;

        -- Milestone 2: Rs.5000 spend = Rs.150
        INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount, is_active)
        VALUES (m_ids[i], 'Rs.5K Spend Club', 'total_spend', 5000, 150.0, true)
        RETURNING id INTO ms_id;
        milestone_ids := milestone_ids || ms_id;
        cnt_milestone_configs := cnt_milestone_configs + 1;

        -- Milestone 3: 10th order = Rs.500
        INSERT INTO milestone_configs (merchant_id, name, milestone_type, threshold, reward_amount, is_active)
        VALUES (m_ids[i], 'Loyalist (10 Orders)', 'order_count', 10, 500.0, true)
        RETURNING id INTO ms_id;
        milestone_ids := milestone_ids || ms_id;
        cnt_milestone_configs := cnt_milestone_configs + 1;
    END LOOP;
    RAISE NOTICE 'Created % milestone configs', cnt_milestone_configs;

    -- Milestone achievements: customers with 3+ orders get milestone 1
    FOR i IN 1..10 LOOP
        FOR wallet_rec IN
            SELECT w.id AS wallet_id, w.customer_id, w.merchant_id,
                   count(DISTINCT le.event_id) AS order_count,
                   sum(CASE WHEN le.movement_type = 'in' THEN le.currency_equivalent ELSE 0 END) AS total_earned
            FROM wallets w
            JOIN ledger_entries le ON le.wallet_id = w.id
            WHERE w.merchant_id = m_ids[i]
              AND w.customer_id IS NOT NULL
              AND w.is_bearer = false
              AND le.bucket_type IN ('earned_credit', 'cod_pending')
              AND le.movement_type IN ('in', 'held')
            GROUP BY w.id, w.customer_id, w.merchant_id
            HAVING count(DISTINCT le.event_id) >= 3
            LIMIT 30
        LOOP
            ts := base_ts + interval '50 days' + ((cnt_milestone_achievements * 17 % 1440) * interval '1 minute');

            -- 3rd order milestone
            INSERT INTO events (
                merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
            ) VALUES (
                m_ids[i], 'milestone.achieved', 'batua',
                'mst_3ord_' || wallet_rec.customer_id::text,
                json_build_object('milestone', '3rd Order Bonus', 'reward', 50)::jsonb,
                'processed',
                'seed:p6:milestone:3ord:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text,
                ts, ts + interval '1 second'
            )
            ON CONFLICT (idempotency_key) DO NOTHING
            RETURNING id INTO tmp_evt_id;

            IF tmp_evt_id IS NOT NULL THEN
                INSERT INTO ledger_entries (
                    wallet_id, bucket_type, movement_type,
                    earning_unit, currency_equivalent, conversion_rate,
                    idempotency_key, event_id,
                    actor_type, actor_id, constraints,
                    expires_at, created_at, state
                ) VALUES (
                    wallet_rec.wallet_id, 'earned_credit', 'in',
                    50.0, 50.0, 1.0,
                    'seed:p6:milestone:3ord:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text || ':le',
                    tmp_evt_id,
                    'system', 'milestone_engine',
                    '{"stackable": true, "transferable": false}'::jsonb,
                    ts + interval '180 days', ts + interval '1 second', 'active'
                )
                RETURNING id INTO tmp_le_id;

                INSERT INTO milestone_achievements (
                    merchant_id, customer_id, milestone_id, ledger_entry_id, achieved_at
                ) VALUES (
                    m_ids[i], wallet_rec.customer_id,
                    milestone_ids[(i - 1) * 3 + 1],
                    tmp_le_id, ts
                )
                ON CONFLICT (customer_id, milestone_id) DO NOTHING;
                cnt_milestone_achievements := cnt_milestone_achievements + 1;
            END IF;

            -- 5K spend milestone (if qualifying)
            IF wallet_rec.total_earned >= 5000 THEN
                ts := ts + interval '10 days';

                INSERT INTO events (
                    merchant_id, event_type, event_source, external_event_id,
                    payload, state, idempotency_key, created_at, processed_at
                ) VALUES (
                    m_ids[i], 'milestone.achieved', 'batua',
                    'mst_5k_' || wallet_rec.customer_id::text,
                    json_build_object('milestone', 'Rs.5K Spend Club', 'reward', 150)::jsonb,
                    'processed',
                    'seed:p6:milestone:5k:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text,
                    ts, ts + interval '1 second'
                )
                ON CONFLICT (idempotency_key) DO NOTHING
                RETURNING id INTO tmp_evt_id;

                IF tmp_evt_id IS NOT NULL THEN
                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, event_id,
                        actor_type, actor_id, constraints,
                        expires_at, created_at, state
                    ) VALUES (
                        wallet_rec.wallet_id, 'earned_credit', 'in',
                        150.0, 150.0, 1.0,
                        'seed:p6:milestone:5k:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text || ':le',
                        tmp_evt_id,
                        'system', 'milestone_engine',
                        '{"stackable": true, "transferable": false}'::jsonb,
                        ts + interval '180 days', ts + interval '1 second', 'active'
                    )
                    RETURNING id INTO tmp_le_id;

                    INSERT INTO milestone_achievements (
                        merchant_id, customer_id, milestone_id, ledger_entry_id, achieved_at
                    ) VALUES (
                        m_ids[i], wallet_rec.customer_id,
                        milestone_ids[(i - 1) * 3 + 2],
                        tmp_le_id, ts
                    )
                    ON CONFLICT (customer_id, milestone_id) DO NOTHING;
                    cnt_milestone_achievements := cnt_milestone_achievements + 1;
                END IF;
            END IF;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created % milestone achievements', cnt_milestone_achievements;

    -- =========================================================================
    -- 9. STREAK CONFIGS + ACHIEVEMENTS (1 per merchant = 10 configs)
    -- =========================================================================
    streak_ids := ARRAY[]::UUID[];

    FOR i IN 1..10 LOOP
        INSERT INTO streak_configs (merchant_id, name, required_orders, window_days, reward_amount, is_active)
        VALUES (m_ids[i], '3 in 30', 3, 30, 75.0, true)
        RETURNING id INTO streak_id;
        streak_ids := streak_ids || streak_id;
        cnt_streak_configs := cnt_streak_configs + 1;
    END LOOP;
    RAISE NOTICE 'Created % streak configs', cnt_streak_configs;

    -- Streak achievements for customers with 3+ orders within a 30-day window
    FOR i IN 1..10 LOOP
        FOR wallet_rec IN
            SELECT w.id AS wallet_id, w.customer_id, w.merchant_id,
                   min(le.created_at) AS first_order_ts,
                   max(le.created_at) AS last_order_ts,
                   count(DISTINCT le.event_id) AS order_count
            FROM wallets w
            JOIN ledger_entries le ON le.wallet_id = w.id
            WHERE w.merchant_id = m_ids[i]
              AND w.customer_id IS NOT NULL
              AND w.is_bearer = false
              AND le.bucket_type IN ('earned_credit', 'cod_pending')
              AND le.movement_type IN ('in', 'held')
              AND le.created_at >= base_ts
              AND le.created_at <= base_ts + interval '30 days'
            GROUP BY w.id, w.customer_id, w.merchant_id
            HAVING count(DISTINCT le.event_id) >= 3
            LIMIT 15
        LOOP
            ts := wallet_rec.last_order_ts + interval '1 minute';

            INSERT INTO events (
                merchant_id, event_type, event_source, external_event_id,
                payload, state, idempotency_key, created_at, processed_at
            ) VALUES (
                m_ids[i], 'streak.achieved', 'batua',
                'strk_' || wallet_rec.customer_id::text || '_' || i::text,
                json_build_object('streak', '3 in 30', 'reward', 75)::jsonb,
                'processed',
                'seed:p6:streak:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text,
                ts, ts + interval '1 second'
            )
            ON CONFLICT (idempotency_key) DO NOTHING
            RETURNING id INTO tmp_evt_id;

            IF tmp_evt_id IS NOT NULL THEN
                INSERT INTO ledger_entries (
                    wallet_id, bucket_type, movement_type,
                    earning_unit, currency_equivalent, conversion_rate,
                    idempotency_key, event_id,
                    actor_type, actor_id, constraints,
                    expires_at, created_at, state
                ) VALUES (
                    wallet_rec.wallet_id, 'earned_credit', 'in',
                    75.0, 75.0, 1.0,
                    'seed:p6:streak:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text || ':le',
                    tmp_evt_id,
                    'system', 'streak_engine',
                    '{"stackable": true, "transferable": false}'::jsonb,
                    ts + interval '180 days', ts + interval '1 second', 'active'
                )
                RETURNING id INTO tmp_le_id;

                INSERT INTO streak_achievements (
                    merchant_id, customer_id, streak_config_id, ledger_entry_id,
                    window_start, window_end, achieved_at
                ) VALUES (
                    m_ids[i], wallet_rec.customer_id,
                    streak_ids[i],
                    tmp_le_id,
                    wallet_rec.first_order_ts,
                    wallet_rec.last_order_ts,
                    ts
                );
                cnt_streak_achievements := cnt_streak_achievements + 1;
            END IF;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created % streak achievements', cnt_streak_achievements;

    -- =========================================================================
    -- 10. SPIN WHEEL (1 wheel per merchant, 6 segments, 200 results)
    -- =========================================================================
    FOR i IN 1..10 LOOP
        INSERT INTO spin_wheel_configs (merchant_id, name, is_active, daily_spin_limit)
        VALUES (m_ids[i], 'Lucky Wheel', true, 1)
        ON CONFLICT (merchant_id) DO NOTHING
        RETURNING id INTO wheel_id;

        IF wheel_id IS NULL THEN
            SELECT id INTO wheel_id FROM spin_wheel_configs WHERE merchant_id = m_ids[i];
        END IF;

        cnt_spin_wheels := cnt_spin_wheels + 1;

        seg_ids := ARRAY[]::UUID[];
        FOR j IN 1..6 LOOP
            INSERT INTO spin_wheel_segments (wheel_id, label, reward_amount, probability, color, position)
            VALUES (wheel_id, seg_labels[j], seg_rewards[j], seg_probs[j], seg_colors[j], j)
            RETURNING id INTO seg_id;
            seg_ids := seg_ids || seg_id;
            cnt_spin_segments := cnt_spin_segments + 1;
        END LOOP;

        -- 20 spin results per merchant
        FOR j IN 1..20 LOOP
            SELECT w.id AS wallet_id, w.customer_id
            INTO wallet_rec
            FROM wallets w
            WHERE w.merchant_id = m_ids[i]
              AND w.customer_id IS NOT NULL
              AND w.is_bearer = false
            ORDER BY w.id
            OFFSET ((j * 7 + i * 3) % greatest(1, (SELECT count(*) FROM wallets
                                                     WHERE merchant_id = m_ids[i]
                                                       AND customer_id IS NOT NULL
                                                       AND is_bearer = false)))
            LIMIT 1;

            IF wallet_rec IS NULL THEN
                CONTINUE;
            END IF;

            -- Pick a weighted random segment (deterministic for seed)
            k := ((j * 13 + i * 7) % 6) + 1;
            ts := base_ts + (((j * 41 + i * 17) % 85) * interval '1 day') + ((j * 73 % 1440) * interval '1 minute');

            IF seg_rewards[k] > 0 THEN
                INSERT INTO events (
                    merchant_id, event_type, event_source, external_event_id,
                    payload, state, idempotency_key, created_at, processed_at
                ) VALUES (
                    m_ids[i], 'spin.completed', 'batua',
                    'spin_' || i::text || '_' || j::text,
                    json_build_object('segment', seg_labels[k], 'reward', seg_rewards[k])::jsonb,
                    'processed',
                    'seed:p6:spin:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text || ':' || j::text,
                    ts, ts + interval '1 second'
                )
                ON CONFLICT (idempotency_key) DO NOTHING
                RETURNING id INTO tmp_evt_id;

                IF tmp_evt_id IS NOT NULL THEN
                    INSERT INTO ledger_entries (
                        wallet_id, bucket_type, movement_type,
                        earning_unit, currency_equivalent, conversion_rate,
                        idempotency_key, event_id,
                        actor_type, actor_id, constraints,
                        expires_at, created_at, state
                    ) VALUES (
                        wallet_rec.wallet_id, 'earned_credit', 'in',
                        seg_rewards[k], seg_rewards[k], 1.0,
                        'seed:p6:spin:' || m_ids[i]::text || ':' || wallet_rec.customer_id::text || ':' || j::text || ':le',
                        tmp_evt_id,
                        'system', 'spin_wheel_engine',
                        '{"stackable": true, "transferable": false}'::jsonb,
                        ts + interval '30 days', ts + interval '1 second', 'active'
                    )
                    RETURNING id INTO tmp_le_id;

                    INSERT INTO spin_results (merchant_id, customer_id, segment_id, reward_amount, ledger_entry_id, spun_at)
                    VALUES (m_ids[i], wallet_rec.customer_id, seg_ids[k], seg_rewards[k], tmp_le_id, ts);
                ELSE
                    INSERT INTO spin_results (merchant_id, customer_id, segment_id, reward_amount, spun_at)
                    VALUES (m_ids[i], wallet_rec.customer_id, seg_ids[k], seg_rewards[k], ts)
                    ON CONFLICT DO NOTHING;
                END IF;
            ELSE
                INSERT INTO spin_results (merchant_id, customer_id, segment_id, reward_amount, spun_at)
                VALUES (m_ids[i], wallet_rec.customer_id, seg_ids[k], 0, ts)
                ON CONFLICT DO NOTHING;
            END IF;
            cnt_spin_results := cnt_spin_results + 1;
        END LOOP;
    END LOOP;
    RAISE NOTICE 'Created % spin wheels, % segments, % spin results', cnt_spin_wheels, cnt_spin_segments, cnt_spin_results;

    -- =========================================================================
    -- 11. MEMBERSHIP PLANS + SUBSCRIPTIONS
    -- =========================================================================
    plan_ids := ARRAY[]::UUID[];

    FOR i IN 1..10 LOOP
        -- Monthly plan
        INSERT INTO membership_plans (merchant_id, name, plan_type, price, earn_rate_multiplier, benefits, is_active)
        VALUES (
            m_ids[i], 'Monthly Plus', 'monthly', 99.0, 1.5,
            '{"free_shipping": true, "early_access": true, "support_priority": "high"}'::jsonb,
            true
        )
        RETURNING id INTO p_id;
        plan_ids := plan_ids || p_id;
        cnt_membership_plans := cnt_membership_plans + 1;

        -- Annual plan
        INSERT INTO membership_plans (merchant_id, name, plan_type, price, earn_rate_multiplier, benefits, is_active)
        VALUES (
            m_ids[i], 'Annual Premium', 'annual', 799.0, 2.0,
            '{"free_shipping": true, "early_access": true, "support_priority": "vip", "birthday_bonus": true, "exclusive_products": true}'::jsonb,
            true
        )
        RETURNING id INTO p_id;
        plan_ids := plan_ids || p_id;
        cnt_membership_plans := cnt_membership_plans + 1;
    END LOOP;
    RAISE NOTICE 'Created % membership plans', cnt_membership_plans;

    -- 100 subscriptions spread across merchants
    FOR i IN 1..100 LOOP
        m_id := m_ids[((i - 1) % 10) + 1];
        ts := base_ts + (((i * 23) % 60) * interval '1 day') + ((i * 59 % 1440) * interval '1 minute');

        SELECT w.id, w.customer_id
        INTO wallet_rec
        FROM wallets w
        WHERE w.merchant_id = m_id
          AND w.customer_id IS NOT NULL
          AND w.is_bearer = false
        ORDER BY w.id
        OFFSET ((i * 13) % greatest(1, (SELECT count(*) FROM wallets
                                          WHERE merchant_id = m_id
                                            AND customer_id IS NOT NULL
                                            AND is_bearer = false)))
        LIMIT 1;

        IF wallet_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Alternate between monthly and annual plans
        IF (i % 3) < 2 THEN
            -- Monthly
            p_id := plan_ids[((i - 1) % 10) * 2 + 1];
            INSERT INTO customer_memberships (
                merchant_id, customer_id, plan_id, status,
                started_at, expires_at, renewed_count, created_at
            ) VALUES (
                m_id, wallet_rec.customer_id, p_id, 'active',
                ts, ts + interval '30 days',
                (i % 4), ts
            )
            ON CONFLICT (merchant_id, customer_id, plan_id) DO NOTHING;
        ELSE
            -- Annual
            p_id := plan_ids[((i - 1) % 10) * 2 + 2];
            INSERT INTO customer_memberships (
                merchant_id, customer_id, plan_id, status,
                started_at, expires_at, renewed_count, created_at
            ) VALUES (
                m_id, wallet_rec.customer_id, p_id, 'active',
                ts, ts + interval '365 days',
                0, ts
            )
            ON CONFLICT (merchant_id, customer_id, plan_id) DO NOTHING;
        END IF;
        cnt_memberships := cnt_memberships + 1;
    END LOOP;
    RAISE NOTICE 'Created % memberships', cnt_memberships;

    -- =========================================================================
    -- 12. NEWSLETTER SIGNUPS (300 total)
    -- =========================================================================
    FOR i IN 1..300 LOOP
        m_id := m_ids[((i - 1) % 10) + 1];
        ts := base_ts + (((i * 31) % 85) * interval '1 day') + ((i * 67 % 1440) * interval '1 minute');

        SELECT w.id AS wallet_id, w.customer_id
        INTO wallet_rec
        FROM wallets w
        WHERE w.merchant_id = m_id
          AND w.customer_id IS NOT NULL
          AND w.is_bearer = false
        ORDER BY w.id
        OFFSET ((i * 9) % greatest(1, (SELECT count(*) FROM wallets
                                         WHERE merchant_id = m_id
                                           AND customer_id IS NOT NULL
                                           AND is_bearer = false)))
        LIMIT 1;

        IF wallet_rec IS NULL THEN
            CONTINUE;
        END IF;

        -- Create a ledger entry for newsletter signup reward
        INSERT INTO events (
            merchant_id, event_type, event_source, external_event_id,
            payload, state, idempotency_key, created_at, processed_at
        ) VALUES (
            m_id, 'newsletter.signup', 'batua',
            'nws_' || m_id::text || '_' || wallet_rec.customer_id::text,
            json_build_object('customer_id', wallet_rec.customer_id, 'source', 'webhook')::jsonb,
            'processed',
            'seed:p6:newsletter:' || m_id::text || ':' || wallet_rec.customer_id::text,
            ts, ts + interval '1 second'
        )
        ON CONFLICT (idempotency_key) DO NOTHING
        RETURNING id INTO tmp_evt_id;

        IF tmp_evt_id IS NULL THEN
            CONTINUE;
        END IF;

        INSERT INTO ledger_entries (
            wallet_id, bucket_type, movement_type,
            earning_unit, currency_equivalent, conversion_rate,
            idempotency_key, event_id,
            actor_type, actor_id, constraints,
            expires_at, created_at, state
        ) VALUES (
            wallet_rec.wallet_id, 'earned_credit', 'in',
            25.0, 25.0, 1.0,
            'seed:p6:newsletter:' || m_id::text || ':' || wallet_rec.customer_id::text || ':le',
            tmp_evt_id,
            'system', 'newsletter_engine',
            '{"stackable": true, "transferable": false}'::jsonb,
            ts + interval '90 days', ts + interval '1 second', 'active'
        )
        RETURNING id INTO tmp_le_id;

        SELECT c.email INTO cust_rec
        FROM customers c
        WHERE c.id = wallet_rec.customer_id;

        INSERT INTO newsletter_signups (
            merchant_id, customer_id, ledger_entry_id, email, source, created_at
        ) VALUES (
            m_id, wallet_rec.customer_id, tmp_le_id,
            coalesce(cust_rec.email, 'unknown_' || i::text || '@example.com'),
            CASE WHEN (i % 3) = 0 THEN 'storefront' WHEN (i % 3) = 1 THEN 'webhook' ELSE 'checkout' END,
            ts
        )
        ON CONFLICT (merchant_id, customer_id) DO NOTHING;
        cnt_newsletter_signups := cnt_newsletter_signups + 1;
    END LOOP;
    RAISE NOTICE 'Created % newsletter signups', cnt_newsletter_signups;

    -- =========================================================================
    -- 13. CUSTOMER ORDER STATS (aggregated from ledger)
    -- =========================================================================
    INSERT INTO customer_order_stats (
        merchant_id, customer_id, total_orders, total_spend,
        first_order_at, last_order_at, updated_at
    )
    SELECT
        w.merchant_id,
        w.customer_id,
        count(DISTINCT le.event_id)::integer AS total_orders,
        coalesce(sum(
            CASE WHEN le.movement_type IN ('in', 'held') AND le.earning_unit > 0
                 THEN le.earning_unit ELSE 0 END
        ), 0) AS total_spend,
        min(le.created_at) AS first_order_at,
        max(le.created_at) AS last_order_at,
        now()
    FROM wallets w
    JOIN ledger_entries le ON le.wallet_id = w.id
    WHERE w.customer_id IS NOT NULL
      AND w.is_bearer = false
      AND le.event_id IS NOT NULL
    GROUP BY w.merchant_id, w.customer_id
    ON CONFLICT (merchant_id, customer_id) DO UPDATE SET
        total_orders = EXCLUDED.total_orders,
        total_spend = EXCLUDED.total_spend,
        first_order_at = EXCLUDED.first_order_at,
        last_order_at = EXCLUDED.last_order_at,
        updated_at = now();

    GET DIAGNOSTICS cnt_customer_order_stats = ROW_COUNT;
    RAISE NOTICE 'Upserted % customer order stats', cnt_customer_order_stats;

    -- =========================================================================
    -- SUMMARY
    -- =========================================================================
    RAISE NOTICE '========================================';
    RAISE NOTICE 'PHASE 6 SEED DATA SUMMARY';
    RAISE NOTICE '========================================';
    RAISE NOTICE 'Campaigns:              %', cnt_campaigns;
    RAISE NOTICE 'Campaign Snapshots:     %', cnt_campaign_snapshots;
    RAISE NOTICE 'Gift Cards:             %', cnt_gift_cards;
    RAISE NOTICE '  Bearer Wallets:       %', cnt_gift_card_wallets;
    RAISE NOTICE '  Ledger Entries:       %', cnt_gift_card_ledger;
    RAISE NOTICE 'Referral Conversions:   %', cnt_referral_conversions;
    RAISE NOTICE 'COD Orders:             %', cnt_cod_orders;
    RAISE NOTICE 'Redemption Requests:    %', cnt_redemption_requests;
    RAISE NOTICE 'Notification Logs:      %', cnt_notification_logs;
    RAISE NOTICE 'Coalition Transfers:    %', cnt_coalition_transfers;
    RAISE NOTICE 'Milestone Configs:      %', cnt_milestone_configs;
    RAISE NOTICE 'Milestone Achievements: %', cnt_milestone_achievements;
    RAISE NOTICE 'Streak Configs:         %', cnt_streak_configs;
    RAISE NOTICE 'Streak Achievements:    %', cnt_streak_achievements;
    RAISE NOTICE 'Spin Wheels:            %', cnt_spin_wheels;
    RAISE NOTICE 'Spin Segments:          %', cnt_spin_segments;
    RAISE NOTICE 'Spin Results:           %', cnt_spin_results;
    RAISE NOTICE 'Membership Plans:       %', cnt_membership_plans;
    RAISE NOTICE 'Memberships:            %', cnt_memberships;
    RAISE NOTICE 'Newsletter Signups:     %', cnt_newsletter_signups;
    RAISE NOTICE 'Customer Order Stats:   %', cnt_customer_order_stats;
    RAISE NOTICE '========================================';

END;
$$;

-- Final verification query
SELECT
    (SELECT count(*) FROM campaigns)              AS campaigns,
    (SELECT count(*) FROM campaign_snapshots)      AS campaign_snapshots,
    (SELECT count(*) FROM gift_cards)              AS gift_cards,
    (SELECT count(*) FROM referral_conversions)    AS referral_conversions,
    (SELECT count(*) FROM cod_orders)              AS cod_orders,
    (SELECT count(*) FROM redemption_requests)     AS redemption_requests,
    (SELECT count(*) FROM notification_logs)       AS notification_logs,
    (SELECT count(*) FROM coalition_transfers)     AS coalition_transfers,
    (SELECT count(*) FROM milestone_configs)       AS milestone_configs,
    (SELECT count(*) FROM milestone_achievements)  AS milestone_achievements,
    (SELECT count(*) FROM streak_configs)          AS streak_configs,
    (SELECT count(*) FROM streak_achievements)     AS streak_achievements,
    (SELECT count(*) FROM spin_wheel_configs)      AS spin_wheels,
    (SELECT count(*) FROM spin_wheel_segments)     AS spin_segments,
    (SELECT count(*) FROM spin_results)            AS spin_results,
    (SELECT count(*) FROM membership_plans)        AS membership_plans,
    (SELECT count(*) FROM customer_memberships)    AS memberships,
    (SELECT count(*) FROM newsletter_signups)      AS newsletter_signups,
    (SELECT count(*) FROM customer_order_stats)    AS customer_order_stats;

COMMIT;
