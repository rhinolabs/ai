---
name: testing-strategies
description: Use for general testing approaches (unit, integration, E2E), test organization, and TDD. Does NOT cover specific framework usage (see playwright) or quality standards (see rhinolabs-standards).
---

# Testing Strategies

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all testing requirements and quality standards
- **rhinolabs-architecture** - For testing architecture decisions
- **rhinolabs-security** - For security testing requirements

When guidance conflicts, always follow the skill with higher precedence.

## Testing Pyramid

```
       /\
      /E2E\
     /------\
    /Integration\
   /--------------\
  /   Unit Tests   \
 /------------------\
```

- **Unit Tests**: 70% - Test individual functions/components
- **Integration Tests**: 20% - Test component interactions
- **E2E Tests**: 10% - Test complete user flows

## Unit Testing

### Testing Functions
```typescript
// sum.ts
export function sum(a: number, b: number): number {
  return a + b;
}

// sum.test.ts
import { sum } from './sum';

describe('sum', () => {
  it('should add two numbers', () => {
    expect(sum(1, 2)).toBe(3);
  });

  it('should handle negative numbers', () => {
    expect(sum(-1, -2)).toBe(-3);
  });
});
```

### Testing React Components
```typescript
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Button from './Button';

describe('Button', () => {
  it('should render with text', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('should call onClick when clicked', async () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click me</Button>);

    await userEvent.click(screen.getByText('Click me'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });
});
```

### Testing Hooks
```typescript
import { renderHook, act } from '@testing-library/react';
import { useCounter } from './useCounter';

describe('useCounter', () => {
  it('should increment counter', () => {
    const { result } = renderHook(() => useCounter());

    act(() => {
      result.current.increment();
    });

    expect(result.current.count).toBe(1);
  });
});
```

## Integration Testing

### Testing API Integration
```typescript
import { render, screen, waitFor } from '@testing-library/react';
import { rest } from 'msw';
import { setupServer } from 'msw/node';
import UserProfile from './UserProfile';

const server = setupServer(
  rest.get('/api/user', (req, res, ctx) => {
    return res(ctx.json({ name: 'John Doe' }));
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

describe('UserProfile', () => {
  it('should display user name', async () => {
    render(<UserProfile userId="1" />);

    await waitFor(() => {
      expect(screen.getByText('John Doe')).toBeInTheDocument();
    });
  });
});
```

### Testing Component Interactions
```typescript
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import TodoApp from './TodoApp';

describe('TodoApp', () => {
  it('should add and display todo', async () => {
    render(<TodoApp />);

    const input = screen.getByPlaceholderText('Add todo');
    const button = screen.getByText('Add');

    await userEvent.type(input, 'Buy milk');
    await userEvent.click(button);

    expect(screen.getByText('Buy milk')).toBeInTheDocument();
  });
});
```

## E2E Testing

### Playwright Example
```typescript
import { test, expect } from '@playwright/test';

test('user can login', async ({ page }) => {
  await page.goto('http://localhost:3000');

  await page.fill('input[name="email"]', 'user@example.com');
  await page.fill('input[name="password"]', 'password123');
  await page.click('button[type="submit"]');

  await expect(page).toHaveURL('http://localhost:3000/dashboard');
  await expect(page.locator('h1')).toContainText('Dashboard');
});
```

### Cypress Example
```typescript
describe('Login Flow', () => {
  it('should login successfully', () => {
    cy.visit('/login');

    cy.get('input[name="email"]').type('user@example.com');
    cy.get('input[name="password"]').type('password123');
    cy.get('button[type="submit"]').click();

    cy.url().should('include', '/dashboard');
    cy.contains('h1', 'Dashboard').should('be.visible');
  });
});
```

## Test-Driven Development (TDD)

### Red-Green-Refactor Cycle

1. **Red**: Write a failing test
```typescript
it('should validate email', () => {
  expect(validateEmail('invalid')).toBe(false);
  expect(validateEmail('valid@example.com')).toBe(true);
});
```

2. **Green**: Write minimal code to pass
```typescript
function validateEmail(email: string): boolean {
  return email.includes('@');
}
```

3. **Refactor**: Improve the code
```typescript
function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}
```

## Best Practices

### Test Organization
- One test file per source file
- Group related tests with `describe`
- Use descriptive test names
- Follow AAA pattern (Arrange, Act, Assert)

### Test Coverage
- Aim for 80%+ code coverage
- Focus on critical paths
- Don't test implementation details
- Test behavior, not implementation

### Mocking
```typescript
// Mock modules
jest.mock('./api', () => ({
  fetchUser: jest.fn(),
}));

// Mock functions
const mockFn = jest.fn().mockReturnValue('mocked value');

// Mock timers
jest.useFakeTimers();
jest.advanceTimersByTime(1000);
```

### Async Testing
```typescript
// Using async/await
it('should fetch data', async () => {
  const data = await fetchData();
  expect(data).toBeDefined();
});

// Using waitFor
await waitFor(() => {
  expect(screen.getByText('Loaded')).toBeInTheDocument();
});
```

---

**Last Updated**: 2026-01-22
