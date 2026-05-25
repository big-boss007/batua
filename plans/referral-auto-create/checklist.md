# Implementation Checklist

## Phase 1: Types
- [ ] Add `CodeCreationTrigger` enum to `src/services/referrals/types.rs`
- [ ] Add `code_creation_trigger` field to `ReferralProgram` struct
- [ ] Add field to `CreateProgramRequest` and `UpdateProgramRequest`
- [ ] Add field to frontend `ReferralProgram` type in `types.ts`

## Phase 2: Storage / Migration
- [ ] Create migration: `ALTER TABLE referral_programs ADD COLUMN code_creation_trigger TEXT NOT NULL DEFAULT 'on_registration'`
- [ ] Run migration: `psql -d batua -f migrations/XXXXXX_referral_code_creation_trigger.sql`
- [ ] Update `get_referral_program()` SELECT to include new column
- [ ] Update `create_referral_program()` INSERT to include new column
- [ ] Update `update_referral_program()` UPDATE to include new column
- [ ] Add `get_active_program_trigger()` lightweight query function

## Phase 3: Earn Flow Integration
- [ ] Add auto-creation logic in `do_process_earn()` after customer/wallet resolution
- [ ] Use `is_new` for `OnRegistration` trigger, `is_first_order` for `OnFirstPurchase`
- [ ] Add idempotency check (skip if code already exists)
- [ ] Make it best-effort (log error, don't fail earn flow)
- [ ] Add `#[tracing::instrument]` to any new functions

## Phase 4: Frontend — Wizard & Edit
- [ ] Add trigger radio buttons to referral program wizard/setup
- [ ] Default to `on_registration`
- [ ] Add tradeoff advice text on confirmation screen
- [ ] Add trigger setting to program edit form
- [ ] Update `remote.ts` to send `code_creation_trigger` in create/update calls

## Phase 5: Verification
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `npx svelte-check --threshold error` passes
- [ ] Manual test: create new referral program with `on_registration` trigger
- [ ] Manual test: process an order for a new customer → verify code auto-created
- [ ] Manual test: switch trigger to `on_first_purchase` → verify behaviour changes
- [ ] Manual test: existing customer processes second order → no duplicate code created
