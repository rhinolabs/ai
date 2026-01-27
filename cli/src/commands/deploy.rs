use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::Deploy;
use std::path::Path;

/// Deploy current configuration to GitHub
pub async fn deploy(version: &str, message: Option<String>) -> Result<()> {
    Ui::header("Deploying Configuration");

    Ui::step(&format!("Version: {}", version));

    let changelog = message.unwrap_or_else(|| format!("Configuration release v{}", version));

    Ui::step("Exporting configuration...");
    Ui::step("Creating GitHub release...");
    Ui::step("Uploading config bundle...");

    match Deploy::deploy(version, &changelog).await {
        Ok(result) => {
            println!();
            Ui::success("Configuration deployed successfully!");
            println!();

            Ui::section("Release Details");
            println!("  Version:    {}", result.version.cyan());
            println!("  Release:    {}", result.release_url);
            println!("  Asset:      {}", result.asset_url);
            println!();

            Ui::section("Configuration Summary");
            println!("  Profiles:      {}", result.manifest.profiles_count);
            println!("  Skills:        {}", result.manifest.skills_count);
            println!("  Instructions:  {}", if result.manifest.has_instructions { "Yes" } else { "No" });
            println!("  Settings:      {}", if result.manifest.has_settings { "Yes" } else { "No" });
            println!("  Output Styles: {}", result.manifest.output_styles_count);
            println!();

            Ui::info("Your team can now sync this configuration with:");
            println!("  {} rhinolabs sync", "$".dimmed());
            println!();
        }
        Err(e) => {
            Ui::error(&format!("Deploy failed: {}", e));
            println!();
            Ui::info("Make sure you have:");
            println!("  1. Configured GitHub repository in Project Settings");
            println!("  2. Set GITHUB_TOKEN environment variable");
            println!();
        }
    }

    Ok(())
}

/// Export configuration to a local zip file (without deploying)
pub fn export(output_path: Option<String>) -> Result<()> {
    Ui::header("Exporting Configuration");

    let output = output_path
        .map(|p| Path::new(&p).to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    Ui::step(&format!("Output directory: {}", output.display()));

    match Deploy::export_config(&output) {
        Ok((zip_path, manifest)) => {
            println!();
            Ui::success("Configuration exported successfully!");
            println!();

            println!("  {} {}", "File:".bold(), zip_path.display());
            println!();

            Ui::section("Contents");
            println!("  Profiles:      {}", manifest.profiles_count);
            println!("  Skills:        {}", manifest.skills_count);
            println!("  Instructions:  {}", if manifest.has_instructions { "Yes" } else { "No" });
            println!("  Settings:      {}", if manifest.has_settings { "Yes" } else { "No" });
            println!("  Output Styles: {}", manifest.output_styles_count);
            println!();

            Ui::info("You can share this file with your team or upload it manually.");
            println!();
        }
        Err(e) => {
            Ui::error(&format!("Export failed: {}", e));
        }
    }

    Ok(())
}

/// Sync configuration from GitHub
pub async fn sync() -> Result<()> {
    Ui::header("Syncing Configuration");

    Ui::step("Fetching latest configuration from GitHub...");

    match Deploy::sync().await {
        Ok(result) => {
            println!();
            Ui::success("Configuration synced successfully!");
            println!();

            println!("  Version: {}", result.version.cyan());
            println!();

            Ui::section("Installed");
            println!("  {} Profiles:      {}", "✓".green(), result.profiles_installed);
            println!("  {} Skills:        {}", "✓".green(), result.skills_installed);
            println!(
                "  {} Instructions:  {}",
                if result.instructions_installed { "✓".green() } else { "○".dimmed() },
                if result.instructions_installed { "Updated" } else { "Skipped" }
            );
            println!(
                "  {} Settings:      {}",
                if result.settings_installed { "✓".green() } else { "○".dimmed() },
                if result.settings_installed { "Updated" } else { "Skipped" }
            );
            println!("  {} Output Styles: {}", "✓".green(), result.output_styles_installed);
            println!();

            Ui::info("Restart Claude Code to apply changes.");
            println!();
        }
        Err(e) => {
            Ui::error(&format!("Sync failed: {}", e));
            println!();
            Ui::info("Make sure:");
            println!("  1. GitHub repository is configured in Project Settings");
            println!("  2. A configuration has been deployed first");
            println!("  3. You have internet access");
            println!();
        }
    }

    Ok(())
}
