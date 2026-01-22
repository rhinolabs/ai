use crate::{Result, RhinolabsError, Paths};
use serde_json::Value;
use std::fs;

pub enum McpSource {
    Remote(String),
    Local(String),
}

pub struct McpSync {
    source: McpSource,
    dry_run: bool,
}

impl McpSync {
    pub fn from_remote(url: String) -> Self {
        Self {
            source: McpSource::Remote(url),
            dry_run: false,
        }
    }

    pub fn from_local(path: String) -> Self {
        Self {
            source: McpSource::Local(path),
            dry_run: false,
        }
    }

    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    /// Sync MCP configuration
    pub async fn sync(&self) -> Result<()> {
        // Check if plugin is installed
        if !Paths::is_plugin_installed() {
            return Err(RhinolabsError::PluginNotInstalled);
        }

        // Fetch configuration
        let config_json = match &self.source {
            McpSource::Remote(url) => self.fetch_remote(url).await?,
            McpSource::Local(path) => self.read_local(path)?,
        };

        // Validate JSON
        let _config: Value = serde_json::from_str(&config_json)?;

        if self.dry_run {
            println!("[DRY RUN] Would update MCP configuration");
            println!("{}", config_json);
            return Ok(());
        }

        // Backup current config
        self.backup_current_config()?;

        // Write new config
        let config_path = Paths::mcp_config_path()?;
        fs::write(config_path, config_json)?;

        Ok(())
    }

    /// Fetch configuration from remote URL
    async fn fetch_remote(&self, url: &str) -> Result<String> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(RhinolabsError::McpSyncFailed(
                format!("HTTP {}", response.status())
            ));
        }

        let text = response.text().await?;
        Ok(text)
    }

    /// Read configuration from local file
    fn read_local(&self, path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    /// Backup current MCP config
    fn backup_current_config(&self) -> Result<()> {
        let config_path = Paths::mcp_config_path()?;

        if config_path.exists() {
            let backup_path = config_path.parent()
                .ok_or_else(|| RhinolabsError::Other("Invalid config path".into()))?
                .join(format!(
                    ".mcp.json.backup.{}",
                    chrono::Utc::now().format("%Y%m%d_%H%M%S")
                ));

            fs::copy(&config_path, &backup_path)?;
        }

        Ok(())
    }
}
