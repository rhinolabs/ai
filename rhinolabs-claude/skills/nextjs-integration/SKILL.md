---
name: nextjs-integration
description: Use for Next.js Server Components, Server Actions, route handlers, and edge runtime for AI features. Does NOT cover general Next.js patterns, deployment (see rhinolabs-architecture), or React hooks (see ai-sdk-react). Auto-synced weekly from Vercel.
---

# Next.js AI Integration Patterns

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For architecture and deployment decisions
- **rhinolabs-security** - For server-side security, API routes, and authentication

**Note**: This skill is auto-synced weekly from Vercel AI SDK official docs. If conflicts with corporate standards are found, they should be resolved during PR review.

When guidance conflicts, always follow the skill with higher precedence.

## Overview
Best practices for integrating AI into Next.js applications.

## Server Actions
```typescript
'use server';

import { generateText } from 'ai';

export async function generateResponse(prompt: string) {
  const { text } = await generateText({
    model: yourModel,
    prompt,
  });

  return text;
}
```

## Route Handlers
```typescript
import { streamText } from 'ai';

export async function POST(req: Request) {
  const { prompt } = await req.json();

  const result = await streamText({
    model: yourModel,
    prompt,
  });

  return result.toAIStreamResponse();
}
```

## Edge Runtime
```typescript
export const runtime = 'edge';

export async function POST(req: Request) {
  // Your AI logic here
}
```

---
*This skill is auto-synced from Vercel AI SDK*
