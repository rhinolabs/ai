# Instructions

## Project Overview

**rhinolabs-ai** — Enterprise skill, profile, and configuration management for AI coding assistants.
Supports Claude Code, Amp, Antigravity, and OpenCode as deployment targets.

## Architecture

Rust monorepo with 3 workspace members:

| Crate | Package | Purpose |
|-------|---------|---------|
| `core/` | `rhinolabs-core` | Shared library — ALL business logic lives here |
| `cli/` | `rhinolabs-ai-cli` | CLI tool (binaries: `rhinolabs-ai`, `rlai`) |
| `gui/src-tauri/` | `rhinolabs-gui` | Tauri desktop app — thin wrapper over core |

Frontend: `gui/src/` — React + TypeScript + Vite (types in `gui/src/types.ts`).

### Key Directories

```
core/src/          → Business logic modules (skills, profiles, settings, paths, etc.)
cli/src/commands/  → CLI command implementations
gui/src-tauri/src/ → commands.rs (Tauri IPC → core function calls)
gui/src-tauri/tests/ → Integration & contract tests
gui/tests/e2e/     → Playwright E2E tests (use mocks, NOT real backend)
rhinolabs-claude/  → Base plugin package (skills/, output-styles/, settings.json, .mcp.json)
```

### Core Module Map

| Module | Exports | Purpose |
|--------|---------|---------|
| `skills` | `Skill, Skills, SkillCategory, SkillSource, SkillSchema` | Skill CRUD, categories, sources |
| `profiles` | `Profile, Profiles, ProfileType, AutoInvokeRule` | Profile management, installation |
| `settings` | `Settings, PluginSettings, PermissionConfig, StatusLineConfig` | Plugin settings |
| `paths` | `Paths` | Cross-platform path resolution |
| `targets` | `DeployTarget, TargetPaths, *Deployer` | Multi-target deployment |
| `mcp_config` | `McpConfig, McpConfigManager, McpServer` | MCP server configuration |
| `output_styles` | `OutputStyle, OutputStyles` | Response format management |
| `diagnostics` | `Doctor, DiagnosticReport, DiagnosticCheck` | System health checks |
| `project` | `Project, ProjectStatus, ProjectConfig` | Git/release management |
| `deploy` | `Deploy, ConfigManifest, SyncResult` | Config export/deploy/sync |
| `installer` | `Installer` | Plugin installation |
| `updater` | `Updater` | Plugin updates |
| `rag` | `Rag, RagConfig` | Project memory (R2 storage) |
| `error` | `RhinolabsError, Result<T>` | Error types |

### Error Handling

`RhinolabsError` variants: `Io`, `Git`, `Http`, `Json`, `Zip`, `ClaudeCodeNotFound`, `PluginNotInstalled`, `ConfigError(String)`, `Other(String)`, etc.

**IMPORTANT**: `Result<T>` is aliased to `std::result::Result<T, RhinolabsError>`. When you need an explicit error type (e.g., `serde_json::Error`), use `std::result::Result<T, E>` instead.

### Environment Variables

| Variable | Type | Purpose |
|----------|------|---------|
| `RHINOLABS_DEV_PATH` | Directory path | Override plugin dir (for development) |
| `RHINOLABS_CONFIG_PATH` | File path | Override config location (`config_dir()` uses `.parent()`) |
| `GITHUB_TOKEN` | Token | GitHub API access (deploy/release) |

## Rules

- NEVER add "Co-Authored-By" or any AI attribution to commits. Use conventional commits format only.
- Never build after changes unless explicitly requested.
- When asking user a question, STOP and wait for response. Never continue or assume answers.
- Never agree with user claims without verification. Verify code/docs first.
- If user is wrong, explain WHY with evidence. If you were wrong, acknowledge with proof.
- Always propose alternatives with tradeoffs when relevant.
- Verify technical claims before stating them. If unsure, investigate first.

## Git Rules

- NEVER commit without asking first
- NEVER push without asking first
- NEVER run reset, revert, or any destructive git operation without asking first
- ALWAYS show pending changes and ask for confirmation before any git operation
- NEVER assume files are unrelated to a commit without verifying. Always `git diff` every changed file to understand what it contains before deciding what to stage

## Rust Workflow (MANDATORY)

After EVERY modification to Rust code, run these three steps IN ORDER before doing anything else:

```bash
cargo fmt --all                              # 1. Format first
cargo clippy --workspace -- -D warnings      # 2. Lint second
cargo test --workspace -- --test-threads=1   # 3. Test third
```

NEVER batch these to the end of a session. Run after EACH file modification.

## Pre-commit Checklist (MANDATORY — only when code was modified)

BEFORE any commit or push that includes code changes (Rust, TypeScript, config files),
you MUST run ALL of the following IN ORDER.
If ANY step fails, DO NOT commit. Fix and re-run ALL steps.

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace -- -D warnings`
3. `cargo test --workspace`
4. `cd gui/tests && pnpm test` (Playwright E2E tests)
5. `act push --job test --matrix os:ubuntu-latest` (CI validation with act)

NEVER skip steps 4 or 5. Rust tests, E2E tests, AND CI validation are ALL required.
NEVER commit or push if you have not seen ALL steps pass in the current session.

This checklist does NOT apply to documentation-only changes (`.md` files, `docs/`, `LICENSE`, `CHANGELOG`).

## Testing Architecture

### 3 Layers of Tests

| Layer | Location | Command | What it tests |
|-------|----------|---------|---------------|
| **Rust unit tests** | `core/src/*.rs` (mod tests) | `cargo test -p rhinolabs-core` | Core logic, serde, config loading |
| **Rust integration + contract** | `gui/src-tauri/tests/` | `cargo test -p rhinolabs-gui` | Command chains, JSON shape validation |
| **Playwright E2E** | `gui/tests/e2e/` | `cd gui/tests && pnpm test` | Frontend UI with mocked backend |

### Testing Patterns

- **ENV_MUTEX**: Global mutex in `core/src/test_utils.rs`. Always acquire BEFORE creating `TestEnv`. Run with `--test-threads=1`.
- **TestEnv**: Sets `RHINOLABS_DEV_PATH` to temp dir, restores on drop.
- **Profile tests**: Also need `RHINOLABS_CONFIG_PATH` pointing to a FILE (not directory).
- **GUI integration tests**: Call `rhinolabs_core` functions directly (commands aren't publicly exported from gui crate).
- **Contract tests**: Serialize Rust structs → verify JSON fields match `gui/src/types.ts`.
- **E2E tests**: Use `gui/tests/e2e/mocks/tauri-mock.js` to simulate `window.__TAURI_INTERNALS__`. The real Rust backend NEVER runs during E2E tests.

### Known Serde Behaviors

- `SkillCategory` uses `#[serde(rename_all = "lowercase")]`:
  - `AiSdk` → `"aisdk"` (NOT `"ai-sdk"` as TypeScript `types.ts` declares)
  - Invalid values like `"workflow"` in `.skills-config.json` cause `load_config()` to fail → breaks `Skills::list()` entirely
- `SkillsConfig` fields `disabled`/`custom`/`sources` are NOT `#[serde(default)]` — empty `{}` JSON fails deserialization
- Profile fields use `#[serde(rename_all = "camelCase")]` — Rust snake_case becomes JS camelCase

## Skill System

### Categories (precedence order)

1. **Corporate** (`rhinolabs-standards`, `rhinolabs-architecture`, `rhinolabs-security`) — ALWAYS take precedence
2. **Backend** (node, api-rest, rust, databases)
3. **Frontend** (`react-patterns`, `typescript-best-practices`, `tailwind-4`, `zod-4`, `zustand-5`)
4. **Testing** (`testing-strategies`, `playwright`)
5. **AiSdk** (`ai-sdk-core`, `ai-sdk-react`, `nextjs-integration`)
6. **Utilities** (`skill-creator`)
7. **Custom** (default for unknown skills)

### Category Resolution Priority

1. User-defined `categoryMap` in `.skills-config.json` (checked first)
2. Hardcoded constants in `skills.rs` (built-in skills)
3. Default: `SkillCategory::Custom`

## Multi-Target Deployment

| Target | User Skills | Project Skills | Instructions File |
|--------|------------|----------------|-------------------|
| Claude Code | `~/.claude/skills/` | `.claude/skills/` | `CLAUDE.md` |
| Amp | `~/.config/agents/skills/` | `.agents/skills/` | `AGENTS.md` |
| Antigravity | `~/.gemini/antigravity/skills/` | `.agent/skills/` | `GEMINI.md` |
| OpenCode | `~/.config/opencode/skills/` | `.opencode/skills/` | `opencode.json` |

Skills deploy uses **symlinks** (Unix) or **NTFS junctions** (Windows), with fallback to `copy_dir_recursive`.

## File System Conventions

- Skill definition: `skills/{id}/SKILL.md` (YAML frontmatter + markdown)
- Skills config: `.skills-config.json` (disabled, custom, sources, categoryMap, skillMeta)
- Profiles config: `~/.config/rhinolabs-ai/profiles.json` (auto-creates Main-Profile if missing)
- Output styles: `output-styles/{id}.md` (YAML frontmatter + markdown)
- Plugin manifest: `.claude-plugin/plugin.json`
- Settings: `settings.json`
- MCP config: `.mcp.json`

## Known Issues

1. **SkillCategory mismatch**: Rust serializes `AiSdk` as `"aisdk"`, TypeScript expects `"ai-sdk"`
2. **Binary name in release.yml**: Should be `--bin rhinolabs-ai`, verify before releases
3. **No releases published yet**: Tag never pushed, workflow never ran
4. **Homebrew tap**: `homebrew-tap` repo doesn't exist yet

## Makefile

```bash
make test          # Run ALL tests (Rust + E2E)
make test-rust     # Run Rust tests only
make test-e2e      # Run E2E tests only
make test-quick    # Run Rust tests only (no E2E)
make build         # Build all components
make setup-hooks   # Configure git hooks (run after clone)
make run           # Run Tauri app (dev mode)
```
