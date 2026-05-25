# UAT Report Generation — Prompt Template

Use this prompt with Claude Code to generate a comprehensive UAT report with screenshots for any feature module in the Batua project.

**CRITICAL RULE: Screenshot everything. Every single state, every page, every interaction, every error, every empty state, every before/after. A human reviewer will use these screenshots to validate your findings. If there is no screenshot, the test case does not count. When in doubt, take MORE screenshots, not fewer.**

---

## Output Directory Convention

Every UAT run creates a NEW folder under `tests/`. The folder name is `UAT Reports N` where N is an auto-incrementing integer.

**Before you start, determine N:**
```bash
# Find the highest existing N, then add 1
ls -d tests/UAT\ Reports* 2>/dev/null | grep -oP '\d+' | sort -n | tail -1
# If no folders exist, N = 1. Otherwise N = highest + 1.
```

**Directory structure for each run:**
```
tests/
  UAT Reports N/
    [feature]-uat-report.html          ← the HTML report
    [feature]-uat/                     ← screenshot directory
      01-admin-page-load-light.png
      02-admin-page-load-dark.png
      ...
```

All file paths in the HTML report use RELATIVE paths from the report's location:
```html
<!-- The report is at: tests/UAT Reports N/[feature]-uat-report.html -->
<!-- Screenshots are at: tests/UAT Reports N/[feature]-uat/01-xxx.png -->
<!-- So the img src is: -->
<img src="[feature]-uat/01-xxx.png" alt="...">
```

**IMPORTANT:** Never overwrite a previous UAT Reports folder. Always create a new one with the next N. This preserves history so you can compare results across runs.

---

## Quick Reference — Feature → Service → Route Mapping

Use this to identify which features, API endpoints, and UI routes are in scope for each UAT report.

| Feature | Backend Service(s) | Admin Route(s) | Storefront Route(s) | Key DB Tables |
|---------|-------------------|-----------------|---------------------|---------------|
| Wallet & Transactions | `wallets`, `ledger` | `/admin`, `/admin/transactions`, `/admin/wallet-policies` | `/s/{slug}` (balance section), `/s/{slug}/balance` | `wallets`, `ledger_entries`, `wallet_policies` |
| Earn Rules | `rules`, `earn` | `/admin/rules` | — | `earn_rules`, `rule_conditions` |
| Loyalty & Tiers | `loyalty` | `/admin/loyalty` | `/s/{slug}` (tier display) | `loyalty_programs`, `loyalty_tiers`, `customer_tiers` |
| Memberships | `loyalty` (memberships) | `/admin/memberships` | `/s/{slug}` (membership display) | `memberships`, `membership_subscribers` |
| Campaigns | `campaigns` | `/admin/campaigns` | — | `campaign_templates`, `campaigns` |
| Customers | `identity` | `/admin/customers` | — | `customers` |
| Gift Cards | `gift_cards` | `/admin/gift-cards` | `/s/{slug}/gift-cards/check` | `gift_cards` |
| Referrals | `referrals` | `/admin/referrals`, `/admin/influencers` | `/s/{slug}/refer`, `/s/{slug}/refer/[code]` | `referral_programs`, `referral_codes`, `referral_conversions` |
| Notifications | `notifications` | `/admin/notifications` | — | `notification_templates`, `notification_connectors`, `notification_logs` |
| Settings | `admin` | `/admin/settings` | — | `merchants` |
| COD | `cod` | — (webhook-driven) | — | `cod_incentives` |
| Earn Mechanics | `earn` | — (configured via API) | `/s/{slug}` (spin wheel, streaks) | `milestones`, `streaks`, `spin_wheel_configs`, `newsletter_signups` |
| Redemption | `redemption` | — (triggered at checkout) | — | `redemptions` |
| Events | `events` | — (webhook ingestion) | — | `events` |
| Platform Admin | `admin` | `/platform/*` | — | `merchants`, `coalitions` |

---

## The Prompt

```
Create a comprehensive UAT report for the [FEATURE NAME] system.

First, determine the next UAT Reports folder number N (see "Output Directory Convention" above). Create the folder `tests/UAT Reports N/`. The HTML report goes at `tests/UAT Reports N/[feature]-uat-report.html` with all screenshots in `tests/UAT Reports N/[feature]-uat/`.

IMPORTANT RULES FOR THE AGENT:
- You MUST screenshot EVERY state you encounter. Not just the final state — the BEFORE state, the DURING state, and the AFTER state. A human will review these screenshots to validate your work.
- You MUST check console errors AND network requests for EVERY page load and EVERY interaction. Do not skip this.
- You MUST verify data integrity: what the UI shows must match what the database has. Run SQL queries to confirm.
- You MUST test BOTH light and dark mode for every page (use ThemeSwitcher toggle).
- You MUST NOT skip a test case because "it's obvious" or "it's the same as another". Test it. Screenshot it.
- You MUST NOT mark something as PASS without a screenshot proving it passes.
- If something looks wrong but you're not 100% sure, mark it as BUG with severity LOW and explain your uncertainty. Let the human decide.
- If a page takes more than 3 seconds to load, note it as a performance concern.


### Step 1: Discover Test Data

Run SQL queries against the `batua` database to find REAL test data. Do NOT use fake/hardcoded values. Do NOT proceed until you have real data to work with.

```sql
-- ALWAYS run these first to establish context
SELECT id, name, slug, shopify_domain, plan FROM merchants LIMIT 10;
SELECT id, phone, name, email, merchant_id FROM customers LIMIT 20;
SELECT id, customer_id, merchant_id FROM wallets LIMIT 10;
```

Then run feature-specific queries. Adapt these to the feature being tested:

```sql
-- Example for referrals
SELECT * FROM referral_programs WHERE merchant_id = '...' LIMIT 5;
SELECT * FROM referral_codes WHERE merchant_id = '...' LIMIT 10;
SELECT * FROM referral_conversions ORDER BY created_at DESC LIMIT 10;

-- Example for gift cards
SELECT * FROM gift_cards WHERE merchant_id = '...' ORDER BY created_at DESC LIMIT 10;

-- Example for loyalty
SELECT * FROM loyalty_programs WHERE merchant_id = '...' LIMIT 5;
SELECT * FROM loyalty_tiers WHERE program_id = '...' ORDER BY min_points ASC;
SELECT * FROM customer_tiers WHERE merchant_id = '...' LIMIT 10;
```

You MUST find and document:
- [ ] At least one merchant with the feature configured (record the merchant ID, name, and slug)
- [ ] At least 3 customers with wallets for that merchant (record phone numbers — you need these for storefront login)
- [ ] Feature-specific records: codes, cards, entries, programs, etc. (record IDs and key fields)
- [ ] Edge case data: expired records, zero-balance wallets, cancelled memberships, etc.
- [ ] A customer with NO data for this feature (to test empty states)
- [ ] Record counts: how many total records exist (for pagination testing)

Write down ALL discovered test data in a scratchpad before proceeding. You will reference this data throughout testing.


### Step 2: Organize Test Scenarios by Stakeholder

Group ALL test cases by who uses the feature. Be exhaustive — it is better to have too many test cases than too few.

**Stakeholder 1: Merchant Admin (`/admin/[feature]`)**

Test these categories IN ORDER:

A. **Page Load & Initial State**
   - Does the page load without console errors?
   - Does the page load without failed network requests?
   - What is the initial visual state? (screenshot it)
   - Are all expected UI elements present? (tabs, buttons, tables, filters, search)
   - Is the URL correct? Do query params persist on refresh?

B. **List Views & Tables**
   - Does the table show data? Screenshot it.
   - Are all columns present with correct headers?
   - Is data formatted correctly? (dates, currency, phone numbers, status badges)
   - Does pagination work? Test: first page, next page, last page, page with 1 item
   - Does search work? Test: valid search, partial match, no results
   - Do filters work? Test: each filter option, combined filters, clear filters
   - Do sort headers work? Test: ascending, descending, default sort
   - Screenshot the empty state (filter to get zero results, or use a merchant with no data)

C. **Detail Views / Panels**
   - Click a row — does a detail panel/page open? Screenshot it.
   - Are all fields present and correctly populated?
   - Does the close/back button work?
   - Does the URL update when opening a detail? Does it work as a direct link?

D. **Create / Add Forms**
   - Screenshot the form in its initial empty state
   - Test validation: submit with empty required fields — what error appears? Screenshot it.
   - Test validation: submit with invalid data (wrong format, too long, negative numbers) — screenshot each error
   - Fill in valid data — screenshot the filled form BEFORE submitting
   - Submit — screenshot the success state
   - Verify the new record appears in the list
   - Verify the new record exists in the database (run a SQL query)
   - Test double-submit: click submit twice quickly — does it create duplicates?

E. **Edit / Update Forms**
   - Open an existing record for editing — screenshot the pre-filled form
   - Change a field — screenshot before saving
   - Save — screenshot the success state
   - Verify the change persists after page refresh
   - Verify the change in the database

F. **Delete / Cancel / Deactivate Actions**
   - If a delete/cancel button exists, screenshot the confirmation dialog
   - Confirm the action — screenshot the result
   - Verify the record is gone/updated in the list and database

G. **Tab Navigation** (if the page has tabs)
   - Screenshot EACH tab
   - Verify the URL changes when switching tabs (e.g., `?tab=codes`)
   - Verify the correct tab is active after a page refresh with the tab in the URL
   - Verify tab content loads correctly (not stale data from another tab)

H. **Dark Mode**
   - Toggle to dark mode using the ThemeSwitcher in the top bar
   - Screenshot the page in dark mode
   - Check for: unreadable text, missing borders, invisible elements, wrong background colors
   - Toggle back to light mode — verify it restores correctly

**Stakeholder 2: Platform Super-Admin (`/platform/[feature]`)** (if applicable)

Most features have admin-only routes. But if the feature has platform-level management:
- Cross-merchant views: can the super-admin see data across merchants?
- Merchant switching: does changing merchant context update the data?
- System-level controls: health checks, bulk operations, coalition management

**Stakeholder 3: Customer Storefront (`/s/{slug}/[feature]`)**

Test these categories IN ORDER:

A. **Unauthenticated State**
   - Navigate to the storefront URL without logging in
   - Screenshot the phone input / login screen
   - Enter an invalid phone number — screenshot the error
   - Enter a phone number with no account — screenshot what happens

B. **Authenticated Happy Path**
   - Enter a valid phone number (from your test data)
   - Screenshot the OTP/verification step (if any)
   - Screenshot the main feature view after login
   - Verify data matches what the database shows (run a SQL query to confirm)

C. **Feature-Specific Interactions**
   - Test every button, link, and interactive element on the page
   - For each interaction: screenshot BEFORE clicking, screenshot AFTER clicking
   - Test the full flow end-to-end (e.g., refer → share → copy link → verify link works)

D. **Edge Cases**
   - Customer with no data for this feature — screenshot the empty state
   - Customer with expired/cancelled data — screenshot how it displays
   - Invalid URL (wrong slug, wrong code) — screenshot the error page
   - Navigate away and back — does state persist?

E. **Visual Design**
   - The storefront uses hardcoded hex colors (#1a1d27, #2a2d3a, #4ade80), NOT CSS variables
   - Verify the dark card design looks correct
   - Check text is readable against dark backgrounds
   - Check spacing and alignment look intentional

For EACH test case, define:
- Test ID (sequential number, e.g., #1, #2, #3)
- Scenario name (short, descriptive, e.g., "View Referral Codes List")
- Steps to reproduce (numbered, specific — include exact URLs, exact button text, exact field values)
- Expected result (what SHOULD happen — be specific about what data should appear)
- Screenshot checklist (list every screenshot you will take for this test case — at minimum: initial state, action, result)
- DB verification query (optional but recommended — a SQL query to confirm the UI matches reality)


### Step 3: Execute Tests — Screenshot Protocol

IMPORTANT: Read this entire section before you start testing. Follow these steps for EVERY SINGLE test case, no exceptions.

**Before you start any testing:**
1. Verify the dev servers are running:
   - Backend on http://localhost:3000 (hit /health endpoint)
   - Frontend on http://localhost:5174
2. Screenshot the health check response
3. Note the current time — you'll need this for the report header

**For EACH test case, follow this EXACT sequence:**

1. **Navigate:** Go to the URL
   - Screenshot the page IMMEDIATELY after navigation, before doing anything else
   - This is your "initial state" screenshot

2. **Check console:** Look for errors
   - If there are console errors, screenshot the console or note them in the test case
   - Console warnings are OK to note but don't fail the test for warnings alone

3. **Check network:** Look for failed API calls
   - Any 4xx or 5xx responses? Note the endpoint, status code, and response body
   - Any requests that are pending/hanging? Note them

4. **Read the page state:** Understand what's on screen
   - Is the data correct? Cross-reference with your test data scratchpad
   - Are all UI elements rendered? (no broken layouts, no missing components, no loading spinners stuck)

5. **Interact:** Perform the action for this test case
   - Screenshot BEFORE the action (e.g., the filled form before clicking Submit)
   - Perform the action (click, type, submit, toggle, etc.)
   - Screenshot AFTER the action (e.g., the success message, the updated list, the opened panel)
   - If there's a loading state, try to screenshot that too

6. **Verify data integrity:** Run a SQL query to confirm
   - For creates: verify the record exists in the database
   - For updates: verify the field changed in the database
   - For deletes: verify the record is removed/marked in the database
   - For displays: verify the UI values match the database values

7. **Check console + network again:** After the interaction
   - Any new errors? Any failed requests triggered by the action?

**Screenshot Naming Convention:**
Save to `tests/UAT Reports N/[feature]-uat/` with sequential numbered filenames:
```
01-admin-page-load-light.png
02-admin-page-load-dark.png
03-admin-list-view-with-data.png
04-admin-list-pagination-page2.png
05-admin-list-search-results.png
06-admin-list-empty-state.png
07-admin-detail-panel-open.png
08-admin-detail-panel-fields.png
09-admin-create-form-empty.png
10-admin-create-form-validation-error.png
11-admin-create-form-filled.png
12-admin-create-form-success.png
13-storefront-login-screen.png
14-storefront-invalid-phone.png
15-storefront-authenticated-view.png
16-storefront-feature-interaction.png
17-storefront-empty-state.png
```

**Rules for screenshots:**
- EVERY test case MUST have at least ONE screenshot. No exceptions.
- Most test cases should have 2-3 screenshots (before, action, after).
- Create/update/delete flows should have 3-4 screenshots (empty form, filled form, submitting, result).
- Use descriptive filenames — a human should understand what the screenshot shows from the filename alone.
- If the page is long, take multiple screenshots or scroll to capture all relevant content.
- If a test case reveals a bug, take EXTRA screenshots showing the bug clearly.


### Step 4: Edge Case & Regression Checklist

After completing the main test cases, run through this checklist. Each item is a test case that needs its own screenshot.

**A. Empty States (MUST test for every feature)**
- [ ] New merchant with zero data — what does the page look like?
- [ ] Table with no matching results (search for nonsense string)
- [ ] Detail view for a record that was just deleted
- [ ] Form dropdowns with no options available

**B. Pagination Boundaries (MUST test if the feature has lists)**
- [ ] Page 1 of multi-page results — screenshot
- [ ] Last page — screenshot
- [ ] Page with exactly 1 result — screenshot
- [ ] Navigate beyond the last page (manually edit URL `?page=9999`) — what happens?

**C. URL State Persistence (MUST test for every page)**
- [ ] Apply filters → refresh the page → are filters still applied?
- [ ] Search for something → refresh → is the search still there?
- [ ] Open a specific tab → refresh → is the same tab still active?
- [ ] Copy the URL with all params → open in new tab → does it load the same state?

**D. Form Validation (MUST test for every form)**
- [ ] Submit completely empty form — screenshot ALL validation errors
- [ ] Submit with only required fields — does it work?
- [ ] Submit with maximum length values — does it accept or truncate?
- [ ] Submit with special characters (emoji, unicode, HTML tags) — is it handled safely?
- [ ] Submit with negative numbers (for amount fields)
- [ ] Submit with zero (for amount fields)
- [ ] Double-click the submit button — does it create duplicates?

**E. Error Handling**
- [ ] What happens when the API returns an error? (If you can trigger one, e.g., invalid merchant_id)
- [ ] What happens when the user has no permission? (If applicable)
- [ ] What does a 404 page look like?

**F. Mobile Viewport (375px width) — STOREFRONT ONLY**
- [ ] Admin and Platform pages are desktop-only — do NOT test mobile viewport for `/admin/*` or `/platform/*`
- [ ] Customer storefront (`/s/{slug}`) MUST be tested at 375px width
- [ ] Check: does the card layout scale correctly? Is text readable? Do buttons fit?

**G. Dark Mode (MUST test for every page)**
- [ ] Screenshot the main page in dark mode
- [ ] Check: text readable, badges visible, borders present, no white-on-white or dark-on-dark
- [ ] Check: form inputs have visible borders and readable text in dark mode
- [ ] Toggle back to light mode and screenshot to confirm it restores

**H. Data Integrity (MUST verify for key displays)**
- [ ] For balance displays: run `SELECT SUM(amount) FROM ledger_entries WHERE wallet_id = '...' AND bucket = '...'` and compare to UI
- [ ] For counts: run `SELECT COUNT(*) FROM [table] WHERE merchant_id = '...'` and compare to UI
- [ ] For status badges: check the database status column matches the UI badge color/text
- [ ] For timestamps: verify the displayed time matches the DB `created_at` in the correct timezone


### Step 5: Cross-Feature Integration Tests (when applicable)

If the feature interacts with other features, test the full flow across systems. These tests are critical because they catch bugs that single-feature tests miss.

**Common Integration Flows to Test:**

| Flow | Steps | What to Screenshot |
|------|-------|--------------------|
| Order → Earn → Wallet | 1. Trigger earn event (via API or Shopify webhook mock). 2. Check earn rule matched. 3. Check wallet balance updated. 4. Check storefront shows new balance. | Before balance, API call/response, after balance, storefront view |
| Referral → Conversion → Credit | 1. Get referral code. 2. Simulate conversion (API call). 3. Check referrer wallet credited. 4. Check referee wallet credited. 5. Check conversion appears in admin. | Referral code, conversion API, both wallets, admin conversions list |
| Gift Card → Claim → Wallet | 1. Issue gift card (admin). 2. Claim gift card (storefront). 3. Check wallet balance updated. 4. Check gift card status changed. | Issue form, claim page, wallet balance, gift card status |
| Loyalty Tier → Earn Multiplier | 1. Verify customer tier. 2. Trigger earn event. 3. Verify multiplier applied to points. | Customer tier, earn response, ledger entry with multiplier |
| Membership → Tier Assignment | 1. Assign membership (admin). 2. Verify tier updated. 3. Verify storefront shows membership. | Assignment form, tier in DB, storefront view |
| Earn → Notification | 1. Trigger earn event. 2. Verify notification sent. 3. Check notification logs. | Earn response, notification log entry |

Only test integration flows that are relevant to the feature you're testing. But if you're testing Referrals, you MUST test the referral → conversion → credit flow, not just the referral admin UI.


### Step 6: Classify Results

For each test case, classify as ONE of:

| Classification | Badge Color | Meaning | Action Required |
|---------------|-------------|---------|-----------------|
| **PASS** | Green (#059669) | Works exactly as expected, screenshot proves it | None |
| **FAIL** | Red (#dc2626) | Feature is completely broken, blocks the user flow | Must fix before release |
| **BUG** | Orange (#d97706) | Works partially OR has incorrect behavior | Fix based on severity |
| **SKIP** | Gray (#6b7280) | Cannot test (dependency missing, data unavailable) | Document WHY it was skipped |

**For bugs, you MUST document ALL of the following:**
- **What happened:** Describe exactly what you observed (include screenshot reference)
- **What was expected:** Describe exactly what should have happened
- **Root cause:** Inspect the code, network response, or database to identify WHY
  - Check the network response body — is the API returning wrong data?
  - Check the frontend code — is it displaying the data incorrectly?
  - Check the database — is the data stored incorrectly?
- **Severity:**
  - **HIGH** — Blocks a user flow entirely. User cannot complete their task. (e.g., form won't submit, page crashes, data loss)
  - **MEDIUM** — Feature works but shows wrong data, wrong formatting, or confusing UX. (e.g., wrong currency symbol, truncated text, misleading status)
  - **LOW** — Cosmetic or minor UX issue. (e.g., alignment off by a few pixels, inconsistent spacing, missing hover state)
- **Reproduction rate:** Always, Sometimes, or Once (try the action 2-3 times to check)

**IMPORTANT:** When in doubt about severity, go HIGHER, not lower. A human reviewer will downgrade if needed. It's worse to miss a real issue than to over-report.


### Step 7: Generate HTML Report

Create `tests/UAT Reports N/[feature]-uat-report.html` with the structure below. The report MUST be self-contained and readable by someone who has never seen the codebase.

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>[Feature Name] UAT Report</title>
<style>
  /* REQUIRED STYLES — copy these exactly */
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #f3f4f6;
    color: #1f2937;
    line-height: 1.6;
    padding: 2rem;
  }

  .container { max-width: 1100px; margin: 0 auto; }

  h1 { font-size: 1.75rem; font-weight: 700; color: #111827; }
  h2 { font-size: 1.35rem; font-weight: 600; color: #111827; margin-bottom: 1rem; border-bottom: 2px solid #e5e7eb; padding-bottom: 0.5rem; }
  h3 { font-size: 1.1rem; font-weight: 600; color: #374151; margin-bottom: 0.5rem; }

  .header {
    background: #fff;
    border-radius: 12px;
    padding: 1.5rem 2rem;
    margin-bottom: 1.5rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.08);
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
  }
  .header-meta { font-size: 0.875rem; color: #6b7280; }

  /* Summary grid */
  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  .summary-item {
    background: #fff;
    border-radius: 10px;
    padding: 1.25rem;
    text-align: center;
    box-shadow: 0 1px 3px rgba(0,0,0,0.08);
  }
  .summary-item .value { font-size: 2rem; font-weight: 700; line-height: 1.2; }
  .summary-item .label { font-size: 0.75rem; color: #6b7280; font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em; margin-top: 0.25rem; }
  .summary-item.pass .value { color: #059669; }
  .summary-item.fail .value { color: #dc2626; }
  .summary-item.bug .value { color: #d97706; }
  .summary-item.total .value { color: #2563eb; }
  .summary-item.skip .value { color: #6b7280; }

  /* Sections */
  .section {
    background: #fff;
    border-radius: 12px;
    padding: 1.5rem 2rem;
    margin-bottom: 1.5rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.08);
  }

  /* Test case cards */
  .test-case {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1.25rem;
    margin-bottom: 1rem;
  }
  .test-case.pass { border-left: 4px solid #059669; }
  .test-case.fail { border-left: 4px solid #dc2626; }
  .test-case.bug { border-left: 4px solid #d97706; }
  .test-case.skip { border-left: 4px solid #6b7280; }

  .test-case h3 { display: flex; align-items: center; gap: 0.75rem; }

  .badge {
    display: inline-block;
    font-size: 0.7rem;
    font-weight: 700;
    padding: 0.2rem 0.6rem;
    border-radius: 4px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .badge-pass { background: #d1fae5; color: #065f46; }
  .badge-fail { background: #fee2e2; color: #991b1b; }
  .badge-bug { background: #fef3c7; color: #92400e; }
  .badge-skip { background: #f3f4f6; color: #4b5563; }

  .severity {
    display: inline-block;
    font-size: 0.65rem;
    font-weight: 700;
    padding: 0.15rem 0.5rem;
    border-radius: 3px;
    text-transform: uppercase;
    margin-left: 0.5rem;
  }
  .severity-high { background: #fee2e2; color: #991b1b; }
  .severity-medium { background: #fef3c7; color: #92400e; }
  .severity-low { background: #f3f4f6; color: #4b5563; }

  .test-id {
    display: inline-block;
    font-size: 0.75rem;
    font-weight: 600;
    color: #6b7280;
    background: #f3f4f6;
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
    font-family: monospace;
  }

  /* Steps */
  .steps { margin: 0.75rem 0; padding-left: 1.5rem; }
  .steps li { margin-bottom: 0.25rem; font-size: 0.9rem; }

  /* Screenshots */
  .screenshot-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 1rem;
    margin: 1rem 0;
  }
  .screenshot-grid img {
    width: 100%;
    border-radius: 6px;
    border: 1px solid #e5e7eb;
    cursor: pointer;
  }
  .screenshot-grid img:hover {
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  }
  .screenshot-caption {
    font-size: 0.8rem;
    color: #6b7280;
    margin-top: 0.25rem;
    text-align: center;
  }

  /* Notes */
  .notes {
    background: #f9fafb;
    border-radius: 6px;
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    margin-top: 0.75rem;
  }
  .notes strong { color: #374151; }

  /* Bug detail */
  .bug-detail {
    background: #fffbeb;
    border: 1px solid #fcd34d;
    border-radius: 6px;
    padding: 0.75rem 1rem;
    font-size: 0.875rem;
    margin-top: 0.75rem;
  }
  .bug-detail p { margin-bottom: 0.25rem; }

  /* Bug summary table */
  .bug-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
    margin-top: 1rem;
  }
  .bug-table th {
    text-align: left;
    padding: 0.6rem 0.8rem;
    background: #f9fafb;
    border-bottom: 2px solid #e5e7eb;
    font-weight: 600;
  }
  .bug-table td {
    padding: 0.6rem 0.8rem;
    border-bottom: 1px solid #f3f4f6;
    vertical-align: top;
  }

  /* Environment info */
  .env-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }
  .env-table td {
    padding: 0.4rem 0.8rem;
    border-bottom: 1px solid #f3f4f6;
  }
  .env-table td:first-child {
    font-weight: 600;
    color: #374151;
    width: 200px;
  }

  /* Data integrity table */
  .data-check {
    background: #eff6ff;
    border: 1px solid #bfdbfe;
    border-radius: 6px;
    padding: 0.75rem 1rem;
    font-size: 0.85rem;
    margin-top: 0.75rem;
    font-family: monospace;
  }

  @media print {
    body { background: #fff; padding: 1rem; }
    .section, .header, .summary-item { box-shadow: none; border: 1px solid #e5e7eb; }
    .test-case { break-inside: avoid; }
  }
</style>
</head>
<body>
<div class="container">

  <!-- HEADER -->
  <div class="header">
    <div>
      <h1>[Feature Name] UAT Report</h1>
      <p class="header-meta">Batua Project &mdash; [Date] &mdash; Merchant: [Name] (<code>[slug]</code>)</p>
    </div>
    <div class="header-meta">
      Generated by Claude Code UAT Agent
    </div>
  </div>

  <!-- SUMMARY GRID: one item per classification -->
  <div class="summary-grid">
    <div class="summary-item total"><span class="value">[N]</span><span class="label">Total Tests</span></div>
    <div class="summary-item pass"><span class="value">[N]</span><span class="label">Passed</span></div>
    <div class="summary-item fail"><span class="value">[N]</span><span class="label">Failed</span></div>
    <div class="summary-item bug"><span class="value">[N]</span><span class="label">Bugs</span></div>
    <div class="summary-item skip"><span class="value">[N]</span><span class="label">Skipped</span></div>
  </div>

  <!-- BUG SUMMARY TABLE: only if there are bugs or fails -->
  <div class="section">
    <h2>Bug Summary</h2>
    <table class="bug-table">
      <thead><tr><th>ID</th><th>Severity</th><th>Test Case</th><th>Description</th><th>Root Cause</th></tr></thead>
      <tbody>
        <tr>
          <td><span class="test-id">#N</span></td>
          <td><span class="severity severity-high">HIGH</span></td>
          <td>Test case name</td>
          <td>What happened vs expected</td>
          <td>Why it happened (code/API/DB analysis)</td>
        </tr>
      </tbody>
    </table>
  </div>

  <!-- STAKEHOLDER SECTIONS -->
  <div class="section">
    <h2>Stakeholder 1: Merchant Admin</h2>

    <!-- ONE CARD PER TEST CASE -->
    <div class="test-case pass">
      <h3><span class="test-id">#1</span> Test Case Name <span class="badge badge-pass">PASS</span></h3>
      <ol class="steps">
        <li>Navigate to http://localhost:5174/admin/[feature]</li>
        <li>Verify the page loads with data</li>
        <li>Check console for errors (none found)</li>
      </ol>
      <div class="screenshot-grid">
        <div>
          <img src="[feature]-uat/01-description.png" alt="Description of screenshot">
          <p class="screenshot-caption">Page after initial load</p>
        </div>
      </div>
      <div class="notes">
        <strong>Notes:</strong> All data renders correctly. Verified balance matches DB: SELECT ... returned [value].
      </div>
    </div>

    <!-- BUG EXAMPLE -->
    <div class="test-case bug">
      <h3><span class="test-id">#N</span> Test Case Name <span class="badge badge-bug">BUG</span> <span class="severity severity-medium">MEDIUM</span></h3>
      <ol class="steps">
        <li>Step 1...</li>
        <li>Step 2...</li>
      </ol>
      <div class="screenshot-grid">
        <div>
          <img src="[feature]-uat/NN-bug-description.png" alt="Bug screenshot">
          <p class="screenshot-caption">Shows the incorrect state</p>
        </div>
      </div>
      <div class="bug-detail">
        <p><strong>Expected:</strong> What should have happened</p>
        <p><strong>Actual:</strong> What actually happened</p>
        <p><strong>Root cause:</strong> Network response shows X / Code at file.ts:line does Y / DB has Z</p>
        <p><strong>Reproduction rate:</strong> Always / Sometimes / Once</p>
      </div>
    </div>
  </div>

  <!-- Repeat for Stakeholder 2, 3 -->

  <!-- DATA INTEGRITY SECTION -->
  <div class="section">
    <h2>Data Integrity Checks</h2>
    <div class="test-case pass">
      <h3><span class="test-id">#N</span> Balance matches database <span class="badge badge-pass">PASS</span></h3>
      <div class="data-check">
        UI shows: 1,250 points<br>
        DB query: SELECT SUM(amount) FROM ledger_entries WHERE wallet_id = '...' → 1250<br>
        Result: MATCH
      </div>
    </div>
  </div>

  <!-- ENVIRONMENT SECTION -->
  <div class="section">
    <h2>Test Environment</h2>
    <table class="env-table">
      <tr><td>Date</td><td>[YYYY-MM-DD HH:MM]</td></tr>
      <tr><td>Backend URL</td><td>http://localhost:3000</td></tr>
      <tr><td>Frontend URL</td><td>http://localhost:5174</td></tr>
      <tr><td>Database</td><td>batua (local PostgreSQL)</td></tr>
      <tr><td>Merchant</td><td>[Name] (ID: [uuid], slug: [slug])</td></tr>
      <tr><td>Test Customers</td><td>[phone1], [phone2], [phone3]</td></tr>
      <tr><td>Browser</td><td>Chrome [version]</td></tr>
      <tr><td>Total Screenshots</td><td>[N]</td></tr>
    </table>
  </div>

</div>
</body>
</html>
```

### Step 8: Verify Report Completeness

BEFORE telling the user the report is done, verify ALL of the following:

1. **Open the HTML file in the browser** — screenshot it to prove you verified it
2. **Count check:** Total tests in summary = number of test case cards in the report
3. **Screenshot check:** Every test case card has at least one `<img>` tag with a valid path
4. **Bug check:** Every BUG/FAIL in the test cases is listed in the Bug Summary table
5. **Image check:** All screenshot files exist in the `tests/UAT Reports N/[feature]-uat/` directory
6. **Classification check:** No test case is missing a badge (PASS/FAIL/BUG/SKIP)
7. **Severity check:** Every BUG has a severity badge (HIGH/MEDIUM/LOW)
8. **Data integrity:** At least one data integrity check was performed (SQL vs UI comparison)
9. **Dark mode:** At least one dark mode screenshot exists
10. **Edge cases:** Empty state was tested and screenshotted

If ANY of these checks fail, fix the report before declaring it done.

```

---

## Key Principles

1. **Screenshot EVERYTHING** — This is the #1 rule. Every state, every interaction, every error, every before/after. The human reviewer uses screenshots to validate your findings. If there is no screenshot, the test case does not count. When in doubt, take MORE screenshots, not fewer. Aim for 2-3 screenshots per test case minimum.

2. **Use real data** — Query the database for actual records. Never fabricate test data. Write down all test data IDs, phone numbers, and values before you start testing so you can cross-reference.

3. **Test BOTH happy and sad paths** — Empty states, errors, validation failures, and edge cases matter as much as success flows. Every feature should have at least one empty state test and one validation error test.

4. **Check console AND network on EVERY page load** — The UI might look fine while the API is returning errors, or the console might be full of warnings. Always check both. Note any errors even if the UI looks correct.

5. **Verify data integrity** — The UI showing "1,250 points" means nothing if the database says 1,000. Run SQL queries to verify key displays match reality. Include the SQL query and result in the test case notes.

6. **Test dark mode for every page** — Toggle the ThemeSwitcher and screenshot. Check readability, borders, backgrounds. This catches a huge class of bugs.

7. **Test URL state persistence** — Apply filters, search, switch tabs — then refresh. If the state is lost, it's a bug. Users share URLs with each other.

8. **Organize by stakeholder** — The reader should immediately find tests relevant to their role. Admin tests first, then platform (if applicable), then storefront.

9. **Classify with severity** — Not all bugs are equal. But when in doubt, go HIGHER on severity. A human will downgrade if needed. It's worse to miss a critical bug than to over-report a minor one.

10. **Document root causes** — Don't just say "it's broken". Say WHY: the API returned a 500 because field X is null in the DB, or the frontend is calling the wrong endpoint, or the CSS is missing a dark mode override. Include file paths and line numbers when you can identify them.

11. **Include environment info** — URLs, ports, browser version, merchant ID, customer phone numbers. The report must be reproducible by someone else.

12. **Never mark PASS without proof** — A PASS classification MUST have a screenshot showing the expected state. If you can't screenshot it, you can't prove it passed.

---

## Minimum Test Count Guidelines

Use these as a floor — go higher if the feature is complex.

| Feature Complexity | Minimum Test Cases | Minimum Screenshots |
|---|---|---|
| Simple (1 page, read-only, e.g., notifications list) | 8-10 | 15+ |
| Medium (2-3 pages, CRUD, e.g., gift cards) | 15-20 | 30+ |
| Complex (multi-tab, multi-stakeholder, e.g., referrals) | 20-30 | 50+ |
| Integration flow (cross-feature, e.g., earn → wallet) | 10-15 | 25+ |

If your report has fewer test cases than the minimum, you are probably missing coverage. Go back and add more.

---

## Feature-Specific Test Checklists

### Wallet & Transactions
- [ ] Admin: Transaction list loads with real data
- [ ] Admin: Search by phone number returns correct results
- [ ] Admin: Filter by bucket type (cashback, loyalty, gift, promotional)
- [ ] Admin: Filter by movement type (credit, debit)
- [ ] Admin: Combined filters (bucket + movement)
- [ ] Admin: Transaction detail panel opens on row click
- [ ] Admin: Detail panel shows all fields (amount, bucket, movement, order_id, notes, timestamps)
- [ ] Admin: Pagination works (first page, next page, last page)
- [ ] Admin: Wallet policies page loads
- [ ] Admin: Policy details expand/collapse
- [ ] Admin: Dark mode for all pages
- [ ] Storefront: Phone login → dashboard loads
- [ ] Storefront: Spendable balance matches DB
- [ ] Storefront: Bucket breakdown shows correct per-bucket amounts
- [ ] Storefront: Lifetime Saved value matches DB
- [ ] Storefront: Activity feed shows recent transactions in correct order
- [ ] Storefront: Customer with zero balance — empty state
- [ ] Data: UI balance = SUM(amount) from ledger_entries for that wallet

### Earn Rules
- [ ] Admin: Rules list loads with all configured rules
- [ ] Admin: Each rule shows type, conditions, reward amount, status
- [ ] Admin: Create new rule form — all fields present
- [ ] Admin: Create rule with all condition types
- [ ] Admin: Edit existing rule — pre-populated form
- [ ] Admin: Rule evaluation dry-run (if UI exists)
- [ ] Admin: Dark mode
- [ ] Admin: Empty state (merchant with no rules)
- [ ] Data: Rule conditions in DB match UI display

### Loyalty & Tiers
- [ ] Admin: Program details page loads
- [ ] Admin: Tier list with correct order (by min_points)
- [ ] Admin: Tier distribution chart (if implemented)
- [ ] Admin: Create tier form
- [ ] Admin: Edit tier
- [ ] Admin: Delete tier
- [ ] Admin: Evaluate tiers button
- [ ] Admin: Dark mode
- [ ] Storefront: Customer's current tier displayed
- [ ] Storefront: Tier progress bar (if implemented)
- [ ] Storefront: Tier benefits listed
- [ ] Data: Customer tier in DB matches storefront display

### Memberships
- [ ] Admin: Memberships page loads (plans tab)
- [ ] Admin: Subscribers tab with member list
- [ ] Admin: Customer names resolve (not truncated UUIDs)
- [ ] Admin: Cancel membership flow
- [ ] Admin: Assign membership — phone lookup form
- [ ] Admin: Assign membership — select plan and confirm
- [ ] Admin: Membership expiry calculation
- [ ] Admin: Status badges (active, expired, cancelled)
- [ ] Admin: Dark mode
- [ ] Admin: Empty state (no subscribers)
- [ ] Data: Membership status in DB matches UI badge

### Campaigns
- [ ] Admin: Campaigns page loads with active campaigns
- [ ] Admin: Campaign card shows name, status, date range
- [ ] Admin: Festive templates grid
- [ ] Admin: Template details (click a template)
- [ ] Admin: Category badge color coding
- [ ] Admin: Dark mode
- [ ] Admin: Empty state (no campaigns)

### Customers
- [ ] Admin: Customer list loads with real data
- [ ] Admin: All columns present (name, phone, email, created_at, etc.)
- [ ] Admin: Search by name, phone, email
- [ ] Admin: Pagination
- [ ] Admin: Customer detail view (click a row)
- [ ] Admin: Detail shows wallet balance, tier, membership, recent activity
- [ ] Admin: Dark mode
- [ ] Admin: Empty state (search with no results)
- [ ] Data: Customer count in UI matches DB count

### Gift Cards
- [ ] Admin: Dashboard stats (total issued, total claimed, total value)
- [ ] Admin: All Cards tab — list view
- [ ] Admin: Gift card detail panel
- [ ] Admin: Issue Card form — validation
- [ ] Admin: Issue Card — successful issue
- [ ] Admin: Bulk Issue — CSV upload
- [ ] Admin: Bulk Issue — validation (wrong format, empty file)
- [ ] Admin: Dark mode
- [ ] Storefront: Gift card check page loads
- [ ] Storefront: Enter valid code — shows card details and balance
- [ ] Storefront: Enter invalid code — shows error
- [ ] Storefront: Enter expired card code — shows expired state
- [ ] Data: Gift card balance in DB matches storefront display

### Referrals
- [ ] Admin: Program tab — view/edit settings
- [ ] Admin: Program tab — save updated settings
- [ ] Admin: Codes tab — list of referral codes
- [ ] Admin: Codes tab — create auto-generated code
- [ ] Admin: Codes tab — create vanity code
- [ ] Admin: Codes tab — create influencer code
- [ ] Admin: Analytics tab — summary metrics
- [ ] Admin: Conversions tab — conversion list with fraud signals
- [ ] Admin: Dark mode
- [ ] Storefront: Refer page — phone login
- [ ] Storefront: View referral card with code and stats
- [ ] Storefront: Copy referral link
- [ ] Storefront: WhatsApp share button
- [ ] Storefront: Customer with no referral code — empty state
- [ ] Storefront: Referee landing page with reward offer
- [ ] Storefront: Shop Now CTA
- [ ] Integration: Conversion flow — referee uses code → both wallets credited

### Influencers
- [ ] Admin: Influencers page loads
- [ ] Admin: Influencer list with codes and stats
- [ ] Admin: Create influencer code
- [ ] Admin: Influencer detail/performance view
- [ ] Admin: Dark mode
- [ ] Admin: Empty state

### Notifications
- [ ] Admin: Notifications page loads
- [ ] Admin: Notification templates list
- [ ] Admin: Create/edit template
- [ ] Admin: Template variable pills
- [ ] Admin: Connectors list
- [ ] Admin: Add connector
- [ ] Admin: Notification logs
- [ ] Admin: Dark mode
- [ ] Admin: Empty state (no logs)

### Settings
- [ ] Admin: My Store tab — store details
- [ ] Admin: Plan and storefront info
- [ ] Admin: Customer links section
- [ ] Admin: Wallet Policies tab — list and toggles
- [ ] Admin: Connectors tab
- [ ] Admin: Notifications tab — templates
- [ ] Admin: Points config tab (if present)
- [ ] Admin: Tab navigation with URL state
- [ ] Admin: Dark mode for all tabs
- [ ] Admin: Save settings — verify persistence

### COD (if UI exists, else API-only)
- [ ] API: COD-to-prepaid incentive creation
- [ ] API: Delivery webhook handling
- [ ] API: COD metrics endpoint returns data
- [ ] API: RTO comparison endpoint returns data
- [ ] Data: Incentive records in DB match API response

### Earn Mechanics (milestones, streaks, spin wheel)
- [ ] API: Create milestone config
- [ ] API: Check milestones for a customer
- [ ] API: Create streak config
- [ ] API: Check streaks for a customer
- [ ] API: Spin wheel config and spin action
- [ ] Storefront: Spin wheel UI (if exists)
- [ ] Storefront: Streak display (if exists)
- [ ] Data: Milestone/streak progress in DB matches API

---

## Customization Points

Replace these placeholders for each feature:
- `[FEATURE NAME]` — e.g., "Referral System", "Gift Card", "Loyalty Program"
- `[feature]` — e.g., "referral", "gift-card", "loyalty" (for file paths)
- `[Date]` — current date in DD Month YYYY format
- `[Name]` / `[slug]` — merchant name and slug from DB discovery
- SQL queries — adapt to the feature's tables
- Stakeholders — some features may have 2 instead of 3
- Test scenarios — specific to the feature's functionality

---

## Report Output Checklist (Agent MUST verify before finishing)

- [ ] UAT Reports folder created at `tests/UAT Reports N/` (N is auto-incremented, never overwrites previous)
- [ ] HTML file created at `tests/UAT Reports N/[feature]-uat-report.html`
- [ ] Screenshot directory created at `tests/UAT Reports N/[feature]-uat/`
- [ ] Every test case has at least 1 screenshot (most have 2-3)
- [ ] Summary counts match actual test case cards
- [ ] Every BUG/FAIL is in the Bug Summary table
- [ ] Every BUG has severity, expected, actual, root cause, reproduction rate
- [ ] At least 1 dark mode screenshot exists
- [ ] At least 1 empty state test exists
- [ ] At least 1 data integrity check (SQL vs UI) exists
- [ ] Environment section filled with real values (URLs, merchant, browser)
- [ ] Report opened in browser and screenshot taken to prove it renders correctly
- [ ] All `<img>` src paths are valid relative paths that resolve to actual files
