use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rhinolabs_core::{Profiles, Updater, Version};

pub async fn run(dry_run: bool) -> Result<()> {
    Ui::header("🔄 Updating Rhinolabs Claude Plugin");

    // Check for updates
    Ui::step("Checking for updates...");

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    match Version::check_update().await? {
        Some(version) => {
            pb.finish_and_clear();
            Ui::success(format!("Update available: v{}", version).as_str());

            if dry_run {
                Ui::info("[DRY RUN] Would update to latest version");
                return Ok(());
            }

            Ui::step("Downloading latest version...");
            let updater = Updater::new().dry_run(dry_run);
            updater.update().await?;

            // Show synced profiles
            Ui::step("Syncing profile configurations...");
            match Profiles::sync_from_plugin() {
                Ok(synced) if !synced.is_empty() => {
                    for id in &synced {
                        println!("  {} Profile '{}' synced", "✓".green(), id);
                    }
                }
                Ok(_) => {
                    Ui::success("Profiles up to date");
                }
                Err(_) => {
                    Ui::warning("Could not sync profiles (plugin may not include profiles.json)");
                }
            }

            // If CWD has an installed project profile, update it too
            let cwd = std::env::current_dir().unwrap_or_default();
            let plugin_json = cwd.join(".claude-plugin").join("plugin.json");
            if plugin_json.exists() {
                if let Ok(content) = std::fs::read_to_string(&plugin_json) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(profile_id) = json["profile"]["id"].as_str() {
                            Ui::step(&format!("Updating project profile '{}'...", profile_id));
                            match Profiles::install(profile_id, Some(&cwd), None) {
                                Ok(result) => {
                                    println!(
                                        "  {} {} skills updated",
                                        "✓".green(),
                                        result.skills_installed.len()
                                    );
                                }
                                Err(e) => {
                                    Ui::warning(&format!(
                                        "Could not update project profile: {}",
                                        e
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            println!();
            Ui::success("Update complete!");
            println!();
            Ui::info("Next steps:");
            println!("  1. Restart Claude Code");
            println!("  2. Run: rhinolabs status");
        }
        None => {
            pb.finish_and_clear();
            Ui::success("Already on latest version");
        }
    }

    Ok(())
}
