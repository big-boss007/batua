const { chromium } = require('/Users/chirag/Developer/batua/site/node_modules/playwright-core');
const path = require('path');
const fs = require('fs');

const BASE = 'http://localhost:5174';
const SHOT_DIR = path.resolve(__dirname, '../docs/screenshots');
const MID = process.argv[2];

if (!MID) { console.error('Usage: node capture-screenshots.js <merchant_id>'); process.exit(1); }

async function shot(page, name, dir) {
  const fp = path.join(SHOT_DIR, dir, `${name}.png`);
  await page.screenshot({ path: fp, fullPage: false });
  console.log(`  ✓ ${dir}/${name}.png`);
}

async function setMerchant(page) {
  await page.evaluate((mid) => localStorage.setItem('batua_merchant_id', mid), MID);
}

async function waitAndShot(page, url, name, dir, opts = {}) {
  await page.goto(`${BASE}${url}`, { waitUntil: 'networkidle', timeout: 15000 }).catch(() => {});
  await page.waitForTimeout(opts.wait || 2000);
  if (opts.scroll) await page.evaluate((y) => window.scrollBy(0, y), opts.scroll);
  if (opts.scrollWait) await page.waitForTimeout(opts.scrollWait);
  await shot(page, name, dir);
}

(async () => {
  const browser = await chromium.launch({ headless: true });

  // ============================================================
  // MERCHANT ADMIN (1280x800)
  // ============================================================
  console.log('\n=== MERCHANT ADMIN ===');
  const adminCtx = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const admin = await adminCtx.newPage();
  await admin.goto(BASE + '/admin');
  await setMerchant(admin);

  // -- Wallet & Dashboard --
  console.log('\nWallet:');
  await waitAndShot(admin, '/admin', '01-dashboard', 'wallet');
  await waitAndShot(admin, '/admin/transactions', '02-transactions', 'wallet');
  await waitAndShot(admin, '/admin/customers', '03-customers-list', 'wallet');
  // Click first customer
  await admin.waitForTimeout(2000);
  const firstRow = admin.locator('table tr').nth(1);
  if (await firstRow.count() > 0) {
    await firstRow.click().catch(() => {});
    await admin.waitForTimeout(2000);
    await shot(admin, '04-customer-detail', 'wallet');
  }
  await waitAndShot(admin, '/admin/settings', '05-settings-store', 'wallet');

  // -- Loyalty --
  console.log('\nLoyalty:');
  await waitAndShot(admin, '/admin/loyalty', '01-loyalty-program', 'loyalty');
  await waitAndShot(admin, '/admin/rules', '02-reward-rules', 'loyalty');

  // -- Gift Cards --
  console.log('\nGift Cards:');
  await waitAndShot(admin, '/admin/gift-cards', '01-gift-cards', 'gift-cards');

  // -- Referrals --
  console.log('\nReferrals:');
  await waitAndShot(admin, '/admin/referrals', '01-referrals-program', 'referrals');
  // Click Codes tab
  await admin.waitForTimeout(1000);
  const codesTab = admin.locator('button', { hasText: 'Codes' });
  if (await codesTab.count() > 0) {
    await codesTab.click();
    await admin.waitForTimeout(1500);
    await shot(admin, '02-referral-codes', 'referrals');
  }

  // -- Campaigns --
  console.log('\nCampaigns:');
  await waitAndShot(admin, '/admin/campaigns', '01-campaigns', 'campaigns');

  // -- Analytics --
  console.log('\nAnalytics:');
  await waitAndShot(admin, '/admin/analytics', '01-analytics', 'analytics');

  // -- Memberships (no dedicated page yet, show via rules or settings) --
  console.log('\nMemberships:');
  await waitAndShot(admin, '/admin/settings', '01-settings-policies', 'memberships');

  // -- Onboarding Wizard (use Fresh Fashion) --
  const freshMid = await admin.evaluate(async () => {
    const resp = await fetch('http://localhost:3000/admin/merchants?page=1&limit=50');
    const merchants = await resp.json();
    const fresh = merchants.find(m => m.slug === 'fresh-fashion' || m.name === 'Fresh Fashion');
    return fresh ? fresh.id : null;
  });
  if (freshMid) {
    await admin.evaluate((mid) => localStorage.setItem('batua_merchant_id', mid), freshMid);
    await waitAndShot(admin, '/admin/setup', '02-onboarding-wizard', 'memberships', { wait: 3000 });
    // Reset back to Desi Threads
    await admin.evaluate((mid) => localStorage.setItem('batua_merchant_id', mid), MID);
  }

  await adminCtx.close();

  // ============================================================
  // CUSTOMER STOREFRONT (375x812 mobile)
  // ============================================================
  console.log('\n=== CUSTOMER STOREFRONT ===');
  const mobileCtx = await browser.newContext({ viewport: { width: 375, height: 812 } });
  const mobile = await mobileCtx.newPage();

  // -- Wallet (Loyalty Hub) --
  console.log('\nWallet (Customer):');
  await mobile.goto(`${BASE}/s/desi-threads`, { waitUntil: 'networkidle', timeout: 15000 }).catch(() => {});
  await mobile.waitForTimeout(2000);
  await shot(mobile, '06-storefront-landing', 'wallet');

  // Enter phone and search
  const phoneInput = mobile.locator('input[type="tel"]');
  if (await phoneInput.count() > 0) {
    await phoneInput.fill('9876543210');  // Use a customer with data
    await mobile.waitForTimeout(500);
    // Try clicking View Rewards button
    const viewBtn = mobile.locator('button', { hasText: 'View Rewards' });
    if (await viewBtn.count() > 0 && await viewBtn.isEnabled()) {
      await viewBtn.click();
      await mobile.waitForTimeout(3000);
      await shot(mobile, '07-storefront-balance', 'wallet');
      // Scroll down to see transactions
      await mobile.evaluate(() => window.scrollBy(0, 400));
      await mobile.waitForTimeout(500);
      await shot(mobile, '08-storefront-transactions', 'wallet');
    }
  }

  // -- Gift Card Check --
  console.log('\nGift Cards (Customer):');
  await waitAndShot(mobile, '/s/desi-threads/gift-cards/check', '02-gift-card-check', 'gift-cards');

  // -- Referral Landing --
  console.log('\nReferrals (Customer):');
  // Find a referral code
  const codeResp = await mobile.evaluate(async (mid) => {
    const resp = await fetch(`http://localhost:3000/referrals/merchant/${mid}/codes?limit=1`);
    const codes = await resp.json();
    return codes.length > 0 ? codes[0].code : null;
  }, MID);
  if (codeResp) {
    await waitAndShot(mobile, `/s/desi-threads/refer/${codeResp}`, '03-referral-landing', 'referrals');
  }

  // -- My Referrals --
  await waitAndShot(mobile, '/s/desi-threads/refer', '04-my-referrals', 'referrals');

  // -- Balance Check --
  await waitAndShot(mobile, '/s/desi-threads/balance', '09-balance-check', 'wallet');

  await mobileCtx.close();

  // ============================================================
  // PLATFORM ADMIN (1440x900)
  // ============================================================
  console.log('\n=== PLATFORM ADMIN ===');
  const platformCtx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const platform = await platformCtx.newPage();

  console.log('\nPlatform:');
  await waitAndShot(platform, '/platform', '01-platform-dashboard', 'platform');
  await waitAndShot(platform, '/platform/merchants', '02-merchants-list', 'platform');

  // Click Desi Threads merchant
  await platform.waitForTimeout(1500);
  const desiRow = platform.locator('text=Desi Threads');
  if (await desiRow.count() > 0) {
    await desiRow.click();
    await platform.waitForTimeout(2000);
    await shot(platform, '03-merchant-detail', 'platform');
  }

  await waitAndShot(platform, '/platform/geo-policies', '04-geo-policies', 'platform');
  await waitAndShot(platform, '/platform/system', '05-system-health', 'platform');

  await platformCtx.close();
  await browser.close();

  // ============================================================
  // Summary
  // ============================================================
  console.log('\n=== SCREENSHOTS CAPTURED ===');
  const dirs = ['wallet', 'loyalty', 'gift-cards', 'referrals', 'campaigns', 'analytics', 'memberships', 'platform'];
  for (const dir of dirs) {
    const files = fs.readdirSync(path.join(SHOT_DIR, dir)).filter(f => f.endsWith('.png'));
    console.log(`  ${dir}: ${files.length} screenshots`);
  }
})();
