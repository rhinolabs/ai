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
            return Err(RhinolabsError::DownloadFailed("Could not fetch release info".into()));
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

        Err(RhinolabsError::DownloadFailed("Plugin asset not found in release".into()))
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
}
