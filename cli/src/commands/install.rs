use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rhinolabs_core::{DeployTarget, Installer, Paths, Profiles};
use std::path::Path;

/// Parse target strings into DeployTarget vec.
fn parse_targets(strs: &[String]) -> Result<Vec<DeployTarget>> {
    if strs.iter().any(|s| s == "all") {
        return Ok(DeployTarget::all().to_vec());
    }
    strs.iter()
        .map(|s| s.parse::<DeployTarget>().map_err(|e| anyhow::anyhow!(e)))
        .collect()
}

pub async fn run(
    local_path: Option<String>,
    target_strs: Vec<String>,
    skip_profile: bool,
    dry_run: bool,
) -> Result<()> {
    Ui::header("Installing Rhinolabs AI");

    let installer = Installer::new().dry_run(dry_run);

    // Pre-flight checks
    Ui::step("Checking Claude Code installation...");
    if !Paths::is_claude_code_installed() {
        Ui::error("Claude Code not found");
        Ui::info("Please install Claude Code from: https://code.claude.com");
        return Ok(());
    }
    Ui::success("Claude Code detected");

    // Check if already installed
    if Paths::is_plugin_installed() && !dry_run {
        Ui::warning("Plugin already installed");
        Ui::info("Use 'rhinolabs-ai update' to update to latest version");
        Ui::info("Use 'rhinolabs-ai uninstall' first if you want to reinstall");
        return Ok(());
    }

    // Step 1: Install plugin
    Ui::step("Installing plugin...");

    if let Some(local) = local_path {
        Ui::step(&format!("Installing from local: {}", local));
        installer.install_from_local(Path::new(&local))?;
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message("Downloading from GitHub releases...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        installer.install().await?;

        pb.finish_with_message("Downloaded and extracted");
    }

    Ui::success("Plugin installed");

    // Sync profiles from plugin (ensures existing config gets updated skills)
    if !dry_run {
        Profiles::sync_from_plugin()?;
    }

    if dry_run || skip_profile {
        if skip_profile {
            Ui::info("Skipped profile installation (--skip-profile)");
            Ui::info("Run 'rhinolabs-ai profile install main' to install skills later.");
        }
        println!();
        return Ok(());
    }

    // Step 2: Install main profile (skills + config)
    println!();
    Ui::step("Installing main profile skills...");

    let targets = parse_targets(&target_strs)?;
    let targets_ref = if targets.is_empty() {
        None
    } else {
        Some(targets.as_slice())
    };

    let result = Profiles::install("main", None, targets_ref)?;

    if !result.skills_installed.is_empty() {
        Ui::success(&format!(
            "{} skills installed",
            result.skills_installed.len()
        ));
        for skill in &result.skills_installed {
            println!("  {} {}", "✓".green(), skill);
        }
    }

    if !result.skills_failed.is_empty() {
        Ui::warning(&format!("{} skills failed", result.skills_failed.len()));
        for error in &result.skills_failed {
            println!("  {} {} - {}", "✗".red(), error.skill_id, error.error);
        }
    }

    if result.skills_installed.is_empty() && result.skills_failed.is_empty() {
        Ui::warning("No skills found in plugin. The plugin may be incomplete.");
    }

    // Summary
    println!();
    Ui::success("Installation complete!");
    println!();
    Ui::info("Restart Claude Code to activate the plugin.");

    Ok(())
}
