-- seed-50-merchants.sql
-- Generates 50 realistic Indian D2C merchants, each with a customer base,
-- >=100 ledger transactions, and reasonable data across every feature
-- (rules, wallet policies, loyalty tiers, memberships, campaigns, referrals,
--  gift cards, COD orders, redemptions).
--
-- Non-destructive: all rows are tagged external_id 'seed50_*'. Re-running first
-- removes the previous seed50 dataset, then regenerates it.
--
-- Usage: psql -d batua -f scripts/seed-50-merchants.sql

BEGIN;

-- ---------------------------------------------------------------------------
-- Fresh slate: clear all merchant/customer domain data (dev test DB).
-- CASCADE transitively clears wallets, ledger, rules, gift cards, campaigns,
-- loyalty, referrals, memberships, COD, redemptions, events, etc.
-- geo_policies is referenced BY merchants, so it is left intact.
-- ---------------------------------------------------------------------------
TRUNCATE merchants, customers RESTART IDENTITY CASCADE;

SELECT setseed(0.4242);

DO $seed$
DECLARE
  brand_names TEXT[] := ARRAY[
    'Desi Threads','Kaya Beauty','Spice Route','VoltEdge','FitFuel','PawSome','TinyTots','Nestora',
    'PageTurner','Aurum Jewels','Chai Chapter','Anaya Ethnics','Herbveda','SoleMate','Bloom & Petal',
    'UrbanCarry','Zaika Foods','Prana Yoga','Lumos Home','Craftsy','Mithai Magic','DenimDen',
    'Bare Necessities','GadgetGrove','Vayu Activewear','Little Leaf','Rustic Roots','Nova Cosmetics',
    'The Coffee Co','Silk & Sari','Trailblaze','Munchbox','Aroma Candles','FreshFold','Ayur Glow',
    'Pixel Prints','Terra Pots','Wanderlust','Cricket Club','Bean There','Luxe Locks','Sneaker Society',
    'Homely Kitchen','Baby Bloom','Vintage Vault','Peppy Pets','Glow & Grace','Threadwork',
    'Snack Attack','Zen Living'];
  brand_slugs TEXT[] := ARRAY[
    'desi-threads','kaya-beauty','spice-route','voltedge','fitfuel','pawsome','tinytots','nestora',
    'pageturner','aurum-jewels','chai-chapter','anaya-ethnics','herbveda','solemate','bloom-petal',
    'urbancarry','zaika-foods','prana-yoga','lumos-home','craftsy','mithai-magic','denimden',
    'bare-necessities','gadgetgrove','vayu-activewear','little-leaf','rustic-roots','nova-cosmetics',
    'the-coffee-co','silk-sari','trailblaze','munchbox','aroma-candles','freshfold','ayur-glow',
    'pixel-prints','terra-pots','wanderlust','cricket-club','bean-there','luxe-locks','sneaker-society',
    'homely-kitchen','baby-bloom','vintage-vault','peppy-pets','glow-grace','threadwork',
    'snack-attack','zen-living'];
  first_names TEXT[] := ARRAY[
    'Aarav','Vivaan','Aditya','Arjun','Sai','Arnav','Ayaan','Ishaan','Kabir','Dhruv','Rohan','Nikhil',
    'Rahul','Amit','Vikram','Rajesh','Deepak','Gaurav','Kunal','Varun','Karan','Pranav','Yash','Dev',
    'Ananya','Diya','Aadhya','Saanvi','Aarohi','Anika','Navya','Riya','Ishita','Kiara','Myra','Sara',
    'Priya','Neha','Pooja','Kavya','Meera','Nisha','Sneha','Divya','Tara','Aisha','Anjali','Ritu'];
  last_names TEXT[] := ARRAY[
    'Sharma','Verma','Gupta','Iyer','Nair','Reddy','Rao','Mehta','Shah','Patel','Singh','Kaur',
    'Chopra','Malhotra','Kapoor','Bose','Banerjee','Das','Menon','Pillai','Desai','Joshi','Kulkarni',
    'Bhat','Naidu','Agarwal','Jain','Mishra','Chauhan','Yadav','Pandey','Sinha','Ghosh','Roy'];
  points_names TEXT[] := ARRAY['Points','Coins','Stars','Credits','Miles','Gems'];
  points_icons TEXT[] := ARRAY['pts','coins','stars','cr','mi','gems'];
  plan_tiers TEXT[]  := ARRAY['free','grow','scale','enterprise'];
  rates DOUBLE PRECISION[] := ARRAY[0.5,1.0,1.0,1.0,2.0];
  tier_names TEXT[] := ARRAY['Bronze','Silver','Gold','Platinum'];

  mi INT; ci INT; oi INT; k INT;
  m_id UUID; c_id UUID; w_id UUID; prog_id UUID; refprog_id UUID;
  cur_rule_id UUID; cur_snap_id UUID; base_rule_id UUID; cur_config JSONB;
  held_id UUID; out_id UUID; earn_id UUID;
  tier_ids UUID[]; gc_wallet UUID; ref_code TEXT;

  earn_pct DOUBLE PRECISION; cod_pct DOUBLE PRECISION;
  order_min DOUBLE PRECISION; order_max DOUBLE PRECISION;
  n_customers INT; n_orders INT;
  order_amt DOUBLE PRECISION; earn_val DOUBLE PRECISION; cod_val DOUBLE PRECISION;
  accumulated DOUBLE PRECISION; redeem_val DOUBLE PRECISION; spend DOUBLE PRECISION;
  is_cod BOOLEAN; ts TIMESTAMPTZ;
  fn TEXT; ln TEXT; cust_name TEXT; pidx INT;
  m_created TIMESTAMPTZ; base_ts TIMESTAMPTZ := now() - interval '120 days';
  phone_seq BIGINT := 7100000000;
  gc_seq INT := 0; rc_seq INT := 0;
  cust_ids UUID[]; cust_accum DOUBLE PRECISION[]; cust_codes UUID[];
  tot_customers INT := 0; tot_entries INT := 0;
BEGIN
  FOR mi IN 1..50 LOOP
    earn_pct   := round((2 + random()*6)::numeric, 1);
    cod_pct    := round((1 + random()*3)::numeric, 1);
    order_min  := 300 + floor(random()*5)::int * 100;
    order_max  := order_min + 1500 + floor(random()*40)::int * 100;
    n_customers := 55 + floor(random()*55)::int;          -- 55..109
    pidx       := 1 + floor(random()*6)::int;
    m_created  := base_ts + (random()*30) * interval '1 day';

    INSERT INTO merchants (external_id, name, domain, slug, plan_tier, points_name, points_icon,
                           points_to_currency_rate, is_active, created_at, updated_at)
    VALUES ('seed50_'||lpad(mi::text,3,'0'), brand_names[mi], brand_slugs[mi]||'.myshopify.com',
            brand_slugs[mi], plan_tiers[1+floor(random()*4)::int], points_names[pidx], points_icons[pidx],
            rates[1+floor(random()*5)::int], true, m_created, m_created)
    RETURNING id INTO m_id;

    -- ---- Rules (3) + snapshots ----
    base_rule_id := NULL;
    FOR k IN 1..3 LOOP
      cur_config := CASE k
        WHEN 1 THEN jsonb_build_object('trigger','order.completed',
                      'condition', jsonb_build_object('payment_method','prepaid','min_order_amount',order_min),
                      'action', jsonb_build_object('type','percentage','value',earn_pct,'bucket','earned_credit','max_reward',500),
                      'description', 'Earn '||earn_pct||'% on prepaid orders above Rs.'||order_min)
        WHEN 2 THEN jsonb_build_object('trigger','order.delivered',
                      'condition', jsonb_build_object('payment_method','cod','min_order_amount',300),
                      'action', jsonb_build_object('type','percentage','value',cod_pct,'bucket','cod_pending','max_reward',200),
                      'description', 'Earn '||cod_pct||'% on COD orders on delivery')
        ELSE jsonb_build_object('trigger','order.completed',
                      'condition', jsonb_build_object('is_first_order',true),
                      'action', jsonb_build_object('type','fixed','value',50,'bucket','earned_credit'),
                      'description', 'Rs.50 bonus on your first order')
      END;
      INSERT INTO rules (merchant_id, rule_type, name, config, version, is_active, created_at)
      VALUES (m_id, 'reward', (ARRAY['Prepaid Cashback','COD Cashback','First Order Bonus'])[k], cur_config, 1, true, m_created)
      RETURNING id INTO cur_rule_id;
      INSERT INTO rule_snapshots (rule_id, version, config) VALUES (cur_rule_id, 1, cur_config);
      IF k = 1 THEN base_rule_id := cur_rule_id; END IF;
    END LOOP;

    -- ---- Wallet policies (2) ----
    INSERT INTO wallet_policies (merchant_id, bucket_type, min_redemption, step_size, max_per_order_pct, default_expiry_days, is_active)
    VALUES (m_id, 'earned_credit', 10 + floor(random()*3)::int*5, 10, 30 + floor(random()*4)::int*5, 365, true),
           (m_id, 'cod_pending', 20, 10, 25, 90, true);

    -- ---- Loyalty program + 4 tiers ----
    INSERT INTO loyalty_programs (merchant_id, name, evaluation_criteria, evaluation_period_days, is_active, created_at)
    VALUES (m_id, brand_names[mi]||' Rewards', 'spend', 365, true, m_created)
    RETURNING id INTO prog_id;
    tier_ids := ARRAY[]::UUID[];
    FOR k IN 1..4 LOOP
      INSERT INTO loyalty_tiers (program_id, name, rank, threshold, earn_rate_multiplier, benefits)
      VALUES (prog_id, tier_names[k], k, (ARRAY[0,2500,6000,15000])[k], 1.0 + (k-1)*0.25,
              jsonb_build_object('free_shipping', k>=2, 'birthday_bonus', k>=3, 'early_access', k>=4))
      RETURNING id INTO cur_rule_id;
      tier_ids := tier_ids || cur_rule_id;
    END LOOP;

    -- ---- Referral program ----
    INSERT INTO referral_programs (merchant_id, referrer_reward_amount, referee_reward_amount, max_referrals_per_customer, is_active, created_at)
    VALUES (m_id, 40 + floor(random()*4)::int*10, 20 + floor(random()*3)::int*10, 20, true, m_created)
    RETURNING id INTO refprog_id;

    -- ---- Campaigns (1-2) ----
    INSERT INTO campaigns (merchant_id, name, campaign_type, config, base_rule_id, multiplier, starts_at, ends_at, is_active, created_at)
    VALUES (m_id, (ARRAY['Diwali Dhamaka','Weekend Double','Republic Day Sale','Summer Splash'])[1+floor(random()*4)::int],
            'multiplier', jsonb_build_object('stacking','multiplicative','max_cap',10),
            base_rule_id, 2 + floor(random()*2)::int, now() - interval '5 days', now() + interval '10 days', true, m_created);
    IF random() < 0.5 THEN
      INSERT INTO campaigns (merchant_id, name, campaign_type, config, base_rule_id, multiplier, starts_at, ends_at, is_active, created_at)
      VALUES (m_id, 'Flash Friday', 'multiplier', jsonb_build_object('stacking','best_of','max_cap',5),
              base_rule_id, 3, now() - interval '40 days', now() - interval '33 days', false, m_created);
    END IF;

    -- ---- Customers + wallets + ledger ----
    cust_ids := ARRAY[]::UUID[]; cust_accum := ARRAY[]::DOUBLE PRECISION[]; cust_codes := ARRAY[]::UUID[];
    FOR ci IN 1..n_customers LOOP
      fn := first_names[1+floor(random()*array_length(first_names,1))::int];
      ln := last_names[1+floor(random()*array_length(last_names,1))::int];
      cust_name := fn||' '||ln;
      phone_seq := phone_seq + 1;
      INSERT INTO customers (phone, email, name, external_id, is_verified, created_at, birthday)
      VALUES ('+91'||phone_seq::text, lower(fn)||'.'||lower(ln)||phone_seq::text||'@example.com', cust_name,
              'seed50_'||m_id::text||'_'||ci::text, random() < 0.7, m_created + (random()*90)*interval '1 day',
              '1980-01-01'::date + floor(random()*9000)::int)
      RETURNING id INTO c_id;
      cust_ids := cust_ids || c_id;

      INSERT INTO wallets (merchant_id, customer_id) VALUES (m_id, c_id) RETURNING id INTO w_id;

      n_orders := 2 + floor(random()*4)::int;   -- 2..5
      accumulated := 0;
      spend := 0;
      FOR oi IN 1..n_orders LOOP
        ts := m_created + (random()*100 + oi) * interval '1 day';
        order_amt := round((order_min + random()*(order_max-order_min))::numeric, 2);
        spend := spend + order_amt;
        is_cod := random() < 0.28;

        IF is_cod THEN
          cod_val := round((order_amt * cod_pct/100)::numeric, 2);
          INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent,
                                      conversion_rate, idempotency_key, actor_type, state, expires_at, created_at)
          VALUES (w_id, 'cod_pending', 'held', cod_val, cod_val, 1.0,
                  'seed50:'||w_id::text||':cod:'||oi::text, 'automation', 'active', ts + interval '90 days', ts);
          tot_entries := tot_entries + 1;
        ELSE
          earn_val := round((order_amt * earn_pct/100)::numeric, 2);
          INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent,
                                      conversion_rate, idempotency_key, actor_type, state, expires_at, created_at)
          VALUES (w_id, 'earned_credit', 'in', earn_val, earn_val, 1.0,
                  'seed50:'||w_id::text||':earn:'||oi::text, 'automation', 'active', ts + interval '365 days', ts);
          accumulated := accumulated + earn_val;
          tot_entries := tot_entries + 1;
        END IF;
      END LOOP;

      -- occasional redemption (needs balance)
      IF accumulated > 120 AND random() < 0.40 THEN
        redeem_val := round((accumulated * (0.2 + random()*0.4))::numeric, 2);
        ts := m_created + (random()*110 + 5) * interval '1 day';
        INSERT INTO ledger_entries (wallet_id, bucket_type, movement_type, earning_unit, currency_equivalent,
                                    conversion_rate, idempotency_key, actor_type, state, created_at)
        VALUES (w_id, 'earned_credit', 'out', redeem_val, redeem_val, 1.0,
                'seed50:'||w_id::text||':redeem', 'human', 'active', ts)
        RETURNING id INTO out_id;
        tot_entries := tot_entries + 1;
        INSERT INTO redemption_requests (merchant_id, wallet_id, requested_amount, eligible_amount, applied_amount,
                                         order_id, order_amount, payment_method, state, debit_entry_id, created_at)
        VALUES (m_id, w_id, redeem_val, redeem_val, redeem_val, 'ORD-'||floor(random()*900000+100000)::text,
                round((order_min + random()*(order_max-order_min))::numeric,2), 'prepaid', 'completed', out_id, ts);
        accumulated := accumulated - redeem_val;
      END IF;

      cust_accum := cust_accum || accumulated;

      -- referral code for ~half of customers
      IF random() < 0.5 THEN
        rc_seq := rc_seq + 1;
        INSERT INTO referral_codes (merchant_id, customer_id, code, is_creator, total_referrals, total_conversions, is_active, created_at)
        VALUES (m_id, c_id, upper(substr(brand_slugs[mi],1,4))||lpad(rc_seq::text,5,'0'),
                random() < 0.15, floor(random()*8)::int, floor(random()*4)::int, true, m_created)
        RETURNING id INTO cur_rule_id;
        cust_codes := cust_codes || cur_rule_id;
      ELSE
        cust_codes := cust_codes || NULL::UUID;
      END IF;

      -- VIP tier assignment based on total spend
      IF spend >= 15000 THEN k := 4;
      ELSIF spend >= 6000 THEN k := 3;
      ELSIF spend >= 2500 THEN k := 2;
      ELSE k := 1; END IF;
      IF k >= 2 THEN
        INSERT INTO customer_tiers (customer_id, merchant_id, tier_id, qualifying_value, qualified_at)
        VALUES (c_id, m_id, tier_ids[k], spend, m_created + interval '30 days');
      END IF;

      -- membership for a small subset (paid tier)
      IF random() < 0.12 THEN
        INSERT INTO customer_memberships (merchant_id, customer_id, tier_id, status, started_at, expires_at, renewed_count)
        VALUES (m_id, c_id, tier_ids[3], 'active', m_created + interval '20 days',
                now() + (interval '1 day' * (30 + floor(random()*300)::int)), floor(random()*3)::int);
      END IF;

      tot_customers := tot_customers + 1;
    END LOOP;

    -- ---- Gift cards (10-25 bearer cards) ----
    FOR k IN 1..(10 + floor(random()*16)::int) LOOP
      gc_seq := gc_seq + 1;
      INSERT INTO wallets (merchant_id, is_bearer, bearer_code)
      VALUES (m_id, true, 'BR-'||lpad(gc_seq::text,8,'0')) RETURNING id INTO gc_wallet;
      order_amt := (ARRAY[250,500,1000,2000,2500,5000])[1+floor(random()*6)::int];
      INSERT INTO gift_cards (merchant_id, wallet_id, code, initial_amount, current_amount, currency,
                              issued_by, issued_by_id, is_claimed, expires_at, is_active, created_at)
      VALUES (m_id, gc_wallet, 'GC-'||upper(substr(md5(random()::text),1,10)), order_amt,
              round((order_amt * (random()*0.9 + 0.1))::numeric,2), 'INR', 'human', 'seed',
              random() < 0.55, now() + interval '365 days', true, m_created + (random()*90)*interval '1 day');
    END LOOP;

    -- ---- Referral conversions (a few per merchant) ----
    FOR k IN 1..array_length(cust_ids,1) LOOP
      IF cust_codes[k] IS NOT NULL AND random() < 0.25 THEN
        ci := 1 + floor(random()*array_length(cust_ids,1))::int;
        IF cust_ids[ci] <> cust_ids[k] THEN
          INSERT INTO referral_conversions (merchant_id, referral_code_id, referrer_id, referee_id, order_id, created_at)
          VALUES (m_id, cust_codes[k], cust_ids[k], cust_ids[ci],
                  'ORD-'||floor(random()*900000+100000)::text, m_created + (random()*90)*interval '1 day');
        END IF;
      END IF;
    END LOOP;

    RAISE NOTICE 'Merchant %/50: % (% customers)', mi, brand_names[mi], n_customers;
  END LOOP;

  RAISE NOTICE 'DONE: 50 merchants, % customers, % ledger entries', tot_customers, tot_entries;
END
$seed$;

COMMIT;
