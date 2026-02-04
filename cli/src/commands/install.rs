use crate::ui::Ui;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rhinolabs_core::{Installer, Paths};
use std::path::Path;

pub async fn run(local_path: Option<String>, dry_run: bool) -> Result<()> {
    Ui::header("🚀 Installing Rhinolabs Claude Plugin");

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

    // Install
    if let Some(local) = local_path {
        Ui::step(format!("Installing from local: {}", local).as_str());
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

    println!();
    Ui::success("Installation complete!");
    println!();
    Ui::info("Next steps:");
    println!("  1. Restart Claude Code");
    println!("  2. Run: rhinolabs status");
    println!();
    Ui::info("Documentation: rhinolabs-ai status");

    Ok(())
}
