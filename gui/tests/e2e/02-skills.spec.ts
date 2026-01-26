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
    await expect(page.getByRole('heading', { name: /skills/i })).toBeVisible();
  });

  test('should list all skills', async ({ page }) => {
    await expect(page.getByText('rhinolabs-standards')).toBeVisible();
    await expect(page.getByText('react-patterns')).toBeVisible();
    await expect(page.getByText('typescript-best-practices')).toBeVisible();
    await expect(page.getByText('playwright')).toBeVisible();
  });

  test('should display skill categories', async ({ page }) => {
    await expect(page.getByText(/corporate/i)).toBeVisible();
    await expect(page.getByText(/frontend/i)).toBeVisible();
    await expect(page.getByText(/testing/i)).toBeVisible();
  });

  test('should show add skill button', async ({ page }) => {
    await expect(page.getByRole('button', { name: /add skill/i })).toBeVisible();
  });

  test('should filter skills by category', async ({ page }) => {
    await page.getByRole('button', { name: /frontend/i }).click();

    await expect(page.getByText('react-patterns')).toBeVisible();
    await expect(page.getByText('typescript-best-practices')).toBeVisible();
    await expect(page.getByText('rhinolabs-standards')).not.toBeVisible();
  });

  test('should search skills by name', async ({ page }) => {
    await page.getByPlaceholder(/search/i).fill('react');

    await expect(page.getByText('react-patterns')).toBeVisible();
    await expect(page.getByText('typescript-best-practices')).not.toBeVisible();
  });
});

test.describe('Skills - View Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should open skill detail when clicking on skill', async ({ page }) => {
    await page.getByText('react-patterns').click();

    await expect(page.getByRole('heading', { name: /react-patterns/i })).toBeVisible();
    await expect(page.getByText(/react component composition/i)).toBeVisible();
  });

  test('should display skill content in detail view', async ({ page }) => {
    await page.getByText('react-patterns').click();

    await expect(page.getByTestId('skill-content')).toBeVisible();
  });

  test('should show back button in detail view', async ({ page }) => {
    await page.getByText('react-patterns').click();

    await expect(page.getByRole('button', { name: /back/i })).toBeVisible();

    await page.getByRole('button', { name: /back/i }).click();
    await expect(page.getByRole('heading', { name: /skills/i })).toBeVisible();
  });
});

test.describe('Skills - Toggle Enable/Disable', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show toggle switch for each skill', async ({ page }) => {
    const toggles = page.getByRole('switch');
    await expect(toggles).toHaveCount(4); // 4 skills in mock
  });

  test('should disable skill when toggle is clicked', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    const toggle = skillRow.getByRole('switch');

    await expect(toggle).toBeChecked();
    await toggle.click();
    await expect(toggle).not.toBeChecked();
  });

  test('should show disabled indicator after disabling', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    const toggle = skillRow.getByRole('switch');

    await toggle.click();
    await expect(skillRow).toHaveClass(/disabled/);
  });
});

test.describe('Skills - Create New Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should open create skill modal', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();

    await expect(page.getByRole('dialog')).toBeVisible();
    await expect(page.getByRole('heading', { name: /create skill/i })).toBeVisible();
  });

  test('should show required fields in create modal', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();

    await expect(page.getByLabel(/id/i)).toBeVisible();
    await expect(page.getByLabel(/name/i)).toBeVisible();
    await expect(page.getByLabel(/description/i)).toBeVisible();
    await expect(page.getByLabel(/category/i)).toBeVisible();
    await expect(page.getByLabel(/content/i)).toBeVisible();
  });

  test('should create new skill with valid data', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();

    await page.getByLabel(/id/i).fill('my-custom-skill');
    await page.getByLabel(/name/i).fill('My Custom Skill');
    await page.getByLabel(/description/i).fill('A custom skill for testing');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('# My Custom Skill\n\nThis is the content.');

    await page.getByRole('button', { name: /create/i }).click();

    // Modal should close
    await expect(page.getByRole('dialog')).not.toBeVisible();

    // New skill should appear in list
    await expect(page.getByText('My Custom Skill')).toBeVisible();
  });

  test('should show validation error for empty required fields', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByRole('button', { name: /create/i }).click();

    await expect(page.getByText(/id is required/i)).toBeVisible();
  });

  test('should show error for duplicate skill id', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();

    await page.getByLabel(/id/i).fill('react-patterns'); // Already exists
    await page.getByLabel(/name/i).fill('Duplicate');
    await page.getByLabel(/description/i).fill('Test');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('Content');

    await page.getByRole('button', { name: /create/i }).click();

    await expect(page.getByText(/already exists/i)).toBeVisible();
  });

  test('should cancel skill creation', async ({ page }) => {
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByLabel(/id/i).fill('test-skill');

    await page.getByRole('button', { name: /cancel/i }).click();

    await expect(page.getByRole('dialog')).not.toBeVisible();
    await expect(page.getByText('test-skill')).not.toBeVisible();
  });
});

test.describe('Skills - Edit Skill', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show edit button for each skill', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await expect(skillRow.getByRole('button', { name: /edit/i })).toBeVisible();
  });

  test('should open edit modal with pre-filled data', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /edit/i }).click();

    await expect(page.getByRole('dialog')).toBeVisible();
    await expect(page.getByLabel(/name/i)).toHaveValue('react-patterns');
  });

  test('should update skill after editing', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /edit/i }).click();

    await page.getByLabel(/description/i).fill('Updated description');
    await page.getByRole('button', { name: /save/i }).click();

    await expect(page.getByRole('dialog')).not.toBeVisible();
    await expect(page.getByText('Updated description')).toBeVisible();
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
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await expect(skillRow.getByRole('button', { name: /delete/i })).not.toBeVisible();
  });

  test('should show delete button only for custom skills', async ({ page }) => {
    // First create a custom skill
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByLabel(/id/i).fill('deletable-skill');
    await page.getByLabel(/name/i).fill('Deletable Skill');
    await page.getByLabel(/description/i).fill('This can be deleted');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('Content');
    await page.getByRole('button', { name: /create/i }).click();

    // Now check delete button is visible
    const skillRow = page.locator('[data-testid="skill-row-deletable-skill"]');
    await expect(skillRow.getByRole('button', { name: /delete/i })).toBeVisible();
  });

  test('should show confirmation dialog before deleting', async ({ page }) => {
    // Create custom skill first
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByLabel(/id/i).fill('to-delete');
    await page.getByLabel(/name/i).fill('To Delete');
    await page.getByLabel(/description/i).fill('Will be deleted');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('Content');
    await page.getByRole('button', { name: /create/i }).click();

    // Click delete
    const skillRow = page.locator('[data-testid="skill-row-to-delete"]');
    await skillRow.getByRole('button', { name: /delete/i }).click();

    await expect(page.getByRole('alertdialog')).toBeVisible();
    await expect(page.getByText(/are you sure/i)).toBeVisible();
  });

  test('should delete skill after confirmation', async ({ page }) => {
    // Create custom skill
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByLabel(/id/i).fill('will-delete');
    await page.getByLabel(/name/i).fill('Will Delete');
    await page.getByLabel(/description/i).fill('Bye');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('Content');
    await page.getByRole('button', { name: /create/i }).click();

    // Delete it
    const skillRow = page.locator('[data-testid="skill-row-will-delete"]');
    await skillRow.getByRole('button', { name: /delete/i }).click();
    await page.getByRole('button', { name: /confirm/i }).click();

    // Should be gone
    await expect(page.getByText('Will Delete')).not.toBeVisible();
  });

  test('should cancel deletion', async ({ page }) => {
    // Create custom skill
    await page.getByRole('button', { name: /add skill/i }).click();
    await page.getByLabel(/id/i).fill('keep-me');
    await page.getByLabel(/name/i).fill('Keep Me');
    await page.getByLabel(/description/i).fill('Do not delete');
    await page.getByLabel(/category/i).selectOption('custom');
    await page.getByLabel(/content/i).fill('Content');
    await page.getByRole('button', { name: /create/i }).click();

    // Try to delete but cancel
    const skillRow = page.locator('[data-testid="skill-row-keep-me"]');
    await skillRow.getByRole('button', { name: /delete/i }).click();
    await page.getByRole('button', { name: /cancel/i }).click();

    // Should still be there
    await expect(page.getByText('Keep Me')).toBeVisible();
  });
});

// ============================================
// File Tree View Tests
// ============================================
test.describe('Skills - File Tree View', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should show View button for each skill', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await expect(skillRow.getByRole('button', { name: /view/i })).toBeVisible();
  });

  test('should display file tree when viewing a skill', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    // Should show file sidebar with Files heading
    await expect(page.getByText('Files')).toBeVisible();

    // Should show SKILL.md file
    await expect(page.getByText('SKILL.md')).toBeVisible();
  });

  test('should display folder in file tree', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    // Should show examples folder
    await expect(page.getByText('examples')).toBeVisible();
  });

  test('should expand folder when clicked', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    // Click on examples folder
    await page.getByText('examples').click();

    // Should show files inside folder
    await expect(page.getByText('good.ts')).toBeVisible();
    await expect(page.getByText('bad.ts')).toBeVisible();
  });

  test('should display file content with syntax highlighting when selected', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    // SKILL.md should be auto-selected
    // Should show content in code viewer
    const codeViewer = page.locator('pre').first();
    await expect(codeViewer).toBeVisible();
  });

  test('should show Back button in detail view', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    await expect(page.getByRole('button', { name: /back/i })).toBeVisible();
  });

  test('should return to skill list when Back is clicked', async ({ page }) => {
    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /view/i }).click();

    await page.getByRole('button', { name: /back/i }).click();

    // Should be back at the list
    await expect(page.getByRole('heading', { name: /skills/i })).toBeVisible();
    await expect(page.getByText('rhinolabs-standards')).toBeVisible();
  });
});

// ============================================
// Open in IDE Tests
// ============================================
test.describe('Skills - Open in IDE', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should open skill in IDE when Edit is clicked', async ({ page }) => {
    const logs: string[] = [];
    page.on('console', (msg) => {
      if (msg.text().includes('TauriMock')) {
        logs.push(msg.text());
      }
    });

    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /edit/i }).click();

    // Should show success toast
    await expect(page.getByText(/opened in/i)).toBeVisible();

    // Verify mock was called
    expect(logs.some((log) => log.includes('open_skill_in_ide'))).toBe(true);
  });

  test('should show error when no IDE is available for skill edit', async ({ page }) => {
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

    const skillRow = page.locator('[data-testid="skill-row-react-patterns"]');
    await skillRow.getByRole('button', { name: /edit/i }).click();

    await expect(page.getByText(/no ide available/i)).toBeVisible();
  });
});

// ============================================
// Skill Sources Tab Tests
// ============================================
test.describe('Skills - Sources Tab', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should display Sources tab', async ({ page }) => {
    await expect(page.getByRole('button', { name: /sources/i })).toBeVisible();
  });

  test('should switch to Sources tab', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    // Should show Add Source button
    await expect(page.getByRole('button', { name: /add source/i })).toBeVisible();
  });

  test('should list existing sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    // Should show the mock sources
    await expect(page.getByText('Anthropic Official')).toBeVisible();
    await expect(page.getByText('Community Skills')).toBeVisible();
  });

  test('should show fetchable badge for auto-fetch sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    await expect(page.getByText(/auto-fetch/i)).toBeVisible();
  });

  test('should show browse-only badge for non-fetchable sources', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    await expect(page.getByText(/browse only/i)).toBeVisible();
  });

  test('should open add source form', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();
    await page.getByRole('button', { name: /add source/i }).click();

    // Should show form fields
    await expect(page.getByLabel(/^id$/i)).toBeVisible();
    await expect(page.getByLabel(/name/i)).toBeVisible();
    await expect(page.getByLabel(/url/i)).toBeVisible();
  });

  test('should add a new source', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();
    await page.getByRole('button', { name: /add source/i }).click();

    await page.getByLabel(/^id$/i).fill('my-source');
    await page.getByLabel(/name/i).fill('My Custom Source');
    await page.getByLabel(/url/i).fill('https://github.com/test/skills');
    await page.getByLabel(/description/i).fill('A test source');

    await page.getByRole('button', { name: /^add$/i }).click();

    // Should show success toast
    await expect(page.getByText(/source added/i)).toBeVisible();
  });

  test('should toggle source enabled state', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    const sourceItem = page.locator('.list-item').first();
    const toggle = sourceItem.getByRole('switch');

    await expect(toggle).toBeChecked();
    await toggle.click();
    await expect(toggle).not.toBeChecked();
  });

  test('should remove source', async ({ page }) => {
    await page.getByRole('button', { name: /sources/i }).click();

    // Community Skills source can be removed
    const communityItem = page.locator('.list-item').filter({ hasText: 'Community Skills' });
    await communityItem.getByRole('button', { name: /remove/i }).click();

    // Confirm dialog
    await page.getByRole('button', { name: /confirm/i }).click();

    // Should show success
    await expect(page.getByText(/source removed/i)).toBeVisible();
  });
});

// ============================================
// Browse Remote Skills Tab Tests
// ============================================
test.describe('Skills - Browse Tab', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/skills');
    await page.waitForLoadState('networkidle');
  });

  test('should display Browse tab', async ({ page }) => {
    await expect(page.getByRole('button', { name: /browse/i })).toBeVisible();
  });

  test('should switch to Browse tab', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Should show source selector
    await expect(page.getByRole('combobox')).toBeVisible();
  });

  test('should load remote skills when source is selected', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Select fetchable source
    await page.getByRole('combobox').selectOption('anthropic-official');

    // Should show remote skills
    await expect(page.getByText('Remote Skill 1')).toBeVisible();
    await expect(page.getByText('Remote Skill 2')).toBeVisible();
  });

  test('should show "In Plugin" badge for installed skills', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.getByRole('combobox').selectOption('anthropic-official');

    // Remote Skill 2 is marked as installed in mock
    await expect(page.getByText(/in plugin/i)).toBeVisible();
  });

  test('should show Preview and Add buttons for non-installed skills', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.getByRole('combobox').selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    await expect(remoteSkill.getByRole('button', { name: /preview/i })).toBeVisible();
    await expect(remoteSkill.getByRole('button', { name: /add/i })).toBeVisible();
  });

  test('should preview remote skill files', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();
    await page.getByRole('combobox').selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    await remoteSkill.getByRole('button', { name: /preview/i }).click();

    // Should show file tree
    await expect(page.getByText('SKILL.md')).toBeVisible();

    // Should show content
    await expect(page.getByText(/Remote Skill Content/i)).toBeVisible();
  });

  test('should add remote skill to plugin', async ({ page }) => {
    const logs: string[] = [];
    page.on('console', (msg) => {
      if (msg.text().includes('TauriMock')) {
        logs.push(msg.text());
      }
    });

    await page.getByRole('button', { name: /browse/i }).click();
    await page.getByRole('combobox').selectOption('anthropic-official');

    const remoteSkill = page.locator('.list-item').filter({ hasText: 'Remote Skill 1' });
    await remoteSkill.getByRole('button', { name: /add/i }).click();

    // Should show success
    await expect(page.getByText(/added.*plugin/i)).toBeVisible();

    // Verify mock was called
    expect(logs.some((log) => log.includes('install_skill_from_remote'))).toBe(true);
  });

  test('should show browse-only message for non-fetchable sources', async ({ page }) => {
    await page.getByRole('button', { name: /browse/i }).click();

    // Select non-fetchable source
    await page.getByRole('combobox').selectOption('community-skills');

    // Should show browse-only message
    await expect(page.getByText(/browse only/i)).toBeVisible();
  });
});
