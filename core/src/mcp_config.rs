use crate::{Paths, Result, RhinolabsError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpSettings {
    pub default_timeout: u32,
    pub retry_attempts: u32,
    pub log_level: String,
}

impl Default for McpSettings {
    fn default() -> Self {
        Self {
            default_timeout: 30000,
            retry_attempts: 3,
            log_level: "info".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _note: Option<String>,
    pub mcp_servers: HashMap<String, McpServer>,
    #[serde(default)]
    pub settings: McpSettings,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            _note: Some("This file is managed by Rhinolabs GUI. See docs/MCP_CENTRALIZED_CONFIG.md for details.".into()),
            mcp_servers: HashMap::new(),
            settings: McpSettings::default(),
        }
    }
}

pub struct McpConfigManager;

impl McpConfigManager {
    /// Get the MCP config file path
    fn config_path() -> Result<PathBuf> {
        Paths::mcp_config_path()
    }

    /// Get the full MCP config
    pub fn get() -> Result<McpConfig> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(McpConfig::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: McpConfig = serde_json::from_str(&content)?;

        Ok(config)
    }

    /// Update the full MCP config
    /// Creates the directory if it doesn't exist
    pub fn update(config: &McpConfig) -> Result<()> {
        let path = Self::config_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = serde_json::to_string_pretty(config)?;
        fs::write(&path, content)?;

        Ok(())
    }

    // ========================================
    // MCP Servers
    // ========================================

    /// List all MCP servers
    pub fn list_servers() -> Result<HashMap<String, McpServer>> {
        let config = Self::get()?;
        Ok(config.mcp_servers)
    }

    /// Get a specific MCP server
    pub fn get_server(name: &str) -> Result<Option<McpServer>> {
        let config = Self::get()?;
        Ok(config.mcp_servers.get(name).cloned())
    }

    /// Add a new MCP server
    pub fn add_server(name: &str, server: McpServer) -> Result<()> {
        let mut config = Self::get()?;

        if config.mcp_servers.contains_key(name) {
            return Err(RhinolabsError::ConfigError(
                format!("MCP server '{}' already exists", name)
            ));
        }

        config.mcp_servers.insert(name.to_string(), server);
        Self::update(&config)
    }

    /// Update an existing MCP server
    pub fn update_server(name: &str, server: McpServer) -> Result<()> {
        let mut config = Self::get()?;

        if !config.mcp_servers.contains_key(name) {
            return Err(RhinolabsError::ConfigError(
                format!("MCP server '{}' not found", name)
            ));
        }

        config.mcp_servers.insert(name.to_string(), server);
        Self::update(&config)
    }

    /// Remove an MCP server
    pub fn remove_server(name: &str) -> Result<()> {
        let mut config = Self::get()?;

        if config.mcp_servers.remove(name).is_none() {
            return Err(RhinolabsError::ConfigError(
                format!("MCP server '{}' not found", name)
            ));
        }

        Self::update(&config)
    }

    // ========================================
    // MCP Settings
    // ========================================

    /// Get MCP settings
    pub fn get_settings() -> Result<McpSettings> {
        let config = Self::get()?;
        Ok(config.settings)
    }

    /// Update MCP settings
    pub fn update_settings(settings: McpSettings) -> Result<()> {
        let mut config = Self::get()?;
        config.settings = settings;
        Self::update(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_config_default() {
        let config = McpConfig::default();

        assert!(config.mcp_servers.is_empty());
        assert_eq!(config.settings.default_timeout, 30000);
        assert_eq!(config.settings.retry_attempts, 3);
        assert_eq!(config.settings.log_level, "info");
    }

    #[test]
    fn test_mcp_settings_default() {
        let settings = McpSettings::default();

        assert_eq!(settings.default_timeout, 30000);
        assert_eq!(settings.retry_attempts, 3);
        assert_eq!(settings.log_level, "info");
    }

    #[test]
    fn test_mcp_server_serialization() {
        let server = McpServer {
            command: "npx".into(),
            args: vec!["-y".into(), "@modelcontextprotocol/server-git".into()],
            env: None,
        };

        let json = serde_json::to_string(&server).unwrap();
        let deserialized: McpServer = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.command, "npx");
        assert_eq!(deserialized.args.len(), 2);
    }

    #[test]
    fn test_mcp_server_with_env() {
        let mut env = HashMap::new();
        env.insert("TOKEN".into(), "secret".into());

        let server = McpServer {
            command: "node".into(),
            args: vec!["server.js".into()],
            env: Some(env),
        };

        let json = serde_json::to_string(&server).unwrap();
        assert!(json.contains("TOKEN"));
        assert!(json.contains("secret"));
    }

    #[test]
    fn test_mcp_config_path() {
        let path = McpConfigManager::config_path();
        assert!(path.is_ok());

        let path = path.unwrap();
        assert!(path.to_str().unwrap().contains(".mcp.json"));
    }
}
