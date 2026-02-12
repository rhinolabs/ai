---
name: team-code-review
description: >
  Orchestrates 3 parallel sub-agents for comprehensive code review: Security, Performance, and Test Coverage.
  Trigger: When the user asks for code review, PR review, review changes, or "review this code".
license: Apache-2.0
metadata:
  author: rhinolabs
  version: "1.0"
---

# Team Code Review

## Precedence

This skill defers to:
- **rhinolabs-security** — For all security review criteria
- **rhinolabs-standards** — For code quality and testing requirements
- **rhinolabs-architecture** — For architectural review decisions

When guidance conflicts, always follow the skill with higher precedence.

## When to Use

Use this skill when:
- User asks for a **code review** or **PR review**
- User says "review this code", "review my changes", "review this PR"
- User asks to **check changes before committing**
- User wants a **comprehensive review** of a feature or module

Do NOT use when:
- User asks for a quick fix or typo correction
- User wants only security review (use `rhinolabs-security` directly)
- User asks to review a single line or trivial change

---

## Workflow

### Step 1: Identify the Scope

Before launching sub-agents, determine what code to review:

```
PR number provided?     → Use `git diff` against base branch
Staged changes?         → Use `git diff --cached`
Specific files/folder?  → Use those paths
Nothing specified?      → Ask the user what to review
```

### Step 2: Launch 3 Sub-agents in Parallel

Use the **Task tool** with `subagent_type: "general-purpose"` for ALL three agents. Launch them in a **single message** so they run in parallel.

#### Agent 1: Security Review

```
Prompt:
You are a security reviewer for a TypeScript/React/Node.js codebase.

FIRST: Read the security standards at ~/.claude/skills/rhinolabs-security/SKILL.md

Then review the following code changes for security issues:
[INSERT DIFF OR FILE PATHS]

Check for:
1. **Authentication & Authorization**: Missing auth checks, improper token handling, session issues
2. **Input Validation**: Unsanitized user input, missing validation, SQL/NoSQL injection vectors
3. **XSS & Injection**: Dangerously set innerHTML, template injection, command injection
4. **Secrets & Config**: Hardcoded credentials, exposed API keys, sensitive data in logs
5. **Dependencies**: Known vulnerable packages, outdated security-critical deps
6. **Data Exposure**: Sensitive data in responses, overly permissive CORS, missing rate limiting

For each finding, report:
- **Severity**: CRITICAL / HIGH / MEDIUM / LOW
- **File & Line**: Exact location
- **Issue**: What's wrong
- **Fix**: How to fix it (with code snippet if applicable)

If no security issues found, explicitly state "No security issues found" with a brief explanation of what was checked.
```

#### Agent 2: Performance Review

```
Prompt:
You are a performance reviewer for a TypeScript/React/Node.js codebase.

Review the following code changes for performance issues:
[INSERT DIFF OR FILE PATHS]

Check for:
1. **React Performance**: Unnecessary re-renders, missing keys, inline object/function creation in JSX, large component trees without lazy loading
2. **Data Fetching**: N+1 queries, missing pagination, unbounded data fetching, waterfall requests
3. **Memory**: Event listener leaks, uncleared intervals/timeouts, growing closures, large object retention
4. **Bundle Size**: Importing entire libraries (e.g., `import _ from 'lodash'` instead of `import get from 'lodash/get'`), missing tree-shaking, large assets
5. **Async Patterns**: Blocking operations, unhandled promise rejections, sequential awaits that could be parallel
6. **Caching**: Missing memoization for expensive computations, redundant API calls, missing HTTP cache headers

For each finding, report:
- **Impact**: HIGH / MEDIUM / LOW
- **File & Line**: Exact location
- **Issue**: What's wrong and WHY it's a problem
- **Fix**: Suggested optimization (with code snippet if applicable)

If no performance issues found, explicitly state "No performance issues found" with a brief explanation of what was checked.
```

#### Agent 3: Test Coverage Review

```
Prompt:
You are a test coverage reviewer for a TypeScript/React/Node.js codebase.

FIRST: Read the testing standards at ~/.claude/skills/testing-strategies/SKILL.md

Then review the following code changes for test coverage gaps:
[INSERT DIFF OR FILE PATHS]

Check for:
1. **Missing Tests**: New functions/components/routes without corresponding test files
2. **Edge Cases**: Null/undefined handling, empty arrays, boundary values, error states not tested
3. **Integration Gaps**: API endpoints without integration tests, component interactions untested
4. **E2E Coverage**: New user flows without E2E tests, critical paths not covered
5. **Test Quality**: Tests that test implementation details instead of behavior, fragile selectors, missing assertions
6. **Error Paths**: Only happy path tested, missing error/rejection/timeout test cases

For each finding, report:
- **Priority**: HIGH / MEDIUM / LOW
- **File**: Source file missing tests
- **Gap**: What's not tested
- **Suggestion**: Specific test case to add (with code skeleton if applicable)

If test coverage is adequate, explicitly state "Test coverage is adequate" with a brief summary of what's covered.
```

### Step 3: Consolidate Results

After all 3 agents complete, compile a unified review report.

---

## Report Format

Present the consolidated review as a single markdown report:

```markdown
## Code Review Report

### Summary

| Area | Findings | Severity |
|------|----------|----------|
| Security | X issues | CRITICAL: N, HIGH: N, MEDIUM: N |
| Performance | X issues | HIGH: N, MEDIUM: N, LOW: N |
| Test Coverage | X gaps | HIGH: N, MEDIUM: N, LOW: N |

### Verdict: [APPROVE / REQUEST CHANGES / NEEDS DISCUSSION]

[1-2 sentence justification for the verdict]

---

### Security Review
[Agent 1 findings]

### Performance Review
[Agent 2 findings]

### Test Coverage Review
[Agent 3 findings]

---

### Recommended Actions
1. [Most critical action]
2. [Second priority]
3. [Third priority]
```

## Verdict Criteria

- **APPROVE**: No CRITICAL/HIGH findings across any area
- **REQUEST CHANGES**: Any CRITICAL or HIGH finding exists
- **NEEDS DISCUSSION**: Findings involve architectural tradeoffs that require team input

---

## Keywords

code review, PR review, review changes, review this code, review my changes, check my code, pull request review
