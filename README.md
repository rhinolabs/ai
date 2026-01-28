# Rhinolabs AI

Enterprise-grade plugin and configuration management system for Claude Code.

## Overview

Rhinolabs AI provides a complete solution for standardizing Claude Code across development teams:

- **Plugin**: Curated skills for consistent coding standards
- **CLI**: Command-line tool for profile installation and team sync
- **GUI**: Desktop application for plugin management (lead developers)
- **Profiles**: Organize skills into reusable bundles (user-level and project-level)
- **Deploy/Sync**: Distribute configurations across your team via GitHub releases

## System Architecture

```mermaid
graph TB
    subgraph "Rhinolabs AI Ecosystem"
        subgraph "Shared Library"
            CORE[rhinolabs-core<br/>Rust Library]
        end

        subgraph "Applications"
            GUI[GUI Desktop App<br/>Tauri + React]
            CLI[CLI Tool<br/>rhinolabs-ai / rlai]
        end

        subgraph "Plugin"
            PLUGIN[rhinolabs-claude<br/>Claude Code Plugin]
            SKILLS[Skills Library]
            MCP[MCP Servers Config]
        end
    end

    GUI --> CORE
    CLI --> CORE
    CORE --> PLUGIN
    PLUGIN --> SKILLS
    PLUGIN --> MCP

    style CORE fill:#4a5568,stroke:#718096,color:#fff
    style GUI fill:#805ad5,stroke:#9f7aea,color:#fff
    style CLI fill:#3182ce,stroke:#63b3ed,color:#fff
    style PLUGIN fill:#38a169,stroke:#68d391,color:#fff
```

## Deploy & Sync Flow

The system separates concerns between lead developers (configuration management) and team developers (consumption):

```mermaid
sequenceDiagram
    participant Lead as Lead Developer
    participant GUI as GUI App
    participant GH as GitHub Releases
    participant CLI as CLI Tool
    participant Dev as Team Developer

    Note over Lead,GUI: Configuration Management (Write)
    Lead->>GUI: Create/Edit Profiles
    Lead->>GUI: Assign Skills
    Lead->>GUI: Click Deploy
    GUI->>GH: Upload rhinolabs-config.zip

    Note over CLI,Dev: Configuration Consumption (Read-Only)
    Dev->>CLI: rhinolabs-ai sync
    CLI->>GH: Download latest config
    CLI->>Dev: Update local configuration
    Dev->>CLI: rhinolabs-ai profile install X
    CLI->>Dev: Install profile to project
```

## Profiles System

Profiles organize skills into reusable bundles that can be applied at different scopes:

```mermaid
graph TB
    subgraph "User Level (Global)"
        MAIN[Main-Profile<br/>~/.claude/]
        MAIN_SKILLS[Agency Standards<br/>Security Rules<br/>Code Quality]
    end

    subgraph "Project Level (Local)"
        PROJ1[react-stack<br/>./apps/web/.claude-plugin/]
        PROJ2[rust-backend<br/>./apps/api/.claude-plugin/]
        PROJ3[ts-lib<br/>./packages/shared/.claude-plugin/]

        SKILLS1[React 19<br/>TypeScript<br/>Tailwind]
        SKILLS2[Rust Patterns<br/>Async/Await<br/>Error Handling]
        SKILLS3[TypeScript<br/>Testing<br/>Documentation]
    end

    MAIN --> MAIN_SKILLS
    PROJ1 --> SKILLS1
    PROJ2 --> SKILLS2
    PROJ3 --> SKILLS3

    subgraph "Claude Code Runtime"
        COMBINED[Combined Skills<br/>Main-Profile + Project Profile]
    end

    MAIN_SKILLS --> COMBINED
    SKILLS1 --> COMBINED
    SKILLS2 --> COMBINED
    SKILLS3 --> COMBINED

    style MAIN fill:#805ad5,stroke:#9f7aea,color:#fff
    style PROJ1 fill:#3182ce,stroke:#63b3ed,color:#fff
    style PROJ2 fill:#3182ce,stroke:#63b3ed,color:#fff
    style PROJ3 fill:#3182ce,stroke:#63b3ed,color:#fff
    style COMBINED fill:#38a169,stroke:#68d391,color:#fff
```

### User Profile (Main-Profile)

| Aspect | Description |
|--------|-------------|
| **Scope** | Applies to ALL projects |
| **Location** | `~/.claude/` |
| **Purpose** | Agency-wide standards, security rules |
| **Installation** | Auto-prompted on first sync |

### Project Profiles

| Aspect | Description |
|--------|-------------|
| **Scope** | Applies only to specific project |
| **Location** | `<project>/.claude-plugin/` |
| **Purpose** | Tech-stack specific skills |
| **Installation** | Manual via `rhinolabs-ai profile install` |

## Quick Start

### For Team Developers

```bash
# 1. Install CLI via Homebrew
brew tap rhinolabs/tap
brew install rhinolabs-ai

# 2. Run any command (auto-syncs configuration on first run)
rhinolabs-ai profile list

# 3. Install Main-Profile (user-level, applies to all projects)
# (Prompted automatically on first sync)

# 4. Install project-specific profile
cd ~/your-project
rhinolabs-ai profile install react-stack
```

### For Lead Developers

```mermaid
flowchart LR
    A[Install GUI] --> B[Configure GitHub]
    B --> C[Create Profiles]
    C --> D[Assign Skills]
    D --> E[Deploy]
    E --> F[Team syncs via CLI]

    style E fill:#38a169,stroke:#68d391,color:#fff
```

1. Download and install the GUI from [Releases](https://github.com/rhinolabs/rhinolabs-ai/releases)
2. Configure GitHub repository in **Project Settings**
3. Create profiles and assign skills
4. Click **Deploy** to publish configuration

## CLI Commands

```bash
# Aliases: rhinolabs-ai or rlai

# Configuration sync (auto-runs on first command of terminal session)
rhinolabs-ai sync                    # Manual sync from GitHub

# Profile management
rhinolabs-ai profile list            # List all profiles
rhinolabs-ai profile show <id>       # Show profile details
rhinolabs-ai profile install <name>  # Install profile (current directory)
rhinolabs-ai profile install <name> -P /path  # Install to specific path
rhinolabs-ai profile update          # Update installed profile
rhinolabs-ai profile uninstall       # Remove profile from current directory

# Plugin management
rhinolabs-ai install                 # Install base plugin
rhinolabs-ai update                  # Update plugin
rhinolabs-ai uninstall               # Remove plugin
rhinolabs-ai status                  # Show installation status
rhinolabs-ai doctor                  # Run diagnostics

# MCP configuration
rhinolabs-ai sync-mcp                # Sync MCP servers from source
```

## Monorepo Example

```mermaid
graph TB
    subgraph "~/monorepo"
        ROOT[Project Root]

        subgraph "apps/"
            WEB[web/<br/>React Frontend]
            API[api/<br/>Rust Backend]
        end

        subgraph "packages/"
            SHARED[shared/<br/>TypeScript Library]
        end
    end

    subgraph "Installed Profiles"
        P1[react-stack]
        P2[rust-backend]
        P3[ts-lib]
    end

    WEB -.->|rhinolabs-ai profile install| P1
    API -.->|rhinolabs-ai profile install| P2
    SHARED -.->|rhinolabs-ai profile install| P3

    style P1 fill:#3182ce,stroke:#63b3ed,color:#fff
    style P2 fill:#dd6b20,stroke:#ed8936,color:#fff
    style P3 fill:#805ad5,stroke:#9f7aea,color:#fff
```

```bash
cd ~/monorepo

# Install different profiles for each subproject
rhinolabs-ai profile install react-stack -P ./apps/web
rhinolabs-ai profile install rust-backend -P ./apps/api
rhinolabs-ai profile install ts-lib -P ./packages/shared

# Claude Code automatically combines:
# - Main-Profile (user-level) + Project Profile (per directory)
```

## Installation Paths

```mermaid
graph LR
    subgraph "User Configuration"
        A1[CLI Config<br/>~/.config/rhinolabs-ai/]
        A2[User Skills<br/>~/.claude/skills/]
    end

    subgraph "Project Configuration"
        B1[Project Skills<br/>./project/.claude/skills/]
        B2[Plugin Manifest<br/>./project/.claude-plugin/]
    end

    subgraph "Plugin Installation"
        C1[macOS<br/>~/Library/Application Support/<br/>Claude Code/plugins/]
        C2[Linux<br/>~/.config/claude-code/plugins/]
        C3[Windows<br/>%APPDATA%/Claude Code/plugins/]
    end
```

| Component | Path |
|-----------|------|
| CLI Config | `~/.config/rhinolabs-ai/` |
| User Skills | `~/.claude/skills/` |
| Project Skills | `<project>/.claude/skills/` |
| Plugin (macOS) | `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude/` |
| Plugin (Linux) | `~/.config/claude-code/plugins/rhinolabs-claude/` |
| Plugin (Windows) | `%APPDATA%\Claude Code\plugins\rhinolabs-claude\` |

## Security Model

```mermaid
graph TB
    subgraph "Lead Developer"
        GUI_ACCESS[GUI App]
        DEPLOY[Deploy to GitHub<br/>REQUIRES GITHUB_TOKEN]
    end

    subgraph "Team Developer"
        CLI_ACCESS[CLI Tool]
        SYNC[Sync from GitHub<br/>Read-Only, No Token]
        INSTALL[Install Profiles<br/>Local Only]
    end

    subgraph "GitHub"
        RELEASES[Releases<br/>rhinolabs-config.zip]
    end

    GUI_ACCESS --> DEPLOY
    DEPLOY -->|Write| RELEASES
    CLI_ACCESS --> SYNC
    CLI_ACCESS --> INSTALL
    SYNC -->|Read| RELEASES

    style DEPLOY fill:#e53e3e,stroke:#fc8181,color:#fff
    style SYNC fill:#38a169,stroke:#68d391,color:#fff
    style INSTALL fill:#38a169,stroke:#68d391,color:#fff
```

- **GUI (Lead Devs)**: Full access - create, edit, deploy configurations
- **CLI (Team Devs)**: Read-only - sync and install, cannot modify shared config
- **GITHUB_TOKEN**: Only required for deploy (GUI), not for sync (CLI)

## Project Structure

```
rhinolabs-ai/
├── cli/                    # Rust CLI (rhinolabs-ai, rlai)
├── core/                   # Shared Rust library
├── gui/                    # Tauri desktop app (React + Rust)
├── rhinolabs-claude/       # Base plugin with skills
└── docs/                   # Documentation
```

## Development

### Prerequisites

- Rust 1.70+
- Node.js 18+
- pnpm (for GUI)

### Building

```bash
# CLI
cd cli && cargo build --release

# GUI
cd gui && pnpm install && pnpm tauri build

# Core library
cd core && cargo build
```

### Testing

```bash
# Unit tests
cargo test --workspace

# GUI E2E tests
cd gui/tests && pnpm test
```

## Documentation

- [Architecture](ARCHITECTURE.md) - System design and data flow
- [CLI Guide](cli/README.md) - Detailed CLI documentation
- [GUI Guide](gui/README.md) - Desktop app documentation
- [Plugin Structure](rhinolabs-claude/README.md) - Skills and plugin details

## Support

- Issues: [GitHub Issues](https://github.com/rhinolabs/rhinolabs-ai/issues)
- Internal: Contact DevOps team

## License

Proprietary - Rhinolabs Internal Use Only

---

**Version**: 2.1.0
**Last Updated**: 2026-01-28
