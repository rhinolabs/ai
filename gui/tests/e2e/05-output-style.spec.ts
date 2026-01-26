import { test, expect } from '@playwright/test';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test.describe('Output Style Page - Read-Only View', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/output-style');
    await page.waitForLoadState('networkidle');
  });

  test('should display output style page with heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /output style/i })).toBeVisible();
  });

  test('should display page description', async ({ page }) => {
    await expect(page.getByText(/personality, tone, and response format/i)).toBeVisible();
  });

  test('should show Edit button', async ({ page }) => {
    await expect(page.getByRole('button', { name: /edit/i })).toBeVisible();
  });

  test('should display active style name', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /rhinolabs/i, level: 3 })).toBeVisible();
  });

  test('should display style description', async ({ page }) => {
    await expect(page.getByText(/professional, helpful, and direct/i)).toBeVisible();
  });

  test('should display keepCodingInstructions setting', async ({ page }) => {
    await expect(page.getByText(/keep coding instructions/i)).toBeVisible();
    await expect(page.getByText(/yes/i)).toBeVisible();
  });

  test('should display style content with syntax highlighting', async ({ page }) => {
    // SyntaxHighlighter renders a pre element
    const codeBlock = page.locator('pre').first();
    await expect(codeBlock).toBeVisible();

    // Content should be visible
    await expect(page.getByText(/Be helpful FIRST/i)).toBeVisible();
  });

  test('should show Quick Reference section', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /quick reference/i })).toBeVisible();
    await expect(page.getByText(/# Personality/)).toBeVisible();
    await expect(page.getByText(/# Tone/)).toBeVisible();
  });

  test('should show line numbers in code view', async ({ page }) => {
    const codeBlock = page.locator('pre').first();
    await expect(codeBlock).toBeVisible();
    await expect(page.locator('.linenumber').first()).toBeVisible();
  });
});

test.describe('Output Style Page - Edit in IDE', () => {
  test.beforeEach(async ({ page }) => {
    const mockContent = fs.readFileSync(path.resolve(__dirname, 'mocks/tauri-mock.js'), 'utf-8');
    await page.addInitScript(mockContent);
    await page.goto('/output-style');
    await page.waitForLoadState('networkidle');
  });

  test('should open output style in IDE when Edit is clicked', async ({ page }) => {
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

    // Verify the mock was called with the style id
    expect(logs.some((log) => log.includes('open_output_style_in_ide'))).toBe(true);
  });

  test('should show error when no IDE is available', async ({ page }) => {
    // Override mock to return no available IDEs
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

    await page.getByRole('button', { name: /edit/i }).click();

    // Should show error about no IDE
    await expect(page.getByText(/no ide available/i)).toBeVisible();
  });
});

test.describe('Output Style Page - No Style Available', () => {
  test('should handle no active style gracefully', async ({ page }) => {
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          if (cmd === 'get_active_output_style') {
            return null;
          }
          if (cmd === 'list_output_styles') {
            return [];
          }
          if (cmd === 'list_available_ides') {
            return [{ id: 'vscode', name: 'VS Code', command: 'code', available: true }];
          }
          throw new Error(`Unknown command: ${cmd}`);
        },
        transformCallback: (callback: unknown) => callback,
      };
    });

    await page.goto('/output-style');
    await page.waitForLoadState('networkidle');

    // Should show placeholder content
    await expect(page.getByText(/no content yet/i)).toBeVisible();
  });
});

test.describe('Output Style Page - Loading State', () => {
  test('should show loading spinner while fetching', async ({ page }) => {
    await page.addInitScript(() => {
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string) => {
          await new Promise((resolve) => setTimeout(resolve, 1000));
          if (cmd === 'get_active_output_style') {
            return { id: 'test', name: 'Test', description: 'Test', keepCodingInstructions: true, content: '# Test' };
          }
          if (cmd === 'list_available_ides') {
            return [];
          }
          throw new Error(`Unknown command: ${cmd}`);
        },
        transformCallback: (callback: unknown) => callback,
      };
    });

    await page.goto('/output-style');

    // Should show loading state
    await expect(page.locator('.spinner')).toBeVisible();
  });
});
