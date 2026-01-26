import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.describe('Profiles Management', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should display profiles page with heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Profiles', level: 1 })).toBeVisible();
  });

  test('should display tabs for All Profiles and Assign Skills', async ({ page }) => {
    await expect(page.getByRole('button', { name: /all profiles/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /assign skills/i })).toBeVisible();
  });

  test('should list all profiles with Main Profile first', async ({ page }) => {
    await expect(page.getByText('Main Profile')).toBeVisible();
    await expect(page.getByText('React 19 Stack')).toBeVisible();
  });

  test('should display profile type badges', async ({ page }) => {
    // Badge has textTransform: capitalize, so text shows as "Project" and "User"
    await expect(page.locator('.badge').filter({ hasText: /^project$/i }).first()).toBeVisible();
    await expect(page.locator('.badge').filter({ hasText: /^user$/i }).first()).toBeVisible();
  });

  test('should display default badge for default user profile', async ({ page }) => {
    await expect(page.getByText('Default')).toBeVisible();
  });

  test('should show create profile button', async ({ page }) => {
    await expect(page.getByRole('button', { name: /create profile/i })).toBeVisible();
  });
});

test.describe('Profiles - Create Profile', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should open create profile form', async ({ page }) => {
    await page.getByRole('button', { name: /create profile/i }).click();
    await expect(page.getByRole('heading', { name: 'Create Profile' })).toBeVisible();
    // Form fields (labels not associated with htmlFor, use placeholder instead)
    await expect(page.getByPlaceholder('react-stack')).toBeVisible();
    await expect(page.getByPlaceholder('React 19 Stack')).toBeVisible();
  });

  test('should create new profile with valid data', async ({ page }) => {
    await page.getByRole('button', { name: /create profile/i }).click();

    // Use placeholders since labels aren't associated with htmlFor
    // Note: Type selector was removed - all new profiles are Project type
    await page.getByPlaceholder('react-stack').fill('test-profile');
    await page.getByPlaceholder('React 19 Stack').fill('Test Profile');
    await page.getByPlaceholder(/skills for/i).fill('A test profile');

    // Click the Create Profile button in the form (not the one that opens the form)
    await page.locator('.card').getByRole('button', { name: /create profile/i }).click();

    // Should show success and new profile in list
    await expect(page.getByText('Test Profile')).toBeVisible();
  });

  test('should cancel profile creation', async ({ page }) => {
    await page.getByRole('button', { name: /create profile/i }).click();
    await page.getByRole('button', { name: /cancel/i }).click();

    // Form should be hidden (check that the card with form is not visible)
    await expect(page.getByPlaceholder('react-stack')).not.toBeVisible();
  });
});

test.describe('Profiles - Edit Profile', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should open edit form for profile', async ({ page }) => {
    // Find the list-item containing "React 19 Stack"
    const profileRow = page.locator('.list-item').filter({ hasText: 'React 19 Stack' });
    await profileRow.getByRole('button', { name: /edit/i }).click();

    await expect(page.getByRole('heading', { name: 'Edit Profile' })).toBeVisible();
    // Use placeholder since labels aren't associated
    await expect(page.getByPlaceholder('React 19 Stack')).toHaveValue('React 19 Stack');
  });

  test('should update profile', async ({ page }) => {
    const profileRow = page.locator('.list-item').filter({ hasText: 'React 19 Stack' });
    await profileRow.getByRole('button', { name: /edit/i }).click();

    await page.getByPlaceholder('React 19 Stack').fill('Updated React Stack');
    await page.getByRole('button', { name: /save changes/i }).click();

    await expect(page.getByText('Updated React Stack')).toBeVisible();
  });
});

test.describe('Profiles - Delete Profile', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should delete project profile with confirmation', async ({ page }) => {
    const profileRow = page.locator('.list-item').filter({ hasText: 'React 19 Stack' });

    page.on('dialog', dialog => dialog.accept());
    await profileRow.getByRole('button', { name: /delete/i }).click();

    await expect(page.getByText('React 19 Stack')).not.toBeVisible();
  });

  test('should cancel profile deletion', async ({ page }) => {
    const profileRow = page.locator('.list-item').filter({ hasText: 'React 19 Stack' });

    page.on('dialog', dialog => dialog.dismiss());
    await profileRow.getByRole('button', { name: /delete/i }).click();

    await expect(page.getByText('React 19 Stack')).toBeVisible();
  });

  test('should not show delete button for Main Profile', async ({ page }) => {
    // Main Profile should not have a delete button (protected)
    const mainProfileRow = page.locator('.list-item').filter({ hasText: 'Main Profile' });

    // The Main Profile row should exist
    await expect(mainProfileRow).toBeVisible();

    // Main Profile should NOT have a delete button
    await expect(mainProfileRow.getByRole('button', { name: /delete/i })).not.toBeVisible();
  });
});

test.describe('Profiles - Assign Skills', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
    await page.getByRole('button', { name: /assign skills/i }).click();
  });

  test('should display profile selector', async ({ page }) => {
    await expect(page.getByText('Select Profile')).toBeVisible();
    await expect(page.getByText('React 19 Stack')).toBeVisible();
    await expect(page.getByText('Main Profile')).toBeVisible();
  });

  test('should show skill checkboxes when profile is selected', async ({ page }) => {
    await page.getByText('React 19 Stack').click();

    await expect(page.getByText('Assign Skills to React 19 Stack')).toBeVisible();
    await expect(page.getByRole('checkbox')).toHaveCount(4); // 4 skills in mock
  });

  test('should show pre-selected skills for profile', async ({ page }) => {
    await page.getByText('React 19 Stack').click();

    // react-patterns and typescript-best-practices should be checked
    const reactCheckbox = page.locator('label', { hasText: 'react-patterns' }).getByRole('checkbox');
    const tsCheckbox = page.locator('label', { hasText: 'typescript-best-practices' }).getByRole('checkbox');

    await expect(reactCheckbox).toBeChecked();
    await expect(tsCheckbox).toBeChecked();
  });

  test('should toggle skill assignment', async ({ page }) => {
    await page.getByText('React 19 Stack').click();

    const playwrightCheckbox = page.locator('label', { hasText: 'playwright' }).getByRole('checkbox');

    await expect(playwrightCheckbox).not.toBeChecked();
    await playwrightCheckbox.click();
    await expect(playwrightCheckbox).toBeChecked();
  });

  test('should save skill assignment', async ({ page }) => {
    await page.getByText('React 19 Stack').click();

    const playwrightCheckbox = page.locator('label', { hasText: 'playwright' }).getByRole('checkbox');
    await playwrightCheckbox.click();

    await page.getByRole('button', { name: /save assignment/i }).click();

    // Should show success toast
    await expect(page.getByText(/skills assigned/i)).toBeVisible();
  });
});

test.describe('Profiles - Set Default', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should show Set Default button only for user profiles without default', async ({ page }) => {
    // Main Profile is already default, so no Set Default button
    const mainProfileRow = page.locator('.list-item').filter({ hasText: 'Main Profile' });
    await expect(mainProfileRow.getByRole('button', { name: /set default/i })).not.toBeVisible();

    // React Stack is project type, so no Set Default button
    const reactProfileRow = page.locator('.list-item').filter({ hasText: 'React 19 Stack' });
    await expect(reactProfileRow.getByRole('button', { name: /set default/i })).not.toBeVisible();
  });
});

test.describe('Profiles - Navigation', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should navigate to profiles page from sidebar', async ({ page }) => {
    await page.getByRole('link', { name: /profiles/i }).click();
    await expect(page).toHaveURL(/profiles/);
    await expect(page.getByRole('heading', { name: 'Profiles', level: 1 })).toBeVisible();
  });
});

test.describe('Profiles - Info Section', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');
  });

  test('should display how profiles work info', async ({ page }) => {
    await expect(page.getByText('How Profiles Work')).toBeVisible();
    await expect(page.getByText(/user profiles/i)).toBeVisible();
    await expect(page.getByText(/project profiles/i)).toBeVisible();
  });

  test('should display CLI usage example', async ({ page }) => {
    await expect(page.getByText(/rhinolabs profile install/i)).toBeVisible();
  });
});
