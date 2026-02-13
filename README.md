# Rhinolabs AI

Enterprise-grade skill, profile, and configuration management for AI coding assistants. Supports deploying to Claude Code, Amp, Antigravity, and OpenCode.

## Overview

Rhinolabs AI provides a complete solution for standardizing AI coding assistants across development teams:

- **Plugin**: Curated skills for consistent coding standards
- **CLI**: Command-line tool for profile installation and team sync
- **GUI**: Desktop application for plugin management (lead developers)
- **Profiles**: Organize skills into reusable bundles (user-level and project-level)
- **Deploy/Sync**: Distribute configurations across your team via GitHub releases
- **Multi-Target**: Deploy skills and instructions to Claude Code, Amp, Antigravity, and OpenCode via `--target` flag

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
    Dev->>CLI: rlai sync
    CLI->>GH: Download latest config
    CLI->>Dev: Update local configuration
    Dev->>CLI: rlai profile install X
    CLI->>Dev: Install profile to project
```

## Profiles System

Profiles organize skills AND instructions into reusable bundles that can be applied at different scopes:

```mermaid
graph TB
    subgraph "User Level (Global)"
        MAIN[Main-Profile<br/>~/.claude/]
        MAIN_CONTENT[CLAUDE.md Instructions<br/>Agency Standards<br/>Security Rules]
    end

    subgraph "Project Level (Local)"
        PROJ1[react-stack<br/>./apps/web/.claude-plugin/]
        PROJ2[rust-backend<br/>./apps/api/.claude-plugin/]
        PROJ3[ts-lib<br/>./packages/shared/.claude-plugin/]

        CONTENT1[Skills + Instructions<br/>React 19, TypeScript, Tailwind<br/>Auto-invoke Rules]
        CONTENT2[Skills + Instructions<br/>Rust Patterns, Error Handling<br/>Auto-invoke Rules]
        CONTENT3[Skills + Instructions<br/>TypeScript, Testing<br/>Auto-invoke Rules]
    end

    MAIN --> MAIN_CONTENT
    PROJ1 --> CONTENT1
    PROJ2 --> CONTENT2
    PROJ3 --> CONTENT3

    subgraph "Claude Code Runtime"
        COMBINED[Combined Skills + Instructions<br/>Main-Profile + Project Profile]
    end

    MAIN_CONTENT --> COMBINED
    CONTENT1 --> COMBINED
    CONTENT2 --> COMBINED
    CONTENT3 --> COMBINED

    style MAIN fill:#805ad5,stroke:#9f7aea,color:#fff
    style PROJ1 fill:#3182ce,stroke:#63b3ed,color:#fff
    style PROJ2 fill:#3182ce,stroke:#63b3ed,color:#fff
    style PROJ3 fill:#3182ce,stroke:#63b3ed,color:#fff
    style COMBINED fill:#38a169,stroke:#68d391,color:#fff
```

### Profile Components

Each profile contains:

- **Skills**: Reusable coding patterns and standards (SKILL.md files)
- **Instructions**: Custom CLAUDE.md content with rules, code standards, and auto-invoke table
- **Auto-invoke Rules**: Define when each skill should be automatically loaded

### User Profile (Main-Profile)

| Aspect           | Description                           |
| ---------------- | ------------------------------------- |
| **Scope**        | Applies to ALL projects               |
| **Location**     | `~/.claude/`                          |
| **Purpose**      | Agency-wide standards, security rules |
| **Instructions** | Shared CLAUDE.md (editable via GUI)   |
| **Installation** | Auto-prompted on first sync           |

### Project Profiles

| Aspect           | Description                                               |
| ---------------- | --------------------------------------------------------- |
| **Scope**        | Applies only to specific project                          |
| **Location**     | `<project>/.claude-plugin/`                               |
| **Purpose**      | Tech-stack specific skills + instructions                 |
| **Instructions** | Generated with auto-invoke table based on assigned skills |
| **Installation** | Manual via `rlai profile install`                         |

### Profile Creation Flow

```mermaid
flowchart LR
    A[Create Profile] --> B[Fill Basic Info]
    B --> C[Assign Skills]
    C --> D[Create]
    D --> E[Instructions Generated<br/>with Auto-invoke Table]
    E --> F[Edit Instructions<br/>in IDE]

    style D fill:#38a169,stroke:#68d391,color:#fff
    style E fill:#805ad5,stroke:#9f7aea,color:#fff
```

When creating a profile with skills, the instructions template is automatically generated with:

- Project context and rules
- Code standards
- **Skills Auto-invoke Table** populated with assigned skills

## Quick Start

### For Team Developers

**1. Install the CLI** — See **[Installation Guide](docs/INSTALLATION.md)** for platform-specific instructions (macOS, Linux, Windows).

**2. Use it:**

```bash
# Run any command (auto-syncs configuration on first run)
rlai profile list

# Install Main-Profile (user-level, applies to all projects)
# (Prompted automatically on first sync)

# Install project-specific profile
cd ~/your-project
rlai profile install react-stack
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

1. Download and install the GUI from the [Releases](../../releases) page
2. Configure GitHub repository in **Project Settings**
3. Create profiles and assign skills
4. Click **Deploy** to publish configuration

## CLI Commands

```bash
# Aliases: rlai (short) or rhinolabs-ai (full)

# Configuration sync (auto-runs on first command of terminal session)
rlai sync                    # Manual sync from GitHub

# Profile management
rlai profile list            # List all profiles
rlai profile show <id>       # Show profile details
rlai profile install <name>  # Install profile (default: Claude Code)
rlai profile install <name> -t amp          # Install for Amp
rlai profile install <name> -t amp -t claude-code  # Multiple targets
rlai profile install <name> -t all          # All targets
rlai profile install <name> -P /path        # Install to specific path
rlai profile update          # Update installed profile
rlai profile update -t amp   # Update for specific target
rlai profile uninstall       # Remove profile from current directory
rlai profile uninstall -t amp  # Uninstall only Amp artifacts

# Plugin management
rlai install                 # Install base plugin
rlai update                  # Update plugin
rlai uninstall               # Remove plugin
rlai status                  # Show installation status
rlai doctor                  # Run diagnostics

# MCP configuration
rlai sync-mcp                # Sync MCP servers from source

# RAG (Project Memory)
rlai rag init --project <id> --api-key <key>  # Initialize RAG
rlai rag status              # Show RAG status
rlai rag create-key --name "Team"  # Create API key (admin)
rlai rag list-keys           # List API keys (admin)
rlai rag remove              # Remove RAG from project
```

## RAG (Project Memory)

RAG provides per-project memory capabilities, allowing Claude Code to save and retrieve architectural decisions, context, and knowledge.

```mermaid
flowchart TB
    subgraph "Cloudflare Account"
        MCP["MCP Worker<br/>rhinolabs-rag-mcp"]
        R2["R2 Bucket<br/>rhinolabs-rag"]
        KV["KV Store<br/>API Keys"]
        AI["AutoRAG<br/>Vector Search"]

        MCP --> R2
        MCP --> KV
        MCP --> AI
        R2 --> AI
    end

    subgraph "Developer Machine"
        CC["Claude Code"]
        CLI["rhinolabs-ai CLI"]
        CONFIG[".claude/rag.json"]
    end

    CC <-->|"MCP Protocol"| MCP
    CLI -->|"Init/Status"| CONFIG
    CONFIG -.->|"projectId + apiKey"| CC

    style MCP fill:#f6993f,stroke:#de751f,color:#fff
    style CC fill:#805ad5,stroke:#9f7aea,color:#fff
    style AI fill:#38a169,stroke:#68d391,color:#fff
```

### RAG Architecture Flow

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant CC as Claude Code
    participant MCP as MCP Worker
    participant R2 as R2 Storage
    participant AI as AutoRAG

    Note over Dev,AI: Saving a Decision
    Dev->>CC: "Save: We'll use JWT for auth"
    CC->>MCP: rag_save(project_id, content)
    MCP->>R2: Store document
    R2->>AI: Auto-index
    MCP-->>CC: {success: true}
    CC-->>Dev: "Decision saved"

    Note over Dev,AI: Searching Decisions
    Dev->>CC: "What did we decide about auth?"
    CC->>MCP: rag_ai_search(project_id, query)
    MCP->>AI: Vector search + AI response
    AI-->>MCP: Results + generated answer
    MCP-->>CC: {response, sources}
    CC-->>Dev: "You decided to use JWT..."
```

### RAG Setup

```bash
# 1. Admin creates API key (one time)
rlai rag set-admin-key <admin-secret>
rlai rag create-key --name "Backend Team"
# → API Key: rl_abc123...

# 2. Initialize RAG in project
cd my-project
rlai rag init --project my-project --api-key rl_abc123...

# 3. Claude Code automatically uses RAG tools
# - Ask Claude to save decisions
# - Ask Claude about previous decisions
```

### MCP Tools

| Tool | Description |
|------|-------------|
| `rag_save` | Save document to project RAG |
| `rag_search` | Vector similarity search |
| `rag_ai_search` | AI-powered search with generated answer |
| `rag_list_documents` | List all documents |
| `rag_delete_document` | Delete a document |
| `rag_project_info` | Get project statistics |

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

    WEB -.->|rlai profile install| P1
    API -.->|rlai profile install| P2
    SHARED -.->|rlai profile install| P3

    style P1 fill:#3182ce,stroke:#63b3ed,color:#fff
    style P2 fill:#dd6b20,stroke:#ed8936,color:#fff
    style P3 fill:#805ad5,stroke:#9f7aea,color:#fff
```

```bash
cd ~/monorepo

# Install different profiles for each subproject
rlai profile install react-stack -P ./apps/web
rlai profile install rust-backend -P ./apps/api
rlai profile install ts-lib -P ./packages/shared

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

| Component        | Path                                                                  |
| ---------------- | --------------------------------------------------------------------- |
| CLI Config       | `~/.config/rhinolabs-ai/`                                             |
| User Skills      | `~/.claude/skills/`                                                   |
| Project Skills   | `<project>/.claude/skills/`                                           |
| Plugin (macOS)   | `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude/` |
| Plugin (Linux)   | `~/.config/claude-code/plugins/rhinolabs-claude/`                     |
| Plugin (Windows) | `%APPDATA%\Claude Code\plugins\rhinolabs-claude\`                     |

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
# Using Makefile (recommended)
make test        # Run ALL tests (Rust + E2E)
make test-rust   # Run Rust tests only
make test-e2e    # Run E2E tests only
make test-quick  # Quick tests (same as test-rust)

# Or manually
cargo test --workspace          # Rust unit tests
cd gui/tests && pnpm test       # GUI E2E tests
```

### Available Make Commands

```bash
make help        # Show all available commands
make setup-hooks # Configure git hooks (run after clone)
make install     # Install all dependencies
make build       # Build all components
make run         # Run Tauri app in dev mode
make clean       # Clean build artifacts
```

## Documentation

- [Installation Guide](docs/INSTALLATION.md) - CLI download and setup (macOS, Linux, Windows)
- [Architecture](ARCHITECTURE.md) - System design and data flow
- [CLI Guide](cli/README.md) - Detailed CLI documentation
- [GUI Guide](gui/README.md) - Desktop app documentation
- [Plugin Structure](rhinolabs-claude/README.md) - Skills and plugin details

## Support

- Issues: [GitHub Issues](../../issues)
- Internal: Contact DevOps team

## License

Proprietary - Rhinolabs Internal Use Only

---

**Version**: 0.1.0
**Last Updated**: 2026-02-13
