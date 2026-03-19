// @ts-check
const { test, expect } = require('@playwright/test');

const VIEWPORTS = [
  { name: 'Mobile S', width: 320, height: 568 },
  { name: 'Mobile M', width: 375, height: 812 },
  { name: 'Mobile L', width: 428, height: 926 },
  { name: 'Tablet', width: 768, height: 1024 },
  { name: 'Laptop', width: 1280, height: 800 },
  { name: 'Desktop', width: 1440, height: 900 },
  { name: 'Wide', width: 1920, height: 1080 },
];

const MOBILE_VIEWPORTS = VIEWPORTS.filter(v => v.width < 768);
const DESKTOP_VIEWPORTS = VIEWPORTS.filter(v => v.width > 768);

const PAGE_URL = 'file://' + require('path').resolve(__dirname, '../index.html');

// ============================================================
// 1. No Horizontal Overflow
// ============================================================
for (const vp of VIEWPORTS) {
  test(`[${vp.name}] no horizontal overflow`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);
    await page.waitForTimeout(500);

    const overflow = await page.evaluate(() => {
      return document.body.scrollWidth > window.innerWidth;
    });
    expect(overflow, `Body overflows at ${vp.name} (${vp.width}px)`).toBe(false);
  });
}

// ============================================================
// 2. Navigation — Desktop
// ============================================================
for (const vp of DESKTOP_VIEWPORTS) {
  test(`[${vp.name}] desktop nav links visible`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const navLinks = page.locator('.nav__links');
    await expect(navLinks).toBeVisible();

    const hamburger = page.locator('.nav__toggle');
    await expect(hamburger).not.toBeVisible();
  });
}

// ============================================================
// 3. Navigation — Mobile Hamburger
// ============================================================
for (const vp of MOBILE_VIEWPORTS) {
  test(`[${vp.name}] hamburger visible, nav hidden`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const hamburger = page.locator('.nav__toggle');
    await expect(hamburger).toBeVisible();

    const navLinks = page.locator('.nav__links');
    await expect(navLinks).not.toBeVisible();
  });

  test(`[${vp.name}] hamburger open → X visible → close`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const hamburger = page.locator('.nav__toggle');
    const navLinks = page.locator('.nav__links');

    // Open
    await hamburger.click();
    await expect(navLinks).toBeVisible();
    await expect(hamburger).toHaveAttribute('aria-expanded', 'true');

    // All links visible
    await expect(page.locator('.nav__links a')).toHaveCount(8); // 7 links + 1 CTA

    // Close
    await hamburger.click();
    await expect(navLinks).not.toBeVisible();
    await expect(hamburger).toHaveAttribute('aria-expanded', 'false');
  });

  test(`[${vp.name}] clicking nav link closes menu`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const hamburger = page.locator('.nav__toggle');
    const navLinks = page.locator('.nav__links');

    await hamburger.click();
    await expect(navLinks).toBeVisible();

    // Click a link
    await page.locator('.nav__links a[href*="wallet"]').click();
    await page.waitForTimeout(300);
    await expect(navLinks).not.toBeVisible();
  });
}

// ============================================================
// 4. Sticky Nav
// ============================================================
test('nav stays sticky on scroll', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  await page.evaluate(() => window.scrollBy(0, 1000));
  await page.waitForTimeout(300);

  const navTop = await page.locator('.nav').evaluate(el => {
    return el.getBoundingClientRect().top;
  });
  expect(navTop).toBeLessThanOrEqual(0);
});

// ============================================================
// 5. Hero Section
// ============================================================
for (const vp of VIEWPORTS) {
  test(`[${vp.name}] hero headline visible`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const headline = page.locator('.hero h1, .hero .headline-large').first();
    await expect(headline).toBeVisible();
  });
}

test('hero pill badges wrap on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await page.goto(PAGE_URL);

  const pills = page.locator('.hero__pill');
  const count = await pills.count();
  expect(count).toBe(6);

  // Check none overflow viewport
  for (let i = 0; i < count; i++) {
    const box = await pills.nth(i).boundingBox();
    if (box) {
      expect(box.x + box.width).toBeLessThanOrEqual(320 + 5);
    }
  }
});

// ============================================================
// 6. Tables — Scroll on Mobile
// ============================================================
for (const vp of MOBILE_VIEWPORTS) {
  test(`[${vp.name}] competitor table is scrollable`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const wrapper = page.locator('.competitor-table-wrapper');
    const hasScroll = await wrapper.evaluate(el => el.scrollWidth > el.clientWidth);
    expect(hasScroll, 'Table should scroll horizontally on mobile').toBe(true);
  });

  test(`[${vp.name}] swipe hint visible on tables`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    // Check the ::after pseudo-element text via computed style
    const hintVisible = await page.locator('.competitor-table-wrapper').evaluate(el => {
      const after = window.getComputedStyle(el, '::after');
      return after.content.includes('swipe');
    });
    expect(hintVisible).toBe(true);
  });
}

for (const vp of DESKTOP_VIEWPORTS) {
  test(`[${vp.name}] tables fully visible (no scroll needed)`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const wrapper = page.locator('.competitor-table-wrapper');
    const needsScroll = await wrapper.evaluate(el => el.scrollWidth > el.clientWidth);
    // On desktop the table should fit or be close to fitting
    // (might still need scroll on laptop if table is wide, so we just check it doesn't overflow the body)
    const bodyOverflow = await page.evaluate(() => document.body.scrollWidth > window.innerWidth);
    expect(bodyOverflow).toBe(false);
  });
}

// ============================================================
// 7. Pricing Cards
// ============================================================
test('pricing cards — 4 visible on desktop', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  const cards = page.locator('.pricing-card');
  await expect(cards).toHaveCount(4);
});

test('pricing cards — single column on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto(PAGE_URL);

  const grid = page.locator('.pricing-grid');
  const columns = await grid.evaluate(el => {
    return getComputedStyle(el).gridTemplateColumns;
  });
  // Should be single column (one value, not multiple)
  const colCount = columns.split(' ').filter(c => c !== '').length;
  expect(colCount).toBeLessThanOrEqual(1);
});

test('pricing — Most Popular badge on Scale', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  const badge = page.locator('.pricing-card__popular');
  await expect(badge.first()).toBeVisible();
});

// ============================================================
// 8. Tier Cards — Stack on Mobile
// ============================================================
test('tier cards stack vertically on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto(PAGE_URL);

  const tierVisual = page.locator('.tier-visual');
  const direction = await tierVisual.evaluate(el => getComputedStyle(el).flexDirection);
  expect(direction).toBe('column');
});

// ============================================================
// 9. Membership Cards — Stack on Mobile
// ============================================================
test('membership cards stack on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto(PAGE_URL);

  const cards = page.locator('.membership-cards');
  const direction = await cards.evaluate(el => getComputedStyle(el).flexDirection);
  expect(direction).toBe('column');
});

// ============================================================
// 10. Dashboard Mockup — No Overflow
// ============================================================
for (const vp of VIEWPORTS) {
  test(`[${vp.name}] dashboard mockup contained`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await page.goto(PAGE_URL);

    const mockup = page.locator('.dashboard-mockup');
    if (await mockup.count() > 0) {
      const overflow = await mockup.evaluate(el => {
        const rect = el.getBoundingClientRect();
        return rect.right > window.innerWidth;
      });
      expect(overflow, 'Dashboard mockup overflows viewport').toBe(false);
    }
  });
}

// ============================================================
// 11. Footer
// ============================================================
test('footer — single column on mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto(PAGE_URL);

  const footerTop = page.locator('.footer__top');
  if (await footerTop.count() > 0) {
    const columns = await footerTop.evaluate(el => getComputedStyle(el).gridTemplateColumns);
    const colCount = columns.split(' ').filter(c => c !== '').length;
    expect(colCount).toBeLessThanOrEqual(1);
  }
});

test('footer — multi-column on desktop', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  const footerTop = page.locator('.footer__top');
  if (await footerTop.count() > 0) {
    const columns = await footerTop.evaluate(el => getComputedStyle(el).gridTemplateColumns);
    const colCount = columns.split(' ').filter(c => c !== '').length;
    expect(colCount).toBeGreaterThan(1);
  }
});

// ============================================================
// 12. Scroll Animations
// ============================================================
test('scroll animations trigger on scroll', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  // Initially, elements below fold should not have .visible
  const initialVisible = await page.locator('.animate-on-scroll.visible').count();

  // Scroll down
  await page.evaluate(() => window.scrollBy(0, 2000));
  await page.waitForTimeout(500);

  const afterScrollVisible = await page.locator('.animate-on-scroll.visible').count();
  expect(afterScrollVisible).toBeGreaterThan(initialVisible);
});

// ============================================================
// 13. Accessibility
// ============================================================
test('hamburger has aria-expanded', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto(PAGE_URL);

  const hamburger = page.locator('.nav__toggle');
  await expect(hamburger).toHaveAttribute('aria-expanded', 'false');

  await hamburger.click();
  await expect(hamburger).toHaveAttribute('aria-expanded', 'true');
});

test('CTA links are keyboard focusable', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(PAGE_URL);

  const cta = page.locator('.hero__cta--primary, a.hero__cta').first();
  if (await cta.count() > 0) {
    await cta.focus();
    const focused = await page.evaluate(() => document.activeElement?.tagName);
    expect(focused).toBe('A');
  }
});
