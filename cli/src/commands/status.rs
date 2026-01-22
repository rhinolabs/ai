use crate::ui::Ui;
use anyhow::Result;
use colored::*;
use rhinolabs_core::{Paths, Version};

pub fn run() -> Result<()> {
    Ui::header("📊 Rhinolabs AI Plugin Status");

    // Plugin info
    Ui::section("Plugin");
    if Paths::is_plugin_installed() {
        match Version::installed()? {
            Some(version_info) => {
                println!("  Version:      {}", version_info.version.green());
                println!("  Installed at: {}", version_info.installed_at.format("%Y-%m-%d %H:%M:%S UTC"));
            }
            None => {
                println!("  Version:      {}", "unknown".yellow());
            }
        }

        if let Ok(plugin_dir) = Paths::plugin_dir() {
            println!("  Location:     {}", plugin_dir.display());
        }

        println!("  Status:       {}", "✓ Installed".green());
    } else {
        println!("  Status:       {}", "✗ Not installed".red());
        println!();
        Ui::info("Run 'rhinolabs install' to install the plugin");
    }

    // Claude Code info
    Ui::section("Claude Code");
    if Paths::is_claude_code_installed() {
        println!("  Status:       {}", "✓ Detected".green());
    } else {
        println!("  Status:       {}", "✗ Not found".red());
    }

    // MCP config
    Ui::section("MCP Configuration");
    if let Ok(config_path) = Paths::mcp_config_path() {
        if config_path.exists() {
            println!("  Status:       {}", "✓ Configured".green());
            println!("  Location:     {}", config_path.display());
        } else {
            println!("  Status:       {}", "⚠ Not synced".yellow());
            println!();
            Ui::info("Run 'rhinolabs sync-mcp --url <URL>' to sync configuration");
        }
    }

    println!();

    Ok(())
}
