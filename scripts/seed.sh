#!/usr/bin/env bash
set -euo pipefail

API="${API_BASE_URL:-http://localhost:3000}"
echo "Seeding Batua demo data at $API..."

# -----------------------------------------------------------
# Helper
# -----------------------------------------------------------
post() { curl -sf -X POST "$API$1" -H "Content-Type: application/json" -d "$2"; }
get()  { curl -sf "$API$1"; }
jq_id() { python3 -c "import sys,json; print(json.load(sys.stdin)$1)"; }

# -----------------------------------------------------------
# 1. Merchant
# -----------------------------------------------------------
echo ""
echo "=== Creating merchant: Desi Threads ==="
M=$(post /admin/merchants '{"external_id":"shop_desi_threads","name":"Desi Threads","domain":"desithreads.myshopify.com","currency":"INR","timezone":"Asia/Kolkata"}')
MID=$(echo "$M" | jq_id "['id']")
echo "  Merchant ID: $MID"

# -----------------------------------------------------------
# 2. Geo Policy (India)
# -----------------------------------------------------------
echo ""
echo "=== Creating geo policy: India ==="
post /admin/geo-policies "{\"geo_code\":\"india\",\"name\":\"India\",\"config\":{\"cod_enabled\":true,\"default_currency\":\"INR\",\"whatsapp_default\":true,\"upi_topup_enabled\":true}}" > /dev/null
echo "  Done"

# -----------------------------------------------------------
# 3. Reward Rules
# -----------------------------------------------------------
echo ""
echo "=== Creating reward rules ==="
post /rules "{\"merchant_id\":\"$MID\",\"rule_type\":\"reward\",\"name\":\"5% cashback on prepaid orders\",\"config\":{\"event_type\":\"order.completed\",\"conditions\":[{\"field\":\"is_cod\",\"operator\":\"eq\",\"value\":false},{\"field\":\"order_amount\",\"operator\":\"gte\",\"value\":500}],\"action\":{\"bucket_type\":\"earned_credit\",\"calculation\":\"percentage\",\"value\":5.0,\"max_amount\":200,\"conversion_rate\":1.0,\"expiry_days\":365}}}" > /dev/null
echo "  5% prepaid cashback (min ₹500, max ₹200)"

post /rules "{\"merchant_id\":\"$MID\",\"rule_type\":\"reward\",\"name\":\"3% cashback on COD orders (pending delivery)\",\"config\":{\"event_type\":\"order.completed\",\"conditions\":[{\"field\":\"is_cod\",\"operator\":\"eq\",\"value\":true},{\"field\":\"order_amount\",\"operator\":\"gte\",\"value\":500}],\"action\":{\"bucket_type\":\"cod_pending\",\"calculation\":\"percentage\",\"value\":3.0,\"max_amount\":150,\"conversion_rate\":1.0,\"expiry_days\":90}}}" > /dev/null
echo "  3% COD cashback (pending delivery, max ₹150)"

post /rules "{\"merchant_id\":\"$MID\",\"rule_type\":\"reward\",\"name\":\"Flat ₹50 on first order\",\"config\":{\"event_type\":\"order.completed\",\"conditions\":[{\"field\":\"is_first_order\",\"operator\":\"eq\",\"value\":true}],\"action\":{\"bucket_type\":\"earned_credit\",\"calculation\":\"fixed\",\"value\":50.0,\"max_amount\":null,\"conversion_rate\":1.0,\"expiry_days\":180}}}" > /dev/null
echo "  Flat ₹50 first-order bonus"

# -----------------------------------------------------------
# 4. Customers
# -----------------------------------------------------------
echo ""
echo "=== Creating customers ==="

declare -a CUSTOMERS=(
  "9876543210|priya@example.com|Priya Sharma"
  "8765432109|arjun@example.com|Arjun Mehta"
  "7654321098|sneha@example.com|Sneha Patel"
  "6543210987|rahul@example.com|Rahul Kumar"
  "9988776655|aarti@example.com|Aarti Singh"
  "8877665544|vikram@example.com|Vikram Reddy"
  "7766554433|meera@example.com|Meera Nair"
  "6655443322|rohit@example.com|Rohit Gupta"
)

declare -a CIDS=()
declare -a WIDS=()

for c in "${CUSTOMERS[@]}"; do
  IFS='|' read -r phone email name <<< "$c"
  RES=$(post /identity/resolve "{\"phone\":\"$phone\",\"email\":\"$email\",\"name\":\"$name\"}")
  CID=$(echo "$RES" | jq_id "['customer_id']")
  CIDS+=("$CID")

  WRES=$(post /wallets/get-or-create "{\"merchant_id\":\"$MID\",\"customer_id\":\"$CID\"}")
  WID=$(echo "$WRES" | jq_id "['wallet']['id']")
  WIDS+=("$WID")
  echo "  $name ($phone) → wallet $WID"
done

# -----------------------------------------------------------
# 5. Simulate orders (prepaid + COD mix)
# -----------------------------------------------------------
echo ""
echo "=== Simulating orders ==="

ORDER_NUM=3001

simulate_order() {
  local cust_idx=$1 amount=$2 gateway=$3 name=$4
  local phone
  IFS='|' read -r phone _ _ <<< "${CUSTOMERS[$cust_idx]}"

  local is_cod="false"
  local gw_arr="[\"$gateway\"]"
  if [[ "$gateway" == *"COD"* ]]; then
    is_cod="true"
  fi

  local eid
  eid=$(post /events/shopify/orders "{\"merchant_id\":\"$MID\",\"payload\":{\"id\":$ORDER_NUM,\"order_number\":$ORDER_NUM,\"total_price\":\"$amount.00\",\"currency\":\"INR\",\"financial_status\":\"paid\",\"gateway\":\"$gateway\",\"payment_gateway_names\":$gw_arr,\"customer\":{\"id\":$((5000+cust_idx)),\"phone\":\"+91$phone\",\"first_name\":\"${name%% *}\",\"last_name\":\"${name#* }\"}}}" | jq_id "['event_id']")

  post /earn/process "{\"event_id\":\"$eid\"}" > /dev/null
  echo "  Order #$ORDER_NUM: $name, ₹$amount ($gateway)"
  ORDER_NUM=$((ORDER_NUM + 1))
}

# Priya — heavy buyer
simulate_order 0 2500 "razorpay" "Priya Sharma"
simulate_order 0 1800 "razorpay" "Priya Sharma"
simulate_order 0 3200 "Cash on Delivery (COD)" "Priya Sharma"
simulate_order 0 4500 "razorpay" "Priya Sharma"

# Arjun — moderate
simulate_order 1 1200 "razorpay" "Arjun Mehta"
simulate_order 1 2800 "Cash on Delivery (COD)" "Arjun Mehta"
simulate_order 1 950 "razorpay" "Arjun Mehta"

# Sneha — COD heavy
simulate_order 2 1500 "Cash on Delivery (COD)" "Sneha Patel"
simulate_order 2 2200 "Cash on Delivery (COD)" "Sneha Patel"
simulate_order 2 1800 "razorpay" "Sneha Patel"

# Rahul — single big order
simulate_order 3 8000 "razorpay" "Rahul Kumar"

# Aarti — small orders
simulate_order 4 600 "razorpay" "Aarti Singh"
simulate_order 4 750 "razorpay" "Aarti Singh"
simulate_order 4 500 "Cash on Delivery (COD)" "Aarti Singh"

# Vikram
simulate_order 5 3500 "razorpay" "Vikram Reddy"
simulate_order 5 1200 "Cash on Delivery (COD)" "Vikram Reddy"

# Meera
simulate_order 6 2000 "razorpay" "Meera Nair"
simulate_order 6 1600 "razorpay" "Meera Nair"

# Rohit
simulate_order 7 900 "razorpay" "Rohit Gupta"
simulate_order 7 1100 "Cash on Delivery (COD)" "Rohit Gupta"

# -----------------------------------------------------------
# 6. Deliver some COD orders (release credits)
# -----------------------------------------------------------
echo ""
echo "=== Delivering COD orders ==="

# Get COD orders and deliver some
COD_ORDERS=$(get "/cod/orders/$MID?limit=20" 2>/dev/null || echo "[]")
DELIVERED=0
echo "$COD_ORDERS" | python3 -c "
import sys,json
orders = json.load(sys.stdin)
if isinstance(orders, list):
    for o in orders[:5]:
        print(o.get('order_id',''))
elif isinstance(orders, dict) and 'orders' in orders:
    for o in orders['orders'][:5]:
        print(o.get('order_id',''))
" 2>/dev/null | while read -r oid; do
  if [ -n "$oid" ]; then
    post /cod/webhook/delivery "{\"order_id\":\"$oid\",\"status\":\"delivered\",\"delivered_at\":\"2026-03-18T14:00:00Z\",\"merchant_id\":\"$MID\"}" > /dev/null 2>&1 && echo "  Delivered: order $oid" || true
    DELIVERED=$((DELIVERED + 1))
  fi
done

# -----------------------------------------------------------
# 7. Redemptions
# -----------------------------------------------------------
echo ""
echo "=== Processing redemptions ==="

# Priya redeems ₹100
post /redemptions "{\"wallet_id\":\"${WIDS[0]}\",\"order_id\":\"REDEEM-001\",\"order_amount\":2000,\"payment_method\":\"razorpay\",\"requested_amount\":100,\"discount_codes\":[]}" > /dev/null 2>&1 && echo "  Priya redeemed ₹100" || echo "  Priya redemption skipped"

# Arjun redeems ₹50
post /redemptions "{\"wallet_id\":\"${WIDS[1]}\",\"order_id\":\"REDEEM-002\",\"order_amount\":1500,\"payment_method\":\"razorpay\",\"requested_amount\":50,\"discount_codes\":[]}" > /dev/null 2>&1 && echo "  Arjun redeemed ₹50" || echo "  Arjun redemption skipped"

# Rahul redeems ₹200
post /redemptions "{\"wallet_id\":\"${WIDS[3]}\",\"order_id\":\"REDEEM-003\",\"order_amount\":5000,\"payment_method\":\"razorpay\",\"requested_amount\":200,\"discount_codes\":[]}" > /dev/null 2>&1 && echo "  Rahul redeemed ₹200" || echo "  Rahul redemption skipped"

# -----------------------------------------------------------
# 8. Referral program + codes
# -----------------------------------------------------------
echo ""
echo "=== Setting up referral program ==="
post /referrals/programs "{\"merchant_id\":\"$MID\",\"referrer_reward_amount\":50,\"referee_reward_amount\":25,\"max_referrals_per_customer\":10}" > /dev/null
echo "  Referrer: ₹50, Referee: ₹25"

post /referrals/codes "{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\",\"code\":\"PRIYA10\",\"is_vanity\":true,\"is_creator\":false}" > /dev/null 2>&1 && echo "  Code: PRIYA10 (Priya)" || echo "  Code creation skipped"
post /referrals/codes "{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[5]}\",\"code\":\"VIKRAM20\",\"is_vanity\":true,\"is_creator\":true,\"commission_rate\":0.02}" > /dev/null 2>&1 && echo "  Code: VIKRAM20 (Vikram, creator)" || echo "  Code creation skipped"

# -----------------------------------------------------------
# 9. Loyalty program + tiers
# -----------------------------------------------------------
echo ""
echo "=== Setting up loyalty program ==="
LP=$(post /loyalty/programs "{\"merchant_id\":\"$MID\",\"name\":\"Desi Rewards\",\"evaluation_criteria\":\"spend\"}")
LPID=$(echo "$LP" | jq_id "['id']")
echo "  Program: Desi Rewards (spend-based)"

post /loyalty/tiers "{\"program_id\":\"$LPID\",\"name\":\"Bronze\",\"rank\":1,\"threshold\":0,\"earn_rate_multiplier\":1.0,\"benefits\":{\"free_shipping\":false}}" > /dev/null
post /loyalty/tiers "{\"program_id\":\"$LPID\",\"name\":\"Silver\",\"rank\":2,\"threshold\":2000,\"earn_rate_multiplier\":1.25,\"benefits\":{\"free_shipping\":true}}" > /dev/null
post /loyalty/tiers "{\"program_id\":\"$LPID\",\"name\":\"Gold\",\"rank\":3,\"threshold\":5000,\"earn_rate_multiplier\":1.5,\"benefits\":{\"free_shipping\":true,\"priority_support\":true}}" > /dev/null
post /loyalty/tiers "{\"program_id\":\"$LPID\",\"name\":\"Platinum\",\"rank\":4,\"threshold\":15000,\"earn_rate_multiplier\":2.0,\"benefits\":{\"free_shipping\":true,\"priority_support\":true,\"early_access\":true}}" > /dev/null
echo "  Tiers: Bronze → Silver (₹2K) → Gold (₹5K) → Platinum (₹15K)"

# -----------------------------------------------------------
# 10. Wallet policies
# -----------------------------------------------------------
echo ""
echo "=== Setting wallet policies ==="
post /admin/wallet-policies "{\"merchant_id\":\"$MID\",\"bucket_type\":\"earned_credit\",\"min_redemption\":10,\"step_size\":1,\"max_per_order_pct\":50,\"max_per_order_fixed\":500,\"stackable_with_discounts\":true,\"default_expiry_days\":365,\"is_transferable\":false}" > /dev/null
echo "  Earned credit: min ₹10, max 50% or ₹500, stackable, 365d expiry"

post /admin/wallet-policies "{\"merchant_id\":\"$MID\",\"bucket_type\":\"gift_card\",\"min_redemption\":1,\"step_size\":1,\"max_per_order_pct\":100,\"max_per_order_fixed\":null,\"stackable_with_discounts\":true,\"default_expiry_days\":730,\"is_transferable\":true}" > /dev/null
echo "  Gift card: no cap, transferable, 2yr expiry"

# -----------------------------------------------------------
# 11. Notification templates
# -----------------------------------------------------------
echo ""
echo "=== Creating notification templates ==="
post /notifications/templates "{\"merchant_id\":\"$MID\",\"name\":\"earn_credit\",\"channel\":\"whatsapp\",\"locale\":\"en\",\"body_template\":\"Hi {{customer_name}}! You earned {{amount}} on your order #{{order_id}}. Your balance is now {{balance}}.\",\"variables\":[\"customer_name\",\"amount\",\"order_id\",\"balance\"]}" > /dev/null
echo "  earn_credit (WhatsApp)"

post /notifications/templates "{\"merchant_id\":\"$MID\",\"name\":\"redemption_success\",\"channel\":\"whatsapp\",\"locale\":\"en\",\"body_template\":\"Hi {{customer_name}}! You redeemed {{amount}} on order #{{order_id}}. Remaining balance: {{balance}}.\",\"variables\":[\"customer_name\",\"amount\",\"order_id\",\"balance\"]}" > /dev/null
echo "  redemption_success (WhatsApp)"

post /notifications/templates "{\"merchant_id\":\"$MID\",\"name\":\"cod_delivered\",\"channel\":\"whatsapp\",\"locale\":\"en\",\"body_template\":\"Great news {{customer_name}}! Your order #{{order_id}} was delivered. {{amount}} credit is now available to spend!\",\"variables\":[\"customer_name\",\"order_id\",\"amount\"]}" > /dev/null
echo "  cod_delivered (WhatsApp)"

# -----------------------------------------------------------
# 12. Connectors
# -----------------------------------------------------------
echo ""
echo "=== Setting up connectors ==="
post /notifications/connectors "{\"merchant_id\":\"$MID\",\"capability\":\"whatsapp-bsp\",\"vendor\":\"interakt\",\"config\":{\"api_key\":\"demo_key\",\"sender_phone\":\"+919999999999\"},\"priority\":1}" > /dev/null
echo "  WhatsApp BSP → Interakt"

post /notifications/connectors "{\"merchant_id\":\"$MID\",\"capability\":\"sms\",\"vendor\":\"msg91\",\"config\":{\"api_key\":\"demo_key\"},\"priority\":1}" > /dev/null
echo "  SMS → MSG91"

# -----------------------------------------------------------
# 13. Milestones
# -----------------------------------------------------------
echo ""
echo "=== Setting up milestones ==="
post /earn/milestones "{\"merchant_id\":\"$MID\",\"name\":\"3rd Order Bonus\",\"milestone_type\":\"order_count\",\"threshold\":3,\"reward_amount\":50}" > /dev/null
echo "  3rd Order Bonus: 3 orders → ₹50"
post /earn/milestones "{\"merchant_id\":\"$MID\",\"name\":\"₹5K Club\",\"milestone_type\":\"lifetime_spend\",\"threshold\":5000,\"reward_amount\":150}" > /dev/null
echo "  ₹5K Club: ₹5,000 spend → ₹150"
post /earn/milestones "{\"merchant_id\":\"$MID\",\"name\":\"10th Order Celebration\",\"milestone_type\":\"order_count\",\"threshold\":10,\"reward_amount\":500}" > /dev/null
echo "  10th Order: 10 orders → ₹500"

# -----------------------------------------------------------
# 14. Streak configs
# -----------------------------------------------------------
echo ""
echo "=== Setting up streak rewards ==="
post /earn/streaks "{\"merchant_id\":\"$MID\",\"name\":\"3 in 30\",\"required_orders\":3,\"window_days\":30,\"reward_amount\":75}" > /dev/null
echo "  3 orders in 30 days → ₹75"

# -----------------------------------------------------------
# 15. Spin-the-wheel
# -----------------------------------------------------------
echo ""
echo "=== Setting up spin wheel ==="
post /earn/spin-wheel/config "{\"merchant_id\":\"$MID\",\"name\":\"Lucky Spin\",\"daily_spin_limit\":3,\"segments\":[{\"label\":\"₹10 Off\",\"reward_amount\":10,\"probability\":30,\"color\":\"#10b981\"},{\"label\":\"₹25 Off\",\"reward_amount\":25,\"probability\":25,\"color\":\"#3b82f6\"},{\"label\":\"₹50 Off\",\"reward_amount\":50,\"probability\":15,\"color\":\"#7c6aff\"},{\"label\":\"₹100 Off\",\"reward_amount\":100,\"probability\":5,\"color\":\"#f59e0b\"},{\"label\":\"Better luck!\",\"reward_amount\":0,\"probability\":20,\"color\":\"#6b7280\"},{\"label\":\"₹5 Off\",\"reward_amount\":5,\"probability\":5,\"color\":\"#ef4444\"}]}" > /dev/null
echo "  Lucky Spin: 6 segments, 3 spins/day"

# -----------------------------------------------------------
# 16. Membership plans
# -----------------------------------------------------------
echo ""
echo "=== Setting up membership plans ==="
post /earn/memberships/plans "{\"merchant_id\":\"$MID\",\"name\":\"Desi Premium Monthly\",\"plan_type\":\"monthly\",\"price\":99,\"earn_rate_multiplier\":2.0,\"benefits\":{\"free_shipping\":true,\"early_access\":true}}" > /dev/null
echo "  Monthly: ₹99/mo, 2x earn, free shipping"
post /earn/memberships/plans "{\"merchant_id\":\"$MID\",\"name\":\"Desi Premium Annual\",\"plan_type\":\"annual\",\"price\":799,\"earn_rate_multiplier\":2.5,\"benefits\":{\"free_shipping\":true,\"early_access\":true,\"priority_support\":true}}" > /dev/null
echo "  Annual: ₹799/yr, 2.5x earn, priority support"

# -----------------------------------------------------------
# 17. Set birthdays (Priya + Arjun = today, others = various)
# -----------------------------------------------------------
echo ""
echo "=== Setting customer birthdays ==="
psql -U chirag -d batua -c "UPDATE customers SET birthday = CURRENT_DATE WHERE phone = '+919876543210';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = CURRENT_DATE WHERE phone = '+918765432109';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1995-06-15' WHERE phone = '+917654321098';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1990-12-25' WHERE phone = '+916543210987';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1998-08-20' WHERE phone = '+919988776655';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1992-01-10' WHERE phone = '+918877665544';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1997-11-05' WHERE phone = '+917766554433';" > /dev/null 2>&1
psql -U chirag -d batua -c "UPDATE customers SET birthday = '1994-03-22' WHERE phone = '+916655443322';" > /dev/null 2>&1
echo "  Priya + Arjun: birthday = today"
echo "  Others: various dates throughout the year"

# -----------------------------------------------------------
# 18. Second merchant for coalition
# -----------------------------------------------------------
echo ""
echo "=== Creating second merchant for coalition ==="
M2=$(post /admin/merchants '{"external_id":"shop_fresh_fashion","name":"Fresh Fashion","domain":"freshfashion.myshopify.com","slug":"fresh-fashion","currency":"INR","timezone":"Asia/Kolkata"}')
M2ID=$(echo "$M2" | jq_id "['id']")
echo "  Fresh Fashion ($M2ID)"

# Create coalition
post /admin/coalitions "{\"name\":\"Fashion Alliance\",\"merchant_ids\":[\"$MID\",\"$M2ID\"]}" > /dev/null
echo "  Coalition: Fashion Alliance (Desi Threads + Fresh Fashion)"

# Create Priya's wallet at Fresh Fashion
post /wallets/get-or-create "{\"merchant_id\":\"$M2ID\",\"customer_id\":\"${CIDS[0]}\"}" > /dev/null
echo "  Priya has wallets at both merchants"

# -----------------------------------------------------------
# Summary
# -----------------------------------------------------------
echo ""
echo "============================================"
echo "  SEED COMPLETE"
echo "============================================"
echo ""
echo "  Merchant:    Desi Threads ($MID)"
echo "  Customers:   8 (with birthdays set)"
echo "  Orders:      20 (mixed prepaid + COD)"
echo "  Rules:       3 reward rules"
echo "  Loyalty:     4 tiers (Bronze/Silver/Gold/Platinum)"
echo "  Referrals:   2 codes (PRIYA10, VIKRAM20)"
echo "  Milestones:  3 configs (3rd order, ₹5K, 10th order)"
echo "  Streak:      3 in 30 → ₹75"
echo "  Spin wheel:  6 segments, 3/day"
echo "  Memberships: Monthly ₹99, Annual ₹799"
echo "  Coalition:   Fashion Alliance (Desi Threads + Fresh Fashion)"
echo "  Templates:   3 WhatsApp templates"
echo "  Connectors:  2 (Interakt, MSG91)"
echo ""
echo "============================================"
echo "  HOW TO TEST PHASE 6 FEATURES"
echo "============================================"
echo ""
echo "  Birthday bonus (₹100 each):"
echo "    curl -X POST $API/earn/birthday-bonus -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"amount\":100}'"
echo ""
echo "  Milestones (check Priya — 4 orders):"
echo "    curl -X POST $API/earn/check-milestones -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\"}'"
echo ""
echo "  Newsletter signup (₹25):"
echo "    curl -X POST $API/earn/newsletter-signup -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"email\":\"rohit@example.com\",\"phone\":\"6655443322\",\"amount\":25}'"
echo ""
echo "  Profile completion (₹30):"
echo "    curl -X POST $API/earn/profile-completion -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\"}'"
echo ""
echo "  Streak check (Priya — 4 orders in 30 days):"
echo "    curl -X POST $API/earn/check-streaks -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\"}'"
echo ""
echo "  Spin wheel:"
echo "    curl -X POST $API/earn/spin-wheel/spin -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\"}'"
echo ""
echo "  Subscribe to membership:"
echo "    PLAN_ID=\$(curl -s $API/earn/memberships/plans/$MID | python3 -c \"import sys,json; print(json.load(sys.stdin)[0]['id'])\")"
echo "    curl -X POST $API/earn/memberships/subscribe -H 'Content-Type: application/json' -d '{\"merchant_id\":\"$MID\",\"customer_id\":\"${CIDS[0]}\",\"plan_id\":\"'\$PLAN_ID'\"}'"
echo ""
echo "  Coalition transfer (₹50 Desi → Fresh):"
echo "    curl -X POST $API/admin/coalitions/transfer -H 'Content-Type: application/json' -d '{\"customer_id\":\"${CIDS[0]}\",\"from_merchant_id\":\"$MID\",\"to_merchant_id\":\"$M2ID\",\"amount\":50}'"
echo ""
echo "  Admin UI:    http://localhost:5174/admin"
echo "  Storefront:  http://localhost:5174/s/desi-threads"
echo "  Platform:    http://localhost:5174/platform"
echo ""
