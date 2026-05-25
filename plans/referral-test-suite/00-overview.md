# Referral Test Suite

## Goal

Set up automated testing for the referral system with two layers:
1. **API integration tests** (Rust) — test all referral endpoints directly
2. **E2E browser tests** (Playwright) — test full user flows with screenshots

Both generate reports. Playwright produces an HTML report with screenshots on every test (pass or fail).

## Scope

**In scope:**
- Playwright setup in `frontend/` (config, install, helpers)
- E2E tests: admin referral flows (program, codes, analytics, conversions tabs)
- E2E tests: storefront referral flows (referrer page, referee landing page)
- API tests: all 8 referral endpoints with happy + error cases
- HTML report with screenshots via `npx playwright show-report`

**Out of scope:**
- Tests for other modules (loyalty, gift cards, etc.) — this is the pattern; others follow later
- CI/CD integration — local-only for now
- Visual regression testing

## Success Criteria

- [ ] `npx playwright test` runs all E2E referral tests
- [ ] `npx playwright show-report` opens HTML report with screenshots
- [ ] `cargo test referral` runs all API tests
- [ ] Tests cover all scenarios from the UAT report (`docs/referral-uat-report.html`)
- [ ] Known bugs from UAT are documented as expected failures

## Dependencies

- Backend running on `:3000`
- Frontend running on `:5174` (via `make dev`)
- PostgreSQL `batua` database with seed data
- Playwright browsers installed (`npx playwright install`)

## Commands

```bash
# Install Playwright (one-time)
cd frontend && npm i -D @playwright/test && npx playwright install chromium

# Run E2E tests
cd frontend && npx playwright test

# Run with browser visible
cd frontend && npx playwright test --headed

# Run single test file
cd frontend && npx playwright test referral-admin

# View HTML report with screenshots
cd frontend && npx playwright show-report

# Run API tests
cargo test referral
```
