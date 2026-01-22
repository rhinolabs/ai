use crate::ui::Ui;
use anyhow::Result;
use dialoguer::Confirm;
use rhinolabs_core::Installer;

pub fn run(dry_run: bool) -> Result<()> {
    Ui::header("🗑️  Uninstalling Rhinolabs Claude Plugin");

    if !dry_run {
        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to uninstall?")
            .default(false)
            .interact()?;

        if !confirmed {
            Ui::info("Uninstall cancelled");
            return Ok(());
        }
    }

    let installer = Installer::new().dry_run(dry_run);
    installer.uninstall()?;

    println!();
    Ui::success("Plugin uninstalled successfully");
    println!();
    Ui::info("To reinstall, run: rhinolabs install");

    Ok(())
}
