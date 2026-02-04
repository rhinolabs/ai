import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.describe('Skills Management', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should display skills page with heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /skills/i, level: 1 })).toBeVisible();
  });

  test('should display tabs', async ({ page }) => {
    await expect(page.getByRole('button', { name: /rhinolabs skills/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /browse/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /sources/i })).toBeVisible();
  });

  test('should list all skills', async ({ page }) => {
    // Use .first() for skills that might appear in multiple places (list item + badge, etc.)
    await expect(page.getByText('rhinolabs-standards').first()).toBeVisible();
    await expect(page.getByText('react-patterns').first()).toBeVisible();
    await expect(page.getByText('typescript-best-practices').first()).toBeVisible();
    await expect(page.getByText('playwright').first()).toBeVisible();
  });

  test('should display skill categories as badges', async ({ page }) => {
    // Use .first() because multiple skills can have the same category
    await expect(page.locator('.category-badge').filter({ hasText: 'corporate' }).first()).toBeVisible();
    await expect(page.locator('.category-badge').filter({ hasText: 'frontend' }).first()).toBeVisible();
    await expect(page.locator('.category-badge').filter({ hasText: 'testing' }).first()).toBeVisible();
  });

  test('should show create skill button', async ({ page }) => {
    await expect(page.getByRole('button', { name: /create skill/i })).toBeVisible();
  });

  test('should show category filter dropdown', async ({ page }) => {
    await expect(page.getByRole('combobox')).toBeVisible();
    await expect(page.locator('select').filter({ hasText: 'All Categories' })).toBeVisible();
  });

  test('should filter skills by category using dropdown', async ({ page }) => {
    await page.locator('select').filter({ hasText: 'All Categories' }).selectOption('frontend');

    await expect(page.getByText('react-patterns')).toBeVisible();
    await expect(page.getByText('typescript-best-practices')).toBeVisible();
    // corporate skill should not be visible
    await expect(page.getByText('rhinolabs-standards')).not.toBeVisible();
  });

  test('should show stats summary', async ({ page }) => {
    await expect(page.locator('.summary-box').filter({ hasText: 'Total' })).toBeVisible();
    await expect(page.locator('.summary-box').filter({ hasText: 'Enabled' })).toBeVisible();
  });
});

test.describe('Skills - View Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show View button for each skill', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await expect(skillItem.getByRole('button', { name: /view/i })).toBeVisible();
  });

  test('should open skill detail when clicking View', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /view/i }).click();

    // Should show the skill name as heading
    await expect(page.getByRole('heading', { name: /react-patterns/i })).toBeVisible();
  });

  test('should show file tree when viewing a skill', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /view/i }).click();

    // Should show Files sidebar - use exact match to avoid matching "Profiles"
    await expect(page.getByText('Files', { exact: true })).toBeVisible();
  });

  test('should show Back button in detail view', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /view/i }).click();

    await expect(page.getByRole('button', { name: /back/i })).toBeVisible();
  });

  test('should return to skill list when Back is clicked', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /view/i }).click();

    await page.getByRole('button', { name: /back/i }).click();

    // Should be back at the list
    await expect(page.getByRole('heading', { name: /skills/i, level: 1 })).toBeVisible();
    await expect(page.getByText('rhinolabs-standards').first()).toBeVisible();
  });
});

test.describe('Skills - Toggle Enable/Disable', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show toggle checkbox for each skill', async ({ page }) => {
    // Toggle switches use checkboxes inside .toggle-switch
    const toggles = page.locator('.toggle-switch input[type="checkbox"]');
    await expect(toggles).toHaveCount(4); // 4 skills in mock
  });

  test('should toggle skill when checkbox clicked', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    const toggle = skillItem.locator('.toggle-switch input[type="checkbox"]');
    // Click the toggle wrapper/label since input may be visually hidden
    const toggleWrapper = skillItem.locator('.toggle-switch');

    await expect(toggle).toBeChecked();
    await toggleWrapper.click();

    // Should show success toast
    await expect(page.getByText(/disabled/i)).toBeVisible();
  });
});

test.describe('Skills - Create New Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should open create skill form', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();

    // Should show Create Skill page (not modal)
    await expect(page.getByRole('heading', { name: /create skill/i })).toBeVisible();
  });

  test('should show form fields', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();

    await expect(page.getByPlaceholder('my-skill')).toBeVisible();
    await expect(page.getByPlaceholder('My Skill')).toBeVisible();
    await expect(page.getByPlaceholder(/brief description/i)).toBeVisible();
  });

  test('should show category select', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();

    await expect(page.locator('select')).toBeVisible();
  });

  test('should create new skill with valid data', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();

    await page.getByPlaceholder('my-skill').fill('my-custom-skill');
    await page.getByPlaceholder('My Skill').fill('My Custom Skill');
    await page.getByPlaceholder(/brief description/i).fill('A custom skill for testing');

    // Click Create button
    await page.getByRole('button', { name: /^create$/i }).click();

    // Should show success toast
    await expect(page.getByText(/skill created/i)).toBeVisible();
  });

  test('should show validation error for empty required fields', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();

    // Click create without filling fields
    await page.getByRole('button', { name: /^create$/i }).click();

    await expect(page.getByText(/required/i)).toBeVisible();
  });

  test('should cancel skill creation', async ({ page }) => {
    await page.getByRole('button', { name: /create skill/i }).click();
    await page.getByPlaceholder('my-skill').fill('test-skill');

    await page.getByRole('button', { name: /cancel/i }).click();

    // Should be back at the list
    await expect(page.getByRole('heading', { name: /skills/i, level: 1 })).toBeVisible();
  });
});

test.describe('Skills - Edit/Open in IDE', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show Edit button for each skill', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await expect(skillItem.getByRole('button', { name: /edit/i })).toBeVisible();
  });

  test('should open skill in IDE when Edit is clicked', async ({ page }) => {
    const logs: string[] = [];
    page.on('console', (msg) => {
      if (msg.text().includes('TauriMock')) {
        logs.push(msg.text());
      }
    });

    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /edit/i }).click();

    // Should show success toast
    await expect(page.getByText(/opened in/i)).toBeVisible();

    // Verify mock was called
    expect(logs.some((log) => log.includes('open_skill_in_ide'))).toBe(true);
  });

  test('should show error when no IDE is available', async ({ page }) => {
    await page.addInitScript(() => {
      const originalInvoke = (window as any).__TAURI_INTERNALS__.invoke;
      (window as any).__TAURI_INTERNALS__.invoke = async (cmd: string, args?: unknown) => {
        if (cmd === 'list_available_ides') {
          return [
            { id: 'vscode', name: 'VS Code', command: 'code', available: false },
          ];
        }
        return originalInvoke(cmd, args);
      };
    });

    await page.reload();
    await page.waitForLoadState('networkidle');

    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /edit/i }).click();

    await expect(page.getByText(/no ide available/i)).toBeVisible();
  });
});

test.describe('Skills - Delete Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should not show delete button for built-in skills', async ({ page }) => {
    // Built-in skills (isCustom: false, no sourceId) should not have delete
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await expect(skillItem.getByRole('button', { name: /delete/i })).not.toBeVisible();
  });
});

test.describe('Skills - Sources Tab', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should switch to Sources tab', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    // Should show Add Source button
    await expect(page.getByRole('button', { name: /add source/i })).toBeVisible();
  });

  test('should list existing sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    await expect(page.getByText('Anthropic Official')).toBeVisible();
    await expect(page.getByText('Community Skills')).toBeVisible();
  });

  test('should show auto-fetch badge for fetchable sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    await expect(page.locator('.category-badge').filter({ hasText: /auto-fetch/i }).first()).toBeVisible();
  });

  test('should show browse-only badge for non-fetchable sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    await expect(page.locator('.category-badge').filter({ hasText: /browse only/i })).toBeVisible();
  });

  test('should open add source form', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();
    await page.getByRole('button', { name: /add source/i }).click();

    // Should show form heading
    await expect(page.getByRole('heading', { name: /add skill source/i })).toBeVisible();
  });

  test('should show form fields in add source', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();
    await page.getByRole('button', { name: /add source/i }).click();

    await expect(page.getByPlaceholder('my-source')).toBeVisible();
    await expect(page.getByPlaceholder('My Skills Repository')).toBeVisible();
    await expect(page.getByPlaceholder(/github/i)).toBeVisible();
  });

  test('should add a new source', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();
    await page.getByRole('button', { name: /add source/i }).click();

    await page.getByPlaceholder('my-source').fill('my-custom-source');
    await page.getByPlaceholder('My Skills Repository').fill('My Custom Source');
    await page.getByPlaceholder(/github/i).fill('https://github.com/test/skills');

    await page.getByRole('button', { name: /^add source$/i }).click();

    // Should show success toast or return to sources list
    await expect(page.getByText(/source added/i).or(page.getByRole('button', { name: /add source/i }))).toBeVisible();
  });

  test('should toggle source enabled state', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    const sourceItem = page.locator('.list-item').first();
    const toggle = sourceItem.locator('.toggle-switch input[type="checkbox"]');
    // Click the toggle wrapper/label since input may be visually hidden
    const toggleWrapper = sourceItem.locator('.toggle-switch');

    await expect(toggle).toBeChecked();
    await toggleWrapper.click();

    // Should show toast
    await expect(page.getByText(/disabled/i)).toBeVisible();
  });

  test('should remove source with confirmation', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    const sourceItem = page.locator('.list-item').filter({ hasText: 'Community Skills' });

    // Click remove and accept dialog
    page.on('dialog', dialog => dialog.accept());
    await sourceItem.getByRole('button', { name: /remove/i }).click();

    // Should show success
    await expect(page.getByText(/source removed/i)).toBeVisible();
  });
});

test.describe('Skills - Browse Tab', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should switch to Browse tab', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Should show source selector
    await expect(page.locator('select').filter({ hasText: /select source/i })).toBeVisible();
  });

  test('should load remote skills when fetchable source is selected', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Select fetchable source
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('anthropic-official');

    // Should show remote skills
    await expect(page.getByText('Remote Skill 1')).toBeVisible();
    await expect(page.getByText('Remote Skill 2')).toBeVisible();
  });

  test('should show "In Plugin" badge for installed skills', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('anthropic-official');

    // Remote Skill 2 is marked as installed in mock
    await expect(page.locator('.status-badge').filter({ hasText: /in plugin/i })).toBeVisible();
  });

  test('should show Preview and Add buttons for non-installed skills', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    await expect(remoteSkill.getByRole('button', { name: /preview/i })).toBeVisible();
    await expect(remoteSkill.getByRole('button', { name: /add/i })).toBeVisible();
  });

  test('should show browse-only message for non-fetchable sources', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Select non-fetchable source
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('community-skills');

    // Should show browse-only message
    await expect(page.getByText(/browse only source/i)).toBeVisible();
  });

  test('should show category popup when adding remote skill', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    const addButton = remoteSkill.getByRole('button', { name: /add/i });

    // Click add button
    await addButton.click();

    // Should show category popup
    await expect(page.getByRole('heading', { name: /change category/i })).toBeVisible();
    await expect(page.locator('select').filter({ hasText: /custom/i })).toBeVisible();
  });

  test('should add remote skill with selected category', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    await remoteSkill.getByRole('button', { name: /add/i }).click();

    // Select category in popup
    await page.locator('.card select').selectOption('frontend');

    // Click Save
    await page.getByRole('button', { name: /save/i }).click();

    // Should show success toast
    await expect(page.getByText(/added/i)).toBeVisible();
  });
});

test.describe('Skills - Change Category', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show Category button for each skill', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await expect(skillItem.getByRole('button', { name: /category/i })).toBeVisible();
  });

  test('should open category popup when clicking Category button', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /category/i }).click();

    // Should show category popup
    await expect(page.getByRole('heading', { name: /change category/i })).toBeVisible();
  });

  test('should change skill category', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /category/i }).click();

    // Select new category
    await page.locator('.card select').selectOption('testing');

    // Click Save
    await page.getByRole('button', { name: /save/i }).click();

    // Should show success toast
    await expect(page.getByText(/category changed/i)).toBeVisible();
  });

  test('should close popup when clicking Cancel', async ({ page }) => {
    const skillItem = page.locator('.list-item').filter({ hasText: 'react-patterns' });
    await skillItem.getByRole('button', { name: /category/i }).click();

    // Click Cancel
    await page.getByRole('button', { name: /cancel/i }).click();

    // Popup should be closed
    await expect(page.getByRole('heading', { name: /change category/i })).not.toBeVisible();
  });
});

test.describe('Skills - Browse Pagination & Search', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
    // Navigate to Browse tab and select the large source
    await page.getByRole('button', { name: /browse/i }).click();
    await page.locator('select').filter({ hasText: /select source/i }).selectOption('skills-sh-source');
    // Wait for skills to load
    await expect(page.getByText(/120 skills/)).toBeVisible();
  });

  test('should show search input', async ({ page }) => {
    await expect(page.getByPlaceholder(/search skills/i)).toBeVisible();
  });

  test('should show total skills count', async ({ page }) => {
    await expect(page.getByText('120 skills')).toBeVisible();
  });

  test('should show pagination controls', async ({ page }) => {
    await expect(page.getByRole('button', { name: 'Prev', exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Next', exact: true })).toBeVisible();
    await expect(page.getByText(/page 1 of 3/i)).toBeVisible();
  });

  test('should show only 50 skills per page', async ({ page }) => {
    const items = page.locator('.list-item');
    await expect(items).toHaveCount(50);
  });

  test('should disable Prev button on first page', async ({ page }) => {
    await expect(page.getByRole('button', { name: 'Prev', exact: true })).toBeDisabled();
  });

  test('should navigate to next page', async ({ page }) => {
    await page.getByRole('button', { name: 'Next', exact: true }).click();
    await expect(page.getByText(/page 2 of 3/i)).toBeVisible();
    // Should show Skill 50 (first of page 2)
    await expect(page.getByText('Skill 50').first()).toBeVisible();
  });

  test('should navigate to previous page', async ({ page }) => {
    // Go to page 2
    await page.getByRole('button', { name: 'Next', exact: true }).click();
    await expect(page.getByText(/page 2 of 3/i)).toBeVisible();
    // Go back to page 1
    await page.getByRole('button', { name: 'Prev', exact: true }).click();
    await expect(page.getByText(/page 1 of 3/i)).toBeVisible();
    await expect(page.getByText('Skill 0').first()).toBeVisible();
  });

  test('should navigate to last page with fewer items', async ({ page }) => {
    // Go to page 3 (last page: 120 skills, 50/page → page 3 has 20 skills)
    await page.getByRole('button', { name: 'Next', exact: true }).click();
    await page.getByRole('button', { name: 'Next', exact: true }).click();
    await expect(page.getByText(/page 3 of 3/i)).toBeVisible();
    const items = page.locator('.list-item');
    await expect(items).toHaveCount(20);
    // Next should be disabled on last page
    await expect(page.getByRole('button', { name: 'Next', exact: true })).toBeDisabled();
  });

  test('should filter skills by search query', async ({ page }) => {
    await page.getByPlaceholder(/search skills/i).fill('Skill 10');
    // Should match: Skill 10, Skill 100-109 = 11 results
    await expect(page.getByText(/11 skills matching/i)).toBeVisible();
  });

  test('should show no results message for empty search', async ({ page }) => {
    await page.getByPlaceholder(/search skills/i).fill('xyznonexistent');
    await expect(page.getByText(/no skills match/i)).toBeVisible();
  });

  test('should reset to page 1 when searching', async ({ page }) => {
    // Go to page 2
    await page.getByRole('button', { name: 'Next', exact: true }).click();
    await expect(page.getByText(/page 2 of 3/i)).toBeVisible();
    // Search for something with >50 results to keep pagination visible
    // "Skill 1" matches: Skill 1, Skill 10-19, Skill 100-119 = 22 items → but also Skill 1x descriptions
    // Use "owner-" which appears in all descriptions → all 120 match → still page 1
    await page.getByPlaceholder(/search skills/i).fill('owner-');
    await expect(page.getByText(/page 1 of/i)).toBeVisible();
  });

  test('should reset search when changing source', async ({ page }) => {
    // Type a search
    await page.getByPlaceholder(/search skills/i).fill('something');
    // Switch source
    await page.locator('select').filter({ hasText: /skills\.sh/i }).selectOption('anthropic-official');
    // Search should be cleared - check that Remote Skill 1 appears (no filter)
    await expect(page.getByText('Remote Skill 1')).toBeVisible();
  });

  test('should show skills-sh source in dropdown', async ({ page }) => {
    // Options inside <select> are hidden in Playwright; verify the select has the option value
    const select = page.locator('select').filter({ hasText: /select source/i });
    await expect(select.locator('option[value="skills-sh-source"]')).toHaveCount(1);
  });
});
