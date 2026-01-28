# Rhinolabs Core

Shared Rust library providing core functionality for the Rhinolabs AI ecosystem.

## Overview

```mermaid
graph TB
    subgraph "Consumers"
        CLI[CLI Tool]
        GUI[GUI App]
    end

    subgraph "rhinolabs-core"
        PROFILES[Profiles]
        SKILLS[Skills]
        DEPLOY[Deploy]
        INSTALLER[Installer]
        PATHS[Paths]
        SETTINGS[Settings]
        MCP[MCP]
    end

    subgraph "Storage"
        FS[File System]
        GH[GitHub API]
    end

    CLI --> PROFILES
    CLI --> SKILLS
    CLI --> DEPLOY
    GUI --> PROFILES
    GUI --> SKILLS
    GUI --> DEPLOY
    GUI --> INSTALLER

    PROFILES --> FS
    SKILLS --> FS
    DEPLOY --> GH
    INSTALLER --> FS

    style CLI fill:#3182ce,stroke:#63b3ed,color:#fff
    style GUI fill:#805ad5,stroke:#9f7aea,color:#fff
```

`rhinolabs-core` is used by both the CLI and GUI to ensure consistent behavior across all interfaces.

## Module Architecture

```mermaid
graph TB
    subgraph "Public API"
        LIB[lib.rs<br/>Public Exports]
    end

    subgraph "Data Management"
        PROFILES[profiles.rs<br/>Profile CRUD]
        SKILLS[skills.rs<br/>Skill Management]
        SETTINGS[settings.rs<br/>Plugin Settings]
        INSTRUCTIONS[instructions.rs<br/>CLAUDE.md]
        OUTPUT[output_styles.rs<br/>Response Formats]
    end

    subgraph "Operations"
        DEPLOY[deploy.rs<br/>Export/Deploy/Sync]
        INSTALLER[installer.rs<br/>Plugin Install]
        MCP[mcp.rs<br/>MCP Config]
    end

    subgraph "Utilities"
        PATHS[paths.rs<br/>Path Resolution]
        PROJECT[project.rs<br/>GitHub Config]
    end

    LIB --> PROFILES
    LIB --> SKILLS
    LIB --> DEPLOY
    LIB --> INSTALLER
    PROFILES --> PATHS
    SKILLS --> PATHS
    DEPLOY --> PROJECT
    DEPLOY --> PATHS

    style LIB fill:#805ad5,stroke:#9f7aea,color:#fff
    style DEPLOY fill:#e53e3e,stroke:#fc8181,color:#fff
```

## Modules

### profiles.rs

Profile management, instructions, and installation.

```mermaid
flowchart LR
    subgraph "Profile Operations"
        LIST[list] --> GET[get]
        GET --> CREATE[create]
        CREATE --> UPDATE[update]
        UPDATE --> DELETE[delete]
    end

    subgraph "Instructions"
        GET_INSTR[get_instructions]
        UPDATE_INSTR[update_instructions]
        OPEN_IDE[open_in_ide]
    end

    subgraph "Installation"
        INSTALL[install]
        UPDATE_INST[update_installed]
        UNINSTALL[uninstall]
    end

    DELETE --> GET_INSTR
    GET_INSTR --> UPDATE_INSTR
    UPDATE_INSTR --> OPEN_IDE
    OPEN_IDE --> INSTALL
    INSTALL --> UPDATE_INST
    UPDATE_INST --> UNINSTALL
```

```rust
use rhinolabs_core::{Profiles, ProfileType, CreateProfileInput};

// List all profiles
let profiles = Profiles::list()?;

// Get specific profile
let profile = Profiles::get("react-stack")?;

// Create profile with skills (generates instructions template with auto-invoke table)
let profile = Profiles::create(CreateProfileInput {
    id: "my-profile".to_string(),
    name: "My Profile".to_string(),
    description: "Custom profile".to_string(),
    profile_type: ProfileType::Project,
    skills: vec!["react-19".to_string(), "typescript".to_string()],
    ..Default::default()
})?;

// Profile Instructions
let content = Profiles::get_instructions("react-stack")?;
Profiles::update_instructions("react-stack", "# New Instructions\n...")?;
let path = Profiles::get_instructions_path("react-stack")?;

// Install profile to path
let result = Profiles::install("react-stack", Some(Path::new("./project")))?;

// Update installed profile
let result = Profiles::update_installed("react-stack", Some(Path::new("./project")))?;

// Uninstall profile
Profiles::uninstall(Path::new("./project"))?;
```

**Instructions Generation:**
When creating a profile with skills, instructions are auto-generated with:
- Project context and rules
- Code standards template
- Auto-invoke table populated with assigned skills

### skills.rs

Skill management and retrieval.

```mermaid
graph TB
    subgraph "Skill Sources"
        BUILTIN[Built-in Skills]
        REMOTE[Remote Skills]
        CUSTOM[Custom Skills]
    end

    subgraph "Operations"
        LIST[list]
        GET[get]
        FETCH[fetch_remote]
        BY_PROFILE[list_by_profile]
    end

    BUILTIN --> LIST
    REMOTE --> LIST
    CUSTOM --> LIST
    LIST --> GET
    GET --> BY_PROFILE
    FETCH --> REMOTE
```

```rust
use rhinolabs_core::Skills;

// List all skills
let skills = Skills::list()?;

// Get skill by ID
let skill = Skills::get("react-19")?;

// List skills by profile
let skills = Skills::list_by_profile("react-stack")?;

// Fetch remote skills
Skills::fetch_remote("anthropic-official")?;
```

### deploy.rs

Configuration export, deploy, and sync.

```mermaid
sequenceDiagram
    participant App as CLI/GUI
    participant Deploy as Deploy Module
    participant FS as File System
    participant GH as GitHub API

    Note over App,GH: Export
    App->>Deploy: export_config(path)
    Deploy->>FS: Read profiles, skills, settings
    Deploy->>FS: Create rhinolabs-config.zip
    Deploy-->>App: (zip_path, manifest)

    Note over App,GH: Deploy (GUI Only)
    App->>Deploy: deploy(version, changelog)
    Deploy->>Deploy: export_config()
    Deploy->>GH: Create Release
    Deploy->>GH: Upload Asset
    GH-->>Deploy: release_url, asset_url
    Deploy-->>App: DeployResult

    Note over App,GH: Sync (CLI)
    App->>Deploy: sync()
    Deploy->>GH: Get Latest Release
    Deploy->>GH: Download Asset
    Deploy->>FS: Extract to config dir
    Deploy-->>App: SyncResult
```

```rust
use rhinolabs_core::Deploy;

// Export configuration to zip
let (zip_path, manifest) = Deploy::export_config(Path::new("./output"))?;

// Deploy to GitHub (requires GITHUB_TOKEN)
let result = Deploy::deploy("1.0.0", "Release notes").await?;

// Sync from GitHub
let result = Deploy::sync().await?;
```

### paths.rs

Cross-platform path resolution.

```mermaid
graph TB
    subgraph "Path Functions"
        PLUGINS[plugins_dir]
        CLAUDE_USER[claude_user_dir]
        RHINOLABS[rhinolabs_config_dir]
        CHECK[is_claude_code_installed]
    end

    subgraph "Platforms"
        MAC[macOS<br/>~/Library/Application Support/]
        LINUX[Linux<br/>~/.config/]
        WIN[Windows<br/>%APPDATA%/]
    end

    PLUGINS --> MAC
    PLUGINS --> LINUX
    PLUGINS --> WIN
    CLAUDE_USER --> MAC
    CLAUDE_USER --> LINUX
    CLAUDE_USER --> WIN
```

```rust
use rhinolabs_core::Paths;

// Plugin installation path
let plugin_path = Paths::plugins_dir()?;

// User Claude directory
let claude_dir = Paths::claude_user_dir()?;

// Rhinolabs config directory
let config_dir = Paths::rhinolabs_config_dir()?;

// Check if Claude Code is installed
if Paths::is_claude_code_installed() {
    // ...
}
```

### installer.rs

Plugin installation and updates.

```mermaid
flowchart TB
    subgraph "Installation Methods"
        GITHUB[From GitHub Releases]
        LOCAL[From Local Directory]
    end

    subgraph "Operations"
        INSTALL[install]
        UPDATE[update]
        UNINSTALL[uninstall]
    end

    GITHUB --> INSTALL
    LOCAL --> INSTALL
    INSTALL --> UPDATE
    UPDATE --> UNINSTALL

    style GITHUB fill:#38a169,stroke:#68d391,color:#fff
    style LOCAL fill:#3182ce,stroke:#63b3ed,color:#fff
```

```rust
use rhinolabs_core::Installer;

let installer = Installer::new()
    .dry_run(false);

// Install from GitHub releases
installer.install().await?;

// Install from local directory
installer.install_from_local(Path::new("./rhinolabs-claude"))?;

// Update existing installation
installer.update().await?;

// Uninstall
installer.uninstall()?;
```

### settings.rs / instructions.rs / output_styles.rs

Configuration management.

```mermaid
graph LR
    subgraph "Settings"
        GET_SET[get/set]
        AUTO_UPDATE[autoUpdate]
        OUTPUT_STYLE[outputStyle]
    end

    subgraph "Instructions"
        GET_INST[get]
        SET_INST[set]
        CLAUDE_MD[CLAUDE.md]
    end

    subgraph "Output Styles"
        LIST_STYLES[list]
        GET_ACTIVE[get_active]
        SET_ACTIVE[set_active]
    end
```

```rust
use rhinolabs_core::{Settings, Instructions, OutputStyles};

// Settings
let settings = Settings::get()?;
Settings::set("autoUpdate", serde_json::json!(true))?;

// Instructions
let content = Instructions::get()?;
Instructions::set("# My Instructions\n...")?;

// Output Styles
let styles = OutputStyles::list()?;
let active = OutputStyles::get_active()?;
OutputStyles::set_active("concise")?;
```

### mcp.rs

MCP server configuration.

```mermaid
graph TB
    subgraph "MCP Operations"
        LIST[list]
        ADD[add]
        REMOVE[remove]
        SYNC_URL[sync_from_url]
        SYNC_FILE[sync_from_file]
    end

    subgraph "Storage"
        MCP_JSON[.mcp.json]
    end

    LIST --> MCP_JSON
    ADD --> MCP_JSON
    REMOVE --> MCP_JSON
    SYNC_URL --> MCP_JSON
    SYNC_FILE --> MCP_JSON
```

```rust
use rhinolabs_core::Mcp;

// List configured servers
let servers = Mcp::list()?;

// Add server
Mcp::add(McpServer {
    name: "my-server".to_string(),
    command: "node".to_string(),
    args: vec!["server.js".to_string()],
    env: HashMap::new(),
})?;

// Remove server
Mcp::remove("my-server")?;

// Sync from source
Mcp::sync_from_url("https://config.example.com/mcp.json").await?;
```

## Data Types

### Profile

```mermaid
classDiagram
    class Profile {
        +String id
        +String name
        +String description
        +ProfileType profile_type
        +Vec~String~ skills
        +Vec~AutoInvokeRule~ auto_invoke_rules
        +Option~String~ instructions
        +bool generate_copilot
        +bool generate_agents
        +String created_at
        +String updated_at
    }

    class ProfileType {
        <<enumeration>>
        User
        Project
    }

    class AutoInvokeRule {
        +String skill_id
        +String trigger
        +String description
    }

    Profile --> ProfileType
    Profile --> AutoInvokeRule
```

```rust
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub profile_type: ProfileType,
    pub skills: Vec<String>,
    pub auto_invoke_rules: Vec<AutoInvokeRule>,
    pub instructions: Option<String>,
    pub generate_copilot: bool,
    pub generate_agents: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub enum ProfileType {
    User,      // Installs to ~/.claude/
    Project,   // Installs to project/.claude-plugin/
}

pub struct AutoInvokeRule {
    pub skill_id: String,
    pub trigger: String,      // "Editing .tsx/.jsx files"
    pub description: String,  // "React 19 patterns and hooks"
}
```

### Skill

```mermaid
classDiagram
    class Skill {
        +String id
        +String name
        +String description
        +String version
        +String category
        +SkillSource source
        +bool enabled
    }

    class SkillSource {
        <<enumeration>>
        Builtin
        Remote(String)
        Custom
    }

    Skill --> SkillSource
```

```rust
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: String,
    pub source: SkillSource,
    pub enabled: bool,
}

pub enum SkillSource {
    Builtin,
    Remote(String),
    Custom,
}
```

### Results

```mermaid
classDiagram
    class ProfileInstallResult {
        +String target_path
        +Vec~String~ skills_installed
        +Vec~SkillError~ skills_failed
        +Option~bool~ instructions_installed
        +Option~bool~ settings_installed
        +Option~String~ output_style_installed
    }

    class SyncResult {
        +String version
        +usize profiles_installed
        +usize skills_installed
        +bool instructions_installed
        +bool settings_installed
        +usize output_styles_installed
    }

    class DeployResult {
        +String version
        +String release_url
        +String asset_url
        +ConfigManifest manifest
    }
```

## Configuration Files

```mermaid
graph TB
    subgraph "~/.config/rhinolabs-ai/"
        PROFILES_JSON[profiles.json<br/>Profile definitions]
        PROJECT_JSON[.project.json<br/>GitHub settings]
    end

    subgraph "Plugin Directory"
        SKILLS_CONFIG[.skills-config.json<br/>Skill states]
        SETTINGS_JSON[settings.json<br/>Plugin settings]
        MCP_JSON[.mcp.json<br/>MCP configuration]
    end
```

| File | Location | Purpose |
|------|----------|---------|
| `profiles.json` | `~/.config/rhinolabs-ai/` | Profile definitions |
| `.project.json` | Plugin directory | GitHub settings |
| `.skills-config.json` | Plugin directory | Skill states |
| `settings.json` | Plugin directory | Plugin settings |
| `.mcp.json` | Plugin directory | MCP configuration |

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Usage in Other Crates

```toml
# Cargo.toml
[dependencies]
rhinolabs-core = { path = "../core" }
```

```rust
use rhinolabs_core::{Profiles, Skills, Deploy};
```

---

**Version**: 1.1.0
**Last Updated**: 2026-01-29
