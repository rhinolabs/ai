# Skill Management Guidelines

## Overview

This document defines how skills are managed in the Rhinolabs Claude plugin to ensure consistency and avoid conflicts. We use a **manual conflict resolution approach** with explicit precedence rules built into each skill.

## Core Principles

### 1. Defense in Depth

Our conflict resolution strategy uses three layers:

```
Layer 1: Exclusive Scopes
↓ (If overlap occurs)
Layer 2: Explicit Precedence Rules
↓ (If still unclear)
Layer 3: Mandatory Human Review
```

### 2. Corporate Standards Are Non-Negotiable

Three skills define Rhinolabs corporate standards and **always take precedence**:

| Skill | Scope | Authority |
|-------|-------|-----------|
| `rhinolabs-standards` | Code quality, testing, documentation | **Source of truth** |
| `rhinolabs-architecture` | System design, project structure | **Source of truth** |
| `rhinolabs-security` | Auth, encryption, compliance | **Source of truth** |

No other skill should contradict these. If contradiction occurs, corporate standards win.

---

## Skill Categories

### Corporate Standards (Non-Negotiable)

**Location**: `skills/rhinolabs-*`

**Characteristics**:
- Define company-wide mandatory practices
- Manually maintained by DevOps/Tech Lead team
- Updated through formal review process
- Override all other skills in case of conflict

**Skills**:
- `rhinolabs-standards` - Development standards
- `rhinolabs-architecture` - Architecture patterns
- `rhinolabs-security` - Security requirements

**Update Process**:
1. PR with justification
2. Review by DevOps + Tech Lead
3. Team announcement after merge
4. Version documented in changelog

---

### Vercel Skills (Auto-Synced)

**Location**: `skills/ai-sdk-*, skills/nextjs-integration`

**Characteristics**:
- Official patterns from Vercel AI SDK
- Auto-synced weekly from https://github.com/vercel/ai
- Represent industry best practices for AI/React/Next.js
- **Must not contradict corporate standards**

**Skills**:
- `ai-sdk-core` - AI SDK core patterns (generateText, streamText)
- `ai-sdk-react` - React integration (useChat, useCompletion)
- `nextjs-integration` - Next.js AI patterns (RSC, Server Actions)

**Update Process**:
1. GitHub Actions runs weekly (Monday 00:00 UTC)
2. Creates PR with synced content
3. **MANDATORY REVIEW** by DevOps + Tech Lead
4. Review checklist validates no conflicts with corporate standards
5. Manual merge after approval

**Review Checklist** (see PR template):
- [ ] No contradiction with rhinolabs-standards
- [ ] No contradiction with rhinolabs-security
- [ ] No contradiction with rhinolabs-architecture
- [ ] Code examples follow corporate guidelines
- [ ] No security anti-patterns

---

### Base Skills (General Practices)

**Location**: `skills/react-patterns`, `skills/typescript-*`, etc.

**Characteristics**:
- General development practices and patterns
- Manually maintained by team
- Specific, non-overlapping scopes
- Defer to corporate standards in conflicts

**Skills**:
- `react-patterns` - React component patterns
- `typescript-best-practices` - TypeScript guidelines
- `tailwind-4` - Tailwind CSS v4 patterns
- `zod-4` - Zod validation patterns
- `zustand-5` - Zustand state management
- `testing-strategies` - General testing approaches
- `playwright` - Playwright E2E testing

**Update Process**:
1. PR with changes
2. Review by 2+ team members
3. Check for scope overlap with existing skills
4. Merge after approval

---

## Conflict Resolution Rules

### Rule 1: Exclusive Scopes (First Defense)

Each skill has a **clearly defined, non-overlapping scope** in its frontmatter description.

**Example**:
```yaml
---
name: react-patterns
description: Use for React component composition, hooks patterns, and prop design. Does NOT cover authentication (see rhinolabs-security), API architecture (see rhinolabs-architecture), or testing (see testing-strategies).
---
```

**Goal**: Prevent overlaps at the description level so only ONE skill activates per context.

### Rule 2: Explicit Precedence (Second Defense)

Every non-corporate skill includes a **Precedence** section that explicitly states which skills override it.

**Template**:
```markdown
## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For system design decisions
- **rhinolabs-security** - For all security-related patterns

When guidance conflicts, always follow the skill with higher precedence.
```

### Rule 3: Mandatory Review (Third Defense)

For auto-synced Vercel skills, **human review is mandatory** before merge to catch conflicts that automated systems can't detect.

---

## Scope Definitions

### Corporate Standards

| Skill | Covers | Does NOT Cover |
|-------|--------|----------------|
| `rhinolabs-standards` | Code quality, testing requirements, documentation standards, commit conventions | Specific framework patterns (defer to base skills) |
| `rhinolabs-architecture` | System design, project structure, API design, state management architecture | Implementation details (defer to base skills) |
| `rhinolabs-security` | Authentication, authorization, encryption, secrets management, compliance | General coding patterns (defer to base skills) |

### Vercel Skills

| Skill | Covers | Does NOT Cover |
|-------|--------|----------------|
| `ai-sdk-core` | AI SDK core APIs (generateText, streamText), streaming patterns, error handling | React integration (see ai-sdk-react), deployment (see rhinolabs-architecture) |
| `ai-sdk-react` | React hooks (useChat, useCompletion), UI streaming patterns | Next.js specifics (see nextjs-integration), state management architecture (see rhinolabs-architecture) |
| `nextjs-integration` | Next.js Server Components, Server Actions, route handlers for AI | General Next.js patterns (see base skills), deployment (see rhinolabs-architecture) |

### Base Skills

| Skill | Covers | Does NOT Cover |
|-------|--------|----------------|
| `react-patterns` | Component composition, hooks patterns, prop design | Authentication (see rhinolabs-security), state management architecture (see rhinolabs-architecture), testing (see testing-strategies) |
| `typescript-best-practices` | Type system, generics, type guards, utility types | Architecture decisions (see rhinolabs-architecture), testing types (see testing-strategies) |
| `tailwind-4` | Utility classes, responsive design, styling patterns | Component architecture (see react-patterns), design system standards (see rhinolabs-standards) |
| `zod-4` | Schema validation, type inference, error handling | API architecture (see rhinolabs-architecture), security validation rules (see rhinolabs-security) |
| `zustand-5` | Store creation, actions, selectors, persistence | State management architecture decisions (see rhinolabs-architecture) |
| `testing-strategies` | Test approaches (unit, integration, E2E), test organization | Specific framework usage (see playwright), quality standards (see rhinolabs-standards) |
| `playwright` | Playwright-specific APIs, E2E test patterns | General testing strategy (see testing-strategies), quality standards (see rhinolabs-standards) |

---

## Adding New Skills

### Before Creating a New Skill

Ask:
1. **Does an existing skill cover this?** - Check scope definitions above
2. **Is this corporate standard or general practice?** - Determines naming and authority
3. **Will this overlap with existing skills?** - Define exclusive scope
4. **Is this a one-off pattern?** - If yes, add to existing skill instead

### Skill Creation Checklist

- [ ] Defined exclusive scope (no overlap with existing skills)
- [ ] Added to scope definitions table in this document
- [ ] Frontmatter includes clear, specific description
- [ ] Includes Precedence section (if not corporate standard)
- [ ] Code examples follow rhinolabs-standards
- [ ] PR reviewed by 2+ team members
- [ ] Announced to team after merge

### Skill Template

```markdown
---
name: skill-name
description: Use when [specific context]. Does NOT cover [other contexts - refer to other skills].
---

# Skill Name

## Precedence

This skill defers to:
- **rhinolabs-standards** - For all code quality and documentation standards
- **rhinolabs-architecture** - For system design decisions
- **rhinolabs-security** - For all security-related patterns

When guidance conflicts, always follow the skill with higher precedence.

## Overview

[Brief description of what this skill covers]

## Patterns

[Specific patterns and best practices]

## Examples

[Code examples]

## Anti-Patterns

[What NOT to do]

## Resources

[Links to docs, tools, etc.]
```

---

## Vercel Sync Management

### Weekly Sync Process

1. **Monday 00:00 UTC**: GitHub Actions workflow runs
2. **Automated extraction**: Clones Vercel AI repo, extracts docs, transforms to SKILL.md
3. **PR created**: With checklist and mandatory reviewers
4. **Review phase**: DevOps + Tech Lead review for conflicts
5. **Manual merge**: After approval and validation

### Review Responsibilities

**DevOps Lead**:
- Check for contradictions with rhinolabs-standards
- Validate YAML frontmatter is correct
- Ensure no MDX syntax remains

**Tech Lead**:
- Check for contradictions with rhinolabs-architecture
- Check for contradictions with rhinolabs-security
- Validate code examples follow our guidelines

### If Conflicts Are Found

**DO NOT MERGE**. Instead:

1. **Document conflict** in PR comments with specific examples
2. **Edit synced skill** to align with corporate standards:
   ```markdown
   ## Rhinolabs Override

   The Vercel docs suggest [X], but Rhinolabs standard requires [Y].

   **Use this approach instead**:
   [code example following corporate standards]
   ```
3. **Commit changes** to the PR branch
4. **Document in commit message**: `fix: align Vercel pattern with rhinolabs-security`
5. **Request re-review**
6. **Merge after approval**

### Manual Sync Trigger

To manually trigger sync outside the weekly schedule:

1. Go to **Actions** tab in GitHub
2. Select **Sync Vercel AI SDK Skills** workflow
3. Click **Run workflow**
4. Select branch `main`
5. Click **Run workflow** button

---

## FAQ

**Q: What if two base skills contradict each other?**
A: This is a bug. File an issue, tag DevOps Lead, and clarify scope boundaries.

**Q: Can I override corporate standards in my project?**
A: No. Corporate standards are mandatory. If you have a valid exception, propose a change to the standard itself via PR.

**Q: What if Vercel best practices contradict our standards?**
A: Corporate standards win. Edit the synced skill to add a "Rhinolabs Override" section.

**Q: How do I know which skill is active?**
A: Claude Code uses keyword matching on the `description` field. Skills with better keyword match activate.

**Q: Can I add custom skills for my team?**
A: Yes, but they must follow this guideline. Create PR and get team review.

**Q: What if I need a one-time pattern?**
A: Don't create a new skill. Either add to existing skill or use project-specific documentation.

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-01-23 | Initial version with manual conflict resolution approach |

---

**Maintained by**: DevOps Team
**Last Review**: 2026-01-23
**Next Review**: 2026-04-23
