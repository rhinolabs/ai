use crate::{Result, RhinolabsError};
use semver::Version as SemVersion;
use serde::{Deserialize, Serialize};
use std::fs;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

impl Version {
    /// Get current version from Cargo.toml
    pub fn current() -> String {
        CURRENT_VERSION.to_string()
    }

    /// Get installed plugin version
    pub fn installed() -> Result<Option<Self>> {
        let version_file = crate::Paths::version_file_path()?;

        if !version_file.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&version_file)?;
        let version: Version = serde_json::from_str(&content)?;
        Ok(Some(version))
    }

    /// Save version info
    pub fn save(&self) -> Result<()> {
        let version_file = crate::Paths::version_file_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(version_file, content)?;
        Ok(())
    }

    /// Check if update is available
    pub async fn check_update() -> Result<Option<String>> {
        let url = "https://api.github.com/repos/rhinolabs/rhinolabs-ai/releases/latest";

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "rhinolabs-cli")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::Other("Failed to check for updates".into()));
        }

        let release: serde_json::Value = response.json().await?;
        let latest_tag = release["tag_name"]
            .as_str()
            .ok_or_else(|| RhinolabsError::Other("Invalid release response".into()))?;

        // Remove 'v' prefix if present
        let latest_version = latest_tag.trim_start_matches('v');
        let current_version = Self::current();

        let latest = SemVersion::parse(latest_version)
            .map_err(|e| RhinolabsError::InvalidVersion(e.to_string()))?;
        let current = SemVersion::parse(&current_version)
            .map_err(|e| RhinolabsError::InvalidVersion(e.to_string()))?;

        if latest > current {
            Ok(Some(latest_version.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Get download URL for latest release
    pub async fn get_latest_download_url() -> Result<String> {
        let url = "https://api.github.com/repos/rhinolabs/rhinolabs-ai/releases/latest";

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "rhinolabs-cli")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::DownloadFailed(
                "Could not fetch release info".into(),
            ));
        }

        let release: serde_json::Value = response.json().await?;
        let assets = release["assets"]
            .as_array()
            .ok_or_else(|| RhinolabsError::DownloadFailed("No assets found".into()))?;

        // Find rhinolabs-claude.zip asset
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.starts_with("rhinolabs-claude") && name.ends_with(".zip") {
                    if let Some(download_url) = asset["browser_download_url"].as_str() {
                        return Ok(download_url.to_string());
                    }
                }
            }
        }

        Err(RhinolabsError::DownloadFailed(
            "Plugin asset not found in release".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version() {
        let version = Version::current();
        assert!(!version.is_empty());
        assert!(SemVersion::parse(&version).is_ok());
    }

    #[test]
    fn test_version_struct_serialization() {
        let version = Version {
            version: "1.2.3".to_string(),
            installed_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&version).unwrap();
        assert!(json.contains("1.2.3"));
        assert!(json.contains("installed_at"));

        // Deserialize back
        let parsed: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, "1.2.3");
    }

    #[test]
    fn test_version_comparison_logic() {
        // Test semver comparison logic used in check_update
        let v1 = SemVersion::parse("1.0.0").unwrap();
        let v2 = SemVersion::parse("1.0.1").unwrap();
        let v3 = SemVersion::parse("2.0.0").unwrap();

        assert!(v2 > v1); // Patch update
        assert!(v3 > v2); // Major update
        assert!(v3 > v1); // Major > patch

        // Same version
        let v1_dup = SemVersion::parse("1.0.0").unwrap();
        assert!(v1 <= v1_dup);
        assert!(v1 >= v1_dup);
    }

    #[test]
    fn test_parse_github_release_response() {
        // Simulate GitHub API response structure
        let mock_response = r#"{
            "tag_name": "v1.2.3",
            "name": "Release v1.2.3",
            "assets": [
                {
                    "name": "rhinolabs-claude.zip",
                    "browser_download_url": "https://github.com/rhinolabs/rhinolabs-ai/releases/download/v1.2.3/rhinolabs-claude.zip"
                },
                {
                    "name": "rhinolabs-linux-x64",
                    "browser_download_url": "https://github.com/rhinolabs/rhinolabs-ai/releases/download/v1.2.3/rhinolabs-linux-x64"
                }
            ]
        }"#;

        let release: serde_json::Value = serde_json::from_str(mock_response).unwrap();

        // Test tag extraction
        let tag = release["tag_name"].as_str().unwrap();
        assert_eq!(tag, "v1.2.3");

        // Test 'v' prefix removal
        let version = tag.trim_start_matches('v');
        assert_eq!(version, "1.2.3");
        assert!(SemVersion::parse(version).is_ok());

        // Test asset finding
        let assets = release["assets"].as_array().unwrap();
        let plugin_asset = assets.iter().find(|a| {
            a["name"]
                .as_str()
                .map(|n| n.starts_with("rhinolabs-claude") && n.ends_with(".zip"))
                .unwrap_or(false)
        });

        assert!(plugin_asset.is_some());
        let download_url = plugin_asset.unwrap()["browser_download_url"]
            .as_str()
            .unwrap();
        assert!(download_url.contains("rhinolabs-claude.zip"));
    }

    #[test]
    fn test_parse_github_response_without_v_prefix() {
        let mock_response = r#"{
            "tag_name": "1.2.3",
            "assets": []
        }"#;

        let release: serde_json::Value = serde_json::from_str(mock_response).unwrap();
        let tag = release["tag_name"].as_str().unwrap();

        // trim_start_matches should handle both cases
        let version = tag.trim_start_matches('v');
        assert_eq!(version, "1.2.3");
    }

    #[test]
    fn test_parse_github_response_missing_asset() {
        let mock_response = r#"{
            "tag_name": "v1.0.0",
            "assets": [
                {
                    "name": "other-file.txt",
                    "browser_download_url": "https://example.com/other.txt"
                }
            ]
        }"#;

        let release: serde_json::Value = serde_json::from_str(mock_response).unwrap();
        let assets = release["assets"].as_array().unwrap();

        let plugin_asset = assets.iter().find(|a| {
            a["name"]
                .as_str()
                .map(|n| n.starts_with("rhinolabs-claude") && n.ends_with(".zip"))
                .unwrap_or(false)
        });

        // Should NOT find the asset
        assert!(plugin_asset.is_none());
    }

    #[test]
    fn test_version_save_and_load_roundtrip() {
        let temp_dir = tempfile::tempdir().unwrap();
        let version_file = temp_dir.path().join(".version");

        let original = Version {
            version: "2.0.0".to_string(),
            installed_at: chrono::Utc::now(),
        };

        // Save
        let content = serde_json::to_string_pretty(&original).unwrap();
        std::fs::write(&version_file, &content).unwrap();

        // Load
        let loaded_content = std::fs::read_to_string(&version_file).unwrap();
        let loaded: Version = serde_json::from_str(&loaded_content).unwrap();

        assert_eq!(loaded.version, original.version);
    }

    #[test]
    fn test_semver_prerelease_comparison() {
        // Test that prerelease versions are handled correctly
        let stable = SemVersion::parse("1.0.0").unwrap();
        let beta = SemVersion::parse("1.0.0-beta.1").unwrap();
        let alpha = SemVersion::parse("1.0.0-alpha.1").unwrap();

        // Stable > beta > alpha (for same base version)
        assert!(stable > beta);
        assert!(stable > alpha);
        assert!(beta > alpha);
    }
}
