pub mod commands;
pub mod ui;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "rhinolabs-ai")]
#[command(about = "Rhinolabs AI Plugin Manager for Claude Code", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the Rhinolabs Claude plugin (plugin + main profile skills)
    Install {
        /// Deploy targets: claude-code (default), amp, antigravity, open-code, all
        #[arg(short, long)]
        target: Vec<String>,

        /// Skip main profile installation (plugin only, no skills)
        #[arg(long)]
        skip_profile: bool,

        /// Dry run - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Update plugin to latest version
    Update {
        /// Dry run - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Uninstall the plugin
    Uninstall {
        /// Dry run - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Sync MCP configuration
    SyncMcp {
        /// Remote URL to fetch configuration from
        #[arg(short, long)]
        url: Option<String>,

        /// Local file path to import configuration from
        #[arg(short, long)]
        file: Option<String>,

        /// Dry run - show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// Show plugin status and version info
    Status,

    /// Run diagnostic checks
    Doctor,

    /// Show version information
    Version,

    /// Manage skill profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },

    /// Manage skills
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },

    /// Sync configuration from GitHub (pull latest deployed config)
    Sync,

    /// Manage RAG (Retrieval-Augmented Generation) for project memory
    Rag {
        #[command(subcommand)]
        action: RagAction,
    },
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List all profiles
    List,

    /// Show details of a specific profile
    Show {
        /// Profile ID to show
        profile_id: String,
    },

    /// Install a profile to a project
    Install {
        /// Profile ID to install
        profile: String,

        /// Target project path (defaults to current directory)
        #[arg(short = 'P', long)]
        path: Option<String>,

        /// Deploy targets: claude-code (default), amp, antigravity, open-code, all
        #[arg(short, long)]
        target: Vec<String>,
    },

    /// Update an installed profile with latest skill versions
    Update {
        /// Profile ID (optional - detects from installed plugin if not specified)
        profile: Option<String>,

        /// Target project path (defaults to current directory)
        #[arg(short = 'P', long)]
        path: Option<String>,

        /// Deploy targets: claude-code (default), amp, antigravity, open-code, all
        #[arg(short, long)]
        target: Vec<String>,
    },

    /// Uninstall profile from a project (removes .claude directory)
    Uninstall {
        /// Target project path (defaults to current directory)
        #[arg(short = 'P', long)]
        path: Option<String>,

        /// Deploy targets: claude-code (default), amp, antigravity, open-code, all
        #[arg(short, long)]
        target: Vec<String>,
    },
}

#[derive(Subcommand)]
enum SkillAction {
    /// List all skills
    List,

    /// Show details of a specific skill
    Show {
        /// Skill ID to show
        skill_id: String,
    },

    /// Create a new custom skill
    Create {
        /// Unique skill identifier (e.g., "my-skill")
        #[arg(long)]
        id: String,

        /// Display name for the skill
        #[arg(long)]
        name: String,

        /// Skill category: corporate, frontend, testing, ai-sdk, utilities, custom
        #[arg(long, default_value = "custom")]
        category: String,

        /// Optional description
        #[arg(long)]
        description: Option<String>,
    },

    /// Set the category for an existing skill
    SetCategory {
        /// Skill ID to update
        skill_id: String,

        /// New category: corporate, frontend, testing, ai-sdk, utilities, custom
        category: String,
    },
}

#[derive(Subcommand)]
enum RagAction {
    /// Initialize RAG for the current project
    Init {
        /// Project identifier (e.g., "prowler-api")
        #[arg(long)]
        project: String,

        /// API key for the MCP Worker
        #[arg(long)]
        api_key: String,
    },

    /// Show RAG status for the current project
    Status,

    /// Create a new API key (requires admin key)
    CreateKey {
        /// Name for the API key (e.g., "Backend Team")
        #[arg(long)]
        name: String,

        /// Limit key to specific projects (default: all projects)
        #[arg(long)]
        projects: Option<Vec<String>>,
    },

    /// List all API keys (requires admin key)
    ListKeys,

    /// Set admin key for key management
    SetAdminKey {
        /// Admin key for the MCP Worker
        key: String,
    },

    /// Remove RAG configuration from the current project
    Remove,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Determine if auto-sync should run for this command
    let should_auto_sync = matches!(
        &cli.command,
        Some(Commands::Profile { .. })
            | Some(Commands::Status)
            | Some(Commands::Doctor)
            | Some(Commands::SyncMcp { .. })
            | None // Interactive mode
    );

    // Run auto-sync for applicable commands
    if should_auto_sync {
        // Auto-sync runs silently if not needed, shows UI if syncing
        let _ = auto_sync::run_auto_sync().await;
    }

    match cli.command {
        Some(Commands::Install {
            target,
            skip_profile,
            dry_run,
        }) => {
            install::run(target, skip_profile, dry_run).await?;
        }
        Some(Commands::Update { dry_run }) => {
            update::run(dry_run).await?;
        }
        Some(Commands::Uninstall { dry_run }) => {
            uninstall::run(dry_run)?;
        }
        Some(Commands::SyncMcp { url, file, dry_run }) => {
            sync_mcp::run(url, file, dry_run).await?;
        }
        Some(Commands::Status) => {
            status::run()?;
        }
        Some(Commands::Doctor) => {
            doctor::run().await?;
        }
        Some(Commands::Version) => {
            version::run();
        }
        Some(Commands::Profile { action }) => match action {
            ProfileAction::List => {
                profile::list()?;
            }
            ProfileAction::Show { profile_id } => {
                profile::show(&profile_id)?;
            }
            ProfileAction::Install {
                profile,
                path,
                target,
            } => {
                profile::install(&profile, path, target)?;
            }
            ProfileAction::Update {
                profile,
                path,
                target,
            } => {
                profile::update(profile, path, target)?;
            }
            ProfileAction::Uninstall { path, target } => {
                profile::uninstall(path, target)?;
            }
        },
        Some(Commands::Skill { action }) => match action {
            SkillAction::List => {
                skill::list()?;
            }
            SkillAction::Show { skill_id } => {
                skill::show(&skill_id)?;
            }
            SkillAction::Create {
                id,
                name,
                category,
                description,
            } => {
                skill::create(id, name, category, description)?;
            }
            SkillAction::SetCategory { skill_id, category } => {
                skill::set_category(skill_id, category)?;
            }
        },
        Some(Commands::Sync) => {
            // Manual sync - always runs regardless of session marker
            deploy::sync().await?;
        }
        Some(Commands::Rag { action }) => match action {
            RagAction::Init { project, api_key } => {
                rag::init(project, api_key)?;
            }
            RagAction::Status => {
                rag::status()?;
            }
            RagAction::CreateKey { name, projects } => {
                rag::create_key(name, projects).await?;
            }
            RagAction::ListKeys => {
                rag::list_keys().await?;
            }
            RagAction::SetAdminKey { key } => {
                rag::set_admin_key(key)?;
            }
            RagAction::Remove => {
                rag::remove()?;
            }
        },
        None => {
            // Interactive mode
            interactive::run().await?;
        }
    }

    Ok(())
}
