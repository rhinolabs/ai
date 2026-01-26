use crate::{Paths, Result, RhinolabsError};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SkillCategory {
    Corporate,
    Frontend,
    Testing,
    AiSdk,
    Utilities,
    Custom,
}

impl Default for SkillCategory {
    fn default() -> Self {
        Self::Custom
    }
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
}

/// Built-in skill categories
const CORPORATE_SKILLS: &[&str] = &["rhinolabs-standards", "rhinolabs-architecture", "rhinolabs-security"];
const FRONTEND_SKILLS: &[&str] = &["react-patterns", "typescript-best-practices", "tailwind-4", "zod-4", "zustand-5"];
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
    fn get_category(id: &str) -> SkillCategory {
        if CORPORATE_SKILLS.contains(&id) {
            SkillCategory::Corporate
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
                "Skill file must start with YAML frontmatter".into()
            ));
        }

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(RhinolabsError::ConfigError(
                "Invalid frontmatter format".into()
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
            return Err(RhinolabsError::ConfigError(
                format!("SKILL.md not found in {:?}", dir)
            ));
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
            SkillCategory::Custom
        } else {
            Self::get_category(&id)
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
            skill_file.metadata()
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
                SkillCategory::Frontend => 1,
                SkillCategory::Testing => 2,
                SkillCategory::AiSdk => 3,
                SkillCategory::Utilities => 4,
                SkillCategory::Custom => 5,
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
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' not found", id)
            ));
        }

        Ok(dir)
    }

    /// Create a new custom skill
    pub fn create(input: CreateSkillInput) -> Result<Skill> {
        let skill_dir = Self::skills_dir()?.join(&input.id);

        if skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' already exists", input.id)
            ));
        }

        // Create skill directory
        fs::create_dir_all(&skill_dir)?;

        // Create SKILL.md
        let file_content = Self::generate_skill_file(&input.name, &input.description, &input.content);
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, &file_content)?;

        // Update config to mark as custom
        let mut config = Self::load_config()?;
        config.custom.push(input.id.clone());
        Self::save_config(&config)?;

        // Return the created skill
        let config = Self::load_config()?;
        Self::load_from_dir(&skill_dir, &config)
    }

    /// Update an existing skill
    pub fn update(id: &str, input: UpdateSkillInput) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' not found", id)
            ));
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
        let file_content = Self::generate_skill_file(&skill.name, &skill.description, &skill.content);
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, &file_content)?;

        // Handle enabled toggle
        if let Some(enabled) = input.enabled {
            Self::toggle(id, enabled)?;
        }

        Ok(())
    }

    /// Toggle skill enabled state
    pub fn toggle(id: &str, enabled: bool) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' not found", id)
            ));
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
        let has_source = config.skill_meta.get(id).map(|m| m.source_id.is_some()).unwrap_or(false);

        if !is_custom && !has_source {
            return Err(RhinolabsError::ConfigError(
                format!("Cannot delete built-in skill '{}'. You can only disable it.", id)
            ));
        }

        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' not found", id)
            ));
        }

        // Remove directory
        fs::remove_dir_all(&skill_dir)?;

        // Update config
        let mut config = Self::load_config()?;
        config.custom.retain(|s| s != id);
        config.disabled.retain(|s| s != id);
        config.skill_meta.remove(id);
        Self::save_config(&config)?;

        Ok(())
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
        let sources: Vec<SkillSource> = config.sources.into_iter().map(|mut s| {
            // For known default sources, ensure fetchable and schema are set correctly
            if let Some(default) = default_sources.iter().find(|d| d.id == s.id) {
                // Only override if the source hasn't been explicitly set to fetchable
                // We check if it's false (the old default) and the default is true
                if !s.fetchable && default.fetchable {
                    s.fetchable = default.fetchable;
                }
            }
            s
        }).collect();

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
            return Err(RhinolabsError::ConfigError(
                format!("Source '{}' already exists", source.id)
            ));
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

        let source = config.sources.iter_mut()
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
            return Err(RhinolabsError::ConfigError(
                format!("Cannot remove default source '{}'. You can disable it instead.", id)
            ));
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
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' already exists", skill_id)
            ));
        }

        // Create skill directory
        fs::create_dir_all(&skill_dir)?;

        // Write SKILL.md
        let skill_file = skill_dir.join("SKILL.md");
        fs::write(&skill_file, skill_content)?;

        // Update config with source metadata
        let mut config = Self::load_config()?;
        let content_hash = Self::hash_content(skill_content);

        config.skill_meta.insert(skill_id.to_string(), SkillMeta {
            source_id: Some(source_id.to_string()),
            source_name: Some(source_name.to_string()),
            original_hash: Some(content_hash),
        });

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
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' already exists", skill_id)
            ));
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

        config.skill_meta.insert(skill_id.to_string(), SkillMeta {
            source_id: Some(source_id.to_string()),
            source_name: Some(source_name.to_string()),
            original_hash: Some(content_hash),
        });

        Self::save_config(&config)?;

        // Return the installed skill
        let config = Self::load_config()?;
        Self::load_from_dir(&skill_dir, &config)
    }

    /// Reset a modified skill to its original content
    pub fn reset_to_original(id: &str, original_content: &str) -> Result<()> {
        let skill_dir = Self::skills_dir()?.join(id);

        if !skill_dir.exists() {
            return Err(RhinolabsError::ConfigError(
                format!("Skill '{}' not found", id)
            ));
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

    /// Fetch skills from a GitHub repository
    /// Expects URL format: https://github.com/owner/repo
    pub async fn fetch_from_github(source: &SkillSource) -> Result<Vec<RemoteSkill>> {
        // Parse GitHub URL to get owner/repo
        let url = &source.url;
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();

        if parts.len() < 2 {
            return Err(RhinolabsError::ConfigError(
                "Invalid GitHub URL format".into()
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
            return Err(RhinolabsError::NetworkError(
                format!("GitHub API error {}: {}", status, body)
            ));
        }

        let contents: Vec<GitHubContent> = response
            .json()
            .await
            .map_err(|e| RhinolabsError::NetworkError(format!("Failed to parse GitHub response: {}", e)))?;

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
                    Ok(skill_content) => {
                        match Self::parse_skill_file(&skill_content) {
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
                        }
                    }
                    Err(e) => {
                        eprintln!("[WARN] Failed to fetch SKILL.md for '{}': {}", item.name, e);
                    }
                }
            }
        }

        Ok(remote_skills)
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
            return Err(RhinolabsError::NetworkError(
                format!("Failed to fetch: {}", response.status())
            ));
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
    pub async fn fetch_remote_skill_files(source_url: &str, skill_id: &str) -> Result<Vec<RemoteSkillFile>> {
        // Validate inputs
        if skill_id.is_empty() {
            return Err(RhinolabsError::ConfigError("skill_id cannot be empty".into()));
        }

        // Parse GitHub URL to get owner/repo
        let parts: Vec<&str> = source_url.trim_end_matches('/').split('/').collect();

        if parts.len() < 2 {
            return Err(RhinolabsError::ConfigError(
                format!("Invalid GitHub URL format: {}", source_url)
            ));
        }

        let repo = parts[parts.len() - 1];
        let owner = parts[parts.len() - 2];

        let client = reqwest::Client::new();
        let mut files = Vec::new();

        let path = format!("skills/{}", skill_id);

        // Recursively fetch directory contents
        Self::fetch_github_directory_contents(
            &client,
            owner,
            repo,
            &path,
            "",
            &mut files,
        ).await?;

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
            return Err(RhinolabsError::NetworkError(
                format!("Failed to fetch '{}' from GitHub: HTTP {}", path, response.status())
            ));
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
                )).await?;
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
    fn test_get_category() {
        assert_eq!(Skills::get_category("rhinolabs-standards"), SkillCategory::Corporate);
        assert_eq!(Skills::get_category("react-patterns"), SkillCategory::Frontend);
        assert_eq!(Skills::get_category("playwright"), SkillCategory::Testing);
        assert_eq!(Skills::get_category("ai-sdk-core"), SkillCategory::AiSdk);
        assert_eq!(Skills::get_category("skill-creator"), SkillCategory::Utilities);
        assert_eq!(Skills::get_category("unknown-skill"), SkillCategory::Custom);
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
}
