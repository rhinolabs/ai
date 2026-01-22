---
name: ai-sdk-core
description: Use for Vercel AI SDK core APIs (generateText, streamText), streaming patterns, and error handling. Does NOT cover React integration (see ai-sdk-react), Next.js specifics (see nextjs-integration), or deployment (see rhinolabs-architecture). Auto-synced weekly from Vercel.
---

# AI SDK Core Patterns

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For AI architecture and deployment decisions
- **rhinolabs-security** - For API key management, rate limiting, and security

**Note**: This skill is auto-synced weekly from Vercel AI SDK official docs. If conflicts with corporate standards are found, they should be resolved during PR review.

When guidance conflicts, always follow the skill with higher precedence.

## Overview
Core patterns and practices for using the Vercel AI SDK.

## Installation
```bash
npm install ai
```

## Basic Usage
```typescript
import { generateText } from 'ai';

const { text } = await generateText({
  model: yourModel,
  prompt: 'Your prompt here',
});
```

## Streaming
```typescript
import { streamText } from 'ai';

const { textStream } = await streamText({
  model: yourModel,
  prompt: 'Your prompt here',
});

for await (const chunk of textStream) {
  console.log(chunk);
}
```

## Error Handling
Always implement proper error handling for AI operations.

---
*This skill is auto-synced from Vercel AI SDK*
