# Rhinolabs AI - Architecture

This document describes the architecture of the Rhinolabs AI system for Claude Code customization.

## Assets

The system is composed of three independent assets that can be installed separately:

### 1. CLI (`rhinolabs-ai`)

**Location**: `/cli/`

Standalone command-line tool for managing skills, profiles, and plugin configuration.

```bash
# Installation
cargo install --path cli

# Or via Homebrew (when published)
brew install rhinolabs/tap/rhinolabs-ai
```

**Commands**:
- `rhinolabs-ai skill list|show|install|disable|enable`
- `rhinolabs-ai profile list|show|install|update|uninstall`
- `rhinolabs-ai mcp list|add|remove|sync`
- `rhinolabs-ai config show|set`
- `rhinolabs-ai sync` - Sync configuration from GitHub

**Alias**: `rlai` can be used as a short form (e.g., `rlai profile list`)

### 2. Plugin (`rhinolabs-claude`)

**Location**: `/rhinolabs-claude/`

The base plugin that gets installed to the user's Claude Code plugins directory.

**Installation paths**:
- Linux: `~/.config/claude-code/plugins/rhinolabs-claude/`
- macOS: `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude/`
- Windows: `%APPDATA%\Claude Code\plugins\rhinolabs-claude\`

**Structure**:
```
rhinolabs-claude/
├── .claude-plugin/
│   └── plugin.json       # Plugin manifest (name, version, author)
├── .claude/
│   └── (empty)           # Claude config placeholder
├── skills/               # Skill definitions
│   ├── react-19/
│   │   └── SKILL.md
│   ├── typescript/
│   │   └── SKILL.md
│   └── ...
├── output-styles/        # Output style definitions
│   └── *.md
├── CLAUDE.md             # Default instructions
├── settings.json         # Default settings
├── .mcp.json             # MCP server configurations
└── .skills-config.json   # Skills state (enabled/disabled)
```

### 3. GUI (`rhinolabs-gui`)

**Location**: `/gui/`

Tauri-based desktop application for visual management.

**Tech stack**:
- Frontend: React + TypeScript + Vite
- Backend: Tauri (Rust)
- Core: Shared Rust library (`rhinolabs-core`)

---

## Profiles System

Profiles organize skills into reusable bundles that can be installed at different scopes.

### Profile Structure

```rust
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub profile_type: ProfileType,      // User | Project
    pub skills: Vec<String>,            // Skill IDs
    pub auto_invoke_rules: Vec<AutoInvokeRule>,  // When to load each skill
    pub instructions: Option<String>,   // Custom instructions for CLAUDE.md
    pub generate_copilot: bool,         // Generate .github/copilot-instructions.md
    pub generate_agents: bool,          // Generate AGENTS.md as master
    pub created_at: String,
    pub updated_at: String,
}

pub struct AutoInvokeRule {
    pub skill_id: String,
    pub trigger: String,       // "Editing .tsx/.jsx files"
    pub description: String,   // "React 19 patterns and hooks"
}
```

### Profile Types

#### User Profile (Main-Profile)

- **Scope**: User-level, applies to ALL projects
- **Install location**: `~/.claude/`
- **Use case**: Agency-wide standards that every dev should have
- **Only one**: The "main" profile is the only User type profile

**What gets installed**:
```
~/.claude/
├── skills/           # All skills from Main-Profile
├── CLAUDE.md         # Instructions (linked from plugin)
├── settings.json     # Settings (linked from plugin)
└── .output-style     # Active output style
```

#### Project Profile

- **Scope**: Project-level, applies only to specific project
- **Install location**: `<project-path>/` (as a plugin)
- **Use case**: Tech-stack specific skills (React, Django, etc.)

**What gets installed** (as a plugin structure):
```
<project-path>/
├── .claude-plugin/
│   └── plugin.json             # Profile as plugin manifest
├── .claude/
│   └── skills/                 # Skills from the profile
├── .github/
│   └── copilot-instructions.md # For GitHub Copilot (if enabled)
├── CLAUDE.md                   # Generated with auto-invoke table
├── AGENTS.md                   # Master file (if enabled)
└── settings.json               # Profile settings (if defined)
```

---

## Multi-AI Support

The system generates instruction files for multiple AI assistants from a single source.

### File Generation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                   Profile Definition (GUI)                       │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Skills: [react-19, typescript, tailwind-4]               │  │
│  │  Auto-invoke Rules:                                       │  │
│  │    - react-19 → "Editing .tsx/.jsx"                       │  │
│  │    - typescript → "Editing .ts files"                     │  │
│  │  Instructions: "Follow corporate standards..."            │  │
│  │  Generate: [x] Copilot  [x] AGENTS.md                     │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │  rhinolabs-ai profile install  │
              └───────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐   ┌───────────────────┐   ┌─────────────────────┐
│  CLAUDE.md    │   │ copilot-instr.md  │   │    AGENTS.md        │
│  (Claude Code)│   │ (GitHub Copilot)  │   │ (Master/Reference)  │
└───────────────┘   └───────────────────┘   └─────────────────────┘
```

### CLAUDE.md Format (Generated)

```markdown
# Project Instructions

> Auto-generated by rhinolabs-ai | Profile: react-stack
> Run `rhinolabs-ai profile update` to regenerate

## Auto-invoke Skills

IMPORTANT: Load these skills based on context:

| Context | Skill | Read First |
|---------|-------|------------|
| Editing .tsx/.jsx | react-19 | `.claude/skills/react-19/SKILL.md` |
| Editing .ts | typescript | `.claude/skills/typescript/SKILL.md` |
| Writing tests | playwright | `.claude/skills/playwright/SKILL.md` |

## Project Standards

[Custom instructions from profile here]

## Available Skills

Skills in `.claude/skills/`:
- react-19: React 19 patterns
- typescript: TypeScript guidelines
- tailwind-4: Tailwind CSS v4

---
*Installed by rhinolabs-ai | Profile: react-stack*
```

### .github/copilot-instructions.md Format

Same content as CLAUDE.md, adapted for GitHub Copilot:
- Removes Claude-specific references
- Adjusts skill paths for Copilot context

### AGENTS.md Format (Optional Master)

When `generate_agents` is enabled, AGENTS.md serves as the canonical source:
- Used as reference for manual regeneration
- Can be version-controlled separately
- Useful for teams using multiple AI tools

### Deploy Target Abstraction Layer (`core/src/targets/`)

The `targets` module provides a formal abstraction for deploying skills, instructions, and MCP configuration to multiple AI coding assistants. Each target has its own filesystem conventions:

| Target | User Skills | Project Skills | Instructions | MCP Config |
|--------|------------|----------------|-------------|------------|
| Claude Code | `~/.claude/skills/` | `.claude/skills/` | `CLAUDE.md` | `.mcp.json` |
| Amp | `~/.config/agents/skills/` | `.agents/skills/` | `AGENTS.md` | `settings.json` |
| Antigravity | `~/.gemini/antigravity/skills/` | `.agent/skills/` | `GEMINI.md` | `config.json` |
| OpenCode | `~/.config/opencode/skills/` | `.opencode/skills/` | `opencode.json` | `opencode.json` |

**Architecture:**

```
DeployTarget (enum)          Traits                    Implementations
┌──────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│ ClaudeCode   │    │ SkillDeployer       │    │ ClaudeCodeDeployer  │
│ Amp          │    │ InstructionsDeployer │◄───│ (all 4 traits)      │
│ Antigravity  │    │ McpDeployer         │    ├─────────────────────┤
│ OpenCode     │    │ TargetDetector      │    │ GenericDeployer     │
└──────────────┘    └─────────────────────┘    │ (any target)        │
                                               └─────────────────────┘
```

`ClaudeCodeDeployer` implements all 4 traits for Claude Code-specific behavior (MCP config, detection). `GenericDeployer` is parameterized by `DeployTarget` and implements `SkillDeployer` + `InstructionsDeployer` for any target — used by `Profiles::install()` for multi-target deployment via `--target` flag.

---

## Auto-invoke Rules

Auto-invoke rules tell AI assistants WHEN to load each skill based on context.

### Rule Structure

```json
{
  "skill_id": "react-19",
  "trigger": "Editing .tsx/.jsx files",
  "description": "React 19 patterns, hooks, Server Components"
}
```

### Common Triggers

| Trigger Pattern | Example Skills |
|-----------------|----------------|
| `Editing .tsx/.jsx files` | react-19 |
| `Editing .ts files` | typescript |
| `Editing Python files` | django-drf, pytest |
| `Writing tests` | playwright, pytest |
| `Working with styles` | tailwind-4 |
| `API development` | zod-4, ai-sdk-5 |
| `Any code change` | rhinolabs-security (Main-Profile) |

### Inheritance

When a Project Profile is installed:
1. **Main-Profile rules** apply globally (from `~/.claude/`)
2. **Project Profile rules** apply in project context
3. Claude Code merges both automatically

---

## Skill Sources

Skills can come from multiple sources:

1. **Built-in**: Bundled with the plugin (`rhinolabs-claude/skills/`)
2. **Remote**: Fetched from GitHub repositories
   - `anthropic-official`: https://github.com/anthropics/skills
   - `vercel-agent-skills`: https://github.com/vercel-labs/agent-skills
3. **Custom**: User-created skills

### Skill Structure

```
skill-name/
├── SKILL.md          # Skill definition with YAML frontmatter
└── assets/           # Optional supporting files
    └── templates/
```

**SKILL.md format**:
```markdown
---
id: skill-name
name: Skill Display Name
description: What this skill does
version: 1.0.0
category: framework|library|tool|pattern
triggers:
  - "keyword1"
  - "keyword2"
---

# Skill Content

Instructions for Claude...
```

---

## Data Storage

### Configuration Files

| File | Location | Purpose |
|------|----------|---------|
| `profiles.json` | `~/.config/rhinolabs-ai/` | Profile definitions with auto-invoke rules |
| `.skills-config.json` | Plugin directory | Skill states |
| `settings.json` | Plugin directory | Plugin settings |
| `.mcp.json` | Plugin directory | MCP server config |
| `.project.json` | Plugin directory | GitHub release config |

### Claude Code Integration

Claude Code automatically loads:
- User-level: `~/.claude/CLAUDE.md`, `~/.claude/skills/*`
- Project-level: `./.claude/CLAUDE.md`, `./.claude/skills/*`
- Plugins: `<plugins-dir>/*/` with `.claude-plugin/plugin.json`

> **Note:** These paths are formalized in `core/src/targets/target_paths.rs` via `TargetPaths::user_skills_dir(DeployTarget::ClaudeCode)` and related methods. See the [Deploy Target Abstraction Layer](#deploy-target-abstraction-layer-coresrctargets) section for the full multi-target path mapping.

---

## Profile Installation Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      rhinolabs-ai CLI                        │
├─────────────────────────────────────────────────────────────┤
│  rhinolabs-ai profile install <name> [--path <dir>]         │
│                                                             │
│  Options:                                                   │
│    --no-copilot    Skip copilot-instructions.md generation │
│    --no-agents     Skip AGENTS.md generation               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
              ┌───────────────────────────────┐
              │     Profile Type Check        │
              └───────────────────────────────┘
                     │              │
            User Profile      Project Profile
                     │              │
                     ▼              ▼
         ┌─────────────────┐  ┌──────────────────────────────┐
         │  Install to     │  │  Install as Plugin           │
         │  ~/.claude/     │  │  to <project-path>/          │
         │                 │  │                              │
         │  + Skills       │  │  + .claude-plugin/plugin.json│
         │  + Instructions │  │  + .claude/skills/           │
         │  + Settings     │  │  + CLAUDE.md (with auto-invoke)│
         │  + Output Style │  │  + copilot-instructions.md   │
         └─────────────────┘  │  + AGENTS.md (if enabled)    │
                              └──────────────────────────────┘
```

---

## Development

### Core Library (`rhinolabs-core`)

Shared Rust library used by CLI and GUI.

**Modules**:
- `skills.rs` - Skill CRUD, remote fetching
- `profiles.rs` - Profile management, installation, file generation
- `settings.rs` - Plugin settings
- `instructions.rs` - CLAUDE.md management
- `output_styles.rs` - Output style management
- `mcp.rs` - MCP server configuration
- `paths.rs` - Cross-platform path resolution
- `project.rs` - GitHub release management
- `deploy.rs` - Configuration export, deploy & sync
- `targets/` - Multi-target deployment abstraction (Claude Code, Amp, Antigravity, OpenCode)

### Building

```bash
# CLI
cd cli && cargo build --release

# GUI
cd gui && pnpm install && pnpm tauri build

# Core (library)
cd core && cargo build
```

### Testing

```bash
# Unit tests
cargo test

# E2E tests (GUI)
cd gui && pnpm test:e2e
```

---

## Deploy & Sync (Team Distribution)

The deploy/sync system allows the lead developer to publish configurations that team members can pull.

### Deploy Flow (Lead Developer - GUI ONLY)

⚠️ **IMPORTANT**: Deploy is ONLY available through the GUI, NOT the CLI.
This ensures only authorized leads can publish configuration changes.

```
Lead Developer (GUI ONLY)
         │
         ▼
┌─────────────────────────────────┐
│  GUI → Project → Deploy button  │
│  (Requires GITHUB_TOKEN)        │
└─────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────┐
│  1. Export current config:      │
│     - profiles.json             │
│       (includes auto-invoke)    │
│     - skills/                   │
│     - CLAUDE.md                 │
│     - settings.json             │
│     - output-styles/            │
│     - .mcp.json                 │
└─────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────┐
│  2. Create GitHub Release       │
│     tag: config-vX.X.X          │
│     asset: rhinolabs-config.zip │
└─────────────────────────────────┘
```

### Sync Flow (Team Developers)

```
Team Developer (CLI)
         │
         ▼
┌─────────────────────────────────┐
│  rhinolabs-ai sync              │
└─────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────┐
│  1. Fetch latest config release │
│     from GitHub (config-v*)     │
└─────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────┐
│  2. Download & extract:         │
│     - profiles → ~/.config/     │
│     - skills → plugin/skills/   │
│     - settings → plugin/        │
│     - etc.                      │
└─────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────┐
│  3. Ready! Use profiles:        │
│  rhinolabs-ai profile install   │
└─────────────────────────────────┘
```

### CLI Commands (Team Developers)

```bash
# Sync latest configuration (auto-runs on first command of session)
rhinolabs-ai sync

# Profile management
rhinolabs-ai profile list
rhinolabs-ai profile show <id>
rhinolabs-ai profile install main
rhinolabs-ai profile install react-stack
rhinolabs-ai profile install react-stack --path ./project
rhinolabs-ai profile install react-stack --no-copilot
```

### GUI Commands (Lead Developer Only)

- **Project → Deploy**: Publish configuration to GitHub
- **Project → Export**: Export configuration to local file

### Requirements

1. **GitHub Repository**: Configure in GUI → Project Settings
2. **GITHUB_TOKEN**: Environment variable with repo write access (Lead only, for deploy)
3. **Config Release**: At least one deploy must exist (for team sync)

---

## Skill Categories

Skills are organized into categories for UI grouping and filtering.

### Current Categories

```rust
pub enum SkillCategory {
    Corporate,   // Agency-wide standards (rhinolabs-*)
    Frontend,    // Frontend frameworks (react-*, typescript-*, tailwind-*, zod-*, zustand-*)
    Testing,     // Testing tools (playwright, testing-strategies)
    AiSdk,       // AI SDK skills (ai-sdk-*, nextjs-integration)
    Utilities,   // Utility skills (skill-creator)
    Custom,      // User-created skills (default)
}
```

### Category Assignment (Current - Hardcoded)

Categories are currently determined by skill ID matching in `core/src/skills.rs`:

```rust
const CORPORATE_SKILLS: &[&str] = &["rhinolabs-standards", "rhinolabs-architecture", "rhinolabs-security"];
const FRONTEND_SKILLS: &[&str] = &["react-patterns", "typescript-best-practices", "tailwind-4", "zod-4", "zustand-5"];
const TESTING_SKILLS: &[&str] = &["testing-strategies", "playwright"];
const AI_SDK_SKILLS: &[&str] = &["ai-sdk-core", "ai-sdk-react", "nextjs-integration"];
const UTILITIES_SKILLS: &[&str] = &["skill-creator"];
```

**Problem**: New skills always fall into `Custom` unless code is modified.

### Planned: Dynamic Category Assignment

Move category mapping to `.skills-config.json`:

```json
{
  "categoryMap": {
    "figma-implement-design": "frontend",
    "my-custom-skill": "utilities"
  },
  "disabled": [...],
  "custom": [...]
}
```

**Changes required**:
1. Add `category_map: HashMap<String, SkillCategory>` to SkillsConfig
2. Modify `get_category()` to check category_map first, then fallback to constants
3. Add `set_category()` function
4. Modify `create()` to save category to config
5. Add CLI command: `rhinolabs skill create --category <cat>`
6. GUI already has category dropdown, just needs to pass it correctly

---

## Distribution

### Initial Setup (New Team Developer)

1. Clone the repo
2. Run install script: `./rhinolabs-claude/scripts/install.sh`
3. Run any command (auto-sync triggers): `rhinolabs-ai profile list`
4. Accept Main-Profile installation when prompted
5. Install project profiles as needed

Note: Team developers do NOT need GITHUB_TOKEN (only for read operations)

### Publishing Configuration Updates

1. Make changes in GUI (profiles, skills, auto-invoke rules, etc.)
2. Deploy via GUI: Project → Deploy
3. Team members run: `rhinolabs-ai sync`

---

---

## External Skill Sources

### Figma MCP Skills (Official)

Figma provides official skills for design-to-code workflows in their MCP server guide:

**Repository**: https://github.com/figma/mcp-server-guide

| Skill | Description |
|-------|-------------|
| `implement-design` | Translates Figma designs to pixel-perfect code (React + Tailwind default) |
| `create-design-system-rules` | Generates project-specific rules for CLAUDE.md |
| `code-connect-components` | Connects Figma components to code components |

**Key workflow** (from implement-design):
1. `get_design_context` - Fetch structured design data
2. `get_screenshot` - Visual reference for validation
3. Download assets from MCP server
4. Translate to project conventions
5. Validate 1:1 visual parity

### Community Skills

| Source | URL | Type |
|--------|-----|------|
| Anthropic Official | https://github.com/anthropics/skills | Official |
| Vercel Agent Skills | https://github.com/vercel-labs/agent-skills | Marketplace |
| Awesome Claude Skills | https://github.com/travisvn/awesome-claude-skills | Community |

---

## CI/CD Validation

Before pushing to GitHub, run these validations locally:

```bash
# One-liner
cargo fmt --all && cargo clippy --workspace -- -D warnings && cargo test --workspace

# Or use Makefile
make test-quick  # Rust only
make test        # Rust + E2E
```

### Common Clippy Fixes

| Error | Fix |
|-------|-----|
| `clone_on_copy` | Remove `.clone()` on Copy types |
| `derivable_impls` | Use `#[derive(Default)]` + `#[default]` attribute |
| `ptr_arg` | Use `&Path` instead of `&PathBuf` |
| `redundant_closure` | Replace `\|x\| Foo::from(x)` with `Foo::from` |

### Local CI with Act

```bash
act -j test --matrix os:ubuntu-latest -P ubuntu-latest=catthehacker/ubuntu:act-latest
```

**Note**: Workflow must have explicit components:
```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    components: clippy, rustfmt
```

---

**Last Updated**: 2026-02-04
**Version**: 2.4.0
