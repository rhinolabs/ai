---
name: typescript-best-practices
description: Use for TypeScript type system, generics, type guards, and utility types. Does NOT cover architecture decisions (see rhinolabs-architecture), testing types (see testing-strategies), or validation schemas (see zod-4).
---

# TypeScript Best Practices

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For architecture and design decisions
- **rhinolabs-security** - For security-sensitive type definitions

When guidance conflicts, always follow the skill with higher precedence.

## Type Safety

### Strict Mode
Always enable strict mode in tsconfig.json:

```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true
  }
}
```

### Avoid `any`
Use specific types or `unknown` instead:

```typescript
// Bad
function process(data: any) {
  return data.value;
}

// Good
function process(data: { value: string }) {
  return data.value;
}

// Or use unknown for truly unknown types
function process(data: unknown) {
  if (typeof data === 'object' && data !== null && 'value' in data) {
    return (data as { value: string }).value;
  }
}
```

## Generics

### Generic Functions
```typescript
function identity<T>(arg: T): T {
  return arg;
}

function map<T, U>(array: T[], fn: (item: T) => U): U[] {
  return array.map(fn);
}
```

### Generic Constraints
```typescript
interface HasLength {
  length: number;
}

function logLength<T extends HasLength>(arg: T): void {
  console.log(arg.length);
}
```

## Utility Types

### Built-in Utilities
```typescript
// Partial - make all properties optional
type PartialUser = Partial<User>;

// Required - make all properties required
type RequiredUser = Required<User>;

// Pick - select specific properties
type UserPreview = Pick<User, 'id' | 'name'>;

// Omit - exclude specific properties
type UserWithoutPassword = Omit<User, 'password'>;

// Record - create object type with specific keys
type UserRoles = Record<string, 'admin' | 'user'>;
```

### Custom Utility Types
```typescript
// Deep Partial
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

// Nullable
type Nullable<T> = T | null;

// ValueOf
type ValueOf<T> = T[keyof T];
```

## Type Guards

### typeof Guards
```typescript
function isString(value: unknown): value is string {
  return typeof value === 'string';
}

function process(value: string | number) {
  if (typeof value === 'string') {
    return value.toUpperCase();
  }
  return value.toFixed(2);
}
```

### instanceof Guards
```typescript
class Dog {
  bark() { console.log('Woof!'); }
}

class Cat {
  meow() { console.log('Meow!'); }
}

function makeSound(animal: Dog | Cat) {
  if (animal instanceof Dog) {
    animal.bark();
  } else {
    animal.meow();
  }
}
```

### Custom Type Guards
```typescript
interface User {
  id: string;
  name: string;
}

function isUser(value: unknown): value is User {
  return (
    typeof value === 'object' &&
    value !== null &&
    'id' in value &&
    'name' in value &&
    typeof (value as User).id === 'string' &&
    typeof (value as User).name === 'string'
  );
}
```

## Discriminated Unions

```typescript
type Success = {
  status: 'success';
  data: string;
};

type Error = {
  status: 'error';
  error: string;
};

type Result = Success | Error;

function handleResult(result: Result) {
  if (result.status === 'success') {
    console.log(result.data); // TypeScript knows this is Success
  } else {
    console.log(result.error); // TypeScript knows this is Error
  }
}
```

## Const Assertions

```typescript
// Without const assertion
const colors = ['red', 'green', 'blue']; // string[]

// With const assertion
const colors = ['red', 'green', 'blue'] as const; // readonly ['red', 'green', 'blue']

// Object const assertion
const config = {
  apiUrl: 'https://api.example.com',
  timeout: 5000,
} as const;
```

## Template Literal Types

```typescript
type HTTPMethod = 'GET' | 'POST' | 'PUT' | 'DELETE';
type Endpoint = '/users' | '/posts' | '/comments';

type APIRoute = `${HTTPMethod} ${Endpoint}`;
// 'GET /users' | 'GET /posts' | ... | 'DELETE /comments'
```

## Best Practices Summary

1. Enable strict mode
2. Avoid `any`, use `unknown` when needed
3. Use generics for reusable code
4. Leverage utility types
5. Implement type guards for runtime checks
6. Use discriminated unions for complex types
7. Apply const assertions for literal types
8. Document complex types with comments

---

**Last Updated**: 2026-01-22
