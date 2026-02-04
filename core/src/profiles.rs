use crate::{
    targets::TargetPaths, DeployTarget, InstructionsManager, OutputStyle, OutputStyles, Result,
    RhinolabsError, Settings, Skill, Skills,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ============================================
// Profile Types
// ============================================

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProfileType {
    /// User-level profile: installs to ~/.claude/
    User,
    /// Project-level profile: installs to /project/.claude/
    #[default]
    Project,
}

// ============================================
// Auto-invoke Rules
// ============================================

/// Defines when a skill should be automatically loaded based on context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AutoInvokeRule {
    /// The skill ID this rule applies to
    pub skill_id: String,
    /// When to trigger (e.g., "Editing .tsx/.jsx files")
    pub trigger: String,
    /// Description of what the skill provides (e.g., "React 19 patterns and hooks")
    pub description: String,
}

// ============================================
// Profile Entity
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub profile_type: ProfileType,
    pub skills: Vec<String>,
    /// Auto-invoke rules: when to load each skill
    #[serde(default)]
    pub auto_invoke_rules: Vec<AutoInvokeRule>,
    /// Custom instructions to include in CLAUDE.md
    #[serde(default)]
    pub instructions: Option<String>,
    /// Generate .github/copilot-instructions.md
    #[serde(default = "default_true")]
    pub generate_copilot: bool,
    /// Generate AGENTS.md as master file
    #[serde(default)]
    pub generate_agents: bool,
    pub created_at: String,
    pub updated_at: String,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub profile_type: ProfileType,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default = "default_true")]
    pub generate_copilot: bool,
    #[serde(default)]
    pub generate_agents: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub profile_type: Option<ProfileType>,
    pub instructions: Option<String>,
    pub generate_copilot: Option<bool>,
    pub generate_agents: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAutoInvokeInput {
    pub profile_id: String,
    pub rules: Vec<AutoInvokeRule>,
}

/// Generated content for multi-AI instruction files (internal use)
struct GeneratedAiContent {
    #[allow(dead_code)]
    claude_md: String,
    copilot_md: String,
    agents_md: String,
}

// ============================================
// Installation Result
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInstallResult {
    pub profile_id: String,
    pub profile_name: String,
    pub target_path: String,
    pub skills_installed: Vec<String>,
    pub skills_failed: Vec<SkillInstallError>,
    /// For Main-Profile: indicates if instructions were installed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_installed: Option<bool>,
    /// For Main-Profile: indicates if settings were installed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_installed: Option<bool>,
    /// For Main-Profile: indicates if output style was installed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_style_installed: Option<String>,
    /// Which deploy targets were installed to
    #[serde(default)]
    pub targets_installed: Vec<DeployTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillInstallError {
    pub skill_id: String,
    pub error: String,
}

// ============================================
// Storage Configuration
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ProfilesConfig {
    profiles: Vec<Profile>,
    default_user_profile: Option<String>,
}

// ============================================
// Profiles Manager
// ============================================

pub struct Profiles;

impl Profiles {
    /// Get the rhinolabs config directory: ~/.config/rhinolabs-ai/
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find config directory".into()))?
            .join("rhinolabs-ai");
        Ok(config_dir)
    }

    /// Get the profiles config file path
    fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("profiles.json"))
    }

    /// Get the Claude user directory: ~/.claude/
    pub fn claude_user_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find home directory".into()))?;
        Ok(home.join(".claude"))
    }

    /// Get the Claude project directory for a given path: /project/.claude/
    pub fn claude_project_dir(project_path: &Path) -> PathBuf {
        project_path.join(".claude")
    }

    /// Create the default Main-Profile
    fn create_main_profile() -> Profile {
        let now = chrono::Utc::now().to_rfc3339();
        Profile {
            id: "main".to_string(),
            name: "Main Profile".to_string(),
            description:
                "User-level skills that apply to all projects. Install with: rhinolabs install"
                    .to_string(),
            profile_type: ProfileType::User,
            skills: Vec::new(),
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false, // Main-Profile doesn't generate copilot (user-level)
            generate_agents: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Generate template instructions for new project profiles
    /// If skills are provided, includes them in the auto-invoke table
    fn generate_template_instructions(
        profile_name: &str,
        profile_description: &str,
        skill_ids: &[String],
    ) -> String {
        Self::generate_template_instructions_for_target(
            profile_name,
            profile_description,
            skill_ids,
            ".claude/skills",
        )
    }

    /// Generate template instructions with a configurable skills prefix
    fn generate_template_instructions_for_target(
        profile_name: &str,
        profile_description: &str,
        skill_ids: &[String],
        skills_prefix: &str,
    ) -> String {
        // Build skills table rows
        let skills_table = if skill_ids.is_empty() {
            "| <!-- Add skills to this profile --> | | |".to_string()
        } else {
            // Load skill details to get names and descriptions
            let rows: Vec<String> = skill_ids
                .iter()
                .map(|skill_id| {
                    if let Ok(Some(skill)) = Skills::get(skill_id) {
                        let context = if skill.description.is_empty() {
                            format!("Working with {}", skill.name)
                        } else {
                            skill.description.clone()
                        };
                        format!(
                            "| {} | `{}` | `{}/{}/SKILL.md` |",
                            context, skill.id, skills_prefix, skill.id
                        )
                    } else {
                        format!(
                            "| Working with {} | `{}` | `{}/{}/SKILL.md` |",
                            skill_id, skill_id, skills_prefix, skill_id
                        )
                    }
                })
                .collect();
            rows.join("\n")
        };

        format!(
            r#"# {} - Project Instructions

> Edit this file to define AI behavior for projects using this profile.
> These instructions are included in CLAUDE.md when the profile is installed.

## Project Context

{}

## Rules

<!-- Define explicit rules for AI behavior in this project -->
- Follow the project's coding standards strictly
- Use conventional commits format for all commits
- Verify technical claims before stating them
- When unsure, investigate first rather than assume

## Code Standards

<!-- Define coding patterns and conventions -->
- Use TypeScript strict mode
- Follow the established project architecture
- Write tests for new functionality
- Document public APIs

## Forbidden Patterns

<!-- List anti-patterns and things to avoid -->
- Never commit sensitive data (API keys, credentials)
- Avoid any/unknown types without justification
- Don't skip error handling

## Skills Auto-invoke

IMPORTANT: When you detect any of these contexts, IMMEDIATELY read the corresponding skill file BEFORE writing any code.

| Context | Skill | Read First |
|---------|-------|------------|
{}

## How to Use Skills

1. Detect context from user request or current file being edited
2. Read the relevant SKILL.md file(s) BEFORE writing code
3. Apply ALL patterns and rules from the skill
4. Multiple skills can apply simultaneously

---
*Profile: {} | Generated by rhinolabs-ai*
"#,
            profile_name, profile_description, skills_table, profile_name
        )
    }

    /// Load profiles config, creating Main-Profile if it doesn't exist
    fn load_config() -> Result<ProfilesConfig> {
        let path = Self::config_path()?;

        if !path.exists() {
            // First time: create config with Main-Profile
            let main_profile = Self::create_main_profile();
            let config = ProfilesConfig {
                profiles: vec![main_profile],
                default_user_profile: Some("main".to_string()),
            };
            Self::save_config(&config)?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)?;
        let mut config: ProfilesConfig = serde_json::from_str(&content)?;

        // Ensure Main-Profile exists (migration for existing configs)
        if !config.profiles.iter().any(|p| p.id == "main") {
            config.profiles.insert(0, Self::create_main_profile());
            if config.default_user_profile.is_none() {
                config.default_user_profile = Some("main".to_string());
            }
            Self::save_config(&config)?;
        }

        Ok(config)
    }

    /// Save profiles config
    fn save_config(config: &ProfilesConfig) -> Result<()> {
        let path = Self::config_path()?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(config)?;
        fs::write(&path, content)?;
        Ok(())
    }

    // ============================================
    // CRUD Operations
    // ============================================

    /// List all profiles
    pub fn list() -> Result<Vec<Profile>> {
        let config = Self::load_config()?;
        Ok(config.profiles)
    }

    /// Get a specific profile by id
    pub fn get(id: &str) -> Result<Option<Profile>> {
        let config = Self::load_config()?;
        Ok(config.profiles.into_iter().find(|p| p.id == id))
    }

    /// Create a new profile
    pub fn create(input: CreateProfileInput) -> Result<Profile> {
        let mut config = Self::load_config()?;

        // Check for duplicate id
        if config.profiles.iter().any(|p| p.id == input.id) {
            return Err(RhinolabsError::ConfigError(format!(
                "Profile '{}' already exists",
                input.id
            )));
        }

        // Only Main-Profile can be User type. All new profiles must be Project type.
        if input.profile_type == ProfileType::User {
            return Err(RhinolabsError::ConfigError(
                "Only the Main-Profile can be of type User. New profiles must be Project type."
                    .into(),
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();

        // Generate template instructions for new profiles (if not provided)
        // Include assigned skills in the auto-invoke table
        let instructions = input.instructions.or_else(|| {
            Some(Self::generate_template_instructions(
                &input.name,
                &input.description,
                &input.skills,
            ))
        });

        let profile = Profile {
            id: input.id,
            name: input.name.clone(),
            description: input.description.clone(),
            profile_type: ProfileType::Project, // Always Project for new profiles
            skills: input.skills.clone(),       // Assign skills during creation
            auto_invoke_rules: Vec::new(),
            instructions,
            generate_copilot: input.generate_copilot,
            generate_agents: input.generate_agents,
            created_at: now.clone(),
            updated_at: now,
        };

        config.profiles.push(profile.clone());
        Self::save_config(&config)?;

        // Write instructions file for the new profile
        if let Some(ref content) = profile.instructions {
            let path = Self::get_instructions_path(&profile.id)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, content)?;
        }

        Ok(profile)
    }

    /// Update an existing profile
    pub fn update(id: &str, input: UpdateProfileInput) -> Result<Profile> {
        let mut config = Self::load_config()?;

        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| RhinolabsError::ConfigError(format!("Profile '{}' not found", id)))?;

        if let Some(name) = input.name {
            profile.name = name;
        }
        if let Some(description) = input.description {
            profile.description = description;
        }
        if let Some(instructions) = input.instructions {
            profile.instructions = Some(instructions);
        }
        if let Some(generate_copilot) = input.generate_copilot {
            profile.generate_copilot = generate_copilot;
        }
        if let Some(generate_agents) = input.generate_agents {
            profile.generate_agents = generate_agents;
        }
        // Note: profile_type is intentionally NOT updated.
        // Main-Profile is User, all others are Project. This cannot be changed.

        profile.updated_at = chrono::Utc::now().to_rfc3339();

        let updated = profile.clone();
        Self::save_config(&config)?;

        Ok(updated)
    }

    /// Delete a profile
    pub fn delete(id: &str) -> Result<()> {
        // Protect Main-Profile from deletion
        if id == "main" {
            return Err(RhinolabsError::ConfigError(
                "Cannot delete the Main Profile. You can remove all skills from it instead.".into(),
            ));
        }

        let mut config = Self::load_config()?;

        let initial_len = config.profiles.len();
        config.profiles.retain(|p| p.id != id);

        if config.profiles.len() == initial_len {
            return Err(RhinolabsError::ConfigError(format!(
                "Profile '{}' not found",
                id
            )));
        }

        // Clear default if deleted profile was the default
        if config.default_user_profile.as_deref() == Some(id) {
            config.default_user_profile = None;
        }

        Self::save_config(&config)?;
        Ok(())
    }

    // ============================================
    // Skill Assignment
    // ============================================

    /// Assign skills to a profile (replaces existing skills)
    pub fn assign_skills(profile_id: &str, skill_ids: Vec<String>) -> Result<Profile> {
        let mut config = Self::load_config()?;

        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or_else(|| {
                RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
            })?;

        profile.skills = skill_ids;
        profile.updated_at = chrono::Utc::now().to_rfc3339();

        let updated = profile.clone();
        Self::save_config(&config)?;

        Ok(updated)
    }

    /// Get skills assigned to a profile
    pub fn get_profile_skills(profile_id: &str) -> Result<Vec<Skill>> {
        let profile = Self::get(profile_id)?.ok_or_else(|| {
            RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
        })?;

        let mut skills = Vec::new();
        for skill_id in &profile.skills {
            if let Some(skill) = Skills::get(skill_id)? {
                skills.push(skill);
            }
        }

        Ok(skills)
    }

    /// Get profiles that contain a specific skill
    pub fn get_profiles_for_skill(skill_id: &str) -> Result<Vec<Profile>> {
        let config = Self::load_config()?;
        let profiles: Vec<Profile> = config
            .profiles
            .into_iter()
            .filter(|p| p.skills.contains(&skill_id.to_string()))
            .collect();
        Ok(profiles)
    }

    // ============================================
    // Auto-invoke Rules Management
    // ============================================

    /// Update auto-invoke rules for a profile
    pub fn update_auto_invoke_rules(
        profile_id: &str,
        rules: Vec<AutoInvokeRule>,
    ) -> Result<Profile> {
        let mut config = Self::load_config()?;

        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or_else(|| {
                RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
            })?;

        profile.auto_invoke_rules = rules;
        profile.updated_at = chrono::Utc::now().to_rfc3339();

        let updated = profile.clone();
        Self::save_config(&config)?;

        Ok(updated)
    }

    /// Get auto-invoke rules for a profile
    pub fn get_auto_invoke_rules(profile_id: &str) -> Result<Vec<AutoInvokeRule>> {
        let profile = Self::get(profile_id)?.ok_or_else(|| {
            RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
        })?;
        Ok(profile.auto_invoke_rules)
    }

    // ============================================
    // Default User Profile
    // ============================================

    /// Get the default user profile
    pub fn get_default_user_profile() -> Result<Option<Profile>> {
        let config = Self::load_config()?;

        match config.default_user_profile {
            Some(id) => {
                let profile = config.profiles.into_iter().find(|p| p.id == id);
                Ok(profile)
            }
            None => Ok(None),
        }
    }

    /// Set the default user profile
    pub fn set_default_user_profile(profile_id: &str) -> Result<()> {
        let mut config = Self::load_config()?;

        // Verify profile exists and is User type
        let profile = config
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .ok_or_else(|| {
                RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
            })?;

        if profile.profile_type != ProfileType::User {
            return Err(RhinolabsError::ConfigError(format!(
                "Profile '{}' is not a User profile",
                profile_id
            )));
        }

        config.default_user_profile = Some(profile_id.to_string());
        Self::save_config(&config)?;

        Ok(())
    }

    // ============================================
    // Profile Installation
    // ============================================

    /// Install a profile to a target path
    /// For User profiles (Main-Profile): installs to ~/.claude/ (and other targets) including:
    ///   - Skills → ~/.claude/skills/ (or target-specific user skills dir)
    ///   - Instructions → ~/.claude/CLAUDE.md
    ///   - Settings → ~/.claude/settings.json
    ///   - Output Style → ~/.claude/output-styles/
    ///
    /// For Project profiles: installs as a plugin to target_path/ including:
    ///   - Plugin manifest → target_path/.claude-plugin/plugin.json (ClaudeCode only)
    ///   - Skills → target_path/.claude/skills/ (or target-specific project skills dir)
    ///   - CLAUDE.md/AGENTS.md/GEMINI.md → target_path/ (generated from profile)
    ///
    /// If `targets` is `None`, defaults to `[ClaudeCode]` for backward compatibility.
    pub fn install(
        profile_id: &str,
        target_path: Option<&Path>,
        targets: Option<&[DeployTarget]>,
    ) -> Result<ProfileInstallResult> {
        let profile = Self::get(profile_id)?.ok_or_else(|| {
            RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
        })?;

        let default_targets = [DeployTarget::ClaudeCode];
        let effective_targets = targets.unwrap_or(&default_targets);

        let mut skills_installed = Vec::new();
        let mut skills_failed = Vec::new();

        // Install skills to each target
        for target in effective_targets {
            let skills_target = match profile.profile_type {
                ProfileType::User => TargetPaths::user_skills_dir(*target)?,
                ProfileType::Project => {
                    let path = target_path.ok_or_else(|| {
                        RhinolabsError::ConfigError("Project profiles require a target path".into())
                    })?;
                    TargetPaths::project_skills_dir(*target, path)
                }
            };

            fs::create_dir_all(&skills_target)?;

            for skill_id in &profile.skills {
                match Self::install_skill(skill_id, &skills_target) {
                    Ok(_) => {
                        if !skills_installed.contains(skill_id) {
                            skills_installed.push(skill_id.clone());
                        }
                    }
                    Err(e) => {
                        if !skills_failed
                            .iter()
                            .any(|f: &SkillInstallError| f.skill_id == *skill_id)
                        {
                            skills_failed.push(SkillInstallError {
                                skill_id: skill_id.clone(),
                                error: e.to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Determine base target path for result display
        let base_target = match profile.profile_type {
            ProfileType::User => Self::claude_user_dir()?,
            ProfileType::Project => target_path
                .ok_or_else(|| {
                    RhinolabsError::ConfigError("Project profiles require a target path".into())
                })?
                .to_path_buf(),
        };

        // For Main-Profile (User type): also install instructions, settings, and output style
        // For Project profiles: install as a plugin structure
        let (instructions_installed, settings_installed, output_style_installed) = if profile
            .profile_type
            == ProfileType::User
        {
            Self::install_main_profile_config_for_targets(effective_targets)?
        } else {
            Self::install_project_profile_for_targets(&base_target, &profile, effective_targets)?
        };

        Ok(ProfileInstallResult {
            profile_id: profile.id,
            profile_name: profile.name,
            target_path: base_target.display().to_string(),
            skills_installed,
            skills_failed,
            instructions_installed,
            settings_installed,
            output_style_installed,
            targets_installed: effective_targets.to_vec(),
        })
    }

    /// Install Project Profile for multiple deploy targets.
    ///
    /// For each target:
    /// 1. Generate instructions content with correct skill path references
    /// 2. Write instructions file (CLAUDE.md / AGENTS.md / GEMINI.md)
    ///
    /// Only for ClaudeCode target:
    /// 3. Create .claude-plugin/plugin.json manifest
    /// 4. Optionally write .github/copilot-instructions.md
    fn install_project_profile_for_targets(
        target_path: &Path,
        profile: &Profile,
        targets: &[DeployTarget],
    ) -> Result<(Option<bool>, Option<bool>, Option<String>)> {
        for target in targets {
            // Generate instructions content for this specific target
            let instructions_content = Self::generate_instructions_for_target(profile, *target);

            // Write instructions file (only if it doesn't exist — don't overwrite user's custom file)
            let instructions_path = TargetPaths::instructions_path(*target, target_path);
            if !instructions_path.exists() {
                fs::write(&instructions_path, &instructions_content)?;
            }

            // ClaudeCode-specific: create plugin manifest and copilot instructions
            if *target == DeployTarget::ClaudeCode {
                let plugin_dir = target_path.join(".claude-plugin");
                fs::create_dir_all(&plugin_dir)?;

                let plugin_manifest = serde_json::json!({
                    "name": format!("profile-{}", profile.id),
                    "description": profile.description,
                    "version": "1.0.0",
                    "author": {
                        "name": "Rhinolabs"
                    },
                    "profile": {
                        "id": profile.id,
                        "name": profile.name,
                        "skills": profile.skills,
                        "autoInvokeRules": profile.auto_invoke_rules
                    }
                });

                let manifest_path = plugin_dir.join("plugin.json");
                fs::write(
                    &manifest_path,
                    serde_json::to_string_pretty(&plugin_manifest)?,
                )?;

                // Create .github/copilot-instructions.md if enabled
                if profile.generate_copilot {
                    let github_dir = target_path.join(".github");
                    fs::create_dir_all(&github_dir)?;
                    let copilot_path = github_dir.join("copilot-instructions.md");
                    let copilot_content = Self::generate_copilot_instructions(profile);
                    fs::write(&copilot_path, &copilot_content)?;
                }
            }
        }

        // If generate_agents is true and Amp is NOT in targets, still generate AGENTS.md
        // as a supplementary cross-reference file (with .claude/skills/ paths)
        if profile.generate_agents && !targets.contains(&DeployTarget::Amp) {
            let agents_path = target_path.join("AGENTS.md");
            let content = Self::generate_ai_instructions_content(profile);
            fs::write(&agents_path, &content.agents_md)?;
        }

        Ok((Some(true), None, None))
    }

    /// Generate content for CLAUDE.md, copilot-instructions.md, and AGENTS.md
    fn generate_ai_instructions_content(profile: &Profile) -> GeneratedAiContent {
        // Build auto-invoke table
        let auto_invoke_table = if !profile.auto_invoke_rules.is_empty() {
            let rows: Vec<String> = profile
                .auto_invoke_rules
                .iter()
                .map(|rule| {
                    format!(
                        "| {} | {} | `.claude/skills/{}/SKILL.md` |",
                        rule.trigger, rule.skill_id, rule.skill_id
                    )
                })
                .collect();

            format!(
                r#"## Auto-invoke Skills

IMPORTANT: Load these skills based on context:

| Context | Skill | Read First |
|---------|-------|------------|
{}

"#,
                rows.join("\n")
            )
        } else {
            String::new()
        };

        // Build skills list
        let skills_list = profile
            .skills
            .iter()
            .map(|s| format!("- `{}`: See `.claude/skills/{}/SKILL.md`", s, s))
            .collect::<Vec<_>>()
            .join("\n");

        // Build custom instructions section
        let custom_instructions = match &profile.instructions {
            Some(instr) if !instr.is_empty() => format!(
                r#"## Project Standards

{}

"#,
                instr
            ),
            _ => String::new(),
        };

        // CLAUDE.md content
        let claude_md = format!(
            r#"# Project Instructions

> Auto-generated by rhinolabs-ai | Profile: {}
> Run `rhinolabs-ai profile update` to regenerate

{}{}## Available Skills

Skills in `.claude/skills/`:

{}

---
*Installed by rhinolabs-ai | Profile: {}*
"#,
            profile.id, auto_invoke_table, custom_instructions, skills_list, profile.id
        );

        // copilot-instructions.md content (adapted for Copilot)
        let copilot_auto_invoke = if !profile.auto_invoke_rules.is_empty() {
            let rows: Vec<String> = profile
                .auto_invoke_rules
                .iter()
                .map(|rule| {
                    format!(
                        "| {} | {} | {} |",
                        rule.trigger, rule.skill_id, rule.description
                    )
                })
                .collect();

            format!(
                r#"## Context-based Guidelines

Apply these guidelines based on context:

| Context | Guideline | Description |
|---------|-----------|-------------|
{}

"#,
                rows.join("\n")
            )
        } else {
            String::new()
        };

        let copilot_md = format!(
            r#"# GitHub Copilot Instructions

> Auto-generated by rhinolabs-ai | Profile: {}
> Source: AGENTS.md (if present) or profile configuration

{}{}## Skills Reference

This project uses the following skill guidelines (see `.claude/skills/` for details):

{}

---
*Generated by rhinolabs-ai*
"#,
            profile.id, copilot_auto_invoke, custom_instructions, skills_list
        );

        // AGENTS.md content (master file)
        let agents_auto_invoke = if !profile.auto_invoke_rules.is_empty() {
            let rows: Vec<String> = profile
                .auto_invoke_rules
                .iter()
                .map(|rule| {
                    format!(
                        "| {} | `{}` | {} |",
                        rule.trigger, rule.skill_id, rule.description
                    )
                })
                .collect();

            format!(
                r#"## Auto-invoke Rules

When performing these actions, load the corresponding skill FIRST:

| Context | Skill | Description |
|---------|-------|-------------|
{}

"#,
                rows.join("\n")
            )
        } else {
            String::new()
        };

        let skills_table = profile
            .skills
            .iter()
            .map(|s| format!("| `{}` | `.claude/skills/{}/SKILL.md` |", s, s))
            .collect::<Vec<_>>()
            .join("\n");

        let agents_md = format!(
            r#"# {} - AI Agent Configuration

> **Single Source of Truth** - This file is the master for all AI assistants.
> Generated by rhinolabs-ai from profile: {}

## Profile Information

- **ID**: {}
- **Name**: {}
- **Description**: {}

{}{}## Available Skills

| Skill | Location |
|-------|----------|
{}

## How Skills Work

1. **Auto-detection**: AI reads this file or CLAUDE.md for context
2. **Context matching**: Based on file type or action, relevant skill loads
3. **Pattern application**: AI follows exact patterns from the skill
4. **Consistency**: Same patterns across all AI assistants

## File Generation

This profile generates:
- `CLAUDE.md` - For Claude Code
{}{}
---
*Generated by rhinolabs-ai | Profile: {} | Version: 1.0.0*
"#,
            profile.name,
            profile.id,
            profile.id,
            profile.name,
            profile.description,
            agents_auto_invoke,
            custom_instructions,
            skills_table,
            if profile.generate_copilot {
                "- `.github/copilot-instructions.md` - For GitHub Copilot\n"
            } else {
                ""
            },
            if profile.generate_agents {
                "- `AGENTS.md` - Master reference file\n"
            } else {
                ""
            },
            profile.id
        );

        GeneratedAiContent {
            claude_md,
            copilot_md,
            agents_md,
        }
    }

    /// Generate instructions content for a specific deploy target.
    /// Returns a single string of instructions content with correct skill path references.
    fn generate_instructions_for_target(profile: &Profile, target: DeployTarget) -> String {
        let skills_prefix = target.project_skills_prefix();

        // Build auto-invoke table
        let auto_invoke_table = if !profile.auto_invoke_rules.is_empty() {
            let rows: Vec<String> = profile
                .auto_invoke_rules
                .iter()
                .map(|rule| {
                    format!(
                        "| {} | {} | `{}/{}/SKILL.md` |",
                        rule.trigger, rule.skill_id, skills_prefix, rule.skill_id
                    )
                })
                .collect();

            format!(
                r#"## Auto-invoke Skills

IMPORTANT: Load these skills based on context:

| Context | Skill | Read First |
|---------|-------|------------|
{}

"#,
                rows.join("\n")
            )
        } else {
            String::new()
        };

        // Build skills list
        let skills_list = profile
            .skills
            .iter()
            .map(|s| format!("- `{}`: See `{}/{}/SKILL.md`", s, skills_prefix, s))
            .collect::<Vec<_>>()
            .join("\n");

        // Build custom instructions section
        let custom_instructions = match &profile.instructions {
            Some(instr) if !instr.is_empty() => format!(
                r#"## Project Standards

{}

"#,
                instr
            ),
            _ => String::new(),
        };

        format!(
            r#"# Project Instructions

> Auto-generated by rhinolabs-ai | Profile: {}
> Run `rhinolabs-ai profile update` to regenerate

{}{}## Available Skills

Skills in `{}/`:

{}

---
*Installed by rhinolabs-ai | Profile: {}*
"#,
            profile.id,
            auto_invoke_table,
            custom_instructions,
            skills_prefix,
            skills_list,
            profile.id
        )
    }

    /// Generate copilot-instructions.md content (adapted for GitHub Copilot)
    fn generate_copilot_instructions(profile: &Profile) -> String {
        let content = Self::generate_ai_instructions_content(profile);
        content.copilot_md
    }

    /// Install Main-Profile configuration for multiple targets
    fn install_main_profile_config_for_targets(
        targets: &[DeployTarget],
    ) -> Result<(Option<bool>, Option<bool>, Option<String>)> {
        let mut instructions_installed = None;
        let mut output_style_installed = None;

        for target in targets {
            let config_dir = TargetPaths::user_config_dir(*target)?;
            fs::create_dir_all(&config_dir)?;

            // Install Instructions
            let instructions = InstructionsManager::get()?;
            if !instructions.content.is_empty() {
                let target_path = TargetPaths::instructions_path(*target, &config_dir);
                fs::write(&target_path, &instructions.content)?;
                instructions_installed = Some(true);
            }

            // ClaudeCode-specific: install settings.json and output styles
            if *target == DeployTarget::ClaudeCode {
                let settings = Settings::get()?;
                let settings_target = config_dir.join("settings.json");
                let settings_json = serde_json::to_string_pretty(&settings)?;
                fs::write(&settings_target, settings_json)?;

                if let Ok(Some(style)) = OutputStyles::get_active() {
                    let styles_dir = config_dir.join("output-styles");
                    fs::create_dir_all(&styles_dir)?;

                    let style_content = Self::generate_output_style_content(&style);
                    let style_path = styles_dir.join(format!("{}.md", style.id));
                    fs::write(&style_path, style_content)?;
                    output_style_installed = Some(style.name.clone());
                }
            }
        }

        let settings_installed = if targets.contains(&DeployTarget::ClaudeCode) {
            Some(true)
        } else {
            None
        };

        Ok((
            instructions_installed,
            settings_installed,
            output_style_installed,
        ))
    }

    /// Generate output style file content with frontmatter
    fn generate_output_style_content(style: &OutputStyle) -> String {
        format!(
            "---\nname: {}\ndescription: {}\nkeepCodingInstructions: {}\n---\n\n{}",
            style.name, style.description, style.keep_coding_instructions, style.content
        )
    }

    /// Install a single skill to a target skills directory
    fn install_skill(skill_id: &str, skills_target: &Path) -> Result<()> {
        let skill_source = Skills::get_skill_path(skill_id)?;
        let skill_target = skills_target.join(skill_id);

        // Remove existing if present
        if skill_target.exists() {
            fs::remove_dir_all(&skill_target)?;
        }

        // Copy skill directory
        Self::copy_dir_recursive(&skill_source, &skill_target)?;

        Ok(())
    }

    /// Uninstall a profile from a target path.
    /// If `targets` is `None`, removes ALL known target artifacts.
    pub fn uninstall(target_path: &Path, targets: Option<&[DeployTarget]>) -> Result<()> {
        let effective_targets = targets.unwrap_or_else(|| DeployTarget::all());

        // Check if any installation exists
        let has_any = effective_targets.iter().any(|target| {
            let config_dir = TargetPaths::project_config_dir(*target, target_path);
            let instructions = TargetPaths::instructions_path(*target, target_path);
            config_dir.exists() || instructions.exists()
        }) || target_path.join(".claude-plugin").exists();

        if !has_any {
            return Err(RhinolabsError::ConfigError(format!(
                "No profile installation found at {}",
                target_path.display()
            )));
        }

        for target in effective_targets {
            // Remove target config directory (skills)
            let config_dir = TargetPaths::project_config_dir(*target, target_path);
            if config_dir.exists() {
                fs::remove_dir_all(&config_dir)?;
            }

            // Remove instructions file if generated by rhinolabs-ai
            let instructions_path = TargetPaths::instructions_path(*target, target_path);
            if instructions_path.exists() {
                if let Ok(content) = fs::read_to_string(&instructions_path) {
                    if content.contains("rhinolabs-ai") {
                        fs::remove_file(&instructions_path)?;
                    }
                }
            }

            // ClaudeCode-specific cleanup
            if *target == DeployTarget::ClaudeCode {
                // Remove .claude-plugin directory
                let plugin_dir = target_path.join(".claude-plugin");
                if plugin_dir.exists() {
                    fs::remove_dir_all(&plugin_dir)?;
                }

                // Remove .github/copilot-instructions.md only if generated by us
                let copilot_md = target_path.join(".github").join("copilot-instructions.md");
                if copilot_md.exists() {
                    if let Ok(content) = fs::read_to_string(&copilot_md) {
                        if content.contains("Generated by rhinolabs-ai")
                            || content.contains("Auto-generated by rhinolabs-ai")
                        {
                            fs::remove_file(&copilot_md)?;
                            // Remove .github dir if empty
                            let github_dir = target_path.join(".github");
                            if github_dir.exists() {
                                if let Ok(entries) = fs::read_dir(&github_dir) {
                                    if entries.count() == 0 {
                                        fs::remove_dir(&github_dir)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Update an installed profile (re-install with latest skill versions)
    pub fn update_installed(
        profile_id: &str,
        target_path: Option<&Path>,
        targets: Option<&[DeployTarget]>,
    ) -> Result<ProfileInstallResult> {
        // Simply re-install - install_skill already handles removing existing
        Self::install(profile_id, target_path, targets)
    }

    /// Copy directory recursively
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                // Skip .git directory
                if entry.file_name() == ".git" {
                    continue;
                }
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    // ============================================
    // Profile by Skill Lookup (for Skills module)
    // ============================================

    /// Get a map of skill_id -> profile_ids for all assignments
    pub fn get_skill_profile_map() -> Result<HashMap<String, Vec<String>>> {
        let config = Self::load_config()?;
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for profile in config.profiles {
            for skill_id in profile.skills {
                map.entry(skill_id).or_default().push(profile.id.clone());
            }
        }

        Ok(map)
    }

    // ============================================
    // Profile Instructions (per-profile CLAUDE.md)
    // ============================================

    /// Get the path for a profile's instructions file
    /// For "main" profile, uses the same path as InstructionsManager (CLAUDE.md)
    /// For other profiles, uses ~/.config/rhinolabs-ai/profile-instructions/{id}.md
    pub fn get_instructions_path(profile_id: &str) -> Result<PathBuf> {
        if profile_id == "main" {
            // Main profile shares the same CLAUDE.md as InstructionsManager
            InstructionsManager::get_path()
        } else {
            Ok(Self::config_dir()?
                .join("profile-instructions")
                .join(format!("{}.md", profile_id)))
        }
    }

    /// Get profile instructions (from file if exists, otherwise from profile config)
    /// For "main" profile, uses InstructionsManager directly
    pub fn get_instructions(profile_id: &str) -> Result<String> {
        if profile_id == "main" {
            // Main profile uses InstructionsManager
            let instructions = InstructionsManager::get()?;
            return Ok(instructions.content);
        }

        let path = Self::get_instructions_path(profile_id)?;
        if path.exists() {
            Ok(fs::read_to_string(&path)?)
        } else {
            let profile = Self::get(profile_id)?.ok_or_else(|| {
                RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
            })?;
            Ok(profile.instructions.unwrap_or_default())
        }
    }

    /// Update profile instructions (writes to file and updates profile config)
    /// For "main" profile, uses InstructionsManager directly
    pub fn update_instructions(profile_id: &str, content: &str) -> Result<()> {
        // Verify profile exists
        Self::get(profile_id)?.ok_or_else(|| {
            RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
        })?;

        if profile_id == "main" {
            // Main profile uses InstructionsManager
            if content.trim().is_empty() {
                return Err(RhinolabsError::ConfigError(
                    "Instructions content cannot be empty".into(),
                ));
            }
            InstructionsManager::update(content)?;
        } else {
            // Other profiles use their own file
            let path = Self::get_instructions_path(profile_id)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, content)?;

            // Update profile config
            let mut config = Self::load_config()?;
            if let Some(profile) = config.profiles.iter_mut().find(|p| p.id == profile_id) {
                profile.instructions = if content.is_empty() {
                    None
                } else {
                    Some(content.to_string())
                };
                profile.updated_at = chrono::Utc::now().to_rfc3339();
                Self::save_config(&config)?;
            }
        }

        Ok(())
    }

    /// Ensure instructions file exists for editing (creates if needed)
    pub fn ensure_instructions_file(profile_id: &str) -> Result<PathBuf> {
        let path = Self::get_instructions_path(profile_id)?;

        // Create directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // If file doesn't exist, create with current content
        if !path.exists() {
            let content = Self::get_instructions(profile_id)?;
            fs::write(&path, &content)?;
        }

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TestEnv as BaseTestEnv, ENV_MUTEX};
    use crate::DeployTarget;

    struct TestEnv {
        base: BaseTestEnv,
        config_dir: tempfile::TempDir,
    }

    impl TestEnv {
        fn new() -> Self {
            let base = BaseTestEnv::new();
            let config_dir = tempfile::tempdir().expect("Failed to create temp config dir");

            // Override config dir for tests
            std::env::set_var("RHINOLABS_CONFIG_PATH", config_dir.path());

            TestEnv { base, config_dir }
        }

        fn plugin_dir(&self) -> PathBuf {
            self.base.plugin_dir()
        }

        fn config_path(&self) -> PathBuf {
            self.config_dir.path().join("profiles.json")
        }

        fn skills_dir(&self) -> PathBuf {
            self.plugin_dir().join("skills")
        }

        fn setup_skills_dir(&self) {
            fs::create_dir_all(self.skills_dir()).expect("Failed to create skills dir");
        }

        #[allow(dead_code)]
        fn create_skill(&self, id: &str, name: &str, description: &str, content: &str) {
            let skill_dir = self.skills_dir().join(id);
            fs::create_dir_all(&skill_dir).expect("Failed to create skill dir");
            let skill_content = format!(
                "---\nname: {}\ndescription: {}\n---\n\n{}",
                name, description, content
            );
            fs::write(skill_dir.join("SKILL.md"), skill_content)
                .expect("Failed to write skill file");
        }

        #[allow(dead_code)]
        fn create_profiles_config(&self, config: &ProfilesConfig) {
            let config_path = self.config_path();
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create config parent dir");
            }
            let content = serde_json::to_string_pretty(config).expect("Failed to serialize config");
            fs::write(&config_path, content).expect("Failed to write config");
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            std::env::remove_var("RHINOLABS_CONFIG_PATH");
        }
    }

    #[test]
    fn test_profile_type_default() {
        assert_eq!(ProfileType::default(), ProfileType::Project);
    }

    #[test]
    fn test_profiles_config_default() {
        let config = ProfilesConfig::default();
        assert!(config.profiles.is_empty());
        assert!(config.default_user_profile.is_none());
    }

    #[test]
    fn test_create_profile() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Override config path for test
        let config_path = env.config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let input = CreateProfileInput {
            id: "test-profile".to_string(),
            name: "Test Profile".to_string(),
            description: "A test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            instructions: None,
            generate_copilot: true,
            generate_agents: false,
        };

        // Note: This test would need proper config path override mechanism
        // For now, testing the serialization
        let now = chrono::Utc::now().to_rfc3339();
        let profile = Profile {
            id: input.id.clone(),
            name: input.name.clone(),
            description: input.description.clone(),
            profile_type: input.profile_type.clone(),
            skills: input.skills.clone(),
            auto_invoke_rules: Vec::new(),
            instructions: Some("# Test Instructions".to_string()),
            generate_copilot: input.generate_copilot,
            generate_agents: input.generate_agents,
            created_at: now.clone(),
            updated_at: now,
        };

        assert_eq!(profile.id, "test-profile");
        assert_eq!(profile.name, "Test Profile");
        assert_eq!(profile.profile_type, ProfileType::Project);
        assert_eq!(profile.skills.len(), 1);
        assert!(profile.instructions.is_some());
        assert!(profile.generate_copilot);
        assert!(!profile.generate_agents);
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::User,
            skills: vec!["skill-a".to_string(), "skill-b".to_string()],
            auto_invoke_rules: vec![AutoInvokeRule {
                skill_id: "skill-a".to_string(),
                trigger: "Editing .tsx files".to_string(),
                description: "React patterns".to_string(),
            }],
            instructions: Some("# My Instructions".to_string()),
            generate_copilot: true,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&profile).unwrap();
        assert!(json.contains("\"profileType\":\"user\""));
        assert!(json.contains("\"skills\":[\"skill-a\",\"skill-b\"]"));
        assert!(json.contains("\"autoInvokeRules\""));
        assert!(json.contains("\"instructions\""));
        assert!(json.contains("\"generateCopilot\":true"));

        let deserialized: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, profile.id);
        assert_eq!(deserialized.profile_type, ProfileType::User);
        assert_eq!(deserialized.auto_invoke_rules.len(), 1);
        assert!(deserialized.instructions.is_some());
    }

    #[test]
    fn test_profile_install_result_serialization() {
        let result = ProfileInstallResult {
            profile_id: "react-stack".to_string(),
            profile_name: "React Stack".to_string(),
            target_path: "/project/.claude/skills".to_string(),
            skills_installed: vec!["react-19".to_string(), "typescript".to_string()],
            skills_failed: vec![SkillInstallError {
                skill_id: "missing-skill".to_string(),
                error: "Skill not found".to_string(),
            }],
            instructions_installed: None,
            settings_installed: None,
            output_style_installed: None,
            targets_installed: vec![DeployTarget::ClaudeCode],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"profileId\":\"react-stack\""));
        assert!(json.contains("\"skillsInstalled\""));
        assert!(json.contains("\"skillsFailed\""));
        // Optional fields should not appear when None
        assert!(!json.contains("\"instructionsInstalled\""));
    }

    #[test]
    fn test_main_profile_install_result_serialization() {
        let result = ProfileInstallResult {
            profile_id: "main".to_string(),
            profile_name: "Main Profile".to_string(),
            target_path: "~/.claude/skills".to_string(),
            skills_installed: vec!["general-skill".to_string()],
            skills_failed: vec![],
            instructions_installed: Some(true),
            settings_installed: Some(true),
            output_style_installed: Some("Rhinolabs".to_string()),
            targets_installed: vec![DeployTarget::ClaudeCode],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"instructionsInstalled\":true"));
        assert!(json.contains("\"settingsInstalled\":true"));
        assert!(json.contains("\"outputStyleInstalled\":\"Rhinolabs\""));
    }

    #[test]
    fn test_claude_project_dir() {
        let path = Path::new("/home/user/myproject");
        let claude_dir = Profiles::claude_project_dir(path);
        assert_eq!(claude_dir, PathBuf::from("/home/user/myproject/.claude"));
    }

    #[test]
    fn test_copy_dir_recursive() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        // Create test structure
        fs::write(source_dir.path().join("file1.txt"), "content1").unwrap();
        fs::create_dir(source_dir.path().join("subdir")).unwrap();
        fs::write(
            source_dir.path().join("subdir").join("file2.txt"),
            "content2",
        )
        .unwrap();

        // Create .git directory (should be skipped)
        fs::create_dir(source_dir.path().join(".git")).unwrap();
        fs::write(source_dir.path().join(".git").join("config"), "git config").unwrap();

        let result = Profiles::copy_dir_recursive(source_dir.path(), target_dir.path());
        assert!(result.is_ok());

        assert!(target_dir.path().join("file1.txt").exists());
        assert!(target_dir.path().join("subdir").join("file2.txt").exists());
        assert!(!target_dir.path().join(".git").exists()); // .git should be skipped
    }

    // ============================================
    // Instructions Tests
    // ============================================

    #[test]
    fn test_generate_template_instructions_without_skills() {
        let content = Profiles::generate_template_instructions(
            "React Stack",
            "React 19 with TypeScript",
            &[],
        );

        // Should contain profile name
        assert!(content.contains("# React Stack - Project Instructions"));
        // Should contain description
        assert!(content.contains("React 19 with TypeScript"));
        // Should contain empty skills placeholder
        assert!(content.contains("<!-- Add skills to this profile -->"));
        // Should contain standard sections
        assert!(content.contains("## Rules"));
        assert!(content.contains("## Code Standards"));
        assert!(content.contains("## Skills Auto-invoke"));
    }

    #[test]
    fn test_generate_template_instructions_with_skills() {
        let skills = vec!["react-19".to_string(), "typescript".to_string()];
        let content = Profiles::generate_template_instructions(
            "React Stack",
            "React 19 with TypeScript",
            &skills,
        );

        // Should contain profile name
        assert!(content.contains("# React Stack - Project Instructions"));
        // Should contain skills in table (skill IDs in backticks)
        assert!(content.contains("`react-19`"));
        assert!(content.contains("`typescript`"));
        // Should NOT contain empty placeholder
        assert!(!content.contains("<!-- Add skills to this profile -->"));
    }

    #[test]
    fn test_auto_invoke_rule_serialization() {
        let rule = AutoInvokeRule {
            skill_id: "react-19".to_string(),
            trigger: "Editing .tsx/.jsx files".to_string(),
            description: "React 19 patterns and hooks".to_string(),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("\"skillId\":\"react-19\""));
        assert!(json.contains("\"trigger\":\"Editing .tsx/.jsx files\""));
        assert!(json.contains("\"description\":\"React 19 patterns and hooks\""));

        let deserialized: AutoInvokeRule = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.skill_id, "react-19");
    }

    // ============================================
    // Multi-Target Deployment Tests
    // ============================================

    #[test]
    fn test_generate_instructions_for_target_claudecode_uses_claude_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string(), "typescript".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content =
            Profiles::generate_instructions_for_target(&profile, DeployTarget::ClaudeCode);
        assert!(content.contains(".claude/skills/"));
        assert!(content.contains(".claude/skills/react-19/SKILL.md"));
        assert!(content.contains(".claude/skills/typescript/SKILL.md"));
        assert!(!content.contains(".agents/skills/"));
        assert!(!content.contains(".agent/skills/"));
        assert!(!content.contains(".opencode/skills/"));
    }

    #[test]
    fn test_generate_instructions_for_target_amp_uses_agents_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content = Profiles::generate_instructions_for_target(&profile, DeployTarget::Amp);
        assert!(content.contains(".agents/skills/"));
        assert!(content.contains(".agents/skills/react-19/SKILL.md"));
        assert!(!content.contains(".claude/skills/"));
    }

    #[test]
    fn test_generate_instructions_for_target_antigravity_uses_agent_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content =
            Profiles::generate_instructions_for_target(&profile, DeployTarget::Antigravity);
        assert!(content.contains(".agent/skills/"));
        assert!(content.contains(".agent/skills/react-19/SKILL.md"));
        assert!(!content.contains(".agents/skills/"));
        assert!(!content.contains(".claude/skills/"));
    }

    #[test]
    fn test_generate_instructions_for_target_opencode_uses_opencode_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content = Profiles::generate_instructions_for_target(&profile, DeployTarget::OpenCode);
        assert!(content.contains(".opencode/skills/"));
        assert!(content.contains(".opencode/skills/react-19/SKILL.md"));
        assert!(!content.contains(".claude/skills/"));
    }

    #[test]
    fn test_generate_instructions_for_target_includes_auto_invoke_rules_with_correct_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: vec![AutoInvokeRule {
                skill_id: "react-19".to_string(),
                trigger: "Editing .tsx files".to_string(),
                description: "React 19 patterns".to_string(),
            }],
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content = Profiles::generate_instructions_for_target(&profile, DeployTarget::Amp);
        assert!(content.contains("Auto-invoke Skills"));
        assert!(content.contains(".agents/skills/react-19/SKILL.md"));
        assert!(!content.contains(".claude/skills/react-19/SKILL.md"));
    }

    #[test]
    fn test_generate_instructions_for_target_includes_custom_instructions() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec![],
            auto_invoke_rules: Vec::new(),
            instructions: Some("Use strict TypeScript always.".to_string()),
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let content = Profiles::generate_instructions_for_target(&profile, DeployTarget::Amp);
        assert!(content.contains("Use strict TypeScript always."));
        assert!(content.contains("Project Standards"));
    }

    #[test]
    fn test_generate_instructions_for_target_contains_rhinolabs_marker() {
        let profile = Profile {
            id: "my-profile".to_string(),
            name: "My Profile".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        // All targets should include the rhinolabs-ai marker (used by uninstall)
        for target in DeployTarget::all() {
            let content = Profiles::generate_instructions_for_target(&profile, *target);
            assert!(
                content.contains("rhinolabs-ai"),
                "Target {:?} is missing rhinolabs-ai marker",
                target
            );
            assert!(
                content.contains("my-profile"),
                "Target {:?} is missing profile id",
                target
            );
        }
    }

    #[test]
    fn test_generate_instructions_for_target_each_target_uses_different_prefix() {
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let claude = Profiles::generate_instructions_for_target(&profile, DeployTarget::ClaudeCode);
        let amp = Profiles::generate_instructions_for_target(&profile, DeployTarget::Amp);
        let antigravity =
            Profiles::generate_instructions_for_target(&profile, DeployTarget::Antigravity);
        let opencode = Profiles::generate_instructions_for_target(&profile, DeployTarget::OpenCode);

        // Each should contain its own prefix and NOT contain others
        assert!(claude.contains(".claude/skills/skill-a/SKILL.md"));
        assert!(amp.contains(".agents/skills/skill-a/SKILL.md"));
        assert!(antigravity.contains(".agent/skills/skill-a/SKILL.md"));
        assert!(opencode.contains(".opencode/skills/skill-a/SKILL.md"));

        // Cross-check: no prefix leaks into wrong target
        assert!(!amp.contains(".claude/skills/"));
        assert!(!antigravity.contains(".claude/skills/"));
        assert!(!opencode.contains(".claude/skills/"));
    }

    #[test]
    fn test_install_project_profile_for_targets_amp_creates_agents_md() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let targets = [DeployTarget::Amp];
        let result =
            Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets);
        assert!(result.is_ok());

        // AGENTS.md should exist (Amp's instructions file)
        let agents_md = target_dir.path().join("AGENTS.md");
        assert!(agents_md.exists(), "AGENTS.md should be created for Amp");

        let content = fs::read_to_string(&agents_md).unwrap();
        assert!(content.contains(".agents/skills/"));
        assert!(!content.contains(".claude/skills/"));

        // CLAUDE.md should NOT exist
        assert!(
            !target_dir.path().join("CLAUDE.md").exists(),
            "CLAUDE.md should NOT be created when only deploying to Amp"
        );

        // .claude-plugin should NOT exist (ClaudeCode-specific)
        assert!(
            !target_dir.path().join(".claude-plugin").exists(),
            ".claude-plugin should NOT be created for Amp"
        );
    }

    #[test]
    fn test_install_project_profile_for_targets_multi_target() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test profile".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let targets = [DeployTarget::ClaudeCode, DeployTarget::Amp];
        let result =
            Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets);
        assert!(result.is_ok());

        // Both CLAUDE.md and AGENTS.md should exist
        assert!(target_dir.path().join("CLAUDE.md").exists());
        assert!(target_dir.path().join("AGENTS.md").exists());

        // Each should contain its own prefix
        let claude_content = fs::read_to_string(target_dir.path().join("CLAUDE.md")).unwrap();
        let agents_content = fs::read_to_string(target_dir.path().join("AGENTS.md")).unwrap();

        assert!(claude_content.contains(".claude/skills/"));
        assert!(agents_content.contains(".agents/skills/"));

        // .claude-plugin should exist (ClaudeCode included)
        assert!(target_dir.path().join(".claude-plugin").exists());
    }

    #[test]
    fn test_install_project_profile_for_targets_claudecode_creates_plugin_manifest() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "my-proj".to_string(),
            name: "My Project".to_string(),
            description: "A test project".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let targets = [DeployTarget::ClaudeCode];
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets)
            .unwrap();

        let manifest_path = target_dir.path().join(".claude-plugin").join("plugin.json");
        assert!(manifest_path.exists());

        let manifest: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&manifest_path).unwrap()).unwrap();
        assert_eq!(manifest["profile"]["id"].as_str().unwrap(), "my-proj");
        assert_eq!(manifest["profile"]["name"].as_str().unwrap(), "My Project");
    }

    #[test]
    fn test_install_project_profile_for_targets_copilot_only_for_claudecode() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec![],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: true,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        // Install only to Amp → no copilot file
        let targets = [DeployTarget::Amp];
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets)
            .unwrap();
        assert!(
            !target_dir.path().join(".github").exists(),
            "copilot-instructions.md should NOT be created for Amp target"
        );

        // Install to ClaudeCode → copilot file should be created
        let target_dir2 = tempfile::tempdir().unwrap();
        let targets = [DeployTarget::ClaudeCode];
        Profiles::install_project_profile_for_targets(target_dir2.path(), &profile, &targets)
            .unwrap();
        assert!(
            target_dir2
                .path()
                .join(".github")
                .join("copilot-instructions.md")
                .exists(),
            "copilot-instructions.md should be created when ClaudeCode target + generate_copilot"
        );
    }

    #[test]
    fn test_install_project_profile_for_targets_generate_agents_supplementary() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: true, // generate supplementary AGENTS.md
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        // Install only to ClaudeCode with generate_agents=true
        // Should still create AGENTS.md as supplementary cross-reference
        let targets = [DeployTarget::ClaudeCode];
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets)
            .unwrap();

        assert!(target_dir.path().join("CLAUDE.md").exists());
        assert!(
            target_dir.path().join("AGENTS.md").exists(),
            "Supplementary AGENTS.md should be created when generate_agents=true"
        );
    }

    #[test]
    fn test_install_project_profile_for_targets_no_supplementary_agents_when_amp_target() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: true,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        // When Amp IS in targets, generate_agents should NOT create supplementary AGENTS.md
        // (because Amp's primary instructions file IS AGENTS.md already)
        let targets = [DeployTarget::ClaudeCode, DeployTarget::Amp];
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets)
            .unwrap();

        // AGENTS.md should exist (from Amp target), but should use .agents/skills/ prefix
        let agents_content = fs::read_to_string(target_dir.path().join("AGENTS.md")).unwrap();
        assert!(agents_content.contains(".agents/skills/"));
    }

    #[test]
    fn test_install_project_profile_does_not_overwrite_existing_instructions() {
        let target_dir = tempfile::tempdir().unwrap();

        // Pre-create a custom AGENTS.md
        fs::write(
            target_dir.path().join("AGENTS.md"),
            "# My Custom AGENTS.md\nDo not overwrite me.",
        )
        .unwrap();

        let profile = Profile {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Desc".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["skill-a".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let targets = [DeployTarget::Amp];
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, &targets)
            .unwrap();

        // The pre-existing file should NOT be overwritten
        let content = fs::read_to_string(target_dir.path().join("AGENTS.md")).unwrap();
        assert!(
            content.contains("Do not overwrite me."),
            "Existing instructions file should not be overwritten"
        );
    }

    #[test]
    fn test_install_project_profile_all_four_targets() {
        let target_dir = tempfile::tempdir().unwrap();
        let profile = Profile {
            id: "full".to_string(),
            name: "Full".to_string(),
            description: "All targets".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string()],
            auto_invoke_rules: Vec::new(),
            instructions: None,
            generate_copilot: false,
            generate_agents: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let targets = DeployTarget::all();
        Profiles::install_project_profile_for_targets(target_dir.path(), &profile, targets)
            .unwrap();

        // All four instructions files should exist
        assert!(target_dir.path().join("CLAUDE.md").exists());
        assert!(target_dir.path().join("AGENTS.md").exists());
        assert!(target_dir.path().join("GEMINI.md").exists());
        assert!(target_dir.path().join("opencode.json").exists());

        // Only ClaudeCode gets .claude-plugin
        assert!(target_dir.path().join(".claude-plugin").exists());
    }

    #[test]
    fn test_uninstall_removes_target_artifacts() {
        let target_dir = tempfile::tempdir().unwrap();

        // Simulate installed artifacts for Amp
        let agents_dir = target_dir.path().join(".agents");
        fs::create_dir_all(agents_dir.join("skills").join("react-19")).unwrap();
        fs::write(
            agents_dir.join("skills").join("react-19").join("SKILL.md"),
            "# React 19",
        )
        .unwrap();
        fs::write(
            target_dir.path().join("AGENTS.md"),
            "# Instructions\n*Installed by rhinolabs-ai*",
        )
        .unwrap();

        let targets = [DeployTarget::Amp];
        let result = Profiles::uninstall(target_dir.path(), Some(&targets));
        assert!(result.is_ok());

        // Amp artifacts should be removed
        assert!(!agents_dir.exists(), ".agents/ should be removed");
        assert!(
            !target_dir.path().join("AGENTS.md").exists(),
            "AGENTS.md should be removed (contains rhinolabs-ai marker)"
        );
    }

    #[test]
    fn test_uninstall_specific_target_preserves_others() {
        let target_dir = tempfile::tempdir().unwrap();

        // Install artifacts for BOTH Claude and Amp
        let claude_dir = target_dir.path().join(".claude");
        let agents_dir = target_dir.path().join(".agents");
        fs::create_dir_all(claude_dir.join("skills")).unwrap();
        fs::create_dir_all(agents_dir.join("skills")).unwrap();
        fs::write(
            target_dir.path().join("CLAUDE.md"),
            "# Instructions\n*Installed by rhinolabs-ai*",
        )
        .unwrap();
        fs::write(
            target_dir.path().join("AGENTS.md"),
            "# Instructions\n*Installed by rhinolabs-ai*",
        )
        .unwrap();
        // Also create .claude-plugin for ClaudeCode
        let plugin_dir = target_dir.path().join(".claude-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(plugin_dir.join("plugin.json"), "{}").unwrap();

        // Uninstall ONLY Amp
        let targets = [DeployTarget::Amp];
        Profiles::uninstall(target_dir.path(), Some(&targets)).unwrap();

        // Amp artifacts should be gone
        assert!(!agents_dir.exists());
        assert!(!target_dir.path().join("AGENTS.md").exists());

        // Claude artifacts should be PRESERVED
        assert!(
            claude_dir.exists(),
            ".claude/ should still exist after uninstalling only Amp"
        );
        assert!(
            target_dir.path().join("CLAUDE.md").exists(),
            "CLAUDE.md should still exist after uninstalling only Amp"
        );
        assert!(
            plugin_dir.exists(),
            ".claude-plugin/ should still exist after uninstalling only Amp"
        );
    }

    #[test]
    fn test_uninstall_none_targets_removes_all() {
        let target_dir = tempfile::tempdir().unwrap();

        // Install artifacts for ALL targets
        fs::create_dir_all(target_dir.path().join(".claude").join("skills")).unwrap();
        fs::create_dir_all(target_dir.path().join(".agents").join("skills")).unwrap();
        fs::create_dir_all(target_dir.path().join(".agent").join("skills")).unwrap();
        fs::create_dir_all(target_dir.path().join(".opencode").join("skills")).unwrap();
        fs::write(
            target_dir.path().join("CLAUDE.md"),
            "rhinolabs-ai generated",
        )
        .unwrap();
        fs::write(
            target_dir.path().join("AGENTS.md"),
            "rhinolabs-ai generated",
        )
        .unwrap();
        fs::write(
            target_dir.path().join("GEMINI.md"),
            "rhinolabs-ai generated",
        )
        .unwrap();
        fs::write(
            target_dir.path().join("opencode.json"),
            "rhinolabs-ai generated",
        )
        .unwrap();

        // Uninstall with None (= remove all)
        Profiles::uninstall(target_dir.path(), None).unwrap();

        assert!(!target_dir.path().join(".claude").exists());
        assert!(!target_dir.path().join(".agents").exists());
        assert!(!target_dir.path().join(".agent").exists());
        assert!(!target_dir.path().join(".opencode").exists());
        assert!(!target_dir.path().join("CLAUDE.md").exists());
        assert!(!target_dir.path().join("AGENTS.md").exists());
        assert!(!target_dir.path().join("GEMINI.md").exists());
        assert!(!target_dir.path().join("opencode.json").exists());
    }

    #[test]
    fn test_uninstall_preserves_non_rhinolabs_instructions() {
        let target_dir = tempfile::tempdir().unwrap();

        // Create a CLAUDE.md that does NOT contain rhinolabs-ai marker
        fs::write(
            target_dir.path().join("CLAUDE.md"),
            "# My custom instructions\nNothing auto-generated here.",
        )
        .unwrap();
        // Need at least one artifact to pass the "has_any" check
        fs::create_dir_all(target_dir.path().join(".claude").join("skills")).unwrap();

        let targets = [DeployTarget::ClaudeCode];
        Profiles::uninstall(target_dir.path(), Some(&targets)).unwrap();

        // The .claude dir should be removed (it's a config dir)
        assert!(!target_dir.path().join(".claude").exists());

        // But the custom CLAUDE.md should be PRESERVED (no rhinolabs-ai marker)
        assert!(
            target_dir.path().join("CLAUDE.md").exists(),
            "Custom CLAUDE.md without rhinolabs-ai marker should be preserved"
        );
    }

    #[test]
    fn test_uninstall_claudecode_removes_plugin_dir() {
        let target_dir = tempfile::tempdir().unwrap();

        // Create .claude-plugin
        let plugin_dir = target_dir.path().join(".claude-plugin");
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(plugin_dir.join("plugin.json"), "{}").unwrap();

        let targets = [DeployTarget::ClaudeCode];
        Profiles::uninstall(target_dir.path(), Some(&targets)).unwrap();

        assert!(
            !plugin_dir.exists(),
            ".claude-plugin should be removed on ClaudeCode uninstall"
        );
    }

    #[test]
    fn test_uninstall_no_artifacts_returns_error() {
        let target_dir = tempfile::tempdir().unwrap();

        // Empty directory — nothing to uninstall
        let result = Profiles::uninstall(target_dir.path(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_template_instructions_for_target_uses_custom_prefix() {
        let content = Profiles::generate_template_instructions_for_target(
            "React Stack",
            "React 19 with TypeScript",
            &["react-19".to_string()],
            ".agents/skills",
        );

        assert!(content.contains(".agents/skills/react-19/SKILL.md"));
        assert!(!content.contains(".claude/skills/"));
    }

    #[test]
    fn test_generate_template_instructions_uses_claude_prefix_by_default() {
        let content = Profiles::generate_template_instructions(
            "React Stack",
            "React 19 with TypeScript",
            &["react-19".to_string()],
        );

        assert!(content.contains(".claude/skills/react-19/SKILL.md"));
    }

    #[test]
    fn test_profile_install_result_targets_installed_serialization() {
        let result = ProfileInstallResult {
            profile_id: "test".to_string(),
            profile_name: "Test".to_string(),
            target_path: "/project".to_string(),
            skills_installed: vec!["react-19".to_string()],
            skills_failed: vec![],
            instructions_installed: Some(true),
            settings_installed: None,
            output_style_installed: None,
            targets_installed: vec![DeployTarget::ClaudeCode, DeployTarget::Amp],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"claude-code\""));
        assert!(json.contains("\"amp\""));

        let deserialized: ProfileInstallResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.targets_installed.len(), 2);
        assert!(deserialized
            .targets_installed
            .contains(&DeployTarget::ClaudeCode));
        assert!(deserialized.targets_installed.contains(&DeployTarget::Amp));
    }

    #[test]
    fn test_create_profile_input_with_skills() {
        let input = CreateProfileInput {
            id: "react-stack".to_string(),
            name: "React Stack".to_string(),
            description: "React 19 with TypeScript".to_string(),
            profile_type: ProfileType::Project,
            skills: vec!["react-19".to_string(), "typescript".to_string()],
            instructions: None,
            generate_copilot: true,
            generate_agents: false,
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"skills\":[\"react-19\",\"typescript\"]"));
        assert!(json.contains("\"generateCopilot\":true"));
        assert!(json.contains("\"generateAgents\":false"));
    }
}
