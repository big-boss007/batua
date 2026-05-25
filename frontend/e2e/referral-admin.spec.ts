import { test, expect } from '@playwright/test';
import { navigateAdminTab } from './helpers';

test.describe('Referral Admin — Program Tab', () => {
  test('should display referral program settings', async ({ page }) => {
    await navigateAdminTab(page, '/admin/referrals', 'Program');

    await expect(page.getByText('Referrer Reward')).toBeVisible();
    await expect(page.getByText('Referee Reward')).toBeVisible();
  });

  test('should show Update Program button', async ({ page }) => {
    await navigateAdminTab(page, '/admin/referrals', 'Program');

    const btn = page.getByRole('button', { name: /Update Program|Create Program/i });
    await expect(btn).toBeVisible();
  });
});

test.describe('Referral Admin — Codes Tab', () => {
  test('should display referral codes list', async ({ page }) => {
    await navigateAdminTab(page, '/admin/referrals', 'Codes');
    await page.waitForTimeout(1500);

    const hasContent = await page.getByText(/MEERA|SANJAY|NISHA|code|no codes/i).first().isVisible({ timeout: 3000 }).catch(() => false);
    expect(hasContent).toBeTruthy();
  });
});

test.describe('Referral Admin — Analytics Tab', () => {
  test('should display analytics section', async ({ page }) => {
    await navigateAdminTab(page, '/admin/referrals', 'Analytics');
    await page.waitForTimeout(1500);

    const hasMetrics = await page.getByText(/codes|referrals|conversion/i).first().isVisible({ timeout: 3000 }).catch(() => false);
    expect(hasMetrics).toBeTruthy();
  });
});

test.describe('Referral Admin — Conversions Tab', () => {
  test('should display conversions section', async ({ page }) => {
    await navigateAdminTab(page, '/admin/referrals', 'Conversions');
    await page.waitForTimeout(1500);

    const hasData = await page.getByText(/referrer|no conversions/i).first().isVisible({ timeout: 3000 }).catch(() => false);
    expect(hasData).toBeTruthy();
  });
});
