---
name: ai-sdk-react
description: Use for Vercel AI SDK React hooks (useChat, useCompletion), message handling, and streaming UI updates. Does NOT cover Next.js specifics (see nextjs-integration), state management architecture (see rhinolabs-architecture), or component patterns (see react-patterns). Auto-synced weekly from Vercel.
---

# AI SDK React Patterns

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For state management architecture and design decisions
- **rhinolabs-security** - For authentication and data handling in AI interfaces

**Note**: This skill is auto-synced weekly from Vercel AI SDK official docs. If conflicts with corporate standards are found, they should be resolved during PR review.

When guidance conflicts, always follow the skill with higher precedence.

## Overview
React-specific patterns for the Vercel AI SDK.

## useChat Hook
```typescript
import { useChat } from 'ai/react';

export default function Chat() {
  const { messages, input, handleInputChange, handleSubmit } = useChat();

  return (
    <div>
      {messages.map(m => (
        <div key={m.id}>{m.content}</div>
      ))}
      <form onSubmit={handleSubmit}>
        <input value={input} onChange={handleInputChange} />
      </form>
    </div>
  );
}
```

## useCompletion Hook
```typescript
import { useCompletion } from 'ai/react';

export default function Completion() {
  const { completion, input, handleInputChange, handleSubmit } = useCompletion();

  return (
    <div>
      <div>{completion}</div>
      <form onSubmit={handleSubmit}>
        <input value={input} onChange={handleInputChange} />
      </form>
    </div>
  );
}
```

---
*This skill is auto-synced from Vercel AI SDK*
