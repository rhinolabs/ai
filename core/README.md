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
        RAG[RAG]
        TARGETS[Targets]
    end

    subgraph "Storage"
        FS[File System]
        GH[GitHub API]
    end

    CLI --> PROFILES
    CLI --> SKILLS
    CLI --> DEPLOY
    CLI --> RAG
    CLI --> TARGETS
    GUI --> PROFILES
    GUI --> SKILLS
    GUI --> DEPLOY
    GUI --> INSTALLER
    GUI --> TARGETS

    PROFILES --> FS
    SKILLS --> FS
    DEPLOY --> GH
    INSTALLER --> FS
    TARGETS --> FS
    RAG --> FS

    style CLI fill:#3182ce,stroke:#63b3ed,color:#fff
    style GUI fill:#805ad5,stroke:#9f7aea,color:#fff
    style RAG fill:#38a169,stroke:#68d391,color:#fff
    style TARGETS fill:#dd6b20,stroke:#ed8936,color:#fff
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
        RAG[rag.rs<br/>RAG Config]
    end

    subgraph "Multi-Target"
        TARGETS[targets/<br/>Deploy Abstraction]
    end

    subgraph "Utilities"
        PATHS[paths.rs<br/>Path Resolution]
        PROJECT[project.rs<br/>GitHub Config]
    end

    LIB --> PROFILES
    LIB --> SKILLS
    LIB --> DEPLOY
    LIB --> INSTALLER
    LIB --> RAG
    PROFILES --> PATHS
    SKILLS --> PATHS
    DEPLOY --> PROJECT
    DEPLOY --> PATHS
    RAG --> PATHS
    TARGETS --> PATHS

    style LIB fill:#805ad5,stroke:#9f7aea,color:#fff
    style TARGETS fill:#dd6b20,stroke:#ed8936,color:#fff
    style DEPLOY fill:#e53e3e,stroke:#fc8181,color:#fff
    style RAG fill:#38a169,stroke:#68d391,color:#fff
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

// Install profile to path (defaults to ClaudeCode target)
let result = Profiles::install("react-stack", Some(Path::new("./project")), None)?;

// Install to specific targets
let targets = vec![DeployTarget::Amp, DeployTarget::ClaudeCode];
let result = Profiles::install("react-stack", Some(Path::new("./project")), Some(&targets))?;

// Update installed profile
let result = Profiles::update_installed("react-stack", Some(Path::new("./project")), None)?;

// Uninstall profile (None = remove all targets)
Profiles::uninstall(Path::new("./project"), None)?;

// Uninstall specific targets only
Profiles::uninstall(Path::new("./project"), Some(&[DeployTarget::Amp]))?;
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
use rhinolabs_core::{Skills, SkillCategory, CreateSkillInput, UpdateSkillInput};

// List all skills
let skills = Skills::list()?;

// Get skill by ID
let skill = Skills::get("react-19")?;

// List skills by profile
let skills = Skills::list_by_profile("react-stack")?;

// Fetch remote skills
Skills::fetch_remote("anthropic-official")?;

// Create a new skill with category
let skill = Skills::create(CreateSkillInput {
    id: "my-skill".to_string(),
    name: "My Skill".to_string(),
    description: "Custom skill".to_string(),
    category: SkillCategory::Frontend,
    content: "# Instructions".to_string(),
})?;

// Update skill (including category)
Skills::update("my-skill", UpdateSkillInput {
    category: Some(SkillCategory::Testing),
    ..Default::default()
})?;

// Change skill category directly
Skills::set_category("my-skill", SkillCategory::Utilities)?;

// Get skill category
let category = Skills::get_skill_category("my-skill")?;
```

**Dynamic Categories:**
- User-defined categories are stored in `.skills-config.json` under `categoryMap`
- Categories in `categoryMap` take precedence over hardcoded built-in categories
- Available categories: `Corporate`, `Frontend`, `Testing`, `AiSdk`, `Utilities`, `Custom`

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

### rag.rs

RAG (Retrieval-Augmented Generation) local configuration management. All actual RAG operations are performed by the centralized MCP Worker.

```mermaid
flowchart TB
    subgraph "Local Configuration"
        RAG_JSON[.claude/rag.json]
        SETTINGS[~/.config/rhinolabs-ai/rag-settings.json]
    end

    subgraph "Rag Module Operations"
        INIT[init]
        LOAD[load_config]
        REMOVE[remove]
        IS_CONFIGURED[is_configured]
        GET_MCP_URL[get_mcp_url]
    end

    subgraph "Admin Settings"
        SET_ADMIN[set_admin_key]
        GET_ADMIN[get_admin_key]
        LOAD_SETTINGS[load_settings]
        SAVE_SETTINGS[save_settings]
    end

    INIT --> RAG_JSON
    LOAD --> RAG_JSON
    REMOVE --> RAG_JSON
    IS_CONFIGURED --> RAG_JSON

    SET_ADMIN --> SETTINGS
    GET_ADMIN --> SETTINGS
    LOAD_SETTINGS --> SETTINGS
    SAVE_SETTINGS --> SETTINGS

    style RAG_JSON fill:#38a169,stroke:#68d391,color:#fff
    style SETTINGS fill:#805ad5,stroke:#9f7aea,color:#fff
```

```mermaid
sequenceDiagram
    participant CLI as CLI/App
    participant Rag as Rag Module
    participant FS as File System
    participant MCP as MCP Worker

    Note over CLI,MCP: Project Initialization
    CLI->>Rag: init(project_id, api_key)
    Rag->>FS: Create .claude/rag.json
    FS-->>Rag: OK
    Rag-->>CLI: RagConfig

    Note over CLI,MCP: Runtime (Claude Code)
    CLI->>Rag: load_config()
    Rag->>FS: Read .claude/rag.json
    FS-->>Rag: RagConfig
    Rag-->>CLI: {project_id, api_key, mcp_url}
    CLI->>MCP: Use MCP tools with config
```

```rust
use rhinolabs_core::{Rag, RagConfig, RagSettings};

// Initialize RAG for a project (creates .claude/rag.json)
let config = Rag::init(Path::new("./my-project"), "project-id", "rl_api_key")?;

// Load existing configuration
let config = Rag::load_config(Path::new("./my-project"))?;

// Check if RAG is configured
if Rag::is_configured(Path::new("./my-project"))? {
    // ...
}

// Get MCP URL (returns custom or default)
let url = Rag::get_mcp_url(&config);

// Remove RAG configuration
Rag::remove(Path::new("./my-project"))?;

// Admin key management (for creating/listing API keys)
Rag::set_admin_key("admin_secret")?;
let admin_key = Rag::get_admin_key()?;

// Settings persistence
let settings = Rag::load_settings()?;
Rag::save_settings(&RagSettings {
    default_mcp_url: Some("https://custom.workers.dev".to_string()),
    admin_key: Some("my_admin_key".to_string()),
})?;
```

**Key Concepts:**

- **RagConfig**: Per-project configuration stored in `.claude/rag.json`
  - `project_id`: Unique identifier for the project
  - `api_key`: API key for authenticating with MCP Worker
  - `mcp_url`: Optional custom MCP Worker URL (uses default if not set)

- **RagSettings**: Global settings stored in `~/.config/rhinolabs-ai/rag-settings.json`
  - `default_mcp_url`: Override default MCP Worker URL
  - `admin_key`: Admin key for key management operations

**Note:** The Rag module only manages local configuration. All actual save/search operations are performed by the MCP Worker and accessed via Claude Code's MCP integration.

### targets/

Multi-target deployment abstraction layer. Provides traits and implementations for deploying skills, instructions, and MCP configuration to different AI coding assistants.

```mermaid
graph TB
    subgraph "DeployTarget Enum"
        CC[ClaudeCode]
        AMP[Amp]
        AG[Antigravity]
        OC[OpenCode]
    end

    subgraph "Deployer Traits"
        SD[SkillDeployer]
        ID[InstructionsDeployer]
        MD[McpDeployer]
        TD[TargetDetector]
    end

    subgraph "Implementations"
        CCD[ClaudeCodeDeployer]
        GD[GenericDeployer]
    end

    CCD --> SD
    CCD --> ID
    CCD --> MD
    CCD --> TD

    GD --> SD
    GD --> ID

    SD --> CC
    SD --> AMP
    SD --> AG
    SD --> OC

    style CCD fill:#3182ce,stroke:#63b3ed,color:#fff
    style GD fill:#2b6cb0,stroke:#63b3ed,color:#fff
    style CC fill:#38a169,stroke:#68d391,color:#fff
    style AMP fill:#805ad5,stroke:#9f7aea,color:#fff
    style AG fill:#dd6b20,stroke:#ed8936,color:#fff
    style OC fill:#e53e3e,stroke:#fc8181,color:#fff
```

**Supported Targets:**

| Target | User Skills | Project Skills | Instructions | MCP Config |
|--------|------------|----------------|-------------|------------|
| Claude Code | `~/.claude/skills/` | `.claude/skills/` | `CLAUDE.md` | `.mcp.json` |
| Amp | `~/.config/agents/skills/` | `.agents/skills/` | `AGENTS.md` | `settings.json` |
| Antigravity | `~/.gemini/antigravity/skills/` | `.agent/skills/` | `GEMINI.md` | `config.json` |
| OpenCode | `~/.config/opencode/skills/` | `.opencode/skills/` | `opencode.json` | `opencode.json` |

```rust
use rhinolabs_core::{
    DeployTarget, TargetPaths, ClaudeCodeDeployer, GenericDeployer,
    SkillDeployer, InstructionsDeployer, McpDeployer, TargetDetector,
};

// Enumerate all targets
for target in DeployTarget::all() {
    println!("{}: installed={}", target, target.is_installed());
}

// Parse target from CLI string
let target: DeployTarget = "amp".parse().unwrap();
let prefix = target.project_skills_prefix(); // ".agents/skills"

// Resolve paths per target
let skills_dir = TargetPaths::user_skills_dir(DeployTarget::ClaudeCode)?;
let project_skills = TargetPaths::project_skills_dir(DeployTarget::Amp, project_path);
let config_dir = TargetPaths::project_config_dir(DeployTarget::Amp, project_path);

// Use ClaudeCodeDeployer (implements all 4 traits including McpDeployer)
let deployer = ClaudeCodeDeployer;
deployer.deploy_skill_project("react-19", &source, &project)?;
deployer.deploy_instructions_project("# Rules\n...", &project)?;

// Use GenericDeployer for any target (implements SkillDeployer + InstructionsDeployer)
let amp_deployer = GenericDeployer::new(DeployTarget::Amp);
amp_deployer.deploy_skill_project("react-19", &source, &project)?;
amp_deployer.deploy_instructions_project("# Rules\n...", &project)?;

// Create deployers for multiple targets at once
let deployers = GenericDeployer::for_targets(&[DeployTarget::Amp, DeployTarget::ClaudeCode]);

// Trait objects for multi-target dispatch
let deployers: Vec<Box<dyn InstructionsDeployer>> = vec![
    Box::new(ClaudeCodeDeployer),
    Box::new(GenericDeployer::new(DeployTarget::Amp)),
    Box::new(GenericDeployer::new(DeployTarget::Antigravity)),
];
```

**Module Structure:**

| File | Purpose |
|------|---------|
| `deploy_target.rs` | `DeployTarget` enum with serde, Display, FromStr, path helpers |
| `target_paths.rs` | `TargetPaths` static path resolver per target |
| `traits.rs` | `SkillDeployer`, `InstructionsDeployer`, `McpDeployer`, `TargetDetector` |
| `claude_code.rs` | `ClaudeCodeDeployer` implementing all 4 traits |
| `generic.rs` | `GenericDeployer` parameterized deployer for any target (SkillDeployer + InstructionsDeployer) |

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
        +Vec~DeployTarget~ targets_installed
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
use rhinolabs_core::{Profiles, Skills, Deploy, DeployTarget, ClaudeCodeDeployer, GenericDeployer};
```

---

**Version**: 1.2.0
**Last Updated**: 2026-02-04
