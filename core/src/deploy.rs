//! Deploy module for publishing and syncing configurations
//!
//! This module handles:
//! - Exporting current configuration (profiles, skills, settings, etc.)
//! - Publishing configuration to GitHub releases
//! - Syncing configuration from GitHub releases

use crate::{Paths, Profiles, Result, RhinolabsError, Settings, InstructionsManager, OutputStyles};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::ZipWriter;

/// Configuration bundle manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigManifest {
    pub version: String,
    pub created_at: String,
    pub profiles_count: usize,
    pub skills_count: usize,
    pub has_instructions: bool,
    pub has_settings: bool,
    pub output_styles_count: usize,
}

/// Deploy result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployResult {
    pub version: String,
    pub release_url: String,
    pub asset_url: String,
    pub manifest: ConfigManifest,
}

/// Sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    pub version: String,
    pub profiles_installed: usize,
    pub skills_installed: usize,
    pub instructions_installed: bool,
    pub settings_installed: bool,
    pub output_styles_installed: usize,
}

pub struct Deploy;

impl Deploy {
    /// Export current configuration to a zip file
    /// Returns the path to the created zip file
    pub fn export_config(output_path: &Path) -> Result<(PathBuf, ConfigManifest)> {
        let plugin_dir = Paths::plugin_dir()?;
        let config_dir = Paths::config_dir()?;

        // Create zip file
        let zip_path = output_path.join("rhinolabs-config.zip");
        let file = File::create(&zip_path)?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<'_, ()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let mut skills_count = 0;
        let mut output_styles_count = 0;

        // 1. Export profiles.json
        let profiles_path = config_dir.join("profiles.json");
        if profiles_path.exists() {
            let content = fs::read_to_string(&profiles_path)?;
            zip.start_file("profiles.json", options.clone())?;
            zip.write_all(content.as_bytes())?;
        }

        // 2. Export skills directory
        let skills_dir = plugin_dir.join("skills");
        if skills_dir.exists() {
            skills_count = Self::add_directory_to_zip(&mut zip, &skills_dir, "skills", &options)?;
        }

        // 3. Export CLAUDE.md (instructions)
        let instructions = InstructionsManager::get()?;
        let has_instructions = !instructions.content.is_empty();
        if has_instructions {
            zip.start_file("CLAUDE.md", options.clone())?;
            zip.write_all(instructions.content.as_bytes())?;
        }

        // 4. Export settings.json
        let settings = Settings::get()?;
        let settings_json = serde_json::to_string_pretty(&settings)?;
        zip.start_file("settings.json", options.clone())?;
        zip.write_all(settings_json.as_bytes())?;

        // 5. Export output-styles directory
        let styles_dir = plugin_dir.join("output-styles");
        if styles_dir.exists() {
            output_styles_count = Self::add_directory_to_zip(&mut zip, &styles_dir, "output-styles", &options)?;
        }

        // 6. Export .mcp.json if exists
        let mcp_path = plugin_dir.join(".mcp.json");
        if mcp_path.exists() {
            let content = fs::read_to_string(&mcp_path)?;
            zip.start_file(".mcp.json", options.clone())?;
            zip.write_all(content.as_bytes())?;
        }

        // 7. Export .skills-config.json if exists
        let skills_config_path = plugin_dir.join(".skills-config.json");
        if skills_config_path.exists() {
            let content = fs::read_to_string(&skills_config_path)?;
            zip.start_file(".skills-config.json", options.clone())?;
            zip.write_all(content.as_bytes())?;
        }

        // Count profiles
        let profiles = Profiles::list()?;
        let profiles_count = profiles.len();

        // Create manifest
        let manifest = ConfigManifest {
            version: Self::get_current_version()?,
            created_at: chrono::Utc::now().to_rfc3339(),
            profiles_count,
            skills_count,
            has_instructions,
            has_settings: true,
            output_styles_count,
        };

        // Add manifest to zip
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        zip.start_file("manifest.json", options)?;
        zip.write_all(manifest_json.as_bytes())?;

        zip.finish()?;

        Ok((zip_path, manifest))
    }

    /// Add a directory recursively to the zip file
    fn add_directory_to_zip<W: Write + std::io::Seek>(
        zip: &mut ZipWriter<W>,
        dir: &Path,
        prefix: &str,
        options: &FileOptions<'_, ()>,
    ) -> Result<usize> {
        let mut count = 0;

        if !dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Skip hidden files except specific ones
            if name_str.starts_with('.') && name_str != ".mcp.json" && name_str != ".skills-config.json" {
                continue;
            }

            let zip_path = format!("{}/{}", prefix, name_str);

            if path.is_dir() {
                count += Self::add_directory_to_zip(zip, &path, &zip_path, options)?;
            } else {
                let mut file = File::open(&path)?;
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;

                zip.start_file(&zip_path, options.clone())?;
                zip.write_all(&content)?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// Get current plugin version
    fn get_current_version() -> Result<String> {
        let plugin_dir = Paths::plugin_dir()?;
        let manifest_path = plugin_dir.join(".claude-plugin").join("plugin.json");

        if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path)?;
            let manifest: serde_json::Value = serde_json::from_str(&content)?;
            if let Some(version) = manifest["version"].as_str() {
                return Ok(version.to_string());
            }
        }

        Ok("1.0.0".to_string())
    }

    /// Deploy configuration to GitHub
    /// Creates a release and uploads the config bundle as an asset
    pub async fn deploy(version: &str, changelog: &str) -> Result<DeployResult> {
        // Get GitHub config
        let project_config = crate::Project::get_config()?;

        if project_config.github.owner.is_empty() || project_config.github.repo.is_empty() {
            return Err(RhinolabsError::ConfigError(
                "GitHub repository not configured. Configure it in Project Settings.".into(),
            ));
        }

        let token = std::env::var("GITHUB_TOKEN").map_err(|_| {
            RhinolabsError::ConfigError(
                "GITHUB_TOKEN environment variable not set".into(),
            )
        })?;

        // 1. Export config to temp directory
        let temp_dir = std::env::temp_dir().join("rhinolabs-deploy");
        fs::create_dir_all(&temp_dir)?;

        let (zip_path, manifest) = Self::export_config(&temp_dir)?;

        // 2. Create GitHub release
        let tag = format!("config-v{}", version);
        let release_body = format!(
            "{}\n\n## Configuration Summary\n- Profiles: {}\n- Skills: {}\n- Instructions: {}\n- Output Styles: {}",
            changelog,
            manifest.profiles_count,
            manifest.skills_count,
            if manifest.has_instructions { "Yes" } else { "No" },
            manifest.output_styles_count
        );

        let client = reqwest::Client::new();

        // Create release
        let release_url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            project_config.github.owner, project_config.github.repo
        );

        let release_body_json = serde_json::json!({
            "tag_name": tag,
            "name": format!("Configuration {}", tag),
            "body": release_body,
            "prerelease": false,
            "draft": false,
        });

        let response = client
            .post(&release_url)
            .header("User-Agent", "rhinolabs-cli")
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .json(&release_body_json)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            // Clean up temp files
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(RhinolabsError::Other(format!(
                "Failed to create release: {}",
                error_text
            )));
        }

        let release: serde_json::Value = response.json().await?;
        let release_id = release["id"].as_u64().ok_or_else(|| {
            RhinolabsError::Other("Failed to get release ID".into())
        })?;
        let release_html_url = release["html_url"]
            .as_str()
            .unwrap_or("")
            .to_string();

        // 3. Upload config zip as release asset
        let upload_url = format!(
            "https://uploads.github.com/repos/{}/{}/releases/{}/assets?name=rhinolabs-config.zip",
            project_config.github.owner, project_config.github.repo, release_id
        );

        let zip_content = fs::read(&zip_path)?;

        let upload_response = client
            .post(&upload_url)
            .header("User-Agent", "rhinolabs-cli")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/zip")
            .body(zip_content)
            .send()
            .await?;

        if !upload_response.status().is_success() {
            let error_text = upload_response.text().await.unwrap_or_default();
            // Clean up temp files
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(RhinolabsError::Other(format!(
                "Failed to upload config bundle: {}",
                error_text
            )));
        }

        let asset: serde_json::Value = upload_response.json().await?;
        let asset_url = asset["browser_download_url"]
            .as_str()
            .unwrap_or("")
            .to_string();

        // Clean up temp files
        let _ = fs::remove_dir_all(&temp_dir);

        Ok(DeployResult {
            version: version.to_string(),
            release_url: release_html_url,
            asset_url,
            manifest,
        })
    }

    /// Sync configuration from GitHub
    /// Downloads the latest config release and installs it
    pub async fn sync() -> Result<SyncResult> {
        // Get GitHub config
        let project_config = crate::Project::get_config()?;

        if project_config.github.owner.is_empty() || project_config.github.repo.is_empty() {
            return Err(RhinolabsError::ConfigError(
                "GitHub repository not configured. Configure it in Project Settings.".into(),
            ));
        }

        let client = reqwest::Client::new();

        // 1. Find the latest config release
        let releases_url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            project_config.github.owner, project_config.github.repo
        );

        let response = client
            .get(&releases_url)
            .header("User-Agent", "rhinolabs-cli")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::Other(
                "Failed to fetch releases from GitHub".into(),
            ));
        }

        let releases: Vec<serde_json::Value> = response.json().await?;

        // Find the latest config release (tag starts with "config-v")
        let config_release = releases.iter().find(|r| {
            r["tag_name"]
                .as_str()
                .map(|t| t.starts_with("config-v"))
                .unwrap_or(false)
        });

        let release = config_release.ok_or_else(|| {
            RhinolabsError::ConfigError(
                "No configuration release found. Deploy a configuration first.".into(),
            )
        })?;

        // Find the config zip asset
        let assets = release["assets"].as_array().ok_or_else(|| {
            RhinolabsError::Other("No assets found in release".into())
        })?;

        let config_asset = assets.iter().find(|a| {
            a["name"]
                .as_str()
                .map(|n| n == "rhinolabs-config.zip")
                .unwrap_or(false)
        });

        let asset = config_asset.ok_or_else(|| {
            RhinolabsError::ConfigError(
                "No config bundle found in release".into(),
            )
        })?;

        let download_url = asset["browser_download_url"]
            .as_str()
            .ok_or_else(|| RhinolabsError::Other("Invalid asset URL".into()))?;

        let version = release["tag_name"]
            .as_str()
            .unwrap_or("unknown")
            .trim_start_matches("config-v")
            .to_string();

        // 2. Download the config zip
        let zip_response = client
            .get(download_url)
            .header("User-Agent", "rhinolabs-cli")
            .send()
            .await?;

        if !zip_response.status().is_success() {
            return Err(RhinolabsError::Other(
                "Failed to download config bundle".into(),
            ));
        }

        let zip_content = zip_response.bytes().await?;

        // 3. Extract and install the config
        let result = Self::import_config(&zip_content)?;

        Ok(SyncResult {
            version,
            profiles_installed: result.0,
            skills_installed: result.1,
            instructions_installed: result.2,
            settings_installed: result.3,
            output_styles_installed: result.4,
        })
    }

    /// Import configuration from a zip buffer
    fn import_config(zip_content: &[u8]) -> Result<(usize, usize, bool, bool, usize)> {
        use std::io::Cursor;
        use zip::ZipArchive;

        let reader = Cursor::new(zip_content);
        let mut archive = ZipArchive::new(reader)?;

        let plugin_dir = Paths::plugin_dir()?;
        let config_dir = Paths::config_dir()?;

        let mut profiles_installed = 0;
        let mut skills_installed = 0;
        let mut instructions_installed = false;
        let mut settings_installed = false;
        let mut output_styles_installed = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();

            // Skip directories
            if name.ends_with('/') {
                continue;
            }

            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            if name == "profiles.json" {
                // Install profiles
                let target = config_dir.join("profiles.json");
                fs::create_dir_all(&config_dir)?;
                fs::write(&target, &content)?;
                // Count profiles in the file
                if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&content) {
                    if let Some(profiles) = json["profiles"].as_array() {
                        profiles_installed = profiles.len();
                    }
                }
            } else if name.starts_with("skills/") {
                // Install skill
                let relative_path = name.trim_start_matches("skills/");
                let target = plugin_dir.join("skills").join(relative_path);
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&target, &content)?;
                if name.ends_with("SKILL.md") {
                    skills_installed += 1;
                }
            } else if name == "CLAUDE.md" {
                // Install instructions
                let target = plugin_dir.join("CLAUDE.md");
                fs::write(&target, &content)?;
                instructions_installed = true;
            } else if name == "settings.json" {
                // Install settings
                let target = plugin_dir.join("settings.json");
                fs::write(&target, &content)?;
                settings_installed = true;
            } else if name.starts_with("output-styles/") {
                // Install output style
                let relative_path = name.trim_start_matches("output-styles/");
                let target = plugin_dir.join("output-styles").join(relative_path);
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&target, &content)?;
                if name.ends_with(".md") {
                    output_styles_installed += 1;
                }
            } else if name == ".mcp.json" {
                // Install MCP config
                let target = plugin_dir.join(".mcp.json");
                fs::write(&target, &content)?;
            } else if name == ".skills-config.json" {
                // Install skills config
                let target = plugin_dir.join(".skills-config.json");
                fs::write(&target, &content)?;
            }
        }

        Ok((
            profiles_installed,
            skills_installed,
            instructions_installed,
            settings_installed,
            output_styles_installed,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manifest_serialization() {
        let manifest = ConfigManifest {
            version: "1.0.0".to_string(),
            created_at: "2026-01-27T00:00:00Z".to_string(),
            profiles_count: 2,
            skills_count: 10,
            has_instructions: true,
            has_settings: true,
            output_styles_count: 3,
        };

        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("profilesCount"));
        assert!(json.contains("skillsCount"));
    }
}
