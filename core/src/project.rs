use crate::{Paths, Result, RhinolabsError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Default GitHub owner for plugin releases.
/// Change this when migrating to an organization account.
pub const DEFAULT_GITHUB_OWNER: &str = "javiermontescarrera";

/// Default GitHub repository for plugin releases.
pub const DEFAULT_GITHUB_REPO: &str = "rhinolabs-ai";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubConfig {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            owner: DEFAULT_GITHUB_OWNER.into(),
            repo: DEFAULT_GITHUB_REPO.into(),
            branch: "main".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAsset {
    pub name: String,
    pub path: String,
    pub description: String,
}

impl Default for ReleaseAsset {
    fn default() -> Self {
        Self {
            name: "rhinolabs-claude.zip".into(),
            path: ".".into(),
            description: "Plugin files".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub github: GitHubConfig,
    pub assets: Vec<ReleaseAsset>,
    #[serde(default)]
    pub auto_changelog: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            github: GitHubConfig::default(),
            assets: vec![ReleaseAsset::default()],
            auto_changelog: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatus {
    pub is_configured: bool,
    pub has_git: bool,
    pub current_branch: Option<String>,
    pub has_remote: bool,
    pub remote_url: Option<String>,
    pub has_uncommitted_changes: bool,
    pub plugin_version: Option<String>,
    pub latest_release: Option<String>,
}

pub struct Project;

impl Project {
    /// Get the project config file path
    fn config_path() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join(".project.json"))
    }

    /// Get project configuration
    pub fn get_config() -> Result<ProjectConfig> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(ProjectConfig::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: ProjectConfig = serde_json::from_str(&content)?;

        Ok(config)
    }

    /// Update project configuration
    pub fn update_config(config: &ProjectConfig) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = serde_json::to_string_pretty(config)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Get current project status
    pub fn get_status() -> Result<ProjectStatus> {
        let plugin_dir = Paths::plugin_dir()?;
        let config = Self::get_config().ok();

        // Check if configured
        let is_configured = config
            .as_ref()
            .map(|c| !c.github.owner.is_empty() && !c.github.repo.is_empty())
            .unwrap_or(false);

        // Git status
        let (has_git, current_branch, has_remote, remote_url, has_uncommitted_changes) =
            Self::get_git_status(&plugin_dir);

        // Plugin version from manifest
        let plugin_version = Self::get_plugin_version(&plugin_dir);

        // Latest release (only if configured)
        let latest_release = None; // Will be fetched async separately

        Ok(ProjectStatus {
            is_configured,
            has_git,
            current_branch,
            has_remote,
            remote_url,
            has_uncommitted_changes,
            plugin_version,
            latest_release,
        })
    }

    fn get_git_status(dir: &PathBuf) -> (bool, Option<String>, bool, Option<String>, bool) {
        // Check if .git exists
        let git_dir = dir.join(".git");
        if !git_dir.exists() {
            // Check parent directories
            let mut current = dir.parent();
            let mut has_git = false;
            while let Some(parent) = current {
                if parent.join(".git").exists() {
                    has_git = true;
                    break;
                }
                current = parent.parent();
            }
            if !has_git {
                return (false, None, false, None, false);
            }
        }

        // Get current branch
        let branch = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(dir)
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            });

        // Get remote URL
        let remote_url = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(dir)
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            });

        let has_remote = remote_url.is_some();

        // Check for uncommitted changes
        let has_changes = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(dir)
            .output()
            .ok()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false);

        (true, branch, has_remote, remote_url, has_changes)
    }

    fn get_plugin_version(dir: &Path) -> Option<String> {
        let manifest_path = dir.join(".claude-plugin").join("plugin.json");
        if manifest_path.exists() {
            fs::read_to_string(&manifest_path)
                .ok()
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|v| v["version"].as_str().map(|s| s.to_string()))
        } else {
            None
        }
    }

    /// Fetch latest release from GitHub
    pub async fn fetch_latest_release() -> Result<Option<String>> {
        let config = Self::get_config()?;

        if config.github.owner.is_empty() || config.github.repo.is_empty() {
            return Ok(None);
        }

        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            config.github.owner, config.github.repo
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "rhinolabs-gui")
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let release: serde_json::Value = resp.json().await?;
                Ok(release["tag_name"]
                    .as_str()
                    .map(|s| s.trim_start_matches('v').to_string()))
            }
            _ => Ok(None),
        }
    }

    /// Create a new release
    pub async fn create_release(
        version: &str,
        changelog: &str,
        prerelease: bool,
    ) -> Result<String> {
        let config = Self::get_config()?;

        if config.github.owner.is_empty() || config.github.repo.is_empty() {
            return Err(RhinolabsError::ConfigError(
                "GitHub repository not configured".into(),
            ));
        }

        // Get GitHub token from environment
        let token = std::env::var("GITHUB_TOKEN").map_err(|_| {
            RhinolabsError::ConfigError("GITHUB_TOKEN environment variable not set".into())
        })?;

        let url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            config.github.owner, config.github.repo
        );

        let tag = format!("v{}", version);
        let body = serde_json::json!({
            "tag_name": tag,
            "name": format!("Release {}", tag),
            "body": changelog,
            "prerelease": prerelease,
            "draft": false,
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("User-Agent", "rhinolabs-gui")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(RhinolabsError::Other(format!(
                "Failed to create release: {}",
                error_text
            )));
        }

        let release: serde_json::Value = response.json().await?;
        let release_url = release["html_url"].as_str().unwrap_or("").to_string();

        Ok(release_url)
    }

    /// Bump version in plugin.json
    pub fn bump_version(bump_type: &str) -> Result<String> {
        let plugin_dir = Paths::plugin_dir()?;
        let manifest_path = plugin_dir.join(".claude-plugin").join("plugin.json");

        if !manifest_path.exists() {
            return Err(RhinolabsError::ConfigError("plugin.json not found".into()));
        }

        let content = fs::read_to_string(&manifest_path)?;
        let mut manifest: serde_json::Value = serde_json::from_str(&content)?;

        let current_version = manifest["version"]
            .as_str()
            .ok_or_else(|| RhinolabsError::ConfigError("version not found in manifest".into()))?;

        let new_version = Self::calculate_new_version(current_version, bump_type)?;

        manifest["version"] = serde_json::Value::String(new_version.clone());

        let new_content = serde_json::to_string_pretty(&manifest)?;
        fs::write(&manifest_path, new_content)?;

        Ok(new_version)
    }

    fn calculate_new_version(current: &str, bump_type: &str) -> Result<String> {
        let version = semver::Version::parse(current)
            .map_err(|e| RhinolabsError::InvalidVersion(e.to_string()))?;

        let new_version = match bump_type {
            "major" => semver::Version::new(version.major + 1, 0, 0),
            "minor" => semver::Version::new(version.major, version.minor + 1, 0),
            "patch" => semver::Version::new(version.major, version.minor, version.patch + 1),
            _ => {
                return Err(RhinolabsError::ConfigError(format!(
                    "Invalid bump type: {}. Use major, minor, or patch",
                    bump_type
                )))
            }
        };

        Ok(new_version.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_default() {
        let config = ProjectConfig::default();

        assert_eq!(config.github.owner, DEFAULT_GITHUB_OWNER);
        assert_eq!(config.github.repo, DEFAULT_GITHUB_REPO);
        assert_eq!(config.github.branch, "main");
        assert!(!config.assets.is_empty());
        assert!(config.auto_changelog);
    }

    #[test]
    fn test_bump_version_patch() {
        let result = Project::calculate_new_version("1.0.0", "patch");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1.0.1");
    }

    #[test]
    fn test_bump_version_minor() {
        let result = Project::calculate_new_version("1.2.3", "minor");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1.3.0");
    }

    #[test]
    fn test_bump_version_major() {
        let result = Project::calculate_new_version("1.2.3", "major");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2.0.0");
    }

    #[test]
    fn test_bump_version_invalid() {
        let result = Project::calculate_new_version("1.0.0", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_github_config_serialization() {
        let config = GitHubConfig {
            owner: "rhinolabs".into(),
            repo: "rhinolabs-ai".into(),
            branch: "main".into(),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("rhinolabs"));

        let parsed: GitHubConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.owner, "rhinolabs");
    }
}
