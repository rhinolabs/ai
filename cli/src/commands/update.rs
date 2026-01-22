use crate::ui::Ui;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rhinolabs_core::{Updater, Version};

pub async fn run(dry_run: bool) -> Result<()> {
    Ui::header("🔄 Updating Rhinolabs Claude Plugin");

    // Check for updates
    Ui::step("Checking for updates...");

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
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
