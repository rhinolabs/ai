//! Auto-sync module for automatic configuration synchronization
//!
//! This module handles:
//! - Checking if sync is needed (once per terminal session)
//! - Auto-syncing configuration from GitHub
//! - Auto-installing Main-Profile if not present

use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::{Deploy, Paths, Profiles, ProfileType};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SESSION_MARKER_FILE: &str = "rhinolabs-session-sync";
const SESSION_TIMEOUT_SECS: u64 = 3600; // 1 hour - consider same session

/// Get the session marker file path
fn session_marker_path() -> PathBuf {
    std::env::temp_dir().join(SESSION_MARKER_FILE)
}

/// Check if we need to sync in this session
fn needs_sync() -> bool {
    let marker_path = session_marker_path();

    if !marker_path.exists() {
        return true;
    }

    // Check if marker is from current session (within timeout)
    if let Ok(metadata) = fs::metadata(&marker_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                return elapsed.as_secs() > SESSION_TIMEOUT_SECS;
            }
        }
    }

    false
}

/// Mark that we've synced in this session
fn mark_synced(version: &str) {
    let marker_path = session_marker_path();
    let _ = fs::write(&marker_path, version);
}

/// Check if Main-Profile is installed
fn is_main_profile_installed() -> bool {
    if let Ok(home) = std::env::var("HOME") {
        let claude_dir = PathBuf::from(home).join(".claude");
        let skills_dir = claude_dir.join("skills");

        // Check if .claude/skills exists and has content
        if skills_dir.exists() {
            if let Ok(entries) = fs::read_dir(&skills_dir) {
                return entries.count() > 0;
            }
        }
    }
    false
}

/// Prompt user for yes/no confirmation
fn prompt_yes_no(prompt: &str, default_yes: bool) -> bool {
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", prompt, suffix);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return default_yes;
    }

    let input = input.trim().to_lowercase();
    if input.is_empty() {
        return default_yes;
    }

    matches!(input.as_str(), "y" | "yes" | "si" | "sí")
}

/// Run auto-sync check and sync if needed
/// Returns true if sync was performed, false otherwise
pub async fn run_auto_sync() -> Result<bool> {
    // Check if GitHub is configured
    let project_config = match rhinolabs_core::Project::get_config() {
        Ok(config) => config,
        Err(_) => return Ok(false), // Not configured, skip silently
    };

    if project_config.github.owner.is_empty() || project_config.github.repo.is_empty() {
        return Ok(false); // Not configured, skip silently
    }

    // Check if we need to sync
    if !needs_sync() {
        return Ok(false);
    }

    println!();
    println!("{}", "━━━ Configuration Sync ━━━".cyan().bold());
    println!("Checking for updates...");

    // Try to sync
    match Deploy::sync().await {
        Ok(result) => {
            println!("{} Configuration synced: {}", "✓".green(), result.version.cyan());
            println!();

            if result.profiles_installed > 0 {
                println!("  {} {} profiles", "✓".green(), result.profiles_installed);
            }
            if result.skills_installed > 0 {
                println!("  {} {} skills", "✓".green(), result.skills_installed);
            }
            if result.instructions_installed {
                println!("  {} Instructions", "✓".green());
            }
            if result.settings_installed {
                println!("  {} Settings", "✓".green());
            }
            if result.output_styles_installed > 0 {
                println!("  {} {} output styles", "✓".green(), result.output_styles_installed);
            }

            // Mark as synced
            mark_synced(&result.version);

            // Check if Main-Profile needs to be installed
            check_and_install_main_profile().await?;

            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
            println!();

            Ok(true)
        }
        Err(e) => {
            let error_msg = e.to_string();

            // Don't show error for "no config release" - just means nothing deployed yet
            if error_msg.contains("No configuration release") {
                println!("{} No configuration deployed yet", "○".dimmed());
                mark_synced("none"); // Mark as checked to avoid repeated checks
            } else if error_msg.contains("GitHub repository not configured") {
                // Silently skip - not configured
                mark_synced("unconfigured");
            } else {
                println!("{} Sync failed: {}", "⚠".yellow(), error_msg);
                println!("  {}", "Continuing without sync. Run 'rhinolabs sync' manually later.".dimmed());
            }

            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
            println!();

            Ok(false)
        }
    }
}

/// Check if Main-Profile is installed and offer to install it
async fn check_and_install_main_profile() -> Result<()> {
    // Check if Main-Profile exists in profiles
    let profiles = Profiles::list()?;
    let main_profile = profiles.iter().find(|p| p.id == "main" && p.profile_type == ProfileType::User);

    if main_profile.is_none() {
        // No Main-Profile defined, skip
        return Ok(());
    }

    // Check if already installed
    if is_main_profile_installed() {
        return Ok(());
    }

    println!();
    println!("{}", "━━━ Main-Profile Setup ━━━".cyan().bold());
    println!("Main-Profile is not installed in your user memory (~/.claude/).");
    println!("This profile contains agency-wide standards that apply to all projects.");
    println!();

    if prompt_yes_no("Install Main-Profile now?", true) {
        println!();
        println!("Installing Main-Profile...");

        match Profiles::install("main", None) {
            Ok(result) => {
                println!();
                println!("{} Main-Profile installed to ~/.claude/", "✓".green());

                if !result.skills_installed.is_empty() {
                    println!("  {} {} skills", "✓".green(), result.skills_installed.len());
                }
                if result.instructions_installed == Some(true) {
                    println!("  {} CLAUDE.md", "✓".green());
                }
                if result.settings_installed == Some(true) {
                    println!("  {} settings.json", "✓".green());
                }
                if let Some(style) = &result.output_style_installed {
                    println!("  {} Output style: {}", "✓".green(), style);
                }
            }
            Err(e) => {
                println!("{} Failed to install Main-Profile: {}", "✗".red(), e);
                println!("  {}", "Run 'rhinolabs profile install --profile main' manually.".dimmed());
            }
        }
    } else {
        println!();
        println!("{}", "Skipped. Install later with:".dimmed());
        println!("  rhinolabs profile install --profile main");
    }

    Ok(())
}

/// Force sync (for manual 'rhinolabs sync' command)
/// This always syncs regardless of session marker
pub async fn force_sync() -> Result<()> {
    super::deploy::sync().await
}
