# Rhinolabs AI GUI

Desktop application for managing the Rhinolabs AI plugin and team configuration.

## Overview

```mermaid
graph TB
    subgraph "GUI Application"
        subgraph "Frontend (React)"
            PAGES[Pages]
            COMPONENTS[Components]
            API[API Layer]
        end

        subgraph "Backend (Tauri/Rust)"
            COMMANDS[Tauri Commands]
            CORE[rhinolabs-core]
        end
    end

    PAGES --> API
    COMPONENTS --> API
    API -->|invoke| COMMANDS
    COMMANDS --> CORE

    style PAGES fill:#3182ce,stroke:#63b3ed,color:#fff
    style COMMANDS fill:#dd6b20,stroke:#ed8936,color:#fff
    style CORE fill:#4a5568,stroke:#718096,color:#fff
```

The GUI is designed for **lead developers** who need to:

- Manage skills and profiles (with integrated instructions)
- Configure MCP servers
- **Deploy configuration to GitHub** (GUI-only feature)
- Manage plugin settings

Team developers use the CLI for read-only operations (sync, profile install).

## Installation

### From Releases

Download the latest release for your platform:

- **macOS**: `rhinolabs-ai_x.x.x_x64.dmg`
- **Windows**: `rhinolabs-ai_x.x.x_x64-setup.exe`
- **Linux**: `rhinolabs-ai_x.x.x_amd64.AppImage`

### From Source

```bash
cd gui
pnpm install
pnpm tauri build
```

## Application Structure

```mermaid
graph LR
    subgraph "Navigation"
        DASH[Dashboard]
        SKILLS[Skills]
        PROFILES[Profiles]
        MCP[MCP]
        SETTINGS[Settings]
        PROJECT[Project]
    end

    DASH --> SKILLS
    SKILLS --> PROFILES
    PROFILES --> MCP
    MCP --> SETTINGS
    SETTINGS --> PROJECT

    style DASH fill:#805ad5,stroke:#9f7aea,color:#fff
    style PROJECT fill:#e53e3e,stroke:#fc8181,color:#fff
```

## Features

### Dashboard

Overview of plugin status and quick actions.

```mermaid
graph TB
    subgraph "Dashboard"
        STATUS[Installation Status]
        VERSION[Version Info]
        ACTIONS[Quick Actions]
    end

    STATUS --> INSTALLED{Installed?}
    INSTALLED -->|Yes| SHOW_VERSION[Show Version]
    INSTALLED -->|No| INSTALL_BTN[Install Button]
```

### Skills Management

```mermaid
flowchart TB
    subgraph "Skills Page"
        LIST[Skills List]
        FILTER[Category Filter]
        TOGGLE[Enable/Disable]
        FETCH[Fetch Remote]
        DETAILS[View Details]
    end

    FILTER --> LIST
    LIST --> TOGGLE
    LIST --> DETAILS
    FETCH --> LIST
```

- Browse built-in and remote skills
- Enable/disable skills
- Fetch skills from remote sources
- View skill details and content

### Profiles Management

Profiles now have integrated instructions management - no separate Instructions page needed.

```mermaid
graph TB
    subgraph "Profiles Page"
        subgraph "List View"
            LIST[Profile List]
            CREATE_BTN[Create Button]
            EDIT_BTN[Edit Button]
        end

        subgraph "Edit/Create Mode"
            HEADER[Header with Save/Close]

            subgraph "Tabs"
                BASIC[Basic Info Tab]
                SKILLS[Skills Tab]
                INSTR[Instructions Tab]
            end
        end
    end

    CREATE_BTN --> HEADER
    EDIT_BTN --> HEADER
    HEADER --> BASIC
    HEADER --> SKILLS
    HEADER --> INSTR

    style HEADER fill:#805ad5,stroke:#9f7aea,color:#fff
    style SKILLS fill:#38a169,stroke:#68d391,color:#fff
    style INSTR fill:#3182ce,stroke:#63b3ed,color:#fff
```

**Create/Edit Flow:**
1. **Basic Info**: ID, name, description
2. **Skills**: Assign skills with category filter (available during creation)
3. **Instructions**: View/edit in IDE (edit mode only)

**Key Features:**
- Skills can be assigned during profile creation
- Instructions template auto-generated with assigned skills in auto-invoke table
- Main Profile instructions = global CLAUDE.md (single source of truth)

### MCP Servers

```mermaid
graph TB
    subgraph "MCP Page"
        subgraph "Add Server Tab"
            MANUAL[Manual Configuration]
            NAME[Server Name]
            CMD[Command]
            ARGS[Arguments]
            ENV[Environment]
        end

        subgraph "Sync from Source Tab"
            URL[Remote URL]
            FILE[Local File]
            SYNC[Sync Button]
        end
    end

    MANUAL --> NAME
    NAME --> CMD
    CMD --> ARGS
    ARGS --> ENV
    URL --> SYNC
    FILE --> SYNC

    style SYNC fill:#38a169,stroke:#68d391,color:#fff
```

### Settings

```mermaid
graph LR
    subgraph "Settings Page"
        AUTO[Auto-Update]
        STYLE[Output Style]
        DIAG[Diagnostics]
    end
```

Note: CLAUDE.md instructions are now managed via Profiles → Main Profile → Instructions tab.

### Project & Deploy

```mermaid
sequenceDiagram
    participant User as Lead Developer
    participant GUI as GUI App
    participant Core as rhinolabs-core
    participant GH as GitHub

    Note over User,GUI: Configuration
    User->>GUI: Set GitHub owner/repo
    GUI->>Core: Save project config

    Note over User,GH: Deploy (GUI-Only)
    User->>GUI: Click Deploy
    GUI->>Core: export_config()
    Core-->>GUI: rhinolabs-config.zip
    GUI->>Core: deploy(version, changelog)
    Core->>GH: Create Release
    Core->>GH: Upload Asset
    GH-->>GUI: Release URL
    GUI->>User: Show success
```

**Settings:**
- Configure GitHub owner/repo
- View current configuration

**Deploy (GUI-Only):**
- Export current configuration
- Deploy to GitHub releases
- Version management

## Tech Stack

```mermaid
graph TB
    subgraph "Frontend"
        REACT[React 18]
        TS[TypeScript]
        VITE[Vite]
        TAILWIND[Tailwind CSS]
    end

    subgraph "Backend"
        TAURI[Tauri 2.0]
        RUST[Rust]
        CORE[rhinolabs-core]
    end

    REACT --> TAURI
    TS --> TAURI
    TAURI --> RUST
    RUST --> CORE

    style REACT fill:#61dafb,stroke:#21a1c4,color:#000
    style TAURI fill:#ffc131,stroke:#d4a017,color:#000
    style RUST fill:#dea584,stroke:#b7410e,color:#000
```

| Layer | Technology |
|-------|------------|
| Frontend | React 18 + TypeScript |
| Build | Vite |
| Backend | Tauri (Rust) |
| Styling | Tailwind CSS |
| Core Logic | rhinolabs-core (shared library) |

## Project Structure

```mermaid
graph TB
    subgraph "gui/"
        subgraph "src/ (React)"
            PAGES_DIR[pages/]
            COMP_DIR[components/]
            API_FILE[api.ts]
            TYPES_FILE[types.ts]
        end

        subgraph "src-tauri/ (Rust)"
            MAIN_RS[main.rs]
            COMMANDS_RS[commands.rs]
            CARGO[Cargo.toml]
        end

        TESTS[tests/]
        PKG[package.json]
    end

    PAGES_DIR --> API_FILE
    API_FILE --> COMMANDS_RS
    COMMANDS_RS --> CORE_LIB[rhinolabs-core]

    style PAGES_DIR fill:#3182ce,stroke:#63b3ed,color:#fff
    style COMMANDS_RS fill:#dd6b20,stroke:#ed8936,color:#fff
```

```
gui/
├── src/                    # React frontend
│   ├── pages/              # Page components
│   │   ├── Dashboard.tsx
│   │   ├── Skills.tsx
│   │   ├── Profiles.tsx
│   │   ├── Mcp.tsx
│   │   ├── Settings.tsx
│   │   └── Project.tsx
│   ├── components/         # Reusable components
│   ├── api.ts              # Tauri command bindings
│   ├── types.ts            # TypeScript types
│   └── App.tsx             # Main app component
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── commands.rs     # Tauri commands
│   │   └── lib.rs          # Library exports
│   ├── Cargo.toml
│   └── tauri.conf.json     # Tauri configuration
├── tests/                  # E2E tests
│   └── e2e/
└── package.json
```

## Tauri Commands

```mermaid
graph TB
    subgraph "Frontend API Calls"
        INVOKE[invoke&lt;T&gt;]
    end

    subgraph "Tauri Commands"
        PROFILES_CMD[Profile Commands]
        SKILLS_CMD[Skill Commands]
        DEPLOY_CMD[Deploy Commands]
        SETTINGS_CMD[Settings Commands]
    end

    subgraph "Core Functions"
        PROFILES_FN[Profiles::]
        SKILLS_FN[Skills::]
        DEPLOY_FN[Deploy::]
        SETTINGS_FN[Settings::]
    end

    INVOKE --> PROFILES_CMD
    INVOKE --> SKILLS_CMD
    INVOKE --> DEPLOY_CMD
    INVOKE --> SETTINGS_CMD

    PROFILES_CMD --> PROFILES_FN
    SKILLS_CMD --> SKILLS_FN
    DEPLOY_CMD --> DEPLOY_FN
    SETTINGS_CMD --> SETTINGS_FN
```

### Profiles

```typescript
// List profiles
const profiles = await invoke<Profile[]>('list_profiles');

// Get profile
const profile = await invoke<Profile>('get_profile', { id: 'react-stack' });

// Create profile (with skills - generates instructions template with auto-invoke table)
await invoke('create_profile', {
  input: {
    id: 'react-stack',
    name: 'React Stack',
    description: 'React 19 with TypeScript and Tailwind',
    skills: ['react-19', 'typescript', 'tailwind-4']  // Skills assigned during creation
  }
});

// Update profile
await invoke('update_profile', { id: 'react-stack', input: { ... } });

// Delete profile
await invoke('delete_profile', { id: 'react-stack' });

// Assign skills (for existing profiles)
await invoke('assign_skills_to_profile', {
  profileId: 'react-stack',
  skillIds: ['react-19', 'typescript']
});

// Get profile skills
const skills = await invoke<Skill[]>('get_profile_skills', { profileId: 'react-stack' });

// Profile Instructions
const content = await invoke<string>('get_profile_instructions', { profileId: 'react-stack' });
await invoke('open_profile_instructions_in_ide', { profileId: 'react-stack', ideCommand: 'code' });
```

### Skills

```typescript
// List skills
const skills = await invoke<Skill[]>('list_skills');

// Get skill
const skill = await invoke<Skill>('get_skill', { id: 'react-19' });

// Toggle skill
await invoke('toggle_skill', { id: 'react-19', enabled: true });

// Fetch remote skills
await invoke('fetch_remote_skills', { source: 'anthropic-official' });
```

### Deploy (GUI-Only)

```typescript
// Deploy to GitHub
const result = await invoke<DeployResult>('deploy_config', {
  version: '1.0.0',
  changelog: 'Release notes'
});

// Export configuration
const result = await invoke<ExportResult>('export_config', {
  outputPath: '/path/to/output'
});
```

## Security Model

```mermaid
graph TB
    subgraph "GUI Exclusive Features"
        CREATE[Create Profiles]
        EDIT[Edit Profiles]
        ASSIGN[Assign Skills]
        DEPLOY[Deploy to GitHub]
        EXPORT[Export Config]
    end

    subgraph "Requirements"
        TOKEN[GITHUB_TOKEN<br/>with repo write access]
        REPO[Configured Repository]
    end

    DEPLOY --> TOKEN
    DEPLOY --> REPO
    EXPORT --> TOKEN

    style DEPLOY fill:#e53e3e,stroke:#fc8181,color:#fff
    style TOKEN fill:#805ad5,stroke:#9f7aea,color:#fff
```

- Only lead developers with the GUI can publish configuration changes
- Team developers use CLI for read-only sync
- Requires `GITHUB_TOKEN` environment variable with repo write access

## Development

### Prerequisites

- Node.js 18+
- pnpm
- Rust 1.70+

### Setup

```bash
cd gui
pnpm install
```

### Development Mode

```bash
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

### Testing

```bash
# E2E tests
cd tests
pnpm install
pnpm test
```

## Troubleshooting

### App Won't Start

```bash
# Check Tauri logs
cd gui
pnpm tauri dev

# Check for missing dependencies
cargo check -p rhinolabs-gui
```

### Deploy Failed

1. Verify `GITHUB_TOKEN` is set
2. Check repository permissions
3. Verify GitHub owner/repo in Project settings

### Skills Not Loading

```bash
# Check plugin installation
rhinolabs-ai status

# Verify skills directory
ls ~/.config/claude-code/plugins/rhinolabs-claude/skills/
```

---

**Version**: 1.1.0
**Last Updated**: 2026-01-29
