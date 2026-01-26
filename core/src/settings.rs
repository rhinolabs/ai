use crate::{Paths, Result, RhinolabsError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusLineConfig {
    #[serde(rename = "type")]
    pub line_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub padding: i32,
}

impl Default for StatusLineConfig {
    fn default() -> Self {
        Self {
            line_type: "command".into(),
            command: Some("~/.claude/statusline.sh".into()),
            text: None,
            padding: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionConfig {
    pub commit: String,
    pub pr: String,
}

impl Default for AttributionConfig {
    fn default() -> Self {
        Self {
            commit: String::new(),
            pr: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub deny: Vec<String>,
    pub ask: Vec<String>,
    pub allow: Vec<String>,
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            deny: vec![
                "Read(.env)".into(),
                "Read(.env.*)".into(),
                "Read(**/secrets/**)".into(),
            ],
            ask: vec![
                "Bash(git commit:*)".into(),
                "Bash(git push:*)".into(),
            ],
            allow: vec![
                "Read".into(),
                "Edit".into(),
                "Write".into(),
                "Glob".into(),
                "Grep".into(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginSettings {
    pub output_style: String,
    pub env: HashMap<String, String>,
    pub attribution: AttributionConfig,
    pub status_line: StatusLineConfig,
    pub permissions: PermissionConfig,
}

impl Default for PluginSettings {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("ENABLE_TOOL_SEARCH".into(), "true".into());

        Self {
            output_style: "Rhinolabs".into(),
            env,
            attribution: AttributionConfig::default(),
            status_line: StatusLineConfig::default(),
            permissions: PermissionConfig::default(),
        }
    }
}

pub struct Settings;

impl Settings {
    /// Get the path to settings.json
    fn settings_path() -> Result<PathBuf> {
        Ok(Paths::plugin_dir()?.join("settings.json"))
    }

    /// Read the plugin settings
    /// Returns default settings if file doesn't exist
    pub fn get() -> Result<PluginSettings> {
        let path = Self::settings_path()?;

        if !path.exists() {
            // Return default settings instead of error
            return Ok(PluginSettings::default());
        }

        let content = fs::read_to_string(&path)?;
        let settings: PluginSettings = serde_json::from_str(&content)?;

        Ok(settings)
    }

    /// Update the plugin settings
    /// Creates the directory if it doesn't exist
    pub fn update(settings: &PluginSettings) -> Result<()> {
        let path = Self::settings_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = serde_json::to_string_pretty(settings)?;
        fs::write(&path, content)?;

        Ok(())
    }

    // ========================================
    // Permissions
    // ========================================

    /// Get permissions config
    pub fn get_permissions() -> Result<PermissionConfig> {
        let settings = Self::get()?;
        Ok(settings.permissions)
    }

    /// Update permissions config
    pub fn update_permissions(permissions: PermissionConfig) -> Result<()> {
        let mut settings = Self::get()?;
        settings.permissions = permissions;
        Self::update(&settings)
    }

    /// Add a permission to a category
    pub fn add_permission(permission_type: &str, permission: &str) -> Result<()> {
        let mut settings = Self::get()?;

        let list = match permission_type {
            "deny" => &mut settings.permissions.deny,
            "ask" => &mut settings.permissions.ask,
            "allow" => &mut settings.permissions.allow,
            _ => return Err(RhinolabsError::ConfigError(
                format!("Invalid permission type: {}", permission_type)
            )),
        };

        if !list.contains(&permission.to_string()) {
            list.push(permission.to_string());
        }

        Self::update(&settings)
    }

    /// Remove a permission from a category
    pub fn remove_permission(permission_type: &str, permission: &str) -> Result<()> {
        let mut settings = Self::get()?;

        let list = match permission_type {
            "deny" => &mut settings.permissions.deny,
            "ask" => &mut settings.permissions.ask,
            "allow" => &mut settings.permissions.allow,
            _ => return Err(RhinolabsError::ConfigError(
                format!("Invalid permission type: {}", permission_type)
            )),
        };

        list.retain(|p| p != permission);

        Self::update(&settings)
    }

    // ========================================
    // Environment Variables
    // ========================================

    /// Get all environment variables
    pub fn get_env_vars() -> Result<HashMap<String, String>> {
        let settings = Self::get()?;
        Ok(settings.env)
    }

    /// Set an environment variable
    pub fn set_env_var(key: &str, value: &str) -> Result<()> {
        let mut settings = Self::get()?;
        settings.env.insert(key.to_string(), value.to_string());
        Self::update(&settings)
    }

    /// Remove an environment variable
    pub fn remove_env_var(key: &str) -> Result<()> {
        let mut settings = Self::get()?;
        settings.env.remove(key);
        Self::update(&settings)
    }

    // ========================================
    // Status Line
    // ========================================

    /// Get status line config
    pub fn get_status_line() -> Result<StatusLineConfig> {
        let settings = Self::get()?;
        Ok(settings.status_line)
    }

    /// Update status line config
    pub fn update_status_line(config: StatusLineConfig) -> Result<()> {
        let mut settings = Self::get()?;
        settings.status_line = config;
        Self::update(&settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_settings_default() {
        let settings = PluginSettings::default();

        assert_eq!(settings.output_style, "Rhinolabs");
        assert!(settings.env.contains_key("ENABLE_TOOL_SEARCH"));
        assert!(!settings.permissions.deny.is_empty());
        assert!(!settings.permissions.allow.is_empty());
    }

    #[test]
    fn test_plugin_settings_serialization() {
        let settings = PluginSettings::default();

        let json = serde_json::to_string_pretty(&settings).unwrap();
        let deserialized: PluginSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.output_style, settings.output_style);
        assert_eq!(deserialized.permissions.deny.len(), settings.permissions.deny.len());
    }

    #[test]
    fn test_status_line_config_default() {
        let config = StatusLineConfig::default();

        assert_eq!(config.line_type, "command");
        assert!(config.command.is_some());
        assert!(config.text.is_none());
        assert_eq!(config.padding, 0);
    }

    #[test]
    fn test_permission_config_default() {
        let config = PermissionConfig::default();

        assert!(!config.deny.is_empty());
        assert!(!config.ask.is_empty());
        assert!(!config.allow.is_empty());
        assert!(config.deny.iter().any(|p| p.contains(".env")));
    }

    #[test]
    fn test_settings_path() {
        let path = Settings::settings_path();
        assert!(path.is_ok());

        let path = path.unwrap();
        assert!(path.to_str().unwrap().contains("settings.json"));
    }
}
