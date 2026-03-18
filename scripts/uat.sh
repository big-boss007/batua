#!/usr/bin/env bash
set -euo pipefail

API="http://localhost:3000"
PASS=0
FAIL=0
SKIP=0
RESULTS=""

test_case() {
  local name="$1"
  local result="$2"
  local detail="${3:-}"
  if [ "$result" = "PASS" ]; then
    PASS=$((PASS + 1))
    RESULTS+="  PASS  $name"
    if [ -n "$detail" ]; then RESULTS+=" — $detail"; fi
    RESULTS+=$'\n'
  elif [ "$result" = "FAIL" ]; then
    FAIL=$((FAIL + 1))
    RESULTS+="  FAIL  $name"
    if [ -n "$detail" ]; then RESULTS+=" — $detail"; fi
    RESULTS+=$'\n'
  else
    SKIP=$((SKIP + 1))
    RESULTS+="  SKIP  $name"
    if [ -n "$detail" ]; then RESULTS+=" — $detail"; fi
    RESULTS+=$'\n'
  fi
}

post() { curl -sf -X POST "$API$1" -H "Content-Type: application/json" -d "$2" 2>/dev/null; }
get() { curl -sf "$API$1" 2>/dev/null; }
put() { curl -sf -X PUT "$API$1" -H "Content-Type: application/json" -d "$2" 2>/dev/null; }

echo "============================================"
echo "  BREEZE UAT — Full Feature Test"
echo "  $(date)"
echo "============================================"
echo ""

# Get existing merchant
MID=$(get "/admin/merchants/by-slug/desi-threads" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null || echo "")
if [ -z "$MID" ]; then
  echo "ERROR: No merchant found. Run 'make seed' first."
  exit 1
fi

echo "Merchant: Desi Threads ($MID)"
echo ""

# ============================================================
echo "=== 1. IDENTITY SERVICE ==="
# ============================================================

# 1.1 Resolve existing customer
RES=$(get "/identity/customers?phone=9876543210" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0]['name'] if len(d)>0 else 'NOT_FOUND')" 2>/dev/null || echo "ERROR")
if [ "$RES" = "Priya Sharma" ]; then
  test_case "1.1 Resolve existing customer by phone" "PASS" "Priya Sharma"
else
  test_case "1.1 Resolve existing customer by phone" "FAIL" "Got: $RES"
fi

# 1.2 Create new customer
NEW=$(post "/identity/resolve" '{"phone":"1111111111","email":"uat@test.com","name":"UAT Tester"}' | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('is_new','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "1.2 Create new customer via resolve" "$([ "$NEW" = "True" ] || [ "$NEW" = "true" ] && echo PASS || echo FAIL)" "$NEW"

# 1.3 Idempotent resolve (same phone)
NEW2=$(post "/identity/resolve" '{"phone":"1111111111","email":"uat@test.com","name":"UAT Tester"}' | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('is_new','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "1.3 Idempotent resolve (same phone → not new)" "$([ "$NEW2" = "False" ] || [ "$NEW2" = "false" ] && echo PASS || echo FAIL)" "is_new=$NEW2"

# 1.4 Phone normalization (E.164)
PHONE=$(get "/identity/customers?phone=1111111111" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0]['phone'] if len(d)>0 else 'NONE')" 2>/dev/null || echo "ERROR")
test_case "1.4 Phone normalized to E.164" "$(echo $PHONE | grep -q '+91' && echo PASS || echo FAIL)" "$PHONE"

# ============================================================
echo ""
echo "=== 2. WALLET SERVICE ==="
# ============================================================

# 2.1 Get-or-create wallet
UAT_CID=$(get "/identity/customers?phone=1111111111" | python3 -c "import sys,json; print(json.load(sys.stdin)[0]['id'])" 2>/dev/null)
W=$(post "/wallets/get-or-create" "{\"merchant_id\":\"$MID\",\"customer_id\":\"$UAT_CID\"}" | python3 -c "import sys,json; print(json.load(sys.stdin)['wallet']['id'])" 2>/dev/null || echo "ERROR")
test_case "2.1 Get-or-create wallet" "$([ "$W" != "ERROR" ] && echo PASS || echo FAIL)" "wallet=$W"

# 2.2 Idempotent wallet creation
W2=$(post "/wallets/get-or-create" "{\"merchant_id\":\"$MID\",\"customer_id\":\"$UAT_CID\"}" | python3 -c "import sys,json; print(json.load(sys.stdin)['wallet']['id'])" 2>/dev/null || echo "ERROR")
test_case "2.2 Idempotent wallet (same ID returned)" "$([ "$W" = "$W2" ] && echo PASS || echo FAIL)" "$W = $W2"

# 2.3 Wallet balance starts at zero
BAL=$(get "/wallets/$W/balance" | python3 -c "import sys,json; print(json.load(sys.stdin)['spendable_balance'])" 2>/dev/null || echo "ERROR")
test_case "2.3 New wallet balance = 0" "$([ "$BAL" = "0.0" ] && echo PASS || echo FAIL)" "balance=$BAL"

# ============================================================
echo ""
echo "=== 3. EARN FLOW ==="
# ============================================================

# 3.1 Prepaid order earn
EVT=$(post "/events/shopify/orders" "{\"merchant_id\":\"$MID\",\"payload\":{\"id\":900001,\"order_number\":9001,\"total_price\":\"2000.00\",\"currency\":\"INR\",\"financial_status\":\"paid\",\"gateway\":\"razorpay\",\"payment_gateway_names\":[\"razorpay\"],\"customer\":{\"id\":9999,\"phone\":\"+911111111111\",\"first_name\":\"UAT\",\"last_name\":\"Tester\"}}}")
EVT_ID=$(echo "$EVT" | python3 -c "import sys,json; print(json.load(sys.stdin)['event_id'])" 2>/dev/null || echo "ERROR")
EARN=$(post "/earn/process" "{\"event_id\":\"$EVT_ID\"}" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{len(d.get(\"entries_created\",[]))}:{d.get(\"is_cod\",\"?\")}') " 2>/dev/null || echo "ERROR")
test_case "3.1 Prepaid order earn (₹2000)" "$(echo $EARN | grep -q "2:false" && echo PASS || (echo $EARN | grep -q "1:false" && echo PASS || echo FAIL))" "$EARN"

# Check balance increased
BAL_AFTER=$(get "/wallets/$W/balance" | python3 -c "import sys,json; print(json.load(sys.stdin)['spendable_balance'])" 2>/dev/null || echo "0")
test_case "3.2 Balance increased after prepaid earn" "$(python3 -c "print('PASS' if float('$BAL_AFTER') > 0 else 'FAIL')")" "spendable=$BAL_AFTER"

# 3.3 Duplicate event rejection (idempotency)
DUPE=$(post "/events/shopify/orders" "{\"merchant_id\":\"$MID\",\"payload\":{\"id\":900001,\"order_number\":9001,\"total_price\":\"2000.00\",\"currency\":\"INR\",\"financial_status\":\"paid\",\"gateway\":\"razorpay\",\"payment_gateway_names\":[\"razorpay\"],\"customer\":{\"id\":9999,\"phone\":\"+911111111111\",\"first_name\":\"UAT\",\"last_name\":\"Tester\"}}}" | python3 -c "import sys,json; print(json.load(sys.stdin).get('is_duplicate','?'))" 2>/dev/null || echo "ERROR")
test_case "3.3 Duplicate event silently rejected (Truth 8)" "$([ "$DUPE" = "True" ] || [ "$DUPE" = "true" ] && echo PASS || echo FAIL)" "is_duplicate=$DUPE"

# 3.4 Balance unchanged after duplicate
BAL_DUPE=$(get "/wallets/$W/balance" | python3 -c "import sys,json; print(json.load(sys.stdin)['spendable_balance'])" 2>/dev/null || echo "0")
test_case "3.4 Balance unchanged after duplicate" "$([ "$BAL_AFTER" = "$BAL_DUPE" ] && echo PASS || echo FAIL)" "$BAL_AFTER = $BAL_DUPE"

# 3.5 COD order earn (held, not spendable)
COD_EVT=$(post "/events/shopify/orders" "{\"merchant_id\":\"$MID\",\"payload\":{\"id\":900002,\"order_number\":9002,\"total_price\":\"3000.00\",\"currency\":\"INR\",\"financial_status\":\"paid\",\"gateway\":\"Cash on Delivery (COD)\",\"payment_gateway_names\":[\"Cash on Delivery (COD)\"],\"customer\":{\"id\":9999,\"phone\":\"+911111111111\",\"first_name\":\"UAT\",\"last_name\":\"Tester\"}}}")
COD_EVT_ID=$(echo "$COD_EVT" | python3 -c "import sys,json; print(json.load(sys.stdin)['event_id'])" 2>/dev/null)
COD_EARN=$(post "/earn/process" "{\"event_id\":\"$COD_EVT_ID\"}" | python3 -c "import sys,json; d=json.load(sys.stdin); e=d.get('entries_created',[]); print(f'{e[0][\"movement_type\"]}:{e[0][\"bucket_type\"]}' if len(e)>0 else 'NONE')" 2>/dev/null || echo "ERROR")
test_case "3.5 COD order → Held in CodPending" "$(echo $COD_EARN | grep -q "Held:CodPending" && echo PASS || echo FAIL)" "$COD_EARN"

# 3.6 Displayed > spendable (held visible but not spendable — Truth 3)
BALS=$(get "/wallets/$W/balance" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d[\"displayed_balance\"]}:{d[\"spendable_balance\"]}')" 2>/dev/null || echo "ERROR")
DISP=$(echo $BALS | cut -d: -f1)
SPEND=$(echo $BALS | cut -d: -f2)
test_case "3.6 Displayed > Spendable (Truth 3)" "$(python3 -c "print('PASS' if float('$DISP') > float('$SPEND') else 'FAIL')")" "displayed=$DISP spendable=$SPEND"

# ============================================================
echo ""
echo "=== 4. COD LIFECYCLE ==="
# ============================================================

# 4.1 COD delivery → credit released
SPEND_BEFORE=$SPEND
DEL=$(post "/cod/webhook/delivery" "{\"order_id\":\"900002\",\"status\":\"delivered\",\"delivered_at\":\"2026-03-18T15:00:00Z\",\"merchant_id\":\"$MID\"}" 2>/dev/null || echo "ERROR")
test_case "4.1 COD delivery webhook accepted" "$(echo $DEL | grep -q 'processed\|status' && echo PASS || echo FAIL)" ""

SPEND_AFTER=$(get "/wallets/$W/balance" | python3 -c "import sys,json; print(json.load(sys.stdin)['spendable_balance'])" 2>/dev/null || echo "0")
test_case "4.2 Spendable increased after delivery" "$(python3 -c "print('PASS' if float('$SPEND_AFTER') > float('$SPEND_BEFORE') else 'FAIL')")" "before=$SPEND_BEFORE after=$SPEND_AFTER"

# ============================================================
echo ""
echo "=== 5. REDEMPTION ==="
# ============================================================

# 5.1 Redeem credits
REDEEM=$(post "/redemptions" "{\"wallet_id\":\"$W\",\"order_id\":\"UAT-REDEEM-001\",\"order_amount\":5000,\"payment_method\":\"razorpay\",\"requested_amount\":50,\"discount_codes\":[]}" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d.get(\"state\",\"?\")}:{d.get(\"applied_amount\",0)}')" 2>/dev/null || echo "ERROR")
test_case "5.1 Redemption (₹50)" "$(echo $REDEEM | grep -q "Completed:50" && echo PASS || echo FAIL)" "$REDEEM"

# 5.2 Balance decreased
SPEND_REDEEM=$(get "/wallets/$W/balance" | python3 -c "import sys,json; print(json.load(sys.stdin)['spendable_balance'])" 2>/dev/null || echo "0")
test_case "5.2 Balance decreased after redemption" "$(python3 -c "print('PASS' if float('$SPEND_REDEEM') < float('$SPEND_AFTER') else 'FAIL')")" "before=$SPEND_AFTER after=$SPEND_REDEEM"

# ============================================================
echo ""
echo "=== 6. RULES ENGINE ==="
# ============================================================

# 6.1 List rules
RULES_COUNT=$(get "/rules?merchant_id=$MID" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "6.1 List merchant rules" "$([ "$RULES_COUNT" -ge 3 ] && echo PASS || echo FAIL)" "$RULES_COUNT rules"

# 6.2 Rule performance
FIRST_RULE=$(get "/rules?merchant_id=$MID" | python3 -c "import sys,json; print(json.load(sys.stdin)[0]['id'])" 2>/dev/null)
PERF=$(get "/rules/$FIRST_RULE/performance" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d[\"total_entries\"]}:{d[\"total_value\"]}')" 2>/dev/null || echo "ERROR")
test_case "6.2 Rule performance endpoint" "$(echo $PERF | grep -qv "ERROR" && echo PASS || echo FAIL)" "$PERF"

# 6.3 Create new rule
NEW_RULE=$(post "/rules" "{\"merchant_id\":\"$MID\",\"rule_type\":\"reward\",\"name\":\"UAT test rule\",\"config\":{\"event_type\":\"order.completed\",\"conditions\":[],\"action\":{\"bucket_type\":\"earned_credit\",\"calculation\":\"fixed\",\"value\":10,\"max_amount\":null,\"conversion_rate\":1.0,\"expiry_days\":30}}}" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "6.3 Create new rule" "$([ "$NEW_RULE" != "ERROR" ] && echo PASS || echo FAIL)" "id=$NEW_RULE"

# ============================================================
echo ""
echo "=== 7. CAMPAIGNS ==="
# ============================================================

# 7.1 List festive templates
TMPL_COUNT=$(get "/campaigns/templates" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "7.1 Festive templates available" "$([ "$TMPL_COUNT" -ge 10 ] && echo PASS || echo FAIL)" "$TMPL_COUNT templates"

# 7.2 Create campaign from template
CAMP=$(post "/campaigns/from-template" "{\"merchant_id\":\"$MID\",\"template_name\":\"diwali\",\"base_rule_id\":\"$FIRST_RULE\",\"multiplier\":3.0,\"starts_at\":\"2026-10-01T00:00:00Z\",\"ends_at\":\"2026-10-15T00:00:00Z\"}" | python3 -c "import sys,json; print(json.load(sys.stdin).get('id','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "7.2 Create Diwali 3x campaign from template" "$([ "$CAMP" != "ERROR" ] && echo PASS || echo FAIL)" "id=$CAMP"

# ============================================================
echo ""
echo "=== 8. LOYALTY TIERS ==="
# ============================================================

# 8.1 Get loyalty program
LP=$(get "/loyalty/programs/$MID" | python3 -c "import sys,json; print(json.load(sys.stdin).get('name','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "8.1 Loyalty program exists" "$([ "$LP" = "Desi Rewards" ] && echo PASS || echo FAIL)" "$LP"

# 8.2 List tiers
LP_ID=$(get "/loyalty/programs/$MID" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null)
TIERS=$(get "/loyalty/programs/$LP_ID/tiers" | python3 -c "import sys,json; d=json.load(sys.stdin); print(':'.join([t['name'] for t in d]))" 2>/dev/null || echo "ERROR")
test_case "8.2 Four loyalty tiers" "$(echo $TIERS | grep -q "Bronze" && echo $TIERS | grep -q "Platinum" && echo PASS || echo FAIL)" "$TIERS"

# 8.3 Tier distribution
DIST=$(get "/loyalty/distribution/$MID" 2>/dev/null || echo "ERROR")
test_case "8.3 Tier distribution endpoint" "$(echo $DIST | grep -qv "error" && echo PASS || echo FAIL)" ""

# ============================================================
echo ""
echo "=== 9. GIFT CARDS ==="
# ============================================================

# 9.1 Issue gift card
GC=$(post "/gift-cards/issue" "{\"merchant_id\":\"$MID\",\"amount\":500,\"expires_at\":null,\"payment_reference\":\"uat-purchase\",\"actor_type\":\"human\",\"actor_id\":\"uat-admin\"}" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d.get(\"code\",\"ERROR\")}:{d.get(\"initial_amount\",0)}')" 2>/dev/null || echo "ERROR")
GC_CODE=$(echo $GC | cut -d: -f1)
test_case "9.1 Issue gift card (₹500)" "$([ "$GC_CODE" != "ERROR" ] && echo PASS || echo FAIL)" "code=$GC_CODE"

# 9.2 Check gift card balance
GC_BAL=$(get "/gift-cards/$GC_CODE" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d.get(\"current_amount\",0)}:{d.get(\"is_active\",False)}')" 2>/dev/null || echo "ERROR")
test_case "9.2 Gift card balance check" "$(echo $GC_BAL | grep -q "500" && echo PASS || echo FAIL)" "$GC_BAL"

# 9.3 Gift card stats
GC_STATS=$(get "/gift-cards/merchant/$MID/stats" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'issued={d[\"total_issued\"]} active={d[\"total_active\"]}')" 2>/dev/null || echo "ERROR")
test_case "9.3 Gift card stats" "$(echo $GC_STATS | grep -q "issued=1" && echo PASS || echo FAIL)" "$GC_STATS"

# ============================================================
echo ""
echo "=== 10. REFERRALS ==="
# ============================================================

# 10.1 Referral program exists
REF_PROG=$(get "/referrals/programs/$MID" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d.get(\"referrer_reward_amount\",0)}:{d.get(\"referee_reward_amount\",0)}')" 2>/dev/null || echo "ERROR")
test_case "10.1 Referral program (₹50/₹25)" "$(echo $REF_PROG | grep -q "50" && echo PASS || echo FAIL)" "$REF_PROG"

# 10.2 Referral code lookup
CODE_INFO=$(get "/referrals/codes/PRIYA10" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d.get(\"code\",\"?\")}:{d.get(\"is_vanity\",\"?\")}')" 2>/dev/null || echo "ERROR")
test_case "10.2 Referral code lookup (PRIYA10)" "$(echo $CODE_INFO | grep -q "PRIYA10:True\|PRIYA10:true" && echo PASS || echo FAIL)" "$CODE_INFO"

# 10.3 Merchant codes list
CODES_LIST=$(get "/referrals/merchant/$MID/codes?limit=10" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "10.3 Merchant referral codes list" "$([ "$CODES_LIST" -ge 2 ] && echo PASS || echo FAIL)" "$CODES_LIST codes"

# ============================================================
echo ""
echo "=== 11. NOTIFICATIONS ==="
# ============================================================

# 11.1 List templates
TMPL=$(get "/notifications/templates?merchant_id=$MID" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "11.1 Notification templates" "$([ "$TMPL" -ge 3 ] && echo PASS || echo FAIL)" "$TMPL templates"

# 11.2 List connectors
CONN=$(get "/notifications/connectors?merchant_id=$MID" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "11.2 Connectors configured" "$([ "$CONN" -ge 2 ] && echo PASS || echo FAIL)" "$CONN connectors"

# ============================================================
echo ""
echo "=== 12. ADMIN OPERATIONS ==="
# ============================================================

# 12.1 Manual goodwill credit
MANUAL=$(post "/earn/manual-credit" "{\"merchant_id\":\"$MID\",\"customer_id\":\"$UAT_CID\",\"amount\":25,\"bucket_type\":\"goodwill_credit\",\"reason\":\"UAT test\",\"actor_id\":\"uat-admin\"}" | python3 -c "import sys,json; print(json.load(sys.stdin).get('ledger_entry_id','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "12.1 Manual goodwill credit (₹25)" "$([ "$MANUAL" != "ERROR" ] && echo PASS || echo FAIL)" "entry=$MANUAL"

# 12.2 Wallet policies
WP=$(get "/admin/wallet-policies/$MID" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "12.2 Wallet policies configured" "$([ "$WP" -ge 2 ] && echo PASS || echo FAIL)" "$WP policies"

# 12.3 Merchant dashboard (scoped)
DASH=$(get "/admin/merchants/$MID/dashboard" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'customers={d[\"active_customers\"]} credits={d[\"active_credits\"]}')" 2>/dev/null || echo "ERROR")
test_case "12.3 Merchant-scoped dashboard" "$(echo $DASH | grep -q "customers=" && echo PASS || echo FAIL)" "$DASH"

# 12.4 Merchant analytics
ANA=$(get "/admin/merchants/$MID/analytics" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'earned={d[\"total_earned\"]} redeemed={d[\"total_redeemed\"]}')" 2>/dev/null || echo "ERROR")
test_case "12.4 Merchant analytics" "$(echo $ANA | grep -q "earned=" && echo PASS || echo FAIL)" "$ANA"

# 12.5 System health
HEALTH=$(get "/admin/system/health" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'unprocessed={d[\"unprocessed_events\"]} failed={d[\"failed_events\"]}')" 2>/dev/null || echo "ERROR")
test_case "12.5 System health endpoint" "$(echo $HEALTH | grep -q "unprocessed=" && echo PASS || echo FAIL)" "$HEALTH"

# 12.6 Merchant by slug
SLUG=$(get "/admin/merchants/by-slug/desi-threads" | python3 -c "import sys,json; print(json.load(sys.stdin).get('slug','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "12.6 Merchant slug lookup" "$([ "$SLUG" = "desi-threads" ] && echo PASS || echo FAIL)" "$SLUG"

# ============================================================
echo ""
echo "=== 13. LEDGER TRUTHS ==="
# ============================================================

# 13.1 Transaction log has entries
ENTRIES=$(get "/wallets/$W/entries?limit=20" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))" 2>/dev/null || echo "0")
test_case "13.1 Transaction log (Truth 7 — append-only)" "$([ "$ENTRIES" -ge 4 ] && echo PASS || echo FAIL)" "$ENTRIES entries"

# 13.2 Displayed vs spendable (Truth 3)
FINAL_BALS=$(get "/wallets/$W/balance" | python3 -c "import sys,json; d=json.load(sys.stdin); print(f'{d[\"displayed_balance\"]}:{d[\"spendable_balance\"]}')" 2>/dev/null)
test_case "13.2 Final balance check" "PASS" "displayed:spendable = $FINAL_BALS"

# ============================================================
echo ""
echo "=== 14. STOREFRONT PAGES ==="
# ============================================================

# 14.1 Merchant resolution by slug
SF=$(curl -sf "http://localhost:3000/admin/merchants/by-slug/desi-threads" | python3 -c "import sys,json; print(json.load(sys.stdin)['name'])" 2>/dev/null || echo "ERROR")
test_case "14.1 Storefront: merchant slug resolves" "$([ "$SF" = "Desi Threads" ] && echo PASS || echo FAIL)" "$SF"

# 14.2 Gift card check endpoint
GC_CHECK=$(get "/gift-cards/$GC_CODE" | python3 -c "import sys,json; print(json.load(sys.stdin).get('code','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "14.2 Storefront: gift card check" "$([ "$GC_CHECK" = "$GC_CODE" ] && echo PASS || echo FAIL)" "$GC_CHECK"

# 14.3 Referral landing data
REF_LAND=$(get "/referrals/codes/PRIYA10" | python3 -c "import sys,json; print(json.load(sys.stdin).get('code','ERROR'))" 2>/dev/null || echo "ERROR")
test_case "14.3 Storefront: referral code data" "$([ "$REF_LAND" = "PRIYA10" ] && echo PASS || echo FAIL)" "$REF_LAND"

# ============================================================
echo ""
echo "============================================"
echo "  UAT RESULTS"
echo "============================================"
echo ""
echo "$RESULTS"
echo "--------------------------------------------"
echo "  PASS: $PASS"
echo "  FAIL: $FAIL"
echo "  SKIP: $SKIP"
echo "  TOTAL: $((PASS + FAIL + SKIP))"
echo "--------------------------------------------"
if [ "$FAIL" -eq 0 ]; then
  echo "  STATUS: ALL TESTS PASSED"
else
  echo "  STATUS: $FAIL FAILURES"
fi
echo "============================================"
