import { test, expect } from '@playwright/test';
import { clearStorefrontSession } from './helpers';

const SLUG = 'desidrapes';
const PHONE = '9000000005';
const VALID_CODE = 'MEERA5';
const INVALID_CODE = 'XXXXXX-INVALID';

test.describe('Storefront — Referrer Flow', () => {
  test('should show phone input on refer page', async ({ page }) => {
    await clearStorefrontSession(page, SLUG);
    await page.goto(`/s/${SLUG}/refer`);
    await page.waitForLoadState('networkidle');

    const phoneInput = page.locator('input[type="tel"], input[placeholder="9876543210"]');
    await expect(phoneInput).toBeVisible({ timeout: 5000 });
  });

  test('should show referral card after phone login', async ({ page }) => {
    await clearStorefrontSession(page, SLUG);
    await page.goto(`/s/${SLUG}/refer`);
    await page.waitForLoadState('networkidle');

    const phoneInput = page.locator('input[type="tel"], input[placeholder="9876543210"]');
    await phoneInput.fill(PHONE);

    const submitBtn = page.getByRole('button').filter({ hasText: /view|check|submit/i });
    await submitBtn.click();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const hasCard = await page.getByText(/Refer & Earn/i).isVisible({ timeout: 3000 }).catch(() => false);
    const hasCode = await page.getByText(VALID_CODE).isVisible({ timeout: 1000 }).catch(() => false);
    expect(hasCard || hasCode).toBeTruthy();
  });

  test('should show error for unknown phone', async ({ page }) => {
    await clearStorefrontSession(page, SLUG);
    await page.goto(`/s/${SLUG}/refer`);
    await page.waitForLoadState('networkidle');

    const phoneInput = page.locator('input[type="tel"], input[placeholder="9876543210"]');
    await phoneInput.fill('1111111111');

    const submitBtn = page.getByRole('button').filter({ hasText: /view|check|submit/i });
    await submitBtn.click();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const hasError = await page.getByText(/not found|no account|no referral|no rewards/i).first().isVisible({ timeout: 3000 }).catch(() => false);
    expect(hasError).toBeTruthy();
  });
});

test.describe('Storefront — Referee Landing Page', () => {
  test('should show referral invitation with valid code', async ({ page }) => {
    await page.goto(`/s/${SLUG}/refer/${VALID_CODE}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const hasInvite = await page.getByText(/invited|earn|reward/i).first().isVisible({ timeout: 3000 }).catch(() => false);
    expect(hasInvite).toBeTruthy();
  });

  test('should display the referral code', async ({ page }) => {
    await page.goto(`/s/${SLUG}/refer/${VALID_CODE}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    await expect(page.getByText(VALID_CODE).first()).toBeVisible({ timeout: 3000 });
  });

  test('should show shop action', async ({ page }) => {
    await page.goto(`/s/${SLUG}/refer/${VALID_CODE}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const hasShop = await page.getByRole('link', { name: /shop/i }).isVisible({ timeout: 3000 }).catch(() => false);
    const hasButton = await page.getByRole('button', { name: /shop/i }).isVisible({ timeout: 1000 }).catch(() => false);
    expect(hasShop || hasButton).toBeTruthy();
  });

  test('should show error for invalid code', async ({ page }) => {
    await page.goto(`/s/${SLUG}/refer/${INVALID_CODE}`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    await expect(page.getByText(/not found/i).first()).toBeVisible({ timeout: 3000 });
  });
});
