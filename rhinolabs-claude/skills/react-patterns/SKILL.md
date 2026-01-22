---
name: react-patterns
description: Use for React component composition, hooks patterns, and prop design. Does NOT cover authentication (see rhinolabs-security), API architecture (see rhinolabs-architecture), testing (see testing-strategies), or AI integration (see ai-sdk-react).
---

# React Patterns and Best Practices

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For state management architecture and system design decisions
- **rhinolabs-security** - For all authentication and security-related patterns

When guidance conflicts, always follow the skill with higher precedence.

## Component Composition

### Container/Presentational Pattern
Separate logic from presentation:

```typescript
// Container (logic)
function UserContainer() {
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetchUser().then(setUser);
  }, []);

  return <UserPresentation user={user} />;
}

// Presentational (UI)
function UserPresentation({ user }) {
  if (!user) return <Loading />;
  return <div>{user.name}</div>;
}
```

### Compound Components
Create flexible, composable components:

```typescript
function Select({ children, value, onChange }) {
  return (
    <SelectContext.Provider value={{ value, onChange }}>
      <div className="select">{children}</div>
    </SelectContext.Provider>
  );
}

Select.Option = function Option({ value, children }) {
  const { value: selectedValue, onChange } = useContext(SelectContext);
  return (
    <div
      className={selectedValue === value ? 'selected' : ''}
      onClick={() => onChange(value)}
    >
      {children}
    </div>
  );
};
```

## Custom Hooks

### Data Fetching Hook
```typescript
function useData<T>(url: string) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    fetch(url)
      .then(res => res.json())
      .then(setData)
      .catch(setError)
      .finally(() => setLoading(false));
  }, [url]);

  return { data, loading, error };
}
```

### Local Storage Hook
```typescript
function useLocalStorage<T>(key: string, initialValue: T) {
  const [value, setValue] = useState<T>(() => {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : initialValue;
  });

  useEffect(() => {
    localStorage.setItem(key, JSON.stringify(value));
  }, [key, value]);

  return [value, setValue] as const;
}
```

## Performance Optimization

### Memoization
```typescript
// Memoize expensive calculations
const expensiveValue = useMemo(() => {
  return computeExpensiveValue(a, b);
}, [a, b]);

// Memoize callback functions
const handleClick = useCallback(() => {
  doSomething(a, b);
}, [a, b]);

// Memoize components
const MemoizedComponent = React.memo(Component);
```

### Code Splitting
```typescript
// Lazy load components
const LazyComponent = lazy(() => import('./LazyComponent'));

function App() {
  return (
    <Suspense fallback={<Loading />}>
      <LazyComponent />
    </Suspense>
  );
}
```

## State Management

### Context + Reducer Pattern
```typescript
type State = { count: number };
type Action = { type: 'increment' } | { type: 'decrement' };

const CountContext = createContext<{
  state: State;
  dispatch: Dispatch<Action>;
} | null>(null);

function countReducer(state: State, action: Action): State {
  switch (action.type) {
    case 'increment':
      return { count: state.count + 1 };
    case 'decrement':
      return { count: state.count - 1 };
    default:
      return state;
  }
}

function CountProvider({ children }) {
  const [state, dispatch] = useReducer(countReducer, { count: 0 });

  return (
    <CountContext.Provider value={{ state, dispatch }}>
      {children}
    </CountContext.Provider>
  );
}
```

## Error Boundaries

```typescript
class ErrorBoundary extends React.Component<
  { children: ReactNode },
  { hasError: boolean }
> {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true };
  }

  componentDidCatch(error, errorInfo) {
    console.error('Error caught:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return <h1>Something went wrong.</h1>;
    }

    return this.props.children;
  }
}
```

---

**Last Updated**: 2026-01-22
