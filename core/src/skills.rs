use crate::{Paths, Profile, Profiles, Result, RhinolabsError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ============================================
// Skill Source Types
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SkillSourceType {
    Official,
    Marketplace,
    Community,
    Local,
}

/// Schema/structure used by a skill source repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum SkillSchema {
    /// Standard Agent Skills format: /skills/{name}/SKILL.md (agentskills.io)
    #[default]
    Standard,
    /// Skills.sh aggregator format - scrapes skills from skills.sh HTML
    #[serde(rename = "skills-sh")]
    SkillsSh,
    /// Custom schema - for future extensibility
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillSource {
    pub id: String,
    pub name: String,
    pub source_type: SkillSourceType,
    pub url: String,
    pub description: String,
    pub enabled: bool,
    /// If true, skills can be fetched automatically from this source.
    /// If false, the source is for browsing only (user must visit URL manually).
    #[serde(default = "default_fetchable")]
    pub fetchable: bool,
    /// The schema/structure used by this source (determines how to parse skills)
    #[serde(default)]
    pub schema: SkillSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_count: Option<u32>,
}

fn default_fetchable() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub source_id: String,
    pub source_name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<u32>,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallSkillInput {
    pub source_id: String,
    pub skill_id: String,
}

// Default sources
impl SkillSource {
    pub fn default_sources() -> Vec<SkillSource> {
        vec![
            SkillSource {
                id: "anthropic-official".to_string(),
                name: "Anthropic Official".to_string(),
                source_type: SkillSourceType::Official,
                url: "https://github.com/anthropics/skills".to_string(),
                description: "Official skills from Anthropic".to_string(),
                enabled: true,
                fetchable: true,
                schema: SkillSchema::Standard,
                skill_count: None,
            },
            SkillSource {
                id: "vercel-agent-skills".to_string(),
                name: "Vercel Agent Skills".to_string(),
                source_type: SkillSourceType::Marketplace,
                url: "https://github.com/vercel-labs/agent-skills".to_string(),
                description: "Agent skills from Vercel Labs".to_string(),
                enabled: true,
                fetchable: true,
                schema: SkillSchema::Standard,
                skill_count: None,
            },
            SkillSource {
                id: "awesome-claude-skills".to_string(),
                name: "Awesome Claude Skills".to_string(),
                source_type: SkillSourceType::Community,
                url: "https://github.com/travisvn/awesome-claude-skills".to_string(),
                description: "Curated list with links - browse README for skill repos".to_string(),
                enabled: true,
                fetchable: false,
                schema: SkillSchema::Custom, // Not applicable, browse only
                skill_count: None,
            },
        ]
    }
}

// ============================================
// Skill Category
// ============================================

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SkillCategory {
    Corporate,
    Backend,
    Frontend,
    Testing,
    AiSdk,
    Utilities,
    #[default]
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub category: SkillCategory,
    pub path: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub is_custom: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    pub is_modified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSkillInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub enabled: Option<bool>,
    pub category: Option<SkillCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkillFrontmatter {
    name: String,
    description: String,
}

/// Metadata for installed skills (tracks source and original content hash)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SkillMeta {
    source_id: Option<String>,
    source_name: Option<String>,
    original_hash: Option<String>,
}

/// Configuration for skill states (enabled/disabled)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SkillsConfig {
    disabled: Vec<String>,
    custom: Vec<String>,
    sources: Vec<SkillSource>,
    #[serde(default)]
    skill_meta: std::collections::HashMap<String, SkillMeta>,
    /// User-defined category mappings (skill_id -> category)
    /// Takes precedence over hardcoded category constants
    #[serde(default)]
    category_map: std::collections::HashMap<String, SkillCategory>,
}

/// Built-in skill categories
const CORPORATE_SKILLS: &[&str] = &[
    "rhinolabs-standards",
    "rhinolabs-architecture",
    "rhinolabs-security",
];
const BACKEND_SKILLS: &[&str] = &[];
const FRONTEND_SKILLS: &[&str] = &[
    "react-patterns",
    "typescript-best-practices",
    "tailwind-4",
    "zod-4",
    "zustand-5",
];
const TESTING_SKILLS: &[&str] = &["testing-strategies", "playwright"];
const AI_SDK_SKILLS: &[&str] = &["ai-sdk-core", "ai-sdk-react", "nextjs-integration"];
const UTILITIES_SKILLS: &[&str] = &["skill-creator"];

pub struct Skills;

impl Skills {
    /// Get the skills directory path
    fn skills_dir() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join("skills"))
    }

    /// Get the skills config file path
    fn config_path() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join(".skills-config.json"))
    }

    /// Load skills config
    fn load_config() -> Result<SkillsConfig> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(SkillsConfig::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: SkillsConfig = serde_json::from_str(&content)?;

        Ok(config)
    }

    /// Save skills config
    fn save_config(config: &SkillsConfig) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Determine the category of a skill by id
    /// Priority: 1) user-defined in category_map, 2) hardcoded constants, 3) Custom
    fn get_category(id: &str, config: &SkillsConfig) -> SkillCategory {
        // First check user-defined category_map
        if let Some(category) = config.category_map.get(id) {
            return category.clone();
        }

        // Fallback to hardcoded constants (built-in skills)
        if CORPORATE_SKILLS.contains(&id) {
            SkillCategory::Corporate
        } else if BACKEND_SKILLS.contains(&id) {
            SkillCategory::Backend
        } else if FRONTEND_SKILLS.contains(&id) {
            SkillCategory::Frontend
        } else if TESTING_SKILLS.contains(&id) {
            SkillCategory::Testing
        } else if AI_SDK_SKILLS.contains(&id) {
            SkillCategory::AiSdk
        } else if UTILITIES_SKILLS.contains(&id) {
            SkillCategory::Utilities
        } else {
            SkillCategory::Custom
        }
    }

    /// Parse frontmatter from a SKILL.md file
    fn parse_skill_file(content: &str) -> Result<(SkillFrontmatter, String)> {
        let content = content.trim();

        if !content.starts_with("---") {
            return Err(RhinolabsError::ConfigError(
                "Skill file must start with YAML frontmatter".into(),
            ));
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(RhinolabsError::ConfigError(
                "Invalid frontmatter format".into(),
            ));
        }

        let frontmatter_str = parts[1].trim();
        let markdown_content = parts[2].trim();

        let frontmatter: SkillFrontmatter = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| RhinolabsError::ConfigError(format!("Invalid YAML frontmatter: {}", e)))?;

        Ok((frontmatter, markdown_content.to_string()))
    }

    /// Generate SKILL.md content
    fn generate_skill_file(name: &str, description: &str, content: &str) -> String {
        format!(
            "---\nname: {}\ndescription: {}\n---\n\n{}",
            name, description, content
        )
    }

    /// Compute a simple hash of content for modification detection
    fn hash_content(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Load a skill from a directory
    fn load_from_dir(dir: &PathBuf, config: &SkillsConfig) -> Result<Skill> {
        let skill_file = dir.join("SKILL.md");

        if !skill_file.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "SKILL.md not found in {:?}",
                dir
            )));
        }

        let id = dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| RhinolabsError::ConfigError("Invalid skill directory name".into()))?
            .to_string();

        let file_content = fs::read_to_string(&skill_file)?;
        let (frontmatter, markdown_content) = Self::parse_skill_file(&file_content)?;

        let is_custom = config.custom.contains(&id);
        let enabled = !config.disabled.contains(&id);
        let category = if is_custom {
            // Custom skills can still have user-defined categories from category_map
            config
                .category_map
                .get(&id)
                .cloned()
                .unwrap_or(SkillCategory::Custom)
        } else {
            Self::get_category(&id, config)
        };

        // Get source info and modification status from meta
        let meta = config.skill_meta.get(&id);
        let source_id = meta.and_then(|m| m.source_id.clone());
        let source_name = meta.and_then(|m| m.source_name.clone());

        // Check if content was modified from original
        let current_hash = Self::hash_content(&file_content);
        let is_modified = meta
            .and_then(|m| m.original_hash.as_ref())
            .map(|orig| orig != &current_hash)
            .unwrap_or(false);

        // Get created_at for custom skills
        let created_at = if is_custom {
            skill_file
                .metadata()
                .ok()
                .and_then(|m| m.created().ok())
                .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
        } else {
            None
        };

        Ok(Skill {
            id,
            name: frontmatter.name,
            description: frontmatter.description,
            enabled,
            category,
            path: skill_file.display().to_string(),
            content: markdown_content,
            created_at,
            is_custom,
            source_id,
            source_name,
            is_modified,
        })
    }

    /// List all skills
    pub fn list() -> Result<Vec<Skill>> {
        let dir = Self::skills_dir()?;

        if !dir.exists() {
            return Ok(vec![]);
        }

        let config = Self::load_config()?;
        let mut skills = Vec::new();

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(skill) = Self::load_from_dir(&path, &config) {
                    skills.push(skill);
                }
            }
        }

        // Sort: corporate first, then by category, then by name
        skills.sort_by(|a, b| {
            let cat_order = |cat: &SkillCategory| match cat {
                SkillCategory::Corporate => 0,
                SkillCategory::Backend => 1,
                SkillCategory::Frontend => 2,
                SkillCategory::Testing => 3,
                SkillCategory::AiSdk => 4,
                SkillCategory::Utilities => 5,
                SkillCategory::Custom => 6,
            };

            cat_order(&a.category)
                .cmp(&cat_order(&b.category))
                .then_with(|| a.name.cmp(&b.name))
        });

        Ok(skills)
    }

    /// Get a specific skill by id
    pub fn get(id: &str) -> Result<Option<Skill>> {
        let dir = Self::skills_dir()?.join(id);

        if !dir.exists() {
            return Ok(None);
        }

        let config = Self::load_config()?;
        Ok(Some(Self::load_from_dir(&dir, &config)?))
    }

    /// Get the path to a skill's directory
    pub fn get_skill_path(id: &str) -> Result<std::path::PathBuf> {
        let dir = Self::skills_dir()?.join(id);

        if !dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                id
            )));
        }

        Ok(dir)
    }

    /// Create a new custom skill
    pub fn create(input: CreateSkillInput) -> Result<Skill> {
        let skills_dir = Self::skills_dir()?;
        let skill_dir = skills_dir.join(&input.id);

        if skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' already exists",
                input.id
            )));
        }

        // Create skill directory (and all parent directories)
        fs::create_dir_all(&skill_dir).map_err(|e| {
            RhinolabsError::ConfigError(format!(
                "Failed to create skill directory '{}': {}. Make sure you have write permissions.",
                skill_dir.display(),
                e
            ))
        })?;

        // Create SKILL.md
        let file_content =
            Self::generate_skill_file(&input.name, &input.description, &input.content);
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, &file_content).map_err(|e| {
            RhinolabsError::ConfigError(format!(
                "Failed to write skill file '{}': {}",
                skill_file.display(),
                e
            ))
        })?;

        // Update config to mark as custom and save category
        let mut config = Self::load_config().map_err(|e| {
            RhinolabsError::ConfigError(format!("Failed to load skills config: {}", e))
        })?;
        config.custom.push(input.id.clone());

        // Save category in category_map if not Custom (Custom is the default)
        if input.category != SkillCategory::Custom {
            config.category_map.insert(input.id.clone(), input.category);
        }

        Self::save_config(&config).map_err(|e| {
            RhinolabsError::ConfigError(format!("Failed to save skills config: {}", e))
        })?;

        // Return the created skill
        let config = Self::load_config()?;
        Self::load_from_dir(&skill_dir, &config)
    }

    /// Update an existing skill
    pub fn update(id: &str, input: UpdateSkillInput) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                id
            )));
        }

        let config = Self::load_config()?;
        let mut skill = Self::load_from_dir(&skill_dir, &config)?;

        // Update fields
        if let Some(name) = input.name {
            skill.name = name;
        }
        if let Some(description) = input.description {
            skill.description = description;
        }
        if let Some(content) = input.content {
            skill.content = content;
        }

        // Write updated SKILL.md
        let file_content =
            Self::generate_skill_file(&skill.name, &skill.description, &skill.content);
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, &file_content)?;

        // Handle enabled toggle
        if let Some(enabled) = input.enabled {
            Self::toggle(id, enabled)?;
        }

        // Handle category change
        if let Some(category) = input.category {
            Self::set_category(id, category)?;
        }

        Ok(())
    }

    /// Toggle skill enabled state
    pub fn toggle(id: &str, enabled: bool) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                id
            )));
        }

        let mut config = Self::load_config()?;

        if enabled {
            config.disabled.retain(|s| s != id);
        } else if !config.disabled.contains(&id.to_string()) {
            config.disabled.push(id.to_string());
        }

        Self::save_config(&config)
    }

    /// Delete a custom skill
    pub fn delete(id: &str) -> Result<()> {
        let config = Self::load_config()?;

        // Allow deletion if skill is custom OR has source metadata (installed from source)
        let is_custom = config.custom.contains(&id.to_string());
        let has_source = config
            .skill_meta
            .get(id)
            .map(|m| m.source_id.is_some())
            .unwrap_or(false);

        if !is_custom && !has_source {
            return Err(RhinolabsError::ConfigError(format!(
                "Cannot delete built-in skill '{}'. You can only disable it.",
                id
            )));
        }

        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                id
            )));
        }

        // Remove directory
        fs::remove_dir_all(&skill_dir)?;

        // Update config
        let mut config = Self::load_config()?;
        config.custom.retain(|s| s != id);
        config.disabled.retain(|s| s != id);
        config.skill_meta.remove(id);
        config.category_map.remove(id);
        Self::save_config(&config)?;

        Ok(())
    }

    // ============================================
    // Profile-based Skill Queries
    // ============================================

    /// List skills assigned to a specific profile
    pub fn list_by_profile(profile_id: &str) -> Result<Vec<Skill>> {
        // Get the profile to get its skill IDs
        let profile = Profiles::get(profile_id)?.ok_or_else(|| {
            RhinolabsError::ConfigError(format!("Profile '{}' not found", profile_id))
        })?;

        let mut skills = Vec::new();
        for skill_id in &profile.skills {
            if let Some(skill) = Self::get(skill_id)? {
                skills.push(skill);
            }
        }

        Ok(skills)
    }

    /// Get profiles that contain a specific skill
    pub fn get_assigned_profiles(skill_id: &str) -> Result<Vec<Profile>> {
        Profiles::get_profiles_for_skill(skill_id)
    }

    // ============================================
    // Source Management
    // ============================================

    /// List all skill sources
    pub fn list_sources() -> Result<Vec<SkillSource>> {
        let config = Self::load_config()?;

        if config.sources.is_empty() {
            // Return default sources if none configured
            return Ok(SkillSource::default_sources());
        }

        // Merge default source values with saved config
        // This ensures that default sources have correct fetchable/schema values
        // even if they were saved before those fields were added
        let default_sources = SkillSource::default_sources();
        let sources: Vec<SkillSource> = config
            .sources
            .into_iter()
            .map(|mut s| {
                // For known default sources, ensure fetchable and schema are set correctly
                if let Some(default) = default_sources.iter().find(|d| d.id == s.id) {
                    // Only override if the source hasn't been explicitly set to fetchable
                    // We check if it's false (the old default) and the default is true
                    if !s.fetchable && default.fetchable {
                        s.fetchable = default.fetchable;
                    }
                }
                s
            })
            .collect();

        Ok(sources)
    }

    /// Add a new skill source
    pub fn add_source(source: SkillSource) -> Result<()> {
        let mut config = Self::load_config()?;

        // Initialize with defaults if empty
        if config.sources.is_empty() {
            config.sources = SkillSource::default_sources();
        }

        // Check for duplicate id
        if config.sources.iter().any(|s| s.id == source.id) {
            return Err(RhinolabsError::ConfigError(format!(
                "Source '{}' already exists",
                source.id
            )));
        }

        config.sources.push(source);
        Self::save_config(&config)
    }

    /// Update an existing skill source
    pub fn update_source(
        id: &str,
        enabled: Option<bool>,
        name: Option<String>,
        url: Option<String>,
        description: Option<String>,
        fetchable: Option<bool>,
        schema: Option<SkillSchema>,
    ) -> Result<()> {
        let mut config = Self::load_config()?;

        // Initialize with defaults if empty
        if config.sources.is_empty() {
            config.sources = SkillSource::default_sources();
        }

        let source = config
            .sources
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or_else(|| RhinolabsError::ConfigError(format!("Source '{}' not found", id)))?;

        if let Some(e) = enabled {
            source.enabled = e;
        }
        if let Some(n) = name {
            source.name = n;
        }
        if let Some(u) = url {
            source.url = u;
        }
        if let Some(d) = description {
            source.description = d;
        }
        if let Some(f) = fetchable {
            source.fetchable = f;
        }
        if let Some(s) = schema {
            source.schema = s;
        }

        Self::save_config(&config)
    }

    /// Remove a skill source
    pub fn remove_source(id: &str) -> Result<()> {
        let mut config = Self::load_config()?;

        // Don't allow removing default sources, just disable them
        let is_default = SkillSource::default_sources().iter().any(|s| s.id == id);
        if is_default {
            return Err(RhinolabsError::ConfigError(format!(
                "Cannot remove default source '{}'. You can disable it instead.",
                id
            )));
        }

        config.sources.retain(|s| s.id != id);
        Self::save_config(&config)
    }

    /// Install a skill from a source (downloads and saves locally)
    pub fn install_from_source(
        skill_id: &str,
        skill_content: &str,
        source_id: &str,
        source_name: &str,
    ) -> Result<Skill> {
        let skill_dir = Self::skills_dir()?.join(skill_id);

        if skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' already exists",
                skill_id
            )));
        }

        // Create skill directory
        fs::create_dir_all(&skill_dir)?;

        // Write SKILL.md
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, skill_content)?;

        // Update config with source metadata
        let mut config = Self::load_config()?;
        let content_hash = Self::hash_content(skill_content);

        config.skill_meta.insert(
            skill_id.to_string(),
            SkillMeta {
                source_id: Some(source_id.to_string()),
                source_name: Some(source_name.to_string()),
                original_hash: Some(content_hash),
            },
        );

        Self::save_config(&config)?;

        // Return the installed skill
        let config = Self::load_config()?;
        Self::load_from_dir(&skill_dir, &config)
    }

    /// Install a skill from a remote source, downloading all files
    pub async fn install_from_remote(
        source_url: &str,
        skill_id: &str,
        source_id: &str,
        source_name: &str,
    ) -> Result<Skill> {
        let skill_dir = Self::skills_dir()?.join(skill_id);

        if skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' already exists",
                skill_id
            )));
        }

        // Get list of files
        let files = Self::fetch_remote_skill_files(source_url, skill_id).await?;

        // Create skill directory
        fs::create_dir_all(&skill_dir)?;

        let client = reqwest::Client::new();
        let mut skill_md_content = String::new();

        // Download and save each file
        for file in &files {
            if file.is_directory {
                // Create subdirectory
                let dir_path = skill_dir.join(&file.relative_path);
                fs::create_dir_all(&dir_path)?;
            } else if let Some(url) = &file.download_url {
                // Download file content
                let content = Self::fetch_skill_content(&client, url).await?;

                // Save to local path
                let file_path = skill_dir.join(&file.relative_path);

                // Create parent directory if needed
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&file_path, &content)?;

                // Keep track of SKILL.md content for hash
                if file.name == "SKILL.md" {
                    skill_md_content = content;
                }
            }
        }

        // Update config with source metadata
        let mut config = Self::load_config()?;
        let content_hash = Self::hash_content(&skill_md_content);

        config.skill_meta.insert(
            skill_id.to_string(),
            SkillMeta {
                source_id: Some(source_id.to_string()),
                source_name: Some(source_name.to_string()),
                original_hash: Some(content_hash),
            },
        );

        Self::save_config(&config)?;

        // Return the installed skill
        let config = Self::load_config()?;
        Self::load_from_dir(&skill_dir, &config)
    }

    /// Reset a modified skill to its original content
    pub fn reset_to_original(id: &str, original_content: &str) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                id
            )));
        }

        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, original_content)?;

        // Update hash in meta
        let mut config = Self::load_config()?;
        if let Some(meta) = config.skill_meta.get_mut(id) {
            meta.original_hash = Some(Self::hash_content(original_content));
        }

        Self::save_config(&config)
    }

    /// Set the category for a skill (saves to category_map in config)
    /// This allows users to override the default category for any skill
    pub fn set_category(skill_id: &str, category: SkillCategory) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(skill_id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(format!(
                "Skill '{}' not found",
                skill_id
            )));
        }

        let mut config = Self::load_config()?;

        if category == SkillCategory::Custom {
            // Remove from category_map if setting to Custom (default)
            config.category_map.remove(skill_id);
        } else {
            config.category_map.insert(skill_id.to_string(), category);
        }

        Self::save_config(&config)
    }

    /// Get the category for a skill
    pub fn get_skill_category(skill_id: &str) -> Result<SkillCategory> {
        let config = Self::load_config()?;
        Ok(Self::get_category(skill_id, &config))
    }

    /// Get list of installed skill IDs for checking installation status
    pub fn installed_ids() -> Result<Vec<String>> {
        let dir = Self::skills_dir()?;

        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut ids = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    ids.push(name.to_string());
                }
            }
        }

        Ok(ids)
    }

    /// Fetch skills from a source, choosing the appropriate method based on schema
    pub async fn fetch_from_source(source: &SkillSource) -> Result<Vec<RemoteSkill>> {
        match source.schema {
            SkillSchema::Standard => Self::fetch_from_github(source).await,
            SkillSchema::SkillsSh => Self::fetch_from_skills_sh(source).await,
            SkillSchema::Custom => Err(RhinolabsError::ConfigError(
                "Custom schema sources cannot be fetched automatically".into(),
            )),
        }
    }

    /// Fetch skills from a GitHub repository
    /// Expects URL format: https://github.com/owner/repo
    pub async fn fetch_from_github(source: &SkillSource) -> Result<Vec<RemoteSkill>> {
        // Parse GitHub URL to get owner/repo
        let url = &source.url;
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();

        if parts.len() < 2 {
            return Err(RhinolabsError::ConfigError(
                "Invalid GitHub URL format".into(),
            ));
        }

        let repo = parts[parts.len() - 1];
        let owner = parts[parts.len() - 2];

        // Fetch the skills directory contents from GitHub API
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/skills?ref=main",
            owner, repo
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&api_url)
            .header("User-Agent", "rhinolabs-ai")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RhinolabsError::NetworkError(format!(
                "GitHub API error {}: {}",
                status, body
            )));
        }

        let contents: Vec<GitHubContent> = response.json().await.map_err(|e| {
            RhinolabsError::NetworkError(format!("Failed to parse GitHub response: {}", e))
        })?;

        // Get installed skill IDs
        let installed = Self::installed_ids().unwrap_or_default();

        let mut remote_skills = Vec::new();

        // For each directory, try to fetch SKILL.md
        for item in contents {
            if item.content_type == "dir" {
                let skill_url = format!(
                    "https://raw.githubusercontent.com/{}/{}/main/skills/{}/SKILL.md",
                    owner, repo, item.name
                );

                match Self::fetch_skill_content(&client, &skill_url).await {
                    Ok(skill_content) => match Self::parse_skill_file(&skill_content) {
                        Ok((frontmatter, _)) => {
                            remote_skills.push(RemoteSkill {
                                id: item.name.clone(),
                                name: frontmatter.name,
                                description: frontmatter.description,
                                category: "custom".to_string(),
                                source_id: source.id.clone(),
                                source_name: source.name.clone(),
                                url: skill_url,
                                stars: None,
                                installed: installed.contains(&item.name),
                            });
                        }
                        Err(e) => {
                            eprintln!("[WARN] Failed to parse SKILL.md for '{}': {}", item.name, e);
                        }
                    },
                    Err(e) => {
                        eprintln!("[WARN] Failed to fetch SKILL.md for '{}': {}", item.name, e);
                    }
                }
            }
        }

        Ok(remote_skills)
    }

    /// Fetch skills from skills.sh by scraping the HTML
    /// The site embeds JSON data in the HTML that we can extract
    pub async fn fetch_from_skills_sh(source: &SkillSource) -> Result<Vec<RemoteSkill>> {
        let client = reqwest::Client::new();

        // Fetch the skills.sh page (use /hot for popular skills)
        let url = if source.url.ends_with('/') {
            format!("{}hot", source.url)
        } else if source.url == "https://skills.sh" || source.url == "https://skills.sh/" {
            "https://skills.sh/hot".to_string()
        } else {
            source.url.clone()
        };

        let response = client
            .get(&url)
            .header("User-Agent", "rhinolabs-ai")
            .send()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(RhinolabsError::NetworkError(format!(
                "Failed to fetch skills.sh: {}",
                response.status()
            )));
        }

        let html = response
            .text()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        // Extract JSON data from HTML - skills.sh embeds data in allTimeSkills array
        let skills_json = Self::extract_skills_sh_data(&html)?;

        // Get installed skill IDs
        let installed = Self::installed_ids().unwrap_or_default();

        let mut remote_skills = Vec::new();

        for skill_data in skills_json {
            let source_repo = skill_data
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let skill_id = skill_data
                .get("skillId")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let name = skill_data
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(skill_id);
            let installs = skill_data
                .get("installs")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            if source_repo.is_empty() || skill_id.is_empty() {
                continue;
            }

            // Build GitHub URL for this skill
            let skill_url = format!(
                "https://github.com/{}/tree/main/skills/{}",
                source_repo, skill_id
            );

            // Create unique ID combining source repo and skill id
            let unique_id = format!("{}/{}", source_repo, skill_id);

            remote_skills.push(RemoteSkill {
                id: unique_id.clone(),
                name: name.to_string(),
                description: format!("From {} ({} installs)", source_repo, installs),
                category: "community".to_string(),
                source_id: source.id.clone(),
                source_name: source.name.clone(),
                url: skill_url,
                stars: Some(installs as u32),
                installed: installed.contains(&skill_id.to_string())
                    || installed.contains(&unique_id),
            });
        }

        Ok(remote_skills)
    }

    /// Extract skills data from skills.sh HTML
    /// Supports both regular JSON and backslash-escaped JSON formats
    fn extract_skills_sh_data(html: &str) -> Result<Vec<serde_json::Value>> {
        use std::collections::HashSet;

        let mut skills = Vec::new();
        let mut seen = HashSet::new();

        // Try regular JSON format first: {"source":"owner/repo",...}
        let mut pos = 0;
        while let Some(start) = html[pos..].find(r#"{"source":""#) {
            let abs_start = pos + start;
            if let Some(end) = html[abs_start..].find('}') {
                let obj_str = &html[abs_start..abs_start + end + 1];
                if let Ok(skill) = serde_json::from_str::<serde_json::Value>(obj_str) {
                    if let Some(skill_id) = skill.get("skillId").and_then(|v| v.as_str()) {
                        if seen.insert(skill_id.to_string()) {
                            skills.push(skill);
                        }
                    }
                }
                pos = abs_start + end + 1;
            } else {
                break;
            }
        }

        // Fallback: try escaped JSON format: {\"source\":\"owner/repo\",...}
        if skills.is_empty() {
            pos = 0;
            while let Some(start) = html[pos..].find("{\\\"source\\\":\\\"") {
                let abs_start = pos + start;
                if let Some(end) = html[abs_start..].find('}') {
                    let obj_str = &html[abs_start..abs_start + end + 1];
                    let unescaped = obj_str.replace("\\\"", "\"");
                    if let Ok(skill) = serde_json::from_str::<serde_json::Value>(&unescaped) {
                        if let Some(skill_id) = skill.get("skillId").and_then(|v| v.as_str()) {
                            if seen.insert(skill_id.to_string()) {
                                skills.push(skill);
                            }
                        }
                    }
                    pos = abs_start + end + 1;
                } else {
                    break;
                }
            }
        }

        if skills.is_empty() {
            return Err(RhinolabsError::ConfigError(
                "Could not find skills data in skills.sh HTML".into(),
            ));
        }

        Ok(skills)
    }

    /// Helper to fetch skill content from URL
    async fn fetch_skill_content(client: &reqwest::Client, url: &str) -> Result<String> {
        let response = client
            .get(url)
            .header("User-Agent", "rhinolabs-ai")
            .send()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(RhinolabsError::NetworkError(format!(
                "Failed to fetch: {}",
                response.status()
            )));
        }

        response
            .text()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))
    }

    /// Fetch a single skill's content from its URL
    pub async fn fetch_skill_by_url(url: &str) -> Result<String> {
        let client = reqwest::Client::new();
        Self::fetch_skill_content(&client, url).await
    }

    /// Fetch the file structure of a remote skill from GitHub
    pub async fn fetch_remote_skill_files(
        source_url: &str,
        skill_id: &str,
    ) -> Result<Vec<RemoteSkillFile>> {
        // Validate inputs
        if skill_id.is_empty() {
            return Err(RhinolabsError::ConfigError(
                "skill_id cannot be empty".into(),
            ));
        }

        // Parse GitHub URL to get owner/repo
        let parts: Vec<&str> = source_url.trim_end_matches('/').split('/').collect();

        if parts.len() < 2 {
            return Err(RhinolabsError::ConfigError(format!(
                "Invalid GitHub URL format: {}",
                source_url
            )));
        }

        let repo = parts[parts.len() - 1];
        let owner = parts[parts.len() - 2];

        let client = reqwest::Client::new();
        let mut files = Vec::new();

        let path = format!("skills/{}", skill_id);

        // Recursively fetch directory contents
        Self::fetch_github_directory_contents(&client, owner, repo, &path, "", &mut files).await?;

        Ok(files)
    }

    async fn fetch_github_directory_contents(
        client: &reqwest::Client,
        owner: &str,
        repo: &str,
        path: &str,
        relative_path: &str,
        files: &mut Vec<RemoteSkillFile>,
    ) -> Result<()> {
        // Explicitly use main branch to ensure consistency
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}?ref=main",
            owner, repo, path
        );

        let response = client
            .get(&api_url)
            .header("User-Agent", "rhinolabs-ai")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(RhinolabsError::NetworkError(format!(
                "Failed to fetch '{}' from GitHub: HTTP {}",
                path,
                response.status()
            )));
        }

        let contents: Vec<GitHubContentExtended> = response
            .json()
            .await
            .map_err(|e| RhinolabsError::NetworkError(e.to_string()))?;

        for item in contents {
            let item_relative_path = if relative_path.is_empty() {
                item.name.clone()
            } else {
                format!("{}/{}", relative_path, item.name)
            };

            if item.content_type == "dir" {
                files.push(RemoteSkillFile {
                    name: item.name.clone(),
                    relative_path: item_relative_path.clone(),
                    is_directory: true,
                    download_url: None,
                    language: None,
                });

                // Recursively fetch subdirectory
                Box::pin(Self::fetch_github_directory_contents(
                    client,
                    owner,
                    repo,
                    &format!("{}/{}", path, item.name),
                    &item_relative_path,
                    files,
                ))
                .await?;
            } else {
                let language = Self::detect_language_from_name(&item.name);
                files.push(RemoteSkillFile {
                    name: item.name,
                    relative_path: item_relative_path,
                    is_directory: false,
                    download_url: item.download_url,
                    language,
                });
            }
        }

        Ok(())
    }

    fn detect_language_from_name(name: &str) -> Option<String> {
        let ext = name.rsplit('.').next()?;
        let lang = match ext {
            "md" => "markdown",
            "ts" | "tsx" => "typescript",
            "js" | "jsx" => "javascript",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "toml" => "toml",
            "rs" => "rust",
            "py" => "python",
            "go" => "go",
            "sh" | "bash" => "bash",
            "css" => "css",
            "html" => "html",
            "sql" => "sql",
            _ => return None,
        };
        Some(lang.to_string())
    }
}

/// GitHub API response structure
#[derive(Debug, Deserialize)]
struct GitHubContent {
    name: String,
    #[serde(rename = "type")]
    content_type: String,
}

/// Extended GitHub API response structure
#[derive(Debug, Deserialize)]
struct GitHubContentExtended {
    name: String,
    #[serde(rename = "type")]
    content_type: String,
    download_url: Option<String>,
}

/// Remote skill file information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSkillFile {
    pub name: String,
    pub relative_path: String,
    pub is_directory: bool,
    pub download_url: Option<String>,
    pub language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TestEnv as BaseTestEnv, ENV_MUTEX};

    /// Extended test environment with skills-specific helpers
    struct TestEnv {
        base: BaseTestEnv,
    }

    impl TestEnv {
        fn new() -> Self {
            TestEnv {
                base: BaseTestEnv::new(),
            }
        }

        fn plugin_dir(&self) -> PathBuf {
            self.base.plugin_dir()
        }

        fn skills_dir(&self) -> PathBuf {
            self.plugin_dir().join("skills")
        }

        fn setup_skills_dir(&self) {
            fs::create_dir_all(self.skills_dir()).expect("Failed to create skills dir");
        }

        fn create_skill(&self, id: &str, name: &str, description: &str, content: &str) {
            let skill_dir = self.skills_dir().join(id);
            fs::create_dir_all(&skill_dir).expect("Failed to create skill dir");
            let skill_content = Skills::generate_skill_file(name, description, content);
            fs::write(skill_dir.join("SKILL.md"), skill_content)
                .expect("Failed to write skill file");
        }

        fn create_config(&self, config: &SkillsConfig) {
            let config_path = self.plugin_dir().join(".skills-config.json");
            let content = serde_json::to_string_pretty(config).expect("Failed to serialize config");
            fs::write(config_path, content).expect("Failed to write config");
        }
    }

    #[test]
    fn test_parse_skill_file() {
        let content = r#"---
name: test-skill
description: A test skill for testing
---

# Test Skill

This is the content.
"#;

        let result = Skills::parse_skill_file(content);
        assert!(result.is_ok());

        let (frontmatter, markdown) = result.unwrap();
        assert_eq!(frontmatter.name, "test-skill");
        assert!(frontmatter.description.contains("test skill"));
        assert!(markdown.contains("# Test Skill"));
    }

    #[test]
    fn test_parse_skill_file_invalid_no_frontmatter() {
        let content = "# Just markdown without frontmatter";
        let result = Skills::parse_skill_file(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("frontmatter"));
    }

    #[test]
    fn test_parse_skill_file_invalid_incomplete_frontmatter() {
        let content = "---\nname: test\n";
        let result = Skills::parse_skill_file(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_category_hardcoded() {
        // Test hardcoded category mapping with empty config
        let config = SkillsConfig::default();

        assert_eq!(
            Skills::get_category("rhinolabs-standards", &config),
            SkillCategory::Corporate
        );
        assert_eq!(
            Skills::get_category("react-patterns", &config),
            SkillCategory::Frontend
        );
        assert_eq!(
            Skills::get_category("playwright", &config),
            SkillCategory::Testing
        );
        assert_eq!(
            Skills::get_category("ai-sdk-core", &config),
            SkillCategory::AiSdk
        );
        assert_eq!(
            Skills::get_category("skill-creator", &config),
            SkillCategory::Utilities
        );
        assert_eq!(
            Skills::get_category("unknown-skill", &config),
            SkillCategory::Custom
        );
    }

    #[test]
    fn test_generate_skill_file() {
        let content = Skills::generate_skill_file("My Skill", "Description", "# Content");

        assert!(content.starts_with("---"));
        assert!(content.contains("name: My Skill"));
        assert!(content.contains("description: Description"));
        assert!(content.contains("# Content"));
    }

    #[test]
    fn test_skills_config_default() {
        let config = SkillsConfig::default();

        assert!(config.disabled.is_empty());
        assert!(config.custom.is_empty());
    }

    #[test]
    fn test_hash_content_consistency() {
        let content = "Some content to hash";
        let hash1 = Skills::hash_content(content);
        let hash2 = Skills::hash_content(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_content_different_for_different_content() {
        let hash1 = Skills::hash_content("Content A");
        let hash2 = Skills::hash_content("Content B");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_detect_language_from_name() {
        assert_eq!(
            Skills::detect_language_from_name("test.md"),
            Some("markdown".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.ts"),
            Some("typescript".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.tsx"),
            Some("typescript".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.js"),
            Some("javascript".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.py"),
            Some("python".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.rs"),
            Some("rust".to_string())
        );
        assert_eq!(
            Skills::detect_language_from_name("test.go"),
            Some("go".to_string())
        );
        assert_eq!(Skills::detect_language_from_name("test.unknown"), None);
        assert_eq!(Skills::detect_language_from_name("noextension"), None);
    }

    // ============================================
    // list_sources() Tests
    // ============================================

    #[test]
    fn test_list_sources_returns_defaults_when_no_config() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let sources = Skills::list_sources().expect("Should list sources");
        assert_eq!(sources.len(), 3);

        let official = sources.iter().find(|s| s.id == "anthropic-official");
        assert!(official.is_some());
        assert!(official.unwrap().fetchable);

        let vercel = sources.iter().find(|s| s.id == "vercel-agent-skills");
        assert!(vercel.is_some());
        assert!(vercel.unwrap().fetchable);

        let awesome = sources.iter().find(|s| s.id == "awesome-claude-skills");
        assert!(awesome.is_some());
        assert!(!awesome.unwrap().fetchable); // Browse only
    }

    #[test]
    fn test_list_sources_merges_fetchable_from_defaults() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Create a config where anthropic-official has fetchable=false (old saved config)
        let config = SkillsConfig {
            sources: vec![SkillSource {
                id: "anthropic-official".to_string(),
                name: "Anthropic Official".to_string(),
                source_type: SkillSourceType::Official,
                url: "https://github.com/anthropics/skills".to_string(),
                description: "Official skills".to_string(),
                enabled: true,
                fetchable: false, // Old saved value
                schema: SkillSchema::Standard,
                skill_count: None,
            }],
            ..Default::default()
        };
        env.create_config(&config);

        let sources = Skills::list_sources().expect("Should list sources");

        // The anthropic-official source should have fetchable=true after merge
        let official = sources
            .iter()
            .find(|s| s.id == "anthropic-official")
            .unwrap();
        assert!(
            official.fetchable,
            "fetchable should be merged from defaults"
        );
    }

    #[test]
    fn test_list_sources_preserves_custom_sources() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let config = SkillsConfig {
            sources: vec![SkillSource {
                id: "custom-source".to_string(),
                name: "My Custom Source".to_string(),
                source_type: SkillSourceType::Local,
                url: "https://example.com/skills".to_string(),
                description: "Custom source".to_string(),
                enabled: true,
                fetchable: false,
                schema: SkillSchema::Custom,
                skill_count: None,
            }],
            ..Default::default()
        };
        env.create_config(&config);

        let sources = Skills::list_sources().expect("Should list sources");
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "custom-source");
    }

    // ============================================
    // delete() Tests
    // ============================================

    #[test]
    fn test_delete_custom_skill_succeeds() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill(
            "my-custom-skill",
            "My Custom Skill",
            "A custom skill",
            "# Content",
        );

        // Mark it as custom in config
        let config = SkillsConfig {
            custom: vec!["my-custom-skill".to_string()],
            ..Default::default()
        };
        env.create_config(&config);

        // Verify it exists
        let skill = Skills::get("my-custom-skill").expect("Should get skill");
        assert!(skill.is_some());

        // Delete it
        let result = Skills::delete("my-custom-skill");
        assert!(result.is_ok());

        // Verify it's gone
        let skill = Skills::get("my-custom-skill").expect("Should get skill");
        assert!(skill.is_none());
    }

    #[test]
    fn test_delete_source_installed_skill_succeeds() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill(
            "installed-skill",
            "Installed Skill",
            "From source",
            "# Content",
        );

        // Mark it with source metadata (installed from source)
        let mut skill_meta = std::collections::HashMap::new();
        skill_meta.insert(
            "installed-skill".to_string(),
            SkillMeta {
                source_id: Some("anthropic-official".to_string()),
                source_name: Some("Anthropic Official".to_string()),
                original_hash: Some("abc123".to_string()),
            },
        );
        let config = SkillsConfig {
            skill_meta,
            ..Default::default()
        };
        env.create_config(&config);

        // Delete it
        let result = Skills::delete("installed-skill");
        assert!(result.is_ok());

        // Verify it's gone
        let skill = Skills::get("installed-skill").expect("Should get skill");
        assert!(skill.is_none());
    }

    #[test]
    fn test_delete_builtin_skill_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill(
            "react-patterns",
            "React Patterns",
            "Built-in skill",
            "# Content",
        );

        // Don't mark it as custom or with source metadata
        let config = SkillsConfig::default();
        env.create_config(&config);

        // Try to delete it
        let result = Skills::delete("react-patterns");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot delete built-in skill"));
    }

    #[test]
    fn test_delete_nonexistent_skill_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Mark it as custom so delete logic passes that check
        let config = SkillsConfig {
            custom: vec!["nonexistent".to_string()],
            ..Default::default()
        };
        env.create_config(&config);

        let result = Skills::delete("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // ============================================
    // get_skill_path() Tests
    // ============================================

    #[test]
    fn test_get_skill_path_returns_correct_path() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("my-skill", "My Skill", "Description", "# Content");

        let path = Skills::get_skill_path("my-skill").expect("Should get path");
        assert!(path.exists());
        assert!(path.is_dir());
        assert!(path.join("SKILL.md").exists());
    }

    #[test]
    fn test_get_skill_path_nonexistent_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let result = Skills::get_skill_path("nonexistent-skill");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // ============================================
    // list() Tests
    // ============================================

    #[test]
    fn test_list_returns_empty_when_no_skills_dir() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _env = TestEnv::new();
        // Don't create skills dir

        let skills = Skills::list().expect("Should list skills");
        assert!(skills.is_empty());
    }

    #[test]
    fn test_list_returns_all_skills() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("skill-a", "Skill A", "First skill", "# A");
        env.create_skill("skill-b", "Skill B", "Second skill", "# B");

        let skills = Skills::list().expect("Should list skills");
        assert_eq!(skills.len(), 2);
    }

    #[test]
    fn test_list_sorts_by_category_then_name() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Create skills in different categories (using known built-in IDs)
        env.create_skill(
            "rhinolabs-standards",
            "Rhinolabs Standards",
            "Corporate",
            "# Corp",
        );
        env.create_skill("react-patterns", "React Patterns", "Frontend", "# Frontend");
        env.create_skill("custom-skill", "Custom Skill", "Custom", "# Custom");

        let skills = Skills::list().expect("Should list skills");
        assert_eq!(skills.len(), 3);

        // Corporate should be first
        assert_eq!(skills[0].id, "rhinolabs-standards");
        // Frontend second
        assert_eq!(skills[1].id, "react-patterns");
        // Custom last
        assert_eq!(skills[2].id, "custom-skill");
    }

    // ============================================
    // get() Tests
    // ============================================

    #[test]
    fn test_get_returns_skill_when_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("test-skill", "Test Skill", "A test", "# Content here");

        let skill = Skills::get("test-skill").expect("Should get skill");
        assert!(skill.is_some());

        let skill = skill.unwrap();
        assert_eq!(skill.id, "test-skill");
        assert_eq!(skill.name, "Test Skill");
        assert_eq!(skill.description, "A test");
        assert!(skill.content.contains("# Content here"));
    }

    #[test]
    fn test_get_returns_none_when_not_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let skill = Skills::get("nonexistent").expect("Should get skill");
        assert!(skill.is_none());
    }

    // ============================================
    // create() Tests
    // ============================================

    #[test]
    fn test_create_skill_succeeds() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let input = CreateSkillInput {
            id: "new-skill".to_string(),
            name: "New Skill".to_string(),
            description: "A brand new skill".to_string(),
            category: SkillCategory::Custom,
            content: "# New Skill Content".to_string(),
        };

        let skill = Skills::create(input).expect("Should create skill");
        assert_eq!(skill.id, "new-skill");
        assert_eq!(skill.name, "New Skill");
        assert!(skill.is_custom);

        // Verify file was created
        let skill_file = env.skills_dir().join("new-skill").join("SKILL.md");
        assert!(skill_file.exists());
    }

    #[test]
    fn test_create_skill_fails_if_exists() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("existing-skill", "Existing", "Already exists", "# Content");

        let input = CreateSkillInput {
            id: "existing-skill".to_string(),
            name: "Duplicate".to_string(),
            description: "Should fail".to_string(),
            category: SkillCategory::Custom,
            content: "# Content".to_string(),
        };

        let result = Skills::create(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    // ============================================
    // toggle() Tests
    // ============================================

    #[test]
    fn test_toggle_disable_skill() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("toggleable", "Toggleable", "Can be toggled", "# Content");

        // Initially enabled
        let skill = Skills::get("toggleable").expect("Should get").unwrap();
        assert!(skill.enabled);

        // Disable it
        Skills::toggle("toggleable", false).expect("Should toggle");

        // Now disabled
        let skill = Skills::get("toggleable").expect("Should get").unwrap();
        assert!(!skill.enabled);
    }

    #[test]
    fn test_toggle_enable_skill() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("toggleable", "Toggleable", "Can be toggled", "# Content");

        // Disable first
        let config = SkillsConfig {
            disabled: vec!["toggleable".to_string()],
            ..Default::default()
        };
        env.create_config(&config);

        let skill = Skills::get("toggleable").expect("Should get").unwrap();
        assert!(!skill.enabled);

        // Enable it
        Skills::toggle("toggleable", true).expect("Should toggle");

        let skill = Skills::get("toggleable").expect("Should get").unwrap();
        assert!(skill.enabled);
    }

    #[test]
    fn test_toggle_nonexistent_skill_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let result = Skills::toggle("nonexistent", true);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // ============================================
    // Default Sources Tests
    // ============================================

    #[test]
    fn test_default_sources_structure() {
        let sources = SkillSource::default_sources();

        assert_eq!(sources.len(), 3);

        // Anthropic Official
        let anthropic = &sources[0];
        assert_eq!(anthropic.id, "anthropic-official");
        assert_eq!(anthropic.source_type, SkillSourceType::Official);
        assert!(anthropic.fetchable);
        assert_eq!(anthropic.schema, SkillSchema::Standard);

        // Vercel
        let vercel = &sources[1];
        assert_eq!(vercel.id, "vercel-agent-skills");
        assert_eq!(vercel.source_type, SkillSourceType::Marketplace);
        assert!(vercel.fetchable);

        // Awesome list (browse only)
        let awesome = &sources[2];
        assert_eq!(awesome.id, "awesome-claude-skills");
        assert_eq!(awesome.source_type, SkillSourceType::Community);
        assert!(!awesome.fetchable);
    }

    // ============================================
    // Skill Modification Detection Tests
    // ============================================

    #[test]
    fn test_skill_is_modified_detection() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let original_content = Skills::generate_skill_file("Test", "Desc", "# Original");
        let original_hash = Skills::hash_content(&original_content);

        // Create skill with source metadata
        let skill_dir = env.skills_dir().join("modified-skill");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), &original_content).unwrap();

        let mut skill_meta = std::collections::HashMap::new();
        skill_meta.insert(
            "modified-skill".to_string(),
            SkillMeta {
                source_id: Some("test-source".to_string()),
                source_name: Some("Test Source".to_string()),
                original_hash: Some(original_hash),
            },
        );
        let config = SkillsConfig {
            skill_meta,
            ..Default::default()
        };
        env.create_config(&config);

        // Not modified yet
        let skill = Skills::get("modified-skill").unwrap().unwrap();
        assert!(!skill.is_modified);

        // Modify the file
        let modified_content = Skills::generate_skill_file("Test", "Desc", "# MODIFIED!");
        fs::write(skill_dir.join("SKILL.md"), modified_content).unwrap();

        // Now it should be detected as modified
        let skill = Skills::get("modified-skill").unwrap().unwrap();
        assert!(skill.is_modified);
    }

    // ============================================
    // Dynamic Category Tests
    // ============================================

    #[test]
    fn test_get_category_uses_category_map_first() {
        // Test that category_map takes precedence over hardcoded constants
        let mut category_map = std::collections::HashMap::new();
        // Override a hardcoded skill's category
        category_map.insert("react-patterns".to_string(), SkillCategory::Corporate);

        let config = SkillsConfig {
            category_map,
            ..Default::default()
        };

        // Should use category_map value, not hardcoded Frontend
        let category = Skills::get_category("react-patterns", &config);
        assert_eq!(category, SkillCategory::Corporate);
    }

    #[test]
    fn test_get_category_falls_back_to_hardcoded() {
        // Test that hardcoded constants are used when no category_map entry
        let config = SkillsConfig::default();

        let category = Skills::get_category("react-patterns", &config);
        assert_eq!(category, SkillCategory::Frontend);

        let category = Skills::get_category("rhinolabs-standards", &config);
        assert_eq!(category, SkillCategory::Corporate);
    }

    #[test]
    fn test_get_category_returns_custom_for_unknown() {
        let config = SkillsConfig::default();

        let category = Skills::get_category("some-unknown-skill", &config);
        assert_eq!(category, SkillCategory::Custom);
    }

    #[test]
    fn test_set_category_saves_to_config() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("my-skill", "My Skill", "Description", "# Content");

        // Initially should be Custom (unknown skill)
        let skill = Skills::get("my-skill").unwrap().unwrap();
        assert_eq!(skill.category, SkillCategory::Custom);

        // Set to Frontend
        Skills::set_category("my-skill", SkillCategory::Frontend).unwrap();

        // Should now be Frontend
        let skill = Skills::get("my-skill").unwrap().unwrap();
        assert_eq!(skill.category, SkillCategory::Frontend);
    }

    #[test]
    fn test_set_category_removes_when_custom() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("my-skill", "My Skill", "Description", "# Content");

        // Set to Frontend first
        Skills::set_category("my-skill", SkillCategory::Frontend).unwrap();

        // Verify it's in category_map
        let config = Skills::load_config().unwrap();
        assert!(config.category_map.contains_key("my-skill"));

        // Set back to Custom
        Skills::set_category("my-skill", SkillCategory::Custom).unwrap();

        // Should be removed from category_map
        let config = Skills::load_config().unwrap();
        assert!(!config.category_map.contains_key("my-skill"));
    }

    #[test]
    fn test_set_category_fails_for_nonexistent_skill() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let result = Skills::set_category("nonexistent-skill", SkillCategory::Frontend);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_create_skill_with_category_saves_to_config() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let input = CreateSkillInput {
            id: "new-frontend-skill".to_string(),
            name: "New Frontend Skill".to_string(),
            description: "A new skill".to_string(),
            category: SkillCategory::Frontend,
            content: "# Content".to_string(),
        };

        let skill = Skills::create(input).unwrap();
        assert_eq!(skill.category, SkillCategory::Frontend);

        // Verify it's in category_map
        let config = Skills::load_config().unwrap();
        assert_eq!(
            config.category_map.get("new-frontend-skill"),
            Some(&SkillCategory::Frontend)
        );
    }

    #[test]
    fn test_create_skill_with_custom_category_not_in_map() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let input = CreateSkillInput {
            id: "custom-skill-2".to_string(),
            name: "Custom Skill 2".to_string(),
            description: "A custom skill".to_string(),
            category: SkillCategory::Custom, // Default
            content: "# Content".to_string(),
        };

        let skill = Skills::create(input).unwrap();
        assert_eq!(skill.category, SkillCategory::Custom);

        // Should NOT be in category_map (Custom is default, no need to store)
        let config = Skills::load_config().unwrap();
        assert!(!config.category_map.contains_key("custom-skill-2"));
    }

    #[test]
    fn test_delete_skill_removes_from_category_map() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill("deletable-skill", "Deletable", "To be deleted", "# Content");

        // Mark as custom so it can be deleted
        let mut category_map = std::collections::HashMap::new();
        category_map.insert("deletable-skill".to_string(), SkillCategory::Testing);

        let config = SkillsConfig {
            custom: vec!["deletable-skill".to_string()],
            category_map,
            ..Default::default()
        };
        env.create_config(&config);

        // Verify category is set
        let skill = Skills::get("deletable-skill").unwrap().unwrap();
        assert_eq!(skill.category, SkillCategory::Testing);

        // Delete the skill
        Skills::delete("deletable-skill").unwrap();

        // Verify category_map entry is removed
        let config = Skills::load_config().unwrap();
        assert!(!config.category_map.contains_key("deletable-skill"));
    }

    #[test]
    fn test_custom_skill_can_have_category_from_map() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        env.create_skill(
            "my-custom-skill-3",
            "My Custom",
            "Custom skill",
            "# Content",
        );

        // Mark as custom with a specific category
        let mut category_map = std::collections::HashMap::new();
        category_map.insert("my-custom-skill-3".to_string(), SkillCategory::Utilities);

        let config = SkillsConfig {
            custom: vec!["my-custom-skill-3".to_string()],
            category_map,
            ..Default::default()
        };
        env.create_config(&config);

        // Should use the category from category_map
        let skill = Skills::get("my-custom-skill-3").unwrap().unwrap();
        assert!(skill.is_custom);
        assert_eq!(skill.category, SkillCategory::Utilities);
    }

    #[test]
    fn test_skills_config_category_map_serialization() {
        let mut category_map = std::collections::HashMap::new();
        category_map.insert("skill-a".to_string(), SkillCategory::Frontend);
        category_map.insert("skill-b".to_string(), SkillCategory::Testing);

        let config = SkillsConfig {
            category_map,
            ..Default::default()
        };

        // Serialize
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("categoryMap"));
        assert!(json.contains("skill-a"));
        assert!(json.contains("frontend"));

        // Deserialize
        let parsed: SkillsConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.category_map.get("skill-a"),
            Some(&SkillCategory::Frontend)
        );
        assert_eq!(
            parsed.category_map.get("skill-b"),
            Some(&SkillCategory::Testing)
        );
    }

    // ============================================
    // SkillCategory Deserialization Tests
    // ============================================

    #[test]
    fn test_skill_category_deserialize_valid_variants() {
        let cases = vec![
            ("\"corporate\"", SkillCategory::Corporate),
            ("\"backend\"", SkillCategory::Backend),
            ("\"frontend\"", SkillCategory::Frontend),
            ("\"testing\"", SkillCategory::Testing),
            ("\"aisdk\"", SkillCategory::AiSdk),
            ("\"utilities\"", SkillCategory::Utilities),
            ("\"custom\"", SkillCategory::Custom),
        ];

        for (json, expected) in cases {
            let result: std::result::Result<SkillCategory, _> = serde_json::from_str(json);
            assert!(result.is_ok(), "Should deserialize '{}' successfully", json);
            assert_eq!(result.unwrap(), expected, "Mismatch for '{}'", json);
        }
    }

    #[test]
    fn test_skill_category_deserialize_invalid_value() {
        // These must ALL fail — especially "workflow" which was the real bug
        let invalid_values = vec![
            "\"workflow\"",
            "\"invalid\"",
            "\"CORPORATE\"", // case-sensitive: "CORPORATE" != "corporate"
            "\"Frontend\"",  // PascalCase not accepted
            "\"\"",          // empty string
        ];

        for json in invalid_values {
            let result: std::result::Result<SkillCategory, _> = serde_json::from_str(json);
            assert!(
                result.is_err(),
                "Should REJECT invalid category '{}' but got {:?}",
                json,
                result.ok()
            );
        }
    }

    #[test]
    fn test_skill_category_serialize_roundtrip() {
        let categories = vec![
            SkillCategory::Corporate,
            SkillCategory::Backend,
            SkillCategory::Frontend,
            SkillCategory::Testing,
            SkillCategory::AiSdk,
            SkillCategory::Utilities,
            SkillCategory::Custom,
        ];

        for category in categories {
            let json = serde_json::to_string(&category).unwrap();
            let deserialized: SkillCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(
                category, deserialized,
                "Roundtrip failed for {:?} (serialized as {})",
                category, json
            );
        }
    }

    #[test]
    fn test_skill_category_default() {
        assert_eq!(SkillCategory::default(), SkillCategory::Custom);
    }

    // ============================================
    // load_config() Tests
    // ============================================

    #[test]
    fn test_load_config_file_not_found() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();
        // No config file created — should return default

        let config = Skills::load_config().expect("Should return default config");
        assert!(config.disabled.is_empty());
        assert!(config.custom.is_empty());
        assert!(config.sources.is_empty());
        assert!(config.category_map.is_empty());
    }

    #[test]
    fn test_load_config_valid_json() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        let mut category_map = std::collections::HashMap::new();
        category_map.insert("my-skill".to_string(), SkillCategory::Frontend);

        let config = SkillsConfig {
            disabled: vec!["skill-x".to_string()],
            custom: vec!["skill-y".to_string()],
            category_map,
            ..Default::default()
        };
        env.create_config(&config);

        let loaded = Skills::load_config().expect("Should load config");
        assert_eq!(loaded.disabled, vec!["skill-x".to_string()]);
        assert_eq!(loaded.custom, vec!["skill-y".to_string()]);
        assert_eq!(
            loaded.category_map.get("my-skill"),
            Some(&SkillCategory::Frontend)
        );
    }

    #[test]
    fn test_load_config_invalid_json() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Write garbage JSON
        let config_path = env.plugin_dir().join(".skills-config.json");
        fs::write(&config_path, "{ this is not valid json }").unwrap();

        let result = Skills::load_config();
        assert!(result.is_err(), "Should fail on invalid JSON");
    }

    #[test]
    fn test_load_config_invalid_category_in_map() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Write config with "workflow" as a category value — THE BUG
        let raw_json = r#"{
            "disabled": [],
            "custom": [],
            "sources": [],
            "skillMeta": {},
            "categoryMap": {
                "my-skill": "workflow"
            }
        }"#;

        let config_path = env.plugin_dir().join(".skills-config.json");
        fs::write(&config_path, raw_json).unwrap();

        let result = Skills::load_config();
        assert!(
            result.is_err(),
            "Config with invalid category 'workflow' in categoryMap should fail deserialization"
        );
    }

    #[test]
    fn test_load_config_empty_json_object_fails() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // `{}` fails because `disabled`, `custom`, `sources` are required fields
        // (they don't have #[serde(default)] on them individually)
        let config_path = env.plugin_dir().join(".skills-config.json");
        fs::write(&config_path, "{}").unwrap();

        let result = Skills::load_config();
        assert!(
            result.is_err(),
            "Empty JSON object should fail: disabled/custom/sources are required"
        );
    }

    #[test]
    fn test_load_config_minimal_valid_json() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let env = TestEnv::new();
        env.setup_skills_dir();

        // Minimal valid JSON with all required fields
        let raw_json = r#"{
            "disabled": [],
            "custom": [],
            "sources": []
        }"#;
        let config_path = env.plugin_dir().join(".skills-config.json");
        fs::write(&config_path, raw_json).unwrap();

        let config = Skills::load_config().expect("Minimal valid JSON should work");
        assert!(config.disabled.is_empty());
        assert!(config.custom.is_empty());
        assert!(config.sources.is_empty());
        // serde(default) fields should be empty
        assert!(config.skill_meta.is_empty());
        assert!(config.category_map.is_empty());
    }

    // ============================================
    // categoryMap Precedence Tests
    // ============================================

    #[test]
    fn test_get_category_from_config_map() {
        // A user-defined category in categoryMap should override defaults
        let mut category_map = std::collections::HashMap::new();
        category_map.insert("unknown-skill".to_string(), SkillCategory::Testing);

        let config = SkillsConfig {
            category_map,
            ..Default::default()
        };

        assert_eq!(
            Skills::get_category("unknown-skill", &config),
            SkillCategory::Testing
        );
    }

    #[test]
    fn test_get_category_hardcoded_check_when_no_map_entry() {
        // When no categoryMap entry, hardcoded constants apply
        let config = SkillsConfig::default();

        assert_eq!(
            Skills::get_category("rhinolabs-standards", &config),
            SkillCategory::Corporate
        );
        assert_eq!(
            Skills::get_category("rhinolabs-architecture", &config),
            SkillCategory::Corporate
        );
        assert_eq!(
            Skills::get_category("rhinolabs-security", &config),
            SkillCategory::Corporate
        );
    }

    #[test]
    fn test_get_category_unknown_skill_returns_custom() {
        let config = SkillsConfig::default();

        assert_eq!(
            Skills::get_category("some-random-skill-nobody-knows", &config),
            SkillCategory::Custom
        );
    }

    #[test]
    fn test_get_category_config_map_overrides_hardcoded() {
        // categoryMap should override even hardcoded corporate skills
        let mut category_map = std::collections::HashMap::new();
        category_map.insert("rhinolabs-standards".to_string(), SkillCategory::Utilities);

        let config = SkillsConfig {
            category_map,
            ..Default::default()
        };

        // categoryMap takes priority over CORPORATE_SKILLS constant
        assert_eq!(
            Skills::get_category("rhinolabs-standards", &config),
            SkillCategory::Utilities
        );
    }
}

#[cfg(test)]
mod skills_sh_tests {
    use super::*;

    #[test]
    fn test_extract_skills_sh_data_regular_json() {
        let html = r#"some prefix allTimeSkills":[{"source":"vercel-labs/skills","skillId":"find-skills","name":"find-skills","installs":98546},{"source":"test/repo","skillId":"test-skill","name":"Test Skill","installs":100}] more suffix"#;

        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok(), "Should parse regular JSON: {:?}", result);

        let skills = result.unwrap();
        assert_eq!(skills.len(), 2, "Should find 2 skills");

        assert_eq!(
            skills[0].get("source").and_then(|v| v.as_str()),
            Some("vercel-labs/skills")
        );
        assert_eq!(
            skills[0].get("skillId").and_then(|v| v.as_str()),
            Some("find-skills")
        );
        assert_eq!(
            skills[1].get("skillId").and_then(|v| v.as_str()),
            Some("test-skill")
        );
    }

    #[test]
    fn test_extract_skills_sh_data_escaped_json() {
        let html = r#"some prefix allTimeSkills\":[{\"source\":\"vercel-labs/skills\",\"skillId\":\"find-skills\",\"name\":\"find-skills\",\"installs\":98546},{\"source\":\"test/repo\",\"skillId\":\"test-skill\",\"name\":\"Test Skill\",\"installs\":100}] more suffix"#;

        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok(), "Should parse escaped JSON: {:?}", result);

        let skills = result.unwrap();
        assert_eq!(skills.len(), 2, "Should find 2 skills");

        assert_eq!(
            skills[0].get("source").and_then(|v| v.as_str()),
            Some("vercel-labs/skills")
        );
    }

    #[test]
    fn test_extract_skills_sh_data_deduplicates() {
        let html = r#"first [{"source":"a/b","skillId":"skill-1","name":"S1","installs":10}] second [{"source":"a/b","skillId":"skill-1","name":"S1","installs":10}]"#;

        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok());

        let skills = result.unwrap();
        assert_eq!(skills.len(), 1, "Should deduplicate by skillId");
    }

    #[test]
    fn test_extract_skills_sh_data_no_data() {
        let html = "<html><body>No skills here</body></html>";
        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_err(), "Should error on empty data");
    }

    #[test]
    fn test_extract_skills_sh_data_skips_missing_fields() {
        // Objects missing skillId should be skipped
        let html = r#"[{"source":"a/b","name":"no-id","installs":1},{"source":"c/d","skillId":"valid","name":"Valid","installs":2}]"#;
        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok());
        let skills = result.unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(
            skills[0].get("skillId").and_then(|v| v.as_str()),
            Some("valid")
        );
    }

    #[test]
    fn test_extract_skills_sh_data_skips_malformed_json() {
        // First object has truncated value (no closing quote on source), second is valid
        let html =
            r#"{"source":"bad} {"source":"a/b","skillId":"good","name":"Good","installs":5}"#;
        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok());
        let skills = result.unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(
            skills[0].get("skillId").and_then(|v| v.as_str()),
            Some("good")
        );
    }

    #[test]
    fn test_extract_skills_sh_data_large_dedup() {
        // Simulate many duplicates (like Next.js SSR hydration repeating data)
        let mut html = String::from("prefix ");
        for i in 0..100 {
            html.push_str(&format!(
                r#"{{"source":"owner/repo","skillId":"skill-{}","name":"Skill {}","installs":{}}}"#,
                i % 10, // Only 10 unique IDs
                i,
                i * 100
            ));
            html.push(',');
        }
        let result = Skills::extract_skills_sh_data(&html);
        assert!(result.is_ok());
        let skills = result.unwrap();
        assert_eq!(skills.len(), 10, "Should deduplicate to 10 unique skills");
    }

    #[test]
    fn test_extract_prefers_regular_over_escaped() {
        // If regular JSON is found, escaped fallback should NOT run
        let html = r#"[{"source":"a/b","skillId":"regular","name":"Regular","installs":1}] also {\"source\":\"c/d\",\"skillId\":\"escaped\",\"name\":\"Escaped\",\"installs\":2}"#;
        let result = Skills::extract_skills_sh_data(html);
        assert!(result.is_ok());
        let skills = result.unwrap();
        // Regular format found first, so escaped fallback is skipped
        assert!(skills
            .iter()
            .any(|s| s.get("skillId").and_then(|v| v.as_str()) == Some("regular")));
    }

    #[test]
    fn test_fetch_from_source_routes_standard_schema() {
        // Verify dispatch: Standard schema should NOT call skills_sh parser
        let source = SkillSource {
            id: "test".to_string(),
            name: "Test".to_string(),
            source_type: SkillSourceType::Community,
            url: "not-a-valid-github-url".to_string(),
            description: "".to_string(),
            enabled: true,
            fetchable: true,
            schema: SkillSchema::Standard,
            skill_count: None,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(Skills::fetch_from_source(&source));
        // Should fail because URL is invalid GitHub, NOT because of skills.sh parsing
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            !err.contains("skills.sh"),
            "Standard schema should NOT try skills.sh parser, got: {}",
            err
        );
    }

    #[test]
    fn test_fetch_from_source_rejects_custom_schema() {
        let source = SkillSource {
            id: "test".to_string(),
            name: "Test".to_string(),
            source_type: SkillSourceType::Community,
            url: "https://example.com".to_string(),
            description: "".to_string(),
            enabled: true,
            fetchable: true,
            schema: SkillSchema::Custom,
            skill_count: None,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(Skills::fetch_from_source(&source));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("cannot be fetched automatically"));
    }
}

#[cfg(test)]
mod skills_sh_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_from_skills_sh_real() {
        let source = SkillSource {
            id: "skills-sh-test".to_string(),
            name: "Skills.sh Test".to_string(),
            source_type: SkillSourceType::Community,
            url: "https://skills.sh".to_string(),
            description: "Test".to_string(),
            enabled: true,
            fetchable: true,
            schema: SkillSchema::SkillsSh,
            skill_count: None,
        };

        let result = Skills::fetch_from_source(&source).await;
        println!("Result: {:?}", result);

        assert!(result.is_ok(), "Should fetch skills: {:?}", result);
        let skills = result.unwrap();
        assert!(!skills.is_empty(), "Should have some skills");
        println!("Found {} skills", skills.len());
    }
}
