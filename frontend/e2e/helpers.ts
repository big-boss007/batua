import type { Page } from '@playwright/test';

export async function loginStorefront(page: Page, slug: string, phone: string) {
  await page.goto(`/s/${slug}`);
  await page.waitForLoadState('networkidle');

  const phoneInput = page.locator('input[type="tel"], input[placeholder="9876543210"]');
  if (await phoneInput.isVisible({ timeout: 3000 }).catch(() => false)) {
    await phoneInput.fill(phone);
    await page.getByRole('button', { name: 'View Rewards' }).click();
    await page.waitForLoadState('networkidle');
  }
}

export async function navigateAdminTab(page: Page, path: string, tabName: string) {
  await page.goto(path);
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(500);
  const tab = page.getByRole('tab', { name: tabName });
  if (await tab.isVisible({ timeout: 3000 }).catch(() => false)) {
    await tab.click();
    await page.waitForTimeout(1000);
  }
}

export async function clearStorefrontSession(page: Page, slug: string) {
  await page.goto(`/s/${slug}`);
  await page.evaluate(() => sessionStorage.clear());
  await page.reload();
  await page.waitForLoadState('networkidle');
}
