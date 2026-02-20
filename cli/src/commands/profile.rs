use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::{DeployTarget, ProfileType, Profiles};
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProfileUninstallResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    pub target_path: String,
}

/// Parse target strings into DeployTarget vec.
/// Handles "all" keyword and individual target names.
fn parse_targets(strs: &[String]) -> Result<Vec<DeployTarget>> {
    if strs.iter().any(|s| s == "all") {
        return Ok(DeployTarget::all().to_vec());
    }
    strs.iter()
        .map(|s| s.parse::<DeployTarget>().map_err(|e| anyhow::anyhow!(e)))
        .collect()
}

/// Detect installed profile from .claude-plugin/plugin.json
fn detect_installed_profile(path: &Path) -> Option<(String, String)> {
    let plugin_json = path.join(".claude-plugin").join("plugin.json");
    if !plugin_json.exists() {
        return None;
    }

    let content = fs::read_to_string(&plugin_json).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;

    // Read profile info from the manifest
    let profile_id = json["profile"]["id"].as_str()?.to_string();
    let profile_name = json["profile"]["name"].as_str()?.to_string();

    Some((profile_id, profile_name))
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

/// Format target list for display
fn format_targets(targets: &[DeployTarget]) -> String {
    targets
        .iter()
        .map(|t| t.display_name())
        .collect::<Vec<_>>()
        .join(", ")
}

/// List all profiles
pub fn list(json: bool) -> Result<()> {
    let profiles = Profiles::list()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&profiles)?);
        return Ok(());
    }

    Ui::header("Profiles");

    if profiles.is_empty() {
        Ui::info("No profiles configured yet.");
        Ui::info("Create profiles in the GUI to organize your skills.");
        return Ok(());
    }

    let default_user = Profiles::get_default_user_profile()?.map(|p| p.id);

    for profile in &profiles {
        let type_badge = match profile.profile_type {
            ProfileType::User => "[User]",
            ProfileType::Project => "[Project]",
        };

        let default_badge = if Some(&profile.id) == default_user.as_ref() {
            " (default)"
        } else {
            ""
        };

        let skill_count = profile.skills.len();

        println!();
        println!(
            "  {} {} {}{}",
            "•".cyan(),
            profile.name.bold(),
            type_badge.dimmed(),
            default_badge.green()
        );
        println!("    ID: {}", profile.id);
        println!("    Skills: {}", skill_count);
        if !profile.description.is_empty() {
            println!("    {}", profile.description.dimmed());
        }
    }

    println!();
    Ok(())
}

/// Show details of a specific profile
pub fn show(profile_id: &str, json: bool) -> Result<()> {
    let profile = Profiles::get(profile_id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&profile)?);
        return Ok(());
    }

    match profile {
        Some(profile) => {
            Ui::header(&format!("Profile: {}", profile.name));

            let type_str = match profile.profile_type {
                ProfileType::User => "User (installs to ~/.claude/)",
                ProfileType::Project => "Project (installs to project/.claude/)",
            };

            println!("  ID:          {}", profile.id);
            println!("  Name:        {}", profile.name);
            println!("  Type:        {}", type_str);
            println!("  Description: {}", profile.description);
            println!("  Targets:     {}", format_targets(DeployTarget::all()));
            println!("  Created:     {}", profile.created_at);
            println!("  Updated:     {}", profile.updated_at);
            println!();

            if profile.skills.is_empty() {
                Ui::info("No skills assigned to this profile.");
            } else {
                Ui::section("Assigned Skills");
                for skill_id in &profile.skills {
                    println!("  • {}", skill_id);
                }
            }

            println!();
        }
        None => {
            Ui::error(&format!("Profile '{}' not found", profile_id));
        }
    }

    Ok(())
}

/// Install a profile to a target path
pub fn install(
    profile_id: &str,
    target_path: Option<String>,
    target_strs: Vec<String>,
    json: bool,
) -> Result<()> {
    if json {
        return install_json(profile_id, target_path, target_strs);
    }

    Ui::header("Installing Profile");

    let targets = parse_targets(&target_strs)?;
    let targets_ref = if targets.is_empty() {
        None
    } else {
        Some(targets.as_slice())
    };

    let profile = Profiles::get(profile_id)?;

    match profile {
        Some(profile) => {
            Ui::step(&format!("Profile: {} ({})", profile.name, profile.id));

            // Show which targets will be used
            let effective_targets = targets_ref.unwrap_or(&[DeployTarget::ClaudeCode]);
            if effective_targets.len() > 1
                || effective_targets.first() != Some(&DeployTarget::ClaudeCode)
            {
                println!(
                    "  {} Targets: {}",
                    "→".cyan(),
                    format_targets(effective_targets).bold()
                );
            }

            // For Project profiles: use current directory if no path specified
            let effective_path = if profile.profile_type == ProfileType::Project {
                let path = target_path
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

                let path_display = path.display().to_string();
                println!();
                println!(
                    "  {} Profile '{}' will be installed in:",
                    "→".cyan(),
                    profile.name
                );
                println!("    {}", path_display.bold());
                println!();
                println!("  This will create:");

                for target in effective_targets {
                    let prefix = target.project_skills_prefix();
                    println!("    {}:", target.display_name().bold());
                    println!("      • {}/  ({} skills)", prefix, profile.skills.len());
                    println!("      • {}", target.instructions_filename());
                    if *target == DeployTarget::ClaudeCode {
                        println!("      • .claude-plugin/plugin.json");
                    }
                }
                println!();

                if !prompt_yes_no("Continue?", true) {
                    Ui::info("Installation cancelled.");
                    return Ok(());
                }
                println!();

                Some(path)
            } else {
                // User profile - path is ignored, installs to ~/.claude/
                if target_path.is_some() {
                    Ui::warning(
                        "User profiles ignore --path and install to user config directories.",
                    );
                }
                None
            };

            if profile.skills.is_empty() {
                if profile_id == "main" {
                    Ui::warning("Main Profile has no skills. The rhinolabs-claude plugin may not be installed.");
                    Ui::info("Install the plugin first: rhinolabs-ai install");
                } else {
                    Ui::warning("This profile has no skills assigned.");
                    Ui::info("Assign skills to this profile in the GUI first.");
                }
                return Ok(());
            }

            Ui::step(&format!("Installing {} skills...", profile.skills.len()));

            let path = effective_path.as_deref();
            let result = Profiles::install(profile_id, path, targets_ref)?;

            println!();
            Ui::success(&format!("Installed to: {}", result.target_path));

            // Show what was created per target
            if profile.profile_type == ProfileType::Project {
                Ui::section("Structure Created");
                for target in &result.targets_installed {
                    println!("  {}:", target.display_name().bold());
                    println!("    {} {}/", "✓".green(), target.project_skills_prefix());
                    println!("    {} {}", "✓".green(), target.instructions_filename());
                    if *target == DeployTarget::ClaudeCode {
                        println!("    {} .claude-plugin/plugin.json", "✓".green());
                    }
                }
            }

            if !result.skills_installed.is_empty() {
                Ui::section("Skills Installed");
                for skill in &result.skills_installed {
                    println!("  {} {}", "✓".green(), skill);
                }
            }

            if !result.skills_failed.is_empty() {
                Ui::section("Failed Skills");
                for error in &result.skills_failed {
                    println!("  {} {} - {}", "✗".red(), error.skill_id, error.error);
                }
            }

            println!();
            if profile.profile_type == ProfileType::Project {
                let target_names = format_targets(&result.targets_installed);
                Ui::info(&format!("Profile installed for: {}.", target_names));
            } else {
                Ui::info("Skills installed to user config directories.");
            }
        }
        None => {
            Ui::error(&format!("Profile '{}' not found", profile_id));
            Ui::info("Use 'rhinolabs-ai profile list' to see available profiles.");
        }
    }

    Ok(())
}

/// Update installed profile (re-install with latest skill versions)
pub fn update(
    profile_id: Option<String>,
    target_path: Option<String>,
    target_strs: Vec<String>,
) -> Result<()> {
    Ui::header("Updating Profile");

    let targets = parse_targets(&target_strs)?;
    let targets_ref = if targets.is_empty() {
        None
    } else {
        Some(targets.as_slice())
    };

    // Determine target path (default to current directory)
    let target = target_path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    // If no profile specified, detect from installed plugin
    let effective_profile_id = match profile_id {
        Some(id) => id,
        None => match detect_installed_profile(&target) {
            Some((id, name)) => {
                Ui::step(&format!("Detected installed profile: {}", name));
                id
            }
            None => {
                Ui::error("No profile installed in this directory.");
                Ui::info("Use 'rhinolabs-ai profile install <profile>' to install one first.");
                return Ok(());
            }
        },
    };

    let profile = Profiles::get(&effective_profile_id)?;

    match profile {
        Some(profile) => {
            let path_display = target.display().to_string();
            println!();
            println!(
                "  {} Profile '{}' will be updated in:",
                "→".cyan(),
                profile.name
            );
            println!("    {}", path_display.bold());

            if let Some(t) = targets_ref {
                println!("  {} Targets: {}", "→".cyan(), format_targets(t).bold());
            }
            println!();

            if !prompt_yes_no("Continue?", true) {
                Ui::info("Update cancelled.");
                return Ok(());
            }
            println!();

            Ui::step("Updating skills to latest versions...");

            let result =
                Profiles::update_installed(&effective_profile_id, Some(&target), targets_ref)?;

            println!();
            Ui::success("Profile updated!");

            println!("  Updated: {} skills", result.skills_installed.len());
            if !result.skills_failed.is_empty() {
                println!("  Failed: {} skills", result.skills_failed.len());
            }

            println!();
        }
        None => {
            Ui::error(&format!(
                "Profile '{}' not found in configuration",
                effective_profile_id
            ));
            Ui::info(
                "The installed profile may have been removed. Run 'rhinolabs-ai sync' to update.",
            );
        }
    }

    Ok(())
}

/// Uninstall profile from a target path
pub fn uninstall(target_path: Option<String>, target_strs: Vec<String>, json: bool) -> Result<()> {
    let targets = parse_targets(&target_strs)?;
    let targets_ref = if targets.is_empty() {
        None
    } else {
        Some(targets.as_slice())
    };

    let path = target_path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    if json {
        return uninstall_json(&path, targets_ref);
    }

    Ui::header("Uninstalling Profile");

    let path_display = path.display().to_string();

    // Detect installed profile name
    let profile_info = detect_installed_profile(&path);

    println!();
    if let Some((_, profile_name)) = &profile_info {
        println!(
            "  {} Profile '{}' will be uninstalled from:",
            "→".cyan(),
            profile_name.bold()
        );
    } else {
        println!("  {} Profile will be uninstalled from:", "→".cyan());
    }
    println!("    {}", path_display.bold());

    if let Some(t) = targets_ref {
        println!("  {} Targets: {}", "→".cyan(), format_targets(t).bold());
    }
    println!();

    if !path.exists() {
        Ui::error("Target path does not exist");
        return Ok(());
    }

    // Check what exists for display
    let effective_targets = targets_ref.unwrap_or_else(|| DeployTarget::all());
    let mut has_anything = false;

    println!("  This will remove:");
    for target in effective_targets {
        let config_dir = path.join(match target {
            DeployTarget::ClaudeCode => ".claude",
            DeployTarget::Amp => ".agents",
            DeployTarget::Antigravity => ".agent",
            DeployTarget::OpenCode => ".opencode",
        });
        if config_dir.exists() {
            println!(
                "    • {}/ (skills)",
                config_dir.file_name().unwrap().to_string_lossy()
            );
            has_anything = true;
        }
        let instructions_file = path.join(target.instructions_filename());
        if instructions_file.exists() {
            println!(
                "    • {} (if generated by rhinolabs-ai)",
                target.instructions_filename()
            );
            has_anything = true;
        }
        if *target == DeployTarget::ClaudeCode {
            let plugin_dir = path.join(".claude-plugin");
            if plugin_dir.exists() {
                println!("    • .claude-plugin/ (plugin manifest)");
                has_anything = true;
            }
        }
    }

    if !has_anything {
        Ui::warning("No profile installation found at this location.");
        return Ok(());
    }
    println!();

    if !prompt_yes_no("Continue?", false) {
        Ui::info("Uninstall cancelled.");
        return Ok(());
    }
    println!();

    Profiles::uninstall(&path, targets_ref)?;

    Ui::success("Profile uninstalled!");

    Ok(())
}

/// Uninstall a profile in JSON mode (non-interactive, no prompts)
fn uninstall_json(path: &std::path::Path, targets: Option<&[DeployTarget]>) -> Result<()> {
    let profile_info = detect_installed_profile(path);
    let profile_id = profile_info.as_ref().map(|(id, _)| id.clone());
    let profile_name = profile_info.map(|(_, name)| name);

    Profiles::uninstall(path, targets)?;

    let result = ProfileUninstallResult {
        success: true,
        profile_id,
        profile_name,
        target_path: path.display().to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

/// Install a profile in JSON mode (non-interactive, no prompts)
fn install_json(
    profile_id: &str,
    target_path: Option<String>,
    target_strs: Vec<String>,
) -> Result<()> {
    let targets = parse_targets(&target_strs)?;
    let targets_ref = if targets.is_empty() {
        None
    } else {
        Some(targets.as_slice())
    };

    let profile = Profiles::get(profile_id)?;
    match profile {
        Some(profile) => {
            let effective_path = if profile.profile_type == ProfileType::Project {
                Some(
                    target_path
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default()),
                )
            } else {
                None
            };

            let path = effective_path.as_deref();
            let result = Profiles::install(profile_id, path, targets_ref)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        None => {
            let err = serde_json::json!({ "error": format!("Profile '{}' not found", profile_id) });
            println!("{}", serde_json::to_string_pretty(&err)?);
        }
    }

    Ok(())
}

/// Sync installed profile: reconcile declared vs installed skills
pub fn sync(target_path: Option<String>, json: bool) -> Result<()> {
    let path = target_path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    let result = Profiles::sync_project(&path)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    // Pretty-print mode
    Ui::header("Profile Sync");

    match result.status.as_str() {
        "no_profile" => {
            Ui::warning("No profile installed in this directory.");
            Ui::info("Use 'rhinolabs-ai profile install <profile>' to install one first.");
        }
        "synced" => {
            Ui::success("Profile already in sync — nothing to do.");
            println!("  Profile: {}", result.profile_id.as_deref().unwrap_or("—"));
            println!("  Skills:  {}", result.unchanged.len());
        }
        "updated" => {
            Ui::success("Profile synced!");
            println!("  Profile: {}", result.profile_id.as_deref().unwrap_or("—"));

            if !result.added.is_empty() {
                Ui::section("Added");
                for skill in &result.added {
                    println!("  {} {}", "✓".green(), skill);
                }
            }

            if !result.removed.is_empty() {
                Ui::section("Removed");
                for skill in &result.removed {
                    println!("  {} {}", "✗".red(), skill);
                }
            }

            if !result.unchanged.is_empty() {
                println!();
                println!("  Unchanged: {} skills", result.unchanged.len());
            }
        }
        _ => {
            Ui::info(&format!("Status: {}", result.status));
        }
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhinolabs_core::DeployTarget;

    #[test]
    fn test_parse_targets_empty_returns_empty() {
        let input: Vec<String> = vec![];
        let result = parse_targets(&input).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_targets_single_valid_target() {
        let input = vec!["amp".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], DeployTarget::Amp);
    }

    #[test]
    fn test_parse_targets_multiple_valid_targets() {
        let input = vec!["claude-code".to_string(), "amp".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], DeployTarget::ClaudeCode);
        assert_eq!(result[1], DeployTarget::Amp);
    }

    #[test]
    fn test_parse_targets_all_returns_all_four() {
        let input = vec!["all".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result.len(), 4);
        assert!(result.contains(&DeployTarget::ClaudeCode));
        assert!(result.contains(&DeployTarget::Amp));
        assert!(result.contains(&DeployTarget::Antigravity));
        assert!(result.contains(&DeployTarget::OpenCode));
    }

    #[test]
    fn test_parse_targets_all_ignores_other_entries() {
        // "all" overrides everything else in the list
        let input = vec!["amp".to_string(), "all".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_parse_targets_invalid_target_returns_error() {
        let input = vec!["vscode".to_string()];
        let result = parse_targets(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_targets_mixed_valid_invalid_returns_error() {
        let input = vec!["amp".to_string(), "invalid-target".to_string()];
        let result = parse_targets(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_targets_alias_claude() {
        let input = vec!["claude".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result[0], DeployTarget::ClaudeCode);
    }

    #[test]
    fn test_parse_targets_alias_gemini() {
        let input = vec!["gemini".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result[0], DeployTarget::Antigravity);
    }

    #[test]
    fn test_parse_targets_alias_opencode() {
        let input = vec!["opencode".to_string()];
        let result = parse_targets(&input).unwrap();
        assert_eq!(result[0], DeployTarget::OpenCode);
    }

    #[test]
    fn test_format_targets_single() {
        let targets = vec![DeployTarget::Amp];
        assert_eq!(format_targets(&targets), "Amp");
    }

    #[test]
    fn test_format_targets_multiple() {
        let targets = vec![DeployTarget::ClaudeCode, DeployTarget::Amp];
        assert_eq!(format_targets(&targets), "Claude Code, Amp");
    }

    #[test]
    fn test_format_targets_all() {
        let targets = DeployTarget::all().to_vec();
        assert_eq!(
            format_targets(&targets),
            "Claude Code, Amp, Antigravity, OpenCode"
        );
    }

    #[test]
    fn test_format_targets_empty() {
        let targets: Vec<DeployTarget> = vec![];
        assert_eq!(format_targets(&targets), "");
    }

    // ── ProfileUninstallResult serialization tests ──────────

    #[test]
    fn test_uninstall_result_serialization_full() {
        let result = ProfileUninstallResult {
            success: true,
            profile_id: Some("react-stack".to_string()),
            profile_name: Some("React Stack".to_string()),
            target_path: "/home/user/project".to_string(),
        };
        let json: serde_json::Value = serde_json::to_value(&result).unwrap();

        assert_eq!(json["success"], true);
        assert_eq!(json["profileId"], "react-stack");
        assert_eq!(json["profileName"], "React Stack");
        assert_eq!(json["targetPath"], "/home/user/project");
    }

    #[test]
    fn test_uninstall_result_omits_none_fields() {
        let result = ProfileUninstallResult {
            success: true,
            profile_id: None,
            profile_name: None,
            target_path: "/tmp/test".to_string(),
        };
        let json: serde_json::Value = serde_json::to_value(&result).unwrap();

        assert_eq!(json["success"], true);
        assert!(json.get("profileId").is_none());
        assert!(json.get("profileName").is_none());
        assert_eq!(json["targetPath"], "/tmp/test");
    }

    #[test]
    fn test_uninstall_result_uses_camel_case() {
        let result = ProfileUninstallResult {
            success: true,
            profile_id: Some("test".to_string()),
            profile_name: Some("Test".to_string()),
            target_path: "/tmp".to_string(),
        };
        let json_str = serde_json::to_string(&result).unwrap();

        // Verify camelCase keys (not snake_case)
        assert!(json_str.contains("profileId"));
        assert!(json_str.contains("profileName"));
        assert!(json_str.contains("targetPath"));
        assert!(!json_str.contains("profile_id"));
        assert!(!json_str.contains("profile_name"));
        assert!(!json_str.contains("target_path"));
    }
}
