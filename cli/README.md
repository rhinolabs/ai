# Rhinolabs AI CLI

Command-line interface for managing Rhinolabs AI profiles and plugin configuration.

## Overview

```mermaid
graph LR
    subgraph "CLI Capabilities"
        SYNC[Sync Config<br/>from GitHub]
        PROFILE[Profile<br/>Management]
        PLUGIN[Plugin<br/>Management]
        MCP[MCP<br/>Configuration]
    end

    subgraph "Local Storage"
        CONFIG[~/.config/rhinolabs-ai/]
        CLAUDE[~/.claude/]
        PROJECT[./project/.claude-plugin/]
    end

    SYNC --> CONFIG
    PROFILE --> CLAUDE
    PROFILE --> PROJECT
    PLUGIN --> CONFIG
    MCP --> CONFIG

    style SYNC fill:#38a169,stroke:#68d391,color:#fff
    style PROFILE fill:#3182ce,stroke:#63b3ed,color:#fff
```

## Installation

### Via Homebrew (Recommended)

```bash
brew tap rhinolabs/tap
brew install rhinolabs-ai
```

### Via Cargo

```bash
cargo install --path .
```

### From Source

```bash
cd cli
cargo build --release
# Binary at: target/release/rhinolabs-ai
```

## Command Structure

```mermaid
graph TB
    CLI[rhinolabs-ai / rlai]

    CLI --> SYNC[sync]
    CLI --> PROFILE[profile]
    CLI --> SKILL[skill]
    CLI --> RAG[rag]
    CLI --> INSTALL[install]
    CLI --> UPDATE[update]
    CLI --> UNINSTALL[uninstall]
    CLI --> STATUS[status]
    CLI --> DOCTOR[doctor]
    CLI --> SYNCMCP[sync-mcp]

    PROFILE --> P_LIST[list]
    PROFILE --> P_SHOW[show]
    PROFILE --> P_INST["install [--target]"]
    PROFILE --> P_UPD["update [--target]"]
    PROFILE --> P_UNINST["uninstall [--target]"]

    SKILL --> S_LIST[list]
    SKILL --> S_SHOW[show]
    SKILL --> S_CREATE[create]
    SKILL --> S_SETCAT[set-category]

    RAG --> R_INIT[init]
    RAG --> R_STATUS[status]
    RAG --> R_REMOVE[remove]
    RAG --> R_CREATE_KEY[create-key]
    RAG --> R_LIST_KEYS[list-keys]
    RAG --> R_SET_ADMIN[set-admin-key]

    style CLI fill:#805ad5,stroke:#9f7aea,color:#fff
    style SYNC fill:#38a169,stroke:#68d391,color:#fff
    style PROFILE fill:#3182ce,stroke:#63b3ed,color:#fff
    style SKILL fill:#dd6b20,stroke:#ed8936,color:#fff
    style RAG fill:#38a169,stroke:#68d391,color:#fff
```

## Auto-Sync Feature

On the first command of each terminal session, the CLI automatically syncs configuration from GitHub.

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Marker as /tmp/rhinolabs-session-sync
    participant GitHub

    User->>CLI: rhinolabs-ai profile list
    CLI->>Marker: Check if exists
    alt First command of session
        Marker-->>CLI: Not found
        CLI->>GitHub: Fetch latest config
        GitHub-->>CLI: rhinolabs-config.zip
        CLI->>CLI: Update local config
        CLI->>Marker: Create marker file
        CLI->>User: Show sync result
        CLI->>User: Prompt Main-Profile install
    else Already synced
        Marker-->>CLI: Exists (< 1 hour old)
        CLI->>User: Skip sync, run command
    end
```

```bash
# First command triggers auto-sync
rhinolabs-ai profile list

# Output:
# ━━━ Configuration Sync ━━━
# Checking for updates...
# ✓ Configuration synced: v1.2.0
#
# ━━━ Main-Profile Setup ━━━
# Main-Profile is not installed in your user memory (~/.claude/).
# Install Main-Profile now? [Y/n]:
```

## Commands Reference

### Profile Management

```mermaid
flowchart LR
    subgraph "Profile Commands"
        LIST[profile list] --> SHOW[profile show]
        SHOW --> INSTALL[profile install]
        INSTALL --> UPDATE[profile update]
        UPDATE --> UNINSTALL[profile uninstall]
    end
```

```bash
# List all available profiles
rhinolabs-ai profile list

# Show profile details
rhinolabs-ai profile show <profile-id>

# Install profile to current directory (default target: Claude Code)
rhinolabs-ai profile install <profile-name>

# Install profile for a specific target
rhinolabs-ai profile install <profile-name> --target amp

# Install for multiple targets
rhinolabs-ai profile install <profile-name> -t amp -t claude-code

# Install for all supported targets
rhinolabs-ai profile install <profile-name> --target all

# Install profile to specific path
rhinolabs-ai profile install <profile-name> -P /path/to/project

# Update installed profile (detects profile automatically)
rhinolabs-ai profile update

# Update specific profile for a target
rhinolabs-ai profile update <profile-name> --target amp

# Uninstall profile from current directory
rhinolabs-ai profile uninstall

# Uninstall only specific target artifacts
rhinolabs-ai profile uninstall --target amp

# Uninstall from specific path
rhinolabs-ai profile uninstall -P /path/to/project
```

**Supported targets**: `claude-code` (default), `amp`, `antigravity`, `open-code`, `all`

| Target | Skills Dir | Instructions File | Config Dir |
|--------|-----------|-------------------|------------|
| `claude-code` | `.claude/skills/` | `CLAUDE.md` | `.claude/` |
| `amp` | `.agents/skills/` | `AGENTS.md` | `.agents/` |
| `antigravity` | `.agent/skills/` | `GEMINI.md` | `.agent/` |
| `open-code` | `.opencode/skills/` | `opencode.json` | `.opencode/` |

### Skill Management

```bash
# List all skills (grouped by category)
rhinolabs-ai skill list

# Show skill details
rhinolabs-ai skill show <skill-id>

# Create a new custom skill
rhinolabs-ai skill create --id my-skill --name "My Skill" --category frontend

# Create with description
rhinolabs-ai skill create --id my-skill --name "My Skill" --category frontend --description "Skill description"

# Change skill category
rhinolabs-ai skill set-category <skill-id> <category>

# Available categories: corporate, frontend, testing, ai-sdk, utilities, custom
```

### Configuration Sync

```bash
# Manual sync from GitHub
rhinolabs-ai sync
```

### Plugin Management

```bash
# Install base plugin
rhinolabs-ai install

# Install from local directory (development)
rhinolabs-ai install --local /path/to/rhinolabs-claude

# Update plugin
rhinolabs-ai update

# Uninstall plugin
rhinolabs-ai uninstall

# Show status
rhinolabs-ai status

# Run diagnostics
rhinolabs-ai doctor
```

### MCP Configuration

```bash
# Sync MCP servers from configured source
rhinolabs-ai sync-mcp

# Sync from URL
rhinolabs-ai sync-mcp --url https://config.example.com/mcp.json

# Sync from file
rhinolabs-ai sync-mcp --file ./mcp-config.json

# Dry run (show what would be done)
rhinolabs-ai sync-mcp --dry-run
```

### RAG (Project Memory)

RAG provides per-project memory capabilities. Claude Code can save and retrieve architectural decisions, context, and knowledge through a centralized MCP Worker.

```mermaid
flowchart TB
    subgraph "RAG Commands"
        INIT[rag init] --> STATUS[rag status]
        STATUS --> REMOVE[rag remove]
        CREATE_KEY[rag create-key] --> LIST_KEYS[rag list-keys]
        SET_ADMIN[rag set-admin-key]
    end

    subgraph "Local Files"
        RAG_JSON[.claude/rag.json]
        SETTINGS[~/.config/rhinolabs-ai/<br/>rag-settings.json]
    end

    INIT --> RAG_JSON
    STATUS --> RAG_JSON
    REMOVE --> RAG_JSON
    SET_ADMIN --> SETTINGS

    style INIT fill:#38a169,stroke:#68d391,color:#fff
    style CREATE_KEY fill:#805ad5,stroke:#9f7aea,color:#fff
```

```bash
# Initialize RAG for current project
rhinolabs-ai rag init --project my-project --api-key rl_xxx

# Show RAG status
rhinolabs-ai rag status

# Remove RAG from project
rhinolabs-ai rag remove

# Admin: Set admin key for key management
rhinolabs-ai rag set-admin-key <admin-secret>

# Admin: Create new API key
rhinolabs-ai rag create-key --name "My Team"
rhinolabs-ai rag create-key --name "Client X" --projects project-a,project-b

# Admin: List all API keys
rhinolabs-ai rag list-keys
```

**RAG Setup Flow:**

```mermaid
sequenceDiagram
    participant Admin
    participant CLI
    participant MCP as MCP Worker
    participant Dev as Developer

    Note over Admin,MCP: One-time: Create API Key
    Admin->>CLI: rag set-admin-key <secret>
    Admin->>CLI: rag create-key --name "Team"
    CLI->>MCP: POST /admin/keys
    MCP-->>CLI: API Key: rl_abc123...
    Admin->>Dev: Share API key

    Note over Dev,MCP: Per-project: Initialize RAG
    Dev->>CLI: rag init --project X --api-key rl_abc123
    CLI->>CLI: Create .claude/rag.json

    Note over Dev,MCP: Runtime: Claude Code uses MCP
    Dev->>CLI: claude (start session)
    CLI->>MCP: MCP tools (rag_save, rag_search, etc.)
```

## Profile Installation Flow

```mermaid
flowchart TB
    START[rhinolabs-ai profile install X]
    CHECK_PROFILE{Profile exists?}
    CHECK_PATH{Path specified?}
    USE_CWD[Use current directory]
    USE_PATH[Use specified path]
    CONFIRM[Show confirmation]
    ACCEPT{User accepts?}
    INSTALL_SKILLS[Copy skills to .claude/skills/]
    CREATE_MANIFEST[Create .claude-plugin/plugin.json]
    CREATE_CLAUDE[Generate CLAUDE.md]
    DONE[Installation complete]
    CANCEL[Installation cancelled]

    START --> CHECK_PROFILE
    CHECK_PROFILE -->|Yes| CHECK_PATH
    CHECK_PROFILE -->|No| ERROR[Profile not found]
    CHECK_PATH -->|No| USE_CWD
    CHECK_PATH -->|Yes| USE_PATH
    USE_CWD --> CONFIRM
    USE_PATH --> CONFIRM
    CONFIRM --> ACCEPT
    ACCEPT -->|Yes| INSTALL_SKILLS
    ACCEPT -->|No| CANCEL
    INSTALL_SKILLS --> CREATE_MANIFEST
    CREATE_MANIFEST --> CREATE_CLAUDE
    CREATE_CLAUDE --> DONE

    style START fill:#805ad5,stroke:#9f7aea,color:#fff
    style DONE fill:#38a169,stroke:#68d391,color:#fff
    style CANCEL fill:#e53e3e,stroke:#fc8181,color:#fff
```

## Profile Types

### User Profile (Main-Profile)

```mermaid
graph TB
    subgraph "Main-Profile Installation"
        MAIN[Main-Profile]
        CLAUDE_DIR[~/.claude/]
        SKILLS[skills/]
        INSTRUCTIONS[CLAUDE.md]
    end

    MAIN --> CLAUDE_DIR
    CLAUDE_DIR --> SKILLS
    CLAUDE_DIR --> INSTRUCTIONS

    style MAIN fill:#805ad5,stroke:#9f7aea,color:#fff
```

- **Scope**: Applies to ALL projects
- **Location**: `~/.claude/`
- **Purpose**: Agency-wide standards and general skills
- **Installation**: Prompted automatically on first sync

### Project Profiles

```mermaid
graph TB
    subgraph "Project Profile Installation"
        PROFILE[react-stack]
        PLUGIN_DIR[.claude-plugin/]
        CLAUDE_DIR[.claude/]
        MANIFEST[plugin.json]
        SKILLS[skills/]
        INSTRUCTIONS[CLAUDE.md]
    end

    PROFILE --> PLUGIN_DIR
    PROFILE --> CLAUDE_DIR
    PLUGIN_DIR --> MANIFEST
    CLAUDE_DIR --> SKILLS
    CLAUDE_DIR --> INSTRUCTIONS

    style PROFILE fill:#3182ce,stroke:#63b3ed,color:#fff
```

- **Scope**: Applies only to specific project
- **Location**: `<project>/.claude-plugin/`
- **Purpose**: Tech-stack specific skills

## Monorepo Example

```mermaid
graph TB
    subgraph "~/monorepo"
        subgraph "apps/"
            WEB[web/]
            API[api/]
        end
        subgraph "packages/"
            SHARED[shared/]
        end
    end

    subgraph "Installed"
        WEB_PLUGIN[.claude-plugin/<br/>react-stack]
        API_PLUGIN[.claude-plugin/<br/>rust-backend]
        SHARED_PLUGIN[.claude-plugin/<br/>ts-lib]
    end

    WEB --> WEB_PLUGIN
    API --> API_PLUGIN
    SHARED --> SHARED_PLUGIN

    style WEB_PLUGIN fill:#3182ce,stroke:#63b3ed,color:#fff
    style API_PLUGIN fill:#dd6b20,stroke:#ed8936,color:#fff
    style SHARED_PLUGIN fill:#805ad5,stroke:#9f7aea,color:#fff
```

```bash
cd ~/monorepo

# Frontend app (React)
rhinolabs-ai profile install react-stack -P ./apps/web

# Backend API (Rust)
rhinolabs-ai profile install rust-backend -P ./apps/api

# Shared library (TypeScript)
rhinolabs-ai profile install ts-lib -P ./packages/shared
```

## Configuration

Configuration is stored in `~/.config/rhinolabs-ai/`:

```
~/.config/rhinolabs-ai/
├── profiles.json       # Profile definitions (synced from GitHub)
├── skills/             # Skill definitions (synced from GitHub)
├── .project.json       # GitHub repository settings
└── ...
```

## Security Model

```mermaid
graph LR
    subgraph "CLI Permissions"
        READ[Read Operations]
        SYNC[Sync from GitHub]
        INSTALL[Install Profiles]
    end

    subgraph "NOT Available in CLI"
        DEPLOY[Deploy to GitHub]
        EXPORT[Export Config]
        MODIFY[Modify Shared Config]
    end

    READ --> ALLOWED[Allowed]
    SYNC --> ALLOWED
    INSTALL --> ALLOWED
    DEPLOY --> BLOCKED[GUI Only]
    EXPORT --> BLOCKED
    MODIFY --> BLOCKED

    style ALLOWED fill:#38a169,stroke:#68d391,color:#fff
    style BLOCKED fill:#e53e3e,stroke:#fc8181,color:#fff
```

Team developers cannot modify shared configuration - only sync and install locally.

## Troubleshooting

### Sync Failed

```bash
# Check GitHub configuration
rhinolabs-ai doctor

# Verify network connectivity
curl -I https://github.com

# Manual sync with verbose output
rhinolabs-ai sync
```

### Profile Not Found

```bash
# List available profiles
rhinolabs-ai profile list

# Ensure sync has completed
rhinolabs-ai sync
```

### Permission Denied

```bash
# Check installation path permissions
ls -la ~/.claude/
ls -la ~/.config/rhinolabs-ai/
```

## Architecture

```mermaid
graph TB
    subgraph "CLI Package"
        MAIN[main.rs<br/>Entry Point]
        COMMANDS[commands/]
        UI[ui.rs<br/>Terminal UI]
    end

    subgraph "Commands"
        PROFILE_CMD[profile.rs]
        SKILL_CMD[skill.rs]
        DEPLOY_CMD[deploy.rs]
        AUTO_SYNC[auto_sync.rs]
        INSTALL_CMD[install.rs]
        STATUS_CMD[status.rs]
        RAG_CMD[rag.rs]
    end

    subgraph "Core Library"
        CORE[rhinolabs-core]
    end

    subgraph "External"
        MCP[MCP Worker]
    end

    MAIN --> COMMANDS
    MAIN --> UI
    COMMANDS --> PROFILE_CMD
    COMMANDS --> SKILL_CMD
    COMMANDS --> DEPLOY_CMD
    COMMANDS --> AUTO_SYNC
    COMMANDS --> INSTALL_CMD
    COMMANDS --> STATUS_CMD
    COMMANDS --> RAG_CMD
    PROFILE_CMD --> CORE
    SKILL_CMD --> CORE
    DEPLOY_CMD --> CORE
    AUTO_SYNC --> CORE
    RAG_CMD --> CORE
    RAG_CMD -.->|Admin API| MCP

    style MAIN fill:#805ad5,stroke:#9f7aea,color:#fff
    style CORE fill:#4a5568,stroke:#718096,color:#fff
    style RAG_CMD fill:#38a169,stroke:#68d391,color:#fff
    style MCP fill:#f6993f,stroke:#de751f,color:#fff
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Locally

```bash
cargo run -- profile list
cargo run -- sync
```

---

**Version**: 1.1.0
**Last Updated**: 2026-01-29
