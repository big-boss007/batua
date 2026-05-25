# Integration: Earn Flow + Frontend Wizard

## Objective
Wire up auto-creation in the earn flow, update the wizard UI with the trigger setting and tradeoff advice.

---

## Part 1: Earn Flow Auto-Creation

### File: `src/services/earn/helpers.rs` — `do_process_earn()`

Insert auto-creation logic after the customer and wallet are resolved, before/after earn processing depending on the trigger:

```
// Existing flow:
// 1. Parse event payload
// 2. resolve_or_create() → (customer, is_new)
// 3. get_or_create_wallet()
// 4. Get order_stats → is_first_order = order_stats.is_none()
// 5. Evaluate rules, create ledger entries
// 6. Update order stats

// NEW: Auto-create referral code
// After step 4 (we know is_new and is_first_order):
//
// let should_create = match trigger_setting {
//     OnRegistration => is_new,
//     OnFirstPurchase => is_first_order,
// };
//
// if should_create {
//     // Check idempotency: does code already exist?
//     // If not, generate and create code
// }
```

**Logic:**
```rust
if let Some(trigger) = referrals::storage::get_active_program_trigger(pool, merchant_id).await? {
    let should_create = match trigger {
        CodeCreationTrigger::OnRegistration => is_new,
        CodeCreationTrigger::OnFirstPurchase => is_first_order,
    };

    if should_create {
        let existing = referrals::storage::get_customer_referral_code(pool, merchant_id, customer.id).await?;
        if existing.is_none() {
            let code = referrals::helpers::generate_referral_code(customer.name.as_deref());
            let req = CreateCodeRequest {
                merchant_id,
                customer_id: customer.id,
                code: None,
                is_vanity: customer.name.is_some(),
                is_creator: false,
                commission_rate: None,
            };
            // Best-effort: log error but don't fail the earn flow
            if let Err(e) = referrals::storage::create_referral_code(pool, &req, &code).await {
                tracing::warn!(error = ?e, "failed to auto-create referral code");
            }
        }
    }
}
```

**Key decisions:**
- Best-effort: referral code creation failure should NOT fail the earn flow
- Idempotency: check for existing code before creating
- Uses `is_new` for registration trigger, `is_first_order` for purchase trigger

---

## Part 2: Frontend — Wizard Setup

### File: `frontend/src/routes/admin/referrals/+page.svelte`

**In the program setup wizard (or edit form), add a setting:**

```svelte
<div class="trigger-setting">
  <h4>When should referral codes be created?</h4>
  <div class="trigger-options">
    <label class="trigger-option">
      <input type="radio" name="trigger" value="on_registration" bind:group={codeCreationTrigger} />
      <div>
        <strong>On registration</strong>
        <span class="trigger-desc">Code created when customer places any order (recommended)</span>
      </div>
    </label>
    <label class="trigger-option">
      <input type="radio" name="trigger" value="on_first_purchase" bind:group={codeCreationTrigger} />
      <div>
        <strong>On first purchase</strong>
        <span class="trigger-desc">Code created only after first successful order</span>
      </div>
    </label>
  </div>
</div>
```

**Default:** `on_registration`

---

## Part 3: Confirmation Screen — Tradeoff Advice

**In the wizard confirmation/review step, show context-aware advice:**

```svelte
<div class="trigger-advice">
  {#if codeCreationTrigger === 'on_registration'}
    <div class="advice-card">
      <strong>On Registration</strong> — Every new customer gets a referral code immediately.
      This maximises referral reach since customers can share their code right away.
      Some codes may go unused if customers don't engage further.
    </div>
  {:else}
    <div class="advice-card">
      <strong>On First Purchase</strong> — Codes are created only after a customer's first order.
      This ensures only active buyers get codes, but means customers can't refer
      friends until they've made a purchase themselves.
    </div>
  {/if}
</div>
```

---

## Part 4: Edit After Setup

The trigger setting should also appear in the program edit form (same component/page), so merchants can change it at any time. Changing it only affects future customers — existing codes are not deleted.
