---
name: team-bug-investigation
description: >
  Orchestrates 3 parallel sub-agents for comprehensive bug investigation: Trace Execution, Related Code Analysis, and Root Cause Analysis.
  Trigger: When the user asks to investigate a bug, debug a complex issue, or find a root cause.
license: Apache-2.0
metadata:
  author: rhinolabs
  version: "1.0"
---

# Team Bug Investigation

## Precedence

This skill defers to:
- **rhinolabs-standards** — For code quality and debugging standards
- **rhinolabs-architecture** — For architectural context
- **rhinolabs-security** — If the bug involves security implications

When guidance conflicts, always follow the skill with higher precedence.

## When to Use

Use this skill when:
- User asks to **investigate a bug** or **debug an issue**
- User reports a **complex error** that spans multiple files or modules
- User asks for **root cause analysis** of a failure
- User says "why is this breaking", "this doesn't work", "find the bug"
- The issue is **non-trivial** — involves data flow across components, async behavior, or state management

Do NOT use when:
- The bug is obvious (typo, syntax error, missing import)
- User already knows the root cause and just wants a fix
- It's a build/config error with a clear error message

---

## Workflow

### Step 1: Gather Bug Context

Before launching sub-agents, collect essential information:

```
Error message provided?     → Include in all agent prompts
Stack trace available?      → Include file paths and line numbers
Reproduction steps?         → Include for trace agent
Specific file/component?    → Use as starting point for all agents
```

If the user hasn't provided enough context, ask:
1. What's the expected behavior?
2. What's the actual behavior?
3. Can you share the error message or stack trace?

### Step 2: Launch 3 Sub-agents in Parallel

Use the **Task tool** with `subagent_type: "general-purpose"` for ALL three agents. Launch them in a **single message** so they run in parallel.

#### Agent 1: Trace Execution

```
Prompt:
You are a code execution tracer for a TypeScript/React/Node.js codebase.

Investigate this bug by tracing the code execution path:
[INSERT BUG DESCRIPTION, ERROR MESSAGE, STACK TRACE]

Your job:
1. **Start at the entry point**: Find the file/function where the bug manifests (from error/stack trace)
2. **Follow imports**: Read each imported module, trace the data flow through function calls
3. **Map the data flow**: Document how data transforms at each step (props → state → render, request → handler → response)
4. **Identify the break point**: Find exactly WHERE the data/behavior diverges from expected

For each step in the trace, document:
- **File:Line** — Exact location
- **Input** — What data enters this function/component
- **Operation** — What happens to the data
- **Output** — What data leaves
- **Suspect?** — YES/NO with reason if suspect

Use Read, Grep, and Glob tools extensively. READ the actual code, don't guess.

Deliver a clear execution trace showing the path from entry to the point of failure.
```

#### Agent 2: Related Code Analysis

```
Prompt:
You are a codebase analyst investigating a bug in a TypeScript/React/Node.js codebase.

The bug is:
[INSERT BUG DESCRIPTION, ERROR MESSAGE]

Your job is to find RELATED code that might be affected by the same root cause or that might CAUSE the issue:

1. **Pattern Search**: Use Grep to find similar patterns to the buggy code. If the bug is in a handler, find all similar handlers. If it's in a component, find all components using the same pattern.
2. **Shared Dependencies**: Identify shared utilities, hooks, services, or stores used by the buggy code. Check if they've been modified recently or have known issues.
3. **Similar Bugs**: Search for TODO/FIXME/HACK/WORKAROUND comments near related code that might indicate known issues.
4. **Type Mismatches**: Look for TypeScript `any` casts, `@ts-ignore`, or `as` assertions near the buggy area — these often mask real issues.
5. **Configuration**: Check for environment-specific config, feature flags, or conditional logic that might cause the bug only in certain conditions.

Use Grep and Glob tools extensively to search the codebase.

Deliver:
- List of **related files** that share patterns with the buggy code
- Any **similar issues** found (TODO/FIXME/HACK comments)
- **Risk assessment**: Other areas that might be affected by the same root cause
```

#### Agent 3: Root Cause Analysis

```
Prompt:
You are a root cause analyst investigating a bug in a TypeScript/React/Node.js codebase.

The bug is:
[INSERT BUG DESCRIPTION, ERROR MESSAGE, FILE PATHS IF KNOWN]

Your job is to investigate the historical and environmental context:

1. **Recent Changes**: Run `git log --oneline -20 -- [relevant-files]` to find recent modifications to the buggy files
2. **Git Blame**: Run `git blame [file]` on the specific lines involved in the bug to find who changed them and when
3. **Dependency Changes**: Check `package.json` and lock file for recent dependency updates that might have introduced breaking changes. Run `git log --oneline -10 -- package.json` and `git log --oneline -10 -- pnpm-lock.yaml`
4. **Breaking Changes**: If a dependency was updated, check its changelog for breaking changes
5. **Environment Diffs**: Look for differences between environments (dev vs prod) — env vars, feature flags, API endpoints
6. **Regression Check**: Determine if this worked before. If so, identify the commit that broke it using the git history

Use Bash for git commands, and Read/Grep for file analysis.

Deliver:
- **Timeline**: When did this likely break? (specific commit or date range)
- **Suspect Commits**: List of commits that might have introduced the bug, with reasons
- **Dependency Changes**: Any relevant package updates
- **Hypothesis**: Your top 1-3 hypotheses for the root cause, ranked by probability
```

### Step 3: Consolidate Results

After all 3 agents complete, synthesize findings into a unified investigation report.

---

## Report Format

Present the consolidated investigation as a single markdown report:

```markdown
## Bug Investigation Report

### Bug Summary
**Reported**: [What the user described]
**Expected**: [What should happen]
**Actual**: [What happens instead]

---

### Hypotheses (Ranked by Probability)

| # | Hypothesis | Probability | Evidence |
|---|-----------|-------------|----------|
| 1 | [Most likely cause] | HIGH | [Supporting evidence from agents] |
| 2 | [Second hypothesis] | MEDIUM | [Supporting evidence] |
| 3 | [Third hypothesis] | LOW | [Supporting evidence] |

---

### Execution Trace
[Agent 1 findings — the code path from entry to failure]

### Related Code
[Agent 2 findings — similar patterns and risk areas]

### Root Cause Analysis
[Agent 3 findings — git history, dependency changes, timeline]

---

### Recommended Next Steps

1. **Immediate**: [Quick fix or verification step]
2. **Investigation**: [If more info needed, what to check next]
3. **Prevention**: [How to prevent similar bugs in the future]

### Affected Areas
- [List of files/modules that might also be affected]
```

---

## Decision Tree

```
Error message clear?       → Include in all agent prompts, start trace there
Stack trace available?     → Use file:line as entry points for trace agent
No clear error?            → Ask user for reproduction steps before launching
Multiple possible causes?  → Let all 3 agents run, cross-reference findings
Single obvious cause?      → Skip this skill, fix directly
```

---

## Keywords

investigate bug, debug issue, root cause, why is this breaking, find the bug, this doesn't work, debug this, trace the bug, what's causing this, bug investigation
