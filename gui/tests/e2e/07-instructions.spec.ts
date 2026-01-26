import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.describe('Instructions Page - Read-Only View', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/instructions');
    await page.waitForLoadState('networkidle');
  });

  test('should display instructions page with heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /instructions/i })).toBeVisible();
  });

  test('should display page description', async ({ page }) => {
    await expect(page.getByText(/CLAUDE\.md instructions file/i)).toBeVisible();
  });

  test('should show Edit button', async ({ page }) => {
    await expect(page.getByRole('button', { name: /edit/i })).toBeVisible();
  });

  test('should display instructions content with syntax highlighting', async ({ page }) => {
    // SyntaxHighlighter renders a pre element
    const codeBlock = page.locator('pre').first();
    await expect(codeBlock).toBeVisible();

    // Content should be visible
    await expect(page.getByText(/NEVER add Co-Authored-By/i)).toBeVisible();
  });

  test('should display last modified date', async ({ page }) => {
    await expect(page.getByText(/last modified/i)).toBeVisible();
  });

  test('should show Quick Reference section', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /quick reference/i })).toBeVisible();
    // Use more specific locators within the Quick Reference card
    const quickRef = page.locator('.card').filter({ hasText: /quick reference/i });
    await expect(quickRef.getByText('# Rules - General guidelines')).toBeVisible();
    await expect(quickRef.getByText('# Personality - Tone and style')).toBeVisible();
  });

  test('should show line numbers in code view', async ({ page }) => {
    // Line numbers are rendered by SyntaxHighlighter with showLineNumbers
    const codeBlock = page.locator('pre').first();
    await expect(codeBlock).toBeVisible();
    // Line numbers create spans with specific classes
    await expect(page.locator('.linenumber').first()).toBeVisible();
  });
});

test.describe('Instructions Page - Edit in IDE', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/instructions');
    await page.waitForLoadState('networkidle');
  });

  test('should open instructions in IDE when Edit is clicked', async ({ page }) => {
    // Track console logs for mock verification
    const logs: string[] = [];
    page.on('console', (msg) => {
      if (msg.text().includes('TauriMock')) {
        logs.push(msg.text());
      }
    });

    await page.getByRole('button', { name: /edit/i }).click();

    // Should show success toast
    await expect(page.getByText(/opened in/i)).toBeVisible();

    // Verify the mock was called
    expect(logs.some((log) => log.includes('open_instructions_in_ide'))).toBe(true);
  });

  test('should show error when no IDE is available', async ({ page }) => {
    // Override mock to return no available IDEs
    await page.addInitScript(() => {
      const originalInvoke = (window as any).__TAURI_INTERNALS__.invoke;
      (window as any).__TAURI_INTERNALS__.invoke = async (cmd: string, args?: unknown) => {
        if (cmd === 'list_available_ides') {
          return [
            { id: 'vscode', name: 'VS Code', command: 'code', available: false },
            { id: 'cursor', name: 'Cursor', command: 'cursor', available: false },
          ];
        }
        return originalInvoke(cmd, args);
      };
    });

    await page.reload();
    await page.waitForLoadState('networkidle');

    await page.getByRole('button', { name: /edit/i }).click();

    // Should show error about no IDE
    await expect(page.getByText(/no ide available/i)).toBeVisible();
  });
});

test.describe('Instructions Page - Loading State', () => {
  test('should show loading spinner while fetching', async ({ page }) => {
    // Add slow mock
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          await new Promise((resolve) => setTimeout(resolve, 1000));
          if (cmd === 'get_instructions') {
            return { content: '# Test', lastModified: new Date().toISOString() };
          }
          if (cmd === 'list_available_ides') {
            return [];
          }
          throw new Error(`Unknown command: ${cmd}`);
        },
        transformCallback: (callback: unknown) => callback,
      };
    });

    await page.goto('/instructions');

    // Should show loading state
    await expect(page.locator('.spinner')).toBeVisible();
  });
});

test.describe('Instructions Page - Empty State', () => {
  test('should show placeholder when no content', async ({ page }) => {
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          if (cmd === 'get_instructions') {
            return { content: '', lastModified: new Date().toISOString() };
          }
          if (cmd === 'list_available_ides') {
            return [{ id: 'vscode', name: 'VS Code', command: 'code', available: true }];
          }
          throw new Error(`Unknown command: ${cmd}`);
        },
        transformCallback: (callback: unknown) => callback,
      };
    });

    await page.goto('/instructions');
    await page.waitForLoadState('networkidle');

    // Should show placeholder text
    await expect(page.getByText(/no content yet/i)).toBeVisible();
  });
});
