mod commands;
mod ui;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "rhinolabs")]
#[command(about = "Rhinolabs AI Plugin Manager for Claude Code", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install the Rhinolabs Claude plugin
    Install {
        /// Install from local directory (for development)
        #[arg(short, long)]
        local: Option<String>,

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

    /// Sync configuration from GitHub (pull latest deployed config)
    Sync,
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
        #[arg(short, long)]
        profile: String,

        /// Target project path (required for Project profiles)
        #[arg(short = 'P', long)]
        path: Option<String>,
    },

    /// Update an installed profile with latest skill versions
    Update {
        /// Profile ID to update
        #[arg(short, long)]
        profile: String,

        /// Target project path (required for Project profiles)
        #[arg(short = 'P', long)]
        path: Option<String>,
    },

    /// Uninstall profile from a project (removes .claude directory)
    Uninstall {
        /// Target project path to uninstall from
        #[arg(short = 'P', long)]
        path: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        Some(Commands::Install { local, dry_run }) => {
            install::run(local, dry_run).await?;
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
        Some(Commands::Profile { action }) => {
            match action {
                ProfileAction::List => {
                    profile::list()?;
                }
                ProfileAction::Show { profile_id } => {
                    profile::show(&profile_id)?;
                }
                ProfileAction::Install { profile, path } => {
                    profile::install(&profile, path)?;
                }
                ProfileAction::Update { profile, path } => {
                    profile::update(&profile, path)?;
                }
                ProfileAction::Uninstall { path } => {
                    profile::uninstall(&path)?;
                }
            }
        }
        Some(Commands::Sync) => {
            // Manual sync - always runs regardless of session marker
            deploy::sync().await?;
        }
        None => {
            // Interactive mode
            interactive::run().await?;
        }
    }

    Ok(())
}
