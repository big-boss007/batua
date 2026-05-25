# Implementation Checklist

## Setup
- [ ] Install `@playwright/test` as dev dependency
- [ ] Install Chromium browser for Playwright
- [ ] Create `playwright.config.ts` with baseURL, screenshot settings, HTML reporter
- [ ] Create `frontend/e2e/` directory

## E2E Tests — Admin
- [ ] `referral-admin.spec.ts` — Program tab: view settings, update program
- [ ] Codes tab: view code list, create auto code, create vanity code, create creator code
- [ ] Analytics tab: view metrics
- [ ] Conversions tab: view conversion list with fraud signals
- [ ] Edge cases: empty states, form validation

## E2E Tests — Storefront
- [ ] `referral-storefront.spec.ts` — Referrer: phone login → view code → copy link
- [ ] Referee: landing page with valid code → reward display → Shop Now
- [ ] Edge cases: invalid code, no account, no referral code

## E2E Helpers
- [ ] `e2e/helpers.ts` — shared login function, merchant navigation, wait helpers

## API Tests
- [ ] `tests/referral_tests.rs` — POST /referrals/programs (create, duplicate)
- [ ] GET /referrals/programs/{merchant_id} (exists, not found)
- [ ] POST /referrals/codes (auto, vanity, creator, missing fields)
- [ ] GET /referrals/codes/{code} (exists, not found)
- [ ] POST /referrals/convert (happy path, self-referral fraud, max limit, inactive program)
- [ ] GET /referrals/analytics/{merchant_id}
- [ ] GET /referrals/conversions/{merchant_id}

## Verification
- [ ] `npx playwright test` passes
- [ ] `npx playwright show-report` shows HTML report with screenshots
- [ ] `cargo test referral` passes
- [ ] All UAT report scenarios covered
