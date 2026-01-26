use crate::ui::Ui;
use anyhow::Result;
use colored::Colorize;
use rhinolabs_core::{Profiles, ProfileType};
use std::path::Path;

/// List all profiles
pub fn list() -> Result<()> {
    Ui::header("Profiles");

    let profiles = Profiles::list()?;

    if profiles.is_empty() {
        Ui::info("No profiles configured yet.");
        Ui::info("Create profiles in the GUI to organize your skills.");
        return Ok(());
    }

    let default_user = Profiles::get_default_user_profile()?.map(|p| p.id);

    for profile in profiles {
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
pub fn show(profile_id: &str) -> Result<()> {
    let profile = Profiles::get(profile_id)?;

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
pub fn install(profile_id: &str, target_path: Option<String>) -> Result<()> {
    Ui::header("Installing Profile");

    let profile = Profiles::get(profile_id)?;

    match profile {
        Some(profile) => {
            Ui::step(&format!("Profile: {} ({})", profile.name, profile.id));

            let path = target_path.as_deref().map(Path::new);

            // Validate path requirements
            if profile.profile_type == ProfileType::Project && path.is_none() {
                Ui::error("Project profiles require a target path.");
                Ui::info("Usage: rhinolabs profile install --profile <id> --path <project-path>");
                return Ok(());
            }

            if profile.profile_type == ProfileType::User && path.is_some() {
                Ui::warning("User profiles ignore --path and install to ~/.claude/");
            }

            if profile.skills.is_empty() {
                Ui::warning("This profile has no skills assigned.");
                Ui::info("Assign skills to this profile in the GUI first.");
                return Ok(());
            }

            Ui::step(&format!("Installing {} skills...", profile.skills.len()));

            let result = Profiles::install(profile_id, path)?;

            println!();
            Ui::success(&format!("Installed to: {}", result.target_path));

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
            Ui::info("Claude Code will automatically load skills from this location.");
        }
        None => {
            Ui::error(&format!("Profile '{}' not found", profile_id));
            Ui::info("Use 'rhinolabs profile list' to see available profiles.");
        }
    }

    Ok(())
}

/// Update installed profile (re-install with latest skill versions)
pub fn update(profile_id: &str, target_path: Option<String>) -> Result<()> {
    Ui::header("Updating Profile");

    let profile = Profiles::get(profile_id)?;

    match profile {
        Some(profile) => {
            Ui::step(&format!("Profile: {} ({})", profile.name, profile.id));

            let path = target_path.as_deref().map(Path::new);

            if profile.profile_type == ProfileType::Project && path.is_none() {
                Ui::error("Project profiles require a target path.");
                Ui::info("Usage: rhinolabs profile update --profile <id> --path <project-path>");
                return Ok(());
            }

            Ui::step("Updating skills to latest versions...");

            let result = Profiles::update_installed(profile_id, path)?;

            println!();
            Ui::success("Profile updated!");

            println!("  Updated: {} skills", result.skills_installed.len());
            if !result.skills_failed.is_empty() {
                println!("  Failed: {} skills", result.skills_failed.len());
            }

            println!();
        }
        None => {
            Ui::error(&format!("Profile '{}' not found", profile_id));
        }
    }

    Ok(())
}

/// Uninstall profile from a target path
pub fn uninstall(target_path: &str) -> Result<()> {
    Ui::header("Uninstalling Profile");

    let path = Path::new(target_path);

    Ui::step(&format!("Target: {}", target_path));

    if !path.exists() {
        Ui::error("Target path does not exist");
        return Ok(());
    }

    let claude_dir = path.join(".claude");
    if !claude_dir.exists() {
        Ui::warning("No .claude directory found at this location.");
        return Ok(());
    }

    Profiles::uninstall(path)?;

    Ui::success("Profile uninstalled!");
    Ui::info(&format!("Removed: {}/.claude", target_path));

    Ok(())
}
