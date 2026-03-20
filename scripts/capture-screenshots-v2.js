const { chromium } = require('/Users/chirag/Developer/batua/site/node_modules/playwright-core');
const path = require('path');
const fs = require('fs');

const BASE = 'http://localhost:5174';
const API = 'http://localhost:3000';
const SHOT_DIR = path.resolve(__dirname, '../docs/screenshots-v2');
const MID = process.argv[2];

if (!MID) {
  console.error('Usage: node capture-screenshots-v2.js <merchant_id>');
  process.exit(1);
}

const FEATURES = [
  'wallet',
  'loyalty',
  'gift-cards',
  'referrals',
  'campaigns',
  'analytics',
  'memberships',
  'platform'
];

const results = {};

function ensureDirs() {
  for (const feature of FEATURES) {
    const dir = path.join(SHOT_DIR, feature);
    fs.mkdirSync(dir, { recursive: true });
  }
}

async function shot(page, name, dir) {
  const fp = path.join(SHOT_DIR, dir, `${name}.png`);
  try {
    await page.screenshot({ path: fp, fullPage: false });
    console.log(`  OK  ${dir}/${name}.png`);
    results[dir] = (results[dir] || 0) + 1;
  } catch (err) {
    console.log(`  FAIL ${dir}/${name}.png - ${err.message}`);
  }
}

async function setMerchant(page, merchantId) {
  await page.evaluate(
    (mid) => localStorage.setItem('batua_merchant_id', mid),
    merchantId
  );
}

async function waitForPage(page, url, opts = {}) {
  const timeout = opts.timeout || 15000;
  try {
    await page.goto(`${BASE}${url}`, {
      waitUntil: 'networkidle',
      timeout
    });
  } catch {
    try {
      await page.goto(`${BASE}${url}`, {
        waitUntil: 'load',
        timeout: 10000
      });
    } catch {
      // page may still be usable
    }
  }
  await page.waitForTimeout(opts.wait || 2500);
}

async function waitAndShot(page, url, name, dir, opts = {}) {
  await waitForPage(page, url, opts);
  if (opts.scroll) {
    await page.evaluate((y) => window.scrollBy(0, y), opts.scroll);
    await page.waitForTimeout(opts.scrollWait || 800);
  }
  await shot(page, name, dir);
}

async function clickTab(page, tabText) {
  try {
    const tab = page.locator('button', { hasText: tabText }).first();
    if ((await tab.count()) > 0 && (await tab.isVisible())) {
      await tab.click();
      await page.waitForTimeout(2000);
      return true;
    }
  } catch {
    // tab not found
  }
  return false;
}

async function clickTabByRole(page, tabText) {
  try {
    const tab = page
      .locator('[role="tab"], .tab, [class*="tab"]', { hasText: tabText })
      .first();
    if ((await tab.count()) > 0 && (await tab.isVisible())) {
      await tab.click();
      await page.waitForTimeout(2000);
      return true;
    }
  } catch {
    // tab not found
  }
  return clickTab(page, tabText);
}

async function fetchFromApi(page, endpoint) {
  try {
    return await page.evaluate(async (url) => {
      const resp = await fetch(url);
      if (!resp.ok) return null;
      return resp.json();
    }, `${API}${endpoint}`);
  } catch {
    return null;
  }
}

async function findAltMerchantId(page, slugPattern) {
  try {
    const merchants = await page.evaluate(async (api) => {
      const resp = await fetch(`${api}/admin/merchants?page=1&limit=50`);
      if (!resp.ok) return [];
      return resp.json();
    }, API);
    if (Array.isArray(merchants)) {
      const match = merchants.find(
        (m) =>
          (m.slug && m.slug.includes(slugPattern)) ||
          (m.name && m.name.toLowerCase().includes(slugPattern))
      );
      return match ? match.id : null;
    }
  } catch {
    // ignore
  }
  return null;
}

// ============================================================
// MAIN
// ============================================================

(async () => {
  ensureDirs();

  console.log('\nCapture Screenshots v2');
  console.log('======================');
  console.log(`Merchant ID: ${MID}`);
  console.log(`Output: ${SHOT_DIR}`);
  console.log('');

  const browser = await chromium.launch({ headless: true });

  // ============================================================
  // 1. WALLET - MERCHANT ADMIN (1280x800)
  // ============================================================
  console.log('\n--- WALLET (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Dashboard
    await waitAndShot(page, '/admin', '01-dashboard', 'wallet', { wait: 3000 });

    // 02 - Dashboard empty (switch to a smaller merchant)
    const bookwormId = await findAltMerchantId(page, 'bookworm');
    if (bookwormId) {
      await setMerchant(page, bookwormId);
      await waitAndShot(page, '/admin', '02-dashboard-empty', 'wallet', { wait: 3000 });
      await setMerchant(page, MID);
    } else {
      console.log('  SKIP 02-dashboard-empty - no bookworm merchant found');
    }

    // 03 - Transactions feed
    await waitAndShot(page, '/admin/transactions', '03-transactions-feed', 'wallet', {
      wait: 3000
    });

    // 04 - Transactions filtered (click bucket type dropdown)
    try {
      const bucketSelect = page
        .locator('.filter-group')
        .first()
        .locator('button, [class*="select"], [role="combobox"]')
        .first();
      if ((await bucketSelect.count()) > 0) {
        await bucketSelect.click();
        await page.waitForTimeout(1000);
        const firstOption = page
          .locator('[role="option"], [class*="option"], [class*="item"]')
          .first();
        if ((await firstOption.count()) > 0) {
          await firstOption.click();
          await page.waitForTimeout(2000);
        }
      }
      await shot(page, '04-transactions-filtered', 'wallet');
    } catch {
      console.log('  SKIP 04-transactions-filtered - filter interaction failed');
    }

    // 05 - Customers list
    await waitAndShot(page, '/admin/customers', '05-customers-list', 'wallet', { wait: 3000 });

    // 06 - Customer detail (click first row)
    try {
      await page.waitForSelector('table', { timeout: 5000 }).catch(() => {});
      const firstRow = page.locator('table tbody tr').first();
      if ((await firstRow.count()) > 0) {
        await firstRow.click();
        await page.waitForTimeout(3000);
        await shot(page, '06-customer-detail', 'wallet');
      } else {
        console.log('  SKIP 06-customer-detail - no customer rows');
      }
    } catch {
      console.log('  SKIP 06-customer-detail - click failed');
    }

    // 07 - Customer search (type a phone number)
    try {
      await waitForPage(page, '/admin/customers', { wait: 2000 });
      const searchInput = page.locator('.search-input, input[placeholder*="Search"]').first();
      if ((await searchInput.count()) > 0) {
        await searchInput.fill('9876');
        await page.waitForTimeout(1500);
        await shot(page, '07-customer-search', 'wallet');
      }
    } catch {
      console.log('  SKIP 07-customer-search - search interaction failed');
    }

    // 08 - Settings -> My Store
    await waitAndShot(page, '/admin/settings?tab=store', '08-settings-store', 'wallet', {
      wait: 2500
    });

    // 09 - Settings -> Wallet Policies
    await waitAndShot(page, '/admin/settings?tab=policies', '09-settings-policies', 'wallet', {
      wait: 2500
    });

    await ctx.close();
  }

  // ============================================================
  // 1b. WALLET - CUSTOMER STOREFRONT (375x812)
  // ============================================================
  console.log('\n--- WALLET (Customer Storefront) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page = await ctx.newPage();

    // 10 - Storefront landing (phone entry)
    await waitAndShot(page, '/s/desi-threads', '10-storefront-landing', 'wallet', { wait: 3000 });

    // 11 - Storefront balance (after entering phone)
    try {
      const phoneInput = page.locator('input[type="tel"]').first();
      if ((await phoneInput.count()) > 0) {
        await phoneInput.fill('9876543210');
        await page.waitForTimeout(500);

        const submitBtn = page
          .locator('button', { hasText: /View Rewards|Submit|Check/ })
          .first();
        if ((await submitBtn.count()) > 0 && (await submitBtn.isEnabled())) {
          await submitBtn.click();
          await page.waitForTimeout(4000);
          await shot(page, '11-storefront-balance', 'wallet');

          // 12 - Scroll to transactions
          await page.evaluate(() => window.scrollBy(0, 400));
          await page.waitForTimeout(1000);
          await shot(page, '12-storefront-transactions', 'wallet');
        } else {
          console.log('  SKIP 11/12 - submit button not found or disabled');
        }
      }
    } catch (err) {
      console.log(`  SKIP 11/12 - storefront interaction failed: ${err.message}`);
    }

    // 13 - Balance check page
    await waitAndShot(page, '/s/desi-threads/balance', '13-balance-check', 'wallet', {
      wait: 2500
    });

    await ctx.close();
  }

  // ============================================================
  // 2. LOYALTY - MERCHANT ADMIN
  // ============================================================
  console.log('\n--- LOYALTY (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Loyalty program config
    await waitAndShot(page, '/admin/loyalty', '01-loyalty-program', 'loyalty', { wait: 3000 });

    // 02 - Reward rules table
    await waitAndShot(page, '/admin/rules', '02-reward-rules', 'loyalty', { wait: 3000 });

    // 03 - Click Edit on first rule to show modal
    try {
      const ruleBtn = page.locator('.rule-row-main, .rule-row button').first();
      if ((await ruleBtn.count()) > 0) {
        await ruleBtn.click();
        await page.waitForTimeout(2000);
        await shot(page, '03-reward-rules-detail', 'loyalty');
        // Close modal if open
        const overlay = page.locator('.modal-overlay, [class*="overlay"]').first();
        if ((await overlay.count()) > 0) {
          await page.keyboard.press('Escape');
          await page.waitForTimeout(500);
        }
      } else {
        console.log('  SKIP 03-reward-rules-detail - no rules found');
      }
    } catch {
      console.log('  SKIP 03-reward-rules-detail - click failed');
    }

    // 04 - Onboarding wizard (use a merchant without rules)
    const freshId = await findAltMerchantId(page, 'fresh');
    if (freshId) {
      await setMerchant(page, freshId);
      await waitAndShot(page, '/admin/setup', '04-onboarding-wizard', 'loyalty', { wait: 3500 });
      await setMerchant(page, MID);
    } else {
      // Fallback: just show setup page with main merchant (might show "already set up")
      await waitAndShot(page, '/admin/setup', '04-onboarding-wizard', 'loyalty', { wait: 3500 });
    }

    await ctx.close();
  }

  // ============================================================
  // 2b. LOYALTY - CUSTOMER STOREFRONT
  // ============================================================
  console.log('\n--- LOYALTY (Customer Storefront) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page = await ctx.newPage();

    // 05 - Tier progress (customer hub, scroll to tier section)
    try {
      await waitForPage(page, '/s/desi-threads', { wait: 2000 });
      const phoneInput = page.locator('input[type="tel"]').first();
      if ((await phoneInput.count()) > 0) {
        await phoneInput.fill('9876543210');
        await page.waitForTimeout(500);
        const submitBtn = page
          .locator('button', { hasText: /View Rewards|Submit|Check/ })
          .first();
        if ((await submitBtn.count()) > 0 && (await submitBtn.isEnabled())) {
          await submitBtn.click();
          await page.waitForTimeout(4000);
          // Scroll to tier card section
          const tierSection = page.locator('[class*="tier"], [class*="Tier"]').first();
          if ((await tierSection.count()) > 0) {
            await tierSection.scrollIntoViewIfNeeded();
            await page.waitForTimeout(800);
          } else {
            await page.evaluate(() => window.scrollBy(0, 300));
            await page.waitForTimeout(800);
          }
          await shot(page, '05-tier-progress', 'loyalty');
        }
      }
    } catch {
      console.log('  SKIP 05-tier-progress - interaction failed');
    }

    await ctx.close();
  }

  // ============================================================
  // 3. GIFT CARDS - MERCHANT ADMIN
  // ============================================================
  console.log('\n--- GIFT CARDS (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Gift cards list with stats
    await waitAndShot(page, '/admin/gift-cards', '01-gift-cards-list', 'gift-cards', {
      wait: 3000
    });

    // 02 - Issue tab
    try {
      const clicked = await clickTabByRole(page, 'Issue Card');
      if (clicked) {
        await shot(page, '02-gift-cards-issue', 'gift-cards');
      } else {
        console.log('  SKIP 02-gift-cards-issue - Issue Card tab not found');
      }
    } catch {
      console.log('  SKIP 02-gift-cards-issue - tab click failed');
    }

    await ctx.close();
  }

  // ============================================================
  // 3b. GIFT CARDS - CUSTOMER STOREFRONT
  // ============================================================
  console.log('\n--- GIFT CARDS (Customer Storefront) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page = await ctx.newPage();

    // 03 - Gift card check (empty input)
    await waitAndShot(
      page,
      '/s/desi-threads/gift-cards/check',
      '03-gift-card-check',
      'gift-cards',
      { wait: 2500 }
    );

    // 04 - Gift card check result (fetch a real code)
    try {
      const gcData = await fetchFromApi(page, `/gift-cards/merchant/${MID}/cards?limit=1`);
      let gcCode = null;

      if (Array.isArray(gcData) && gcData.length > 0 && gcData[0].code) {
        gcCode = gcData[0].code;
      }

      if (gcCode === null) {
        const statsData = await fetchFromApi(page, `/gift-cards/merchant/${MID}/stats`);
        if (statsData && statsData.sample_code) {
          gcCode = statsData.sample_code;
        }
      }

      if (gcCode) {
        const codeInput = page
          .locator('input[placeholder*="BRZE"], input[placeholder*="code"], input')
          .first();
        if ((await codeInput.count()) > 0) {
          await codeInput.fill(gcCode);
          await page.waitForTimeout(500);
          const checkBtn = page.locator('button', { hasText: /Check|Balance/ }).first();
          if ((await checkBtn.count()) > 0) {
            await checkBtn.click();
            await page.waitForTimeout(3000);
            await shot(page, '04-gift-card-check-result', 'gift-cards');
          }
        }
      } else {
        console.log('  SKIP 04-gift-card-check-result - no gift card code found in API');
      }
    } catch (err) {
      console.log(`  SKIP 04-gift-card-check-result - ${err.message}`);
    }

    await ctx.close();
  }

  // ============================================================
  // 4. REFERRALS - MERCHANT ADMIN
  // ============================================================
  console.log('\n--- REFERRALS (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Referral program config (Program tab)
    await waitAndShot(page, '/admin/referrals', '01-program-config', 'referrals', { wait: 3000 });

    // 02 - Codes tab
    const clickedCodes = await clickTab(page, 'Codes');
    if (clickedCodes) {
      await shot(page, '02-codes-list', 'referrals');
    } else {
      console.log('  SKIP 02-codes-list - Codes tab not found');
    }

    // 03 - Analytics tab
    const clickedAnalytics = await clickTab(page, 'Analytics');
    if (clickedAnalytics) {
      await shot(page, '03-analytics', 'referrals');
    } else {
      console.log('  SKIP 03-analytics - Analytics tab not found');
    }

    // 04 - Conversions tab
    const clickedConversions = await clickTab(page, 'Conversions');
    if (clickedConversions) {
      await shot(page, '04-conversions', 'referrals');
    } else {
      console.log('  SKIP 04-conversions - Conversions tab not found');
    }

    await ctx.close();
  }

  // ============================================================
  // 4b. REFERRALS - CUSTOMER STOREFRONT
  // ============================================================
  console.log('\n--- REFERRALS (Customer Storefront) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const page = await ctx.newPage();

    // 05 - Referral landing page (fetch a real code)
    try {
      const codesData = await page.evaluate(
        async ({ api, mid }) => {
          const resp = await fetch(`${api}/referrals/merchant/${mid}/codes?limit=1`);
          if (!resp.ok) return [];
          return resp.json();
        },
        { api: API, mid: MID }
      );

      if (Array.isArray(codesData) && codesData.length > 0 && codesData[0].code) {
        const refCode = codesData[0].code;
        await waitAndShot(
          page,
          `/s/desi-threads/refer/${refCode}`,
          '05-referral-landing',
          'referrals',
          { wait: 3000 }
        );
      } else {
        console.log('  SKIP 05-referral-landing - no referral code found in API');
      }
    } catch (err) {
      console.log(`  SKIP 05-referral-landing - ${err.message}`);
    }

    // 06 - My referrals page (phone entry)
    await waitAndShot(page, '/s/desi-threads/refer', '06-my-referrals', 'referrals', {
      wait: 2500
    });

    await ctx.close();
  }

  // ============================================================
  // 5. CAMPAIGNS - MERCHANT ADMIN
  // ============================================================
  console.log('\n--- CAMPAIGNS (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Campaigns list
    await waitAndShot(page, '/admin/campaigns', '01-campaigns-list', 'campaigns', { wait: 3000 });

    // 02 - Festive templates (scroll down to see them)
    await page.evaluate(() => window.scrollBy(0, 500));
    await page.waitForTimeout(1000);
    await shot(page, '02-festive-templates', 'campaigns');

    await ctx.close();
  }

  // ============================================================
  // 6. ANALYTICS - MERCHANT ADMIN
  // ============================================================
  console.log('\n--- ANALYTICS (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Analytics overview
    await waitAndShot(page, '/admin/analytics', '01-analytics-overview', 'analytics', {
      wait: 3500
    });

    // 02 - Scroll to COD section + RTO comparison
    await page.evaluate(() => window.scrollBy(0, 600));
    await page.waitForTimeout(1000);
    await shot(page, '02-analytics-scroll', 'analytics');

    await ctx.close();
  }

  // ============================================================
  // 7. MEMBERSHIPS (Settings connectors, Notifications)
  // ============================================================
  console.log('\n--- MEMBERSHIPS (Merchant Admin) ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
    const page = await ctx.newPage();

    await page.goto(BASE + '/admin', { waitUntil: 'load', timeout: 10000 }).catch(() => {});
    await setMerchant(page, MID);
    await page.waitForTimeout(500);

    // 01 - Settings -> Connectors tab
    await waitAndShot(
      page,
      '/admin/settings?tab=connectors',
      '01-settings-connectors',
      'memberships',
      { wait: 2500 }
    );

    // 02 - Notifications page -> Templates tab
    await waitAndShot(
      page,
      '/admin/notifications',
      '02-notifications-templates',
      'memberships',
      { wait: 3000 }
    );

    // Click first template if available to show editor
    try {
      const templateItem = page.locator('.template-item, button[class*="template"]').first();
      if ((await templateItem.count()) > 0) {
        await templateItem.click();
        await page.waitForTimeout(1500);
        await shot(page, '02-notifications-templates', 'memberships');
      }
    } catch {
      // already captured
    }

    // 03 - Notifications -> Logs tab
    try {
      const clickedLogs = await clickTabByRole(page, 'Logs');
      if (clickedLogs) {
        await shot(page, '03-notifications-logs', 'memberships');
      } else {
        console.log('  SKIP 03-notifications-logs - Logs tab not found');
      }
    } catch {
      console.log('  SKIP 03-notifications-logs - tab click failed');
    }

    await ctx.close();
  }

  // ============================================================
  // 8. PLATFORM ADMIN (1440x900)
  // ============================================================
  console.log('\n--- PLATFORM ADMIN ---');
  {
    const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
    const page = await ctx.newPage();

    // 01 - Platform dashboard
    await waitAndShot(page, '/platform', '01-dashboard', 'platform', { wait: 3000 });

    // 02 - Merchants list (all 10 merchants)
    await waitAndShot(page, '/platform/merchants', '02-merchants-list', 'platform', {
      wait: 3000
    });

    // 03 - Merchant detail (click Desi Threads)
    try {
      const desiLink = page.locator('a, tr, button', { hasText: 'Desi Threads' }).first();
      if ((await desiLink.count()) > 0) {
        await desiLink.click();
        await page.waitForTimeout(3000);
        await shot(page, '03-merchant-detail', 'platform');
      } else {
        // Try navigating directly via merchant ID
        await waitForPage(page, `/platform/merchants/${MID}`, { wait: 3000 });
        await shot(page, '03-merchant-detail', 'platform');
      }
    } catch {
      // Fallback to direct navigation
      await waitAndShot(
        page,
        `/platform/merchants/${MID}`,
        '03-merchant-detail',
        'platform',
        { wait: 3000 }
      );
    }

    // 04 - Geo policies
    await waitAndShot(page, '/platform/geo-policies', '04-geo-policies', 'platform', {
      wait: 3000
    });

    // 05 - System health
    await waitAndShot(page, '/platform/system', '05-system-health', 'platform', { wait: 3000 });

    // 06 - Different merchant detail (TechBazaar)
    try {
      const techId = await findAltMerchantId(page, 'techbazaar');
      if (techId) {
        await waitAndShot(
          page,
          `/platform/merchants/${techId}`,
          '06-merchant-detail-2',
          'platform',
          { wait: 3000 }
        );
      } else {
        // Try finding any other merchant
        const altId = await findAltMerchantId(page, 'tech');
        if (altId) {
          await waitAndShot(
            page,
            `/platform/merchants/${altId}`,
            '06-merchant-detail-2',
            'platform',
            { wait: 3000 }
          );
        } else {
          console.log('  SKIP 06-merchant-detail-2 - no alternate merchant found');
        }
      }
    } catch {
      console.log('  SKIP 06-merchant-detail-2 - navigation failed');
    }

    await ctx.close();
  }

  // ============================================================
  // CLEANUP
  // ============================================================
  await browser.close();

  // ============================================================
  // SUMMARY
  // ============================================================
  console.log('\n');
  console.log('=== SCREENSHOT SUMMARY ===');
  let total = 0;
  for (const feature of FEATURES) {
    const dir = path.join(SHOT_DIR, feature);
    let count = 0;
    try {
      count = fs.readdirSync(dir).filter((f) => f.endsWith('.png')).length;
    } catch {
      count = 0;
    }
    total += count;
    const pad = feature.padEnd(16);
    console.log(`  ${pad} ${count} screenshots`);
  }
  console.log(`  ${'─'.repeat(30)}`);
  console.log(`  ${'TOTAL'.padEnd(16)} ${total} screenshots`);
  console.log('');

  const expected = 43;
  if (total >= expected * 0.8) {
    console.log(`Captured ${total}/${expected} expected screenshots.`);
  } else {
    console.log(
      `WARNING: Only ${total}/${expected} expected screenshots captured. Check logs above for SKIP/FAIL entries.`
    );
  }
})();
