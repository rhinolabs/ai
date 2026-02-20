use crate::ui::Ui;
use anyhow::Result;
use colored::*;
use rhinolabs_core::{Paths, Version};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusOutput {
    plugin_installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_installed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_location: Option<String>,
    claude_code_detected: bool,
    mcp_configured: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp_location: Option<String>,
}

pub fn run(json: bool) -> Result<()> {
    if json {
        return run_json();
    }

    Ui::header("📊 Rhinolabs AI Plugin Status");

    // Plugin info
    Ui::section("Plugin");
    if Paths::is_plugin_installed() {
        match Version::installed()? {
            Some(version_info) => {
                println!("  Version:      {}", version_info.version.green());
                println!(
                    "  Installed at: {}",
                    version_info.installed_at.format("%Y-%m-%d %H:%M:%S UTC")
                );
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
        Ui::info("Run 'rhinolabs-ai install' to install the plugin");
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
            Ui::info("Run 'rhinolabs-ai sync-mcp --url <URL>' to sync configuration");
        }
    }

    println!();

    Ok(())
}

fn run_json() -> Result<()> {
    let plugin_installed = Paths::is_plugin_installed();

    let (plugin_version, plugin_installed_at) = if plugin_installed {
        match Version::installed()? {
            Some(v) => (
                Some(v.version),
                Some(v.installed_at.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
            ),
            None => (None, None),
        }
    } else {
        (None, None)
    };

    let plugin_location = if plugin_installed {
        Paths::plugin_dir().ok().map(|p| p.display().to_string())
    } else {
        None
    };

    let claude_code_detected = Paths::is_claude_code_installed();

    let (mcp_configured, mcp_location) = match Paths::mcp_config_path() {
        Ok(path) if path.exists() => (true, Some(path.display().to_string())),
        _ => (false, None),
    };

    let output = StatusOutput {
        plugin_installed,
        plugin_version,
        plugin_installed_at,
        plugin_location,
        claude_code_detected,
        mcp_configured,
        mcp_location,
    };

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
