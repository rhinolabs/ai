use crate::{Result, RhinolabsError};
use std::path::PathBuf;

/// Platform-specific path resolution
pub struct Paths;

impl Paths {
    /// Check if running in development mode (local project directory)
    pub fn is_dev_mode() -> bool {
        std::env::var("RHINOLABS_DEV_PATH").is_ok()
    }

    /// Get the development path if set
    fn dev_path() -> Option<PathBuf> {
        std::env::var("RHINOLABS_DEV_PATH").ok().map(PathBuf::from)
    }

    /// Get Claude Code plugins directory
    pub fn claude_code_plugins_dir() -> Result<PathBuf> {
        let base = if cfg!(target_os = "macos") {
            dirs::home_dir()
                .ok_or_else(|| RhinolabsError::Other("Could not find home directory".into()))?
                .join("Library")
                .join("Application Support")
                .join("Claude Code")
                .join("plugins")
        } else if cfg!(target_os = "windows") {
            dirs::config_dir()
                .ok_or_else(|| RhinolabsError::Other("Could not find config directory".into()))?
                .join("Claude Code")
                .join("plugins")
        } else {
            // Linux
            dirs::config_dir()
                .ok_or_else(|| RhinolabsError::Other("Could not find config directory".into()))?
                .join("claude-code")
                .join("plugins")
        };

        Ok(base)
    }

    /// Get rhinolabs-claude plugin directory
    /// In dev mode, uses RHINOLABS_DEV_PATH environment variable
    pub fn plugin_dir() -> Result<PathBuf> {
        if let Some(dev_path) = Self::dev_path() {
            return Ok(dev_path);
        }
        Ok(Self::claude_code_plugins_dir()?.join("rhinolabs-claude"))
    }

    /// Get MCP config file path
    pub fn mcp_config_path() -> Result<PathBuf> {
        Ok(Self::plugin_dir()?.join(".mcp.json"))
    }

    /// Get plugin version file path
    pub fn version_file_path() -> Result<PathBuf> {
        Ok(Self::plugin_dir()?.join(".version"))
    }

    /// Check if Claude Code is installed
    pub fn is_claude_code_installed() -> bool {
        if cfg!(target_os = "macos") {
            std::path::Path::new("/Applications/Claude Code.app").exists()
        } else if cfg!(target_os = "windows") {
            // Check common installation paths
            let program_files = std::env::var("ProgramFiles").unwrap_or_default();
            let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();

            std::path::Path::new(&format!("{}/Claude Code/Claude Code.exe", program_files)).exists()
                || std::path::Path::new(&format!("{}/Programs/Claude Code/Claude Code.exe", local_appdata)).exists()
        } else {
            // Linux - check if command exists
            which::which("claude").is_ok()
        }
    }

    /// Check if plugin is installed
    pub fn is_plugin_installed() -> bool {
        Self::plugin_dir()
            .map(|p| p.exists())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_code_plugins_dir() {
        let dir = Paths::claude_code_plugins_dir();
        assert!(dir.is_ok());

        let path = dir.unwrap();
        assert!(path.to_str().unwrap().contains("Claude Code") ||
                path.to_str().unwrap().contains("claude-code"));
    }

    #[test]
    fn test_plugin_dir() {
        let dir = Paths::plugin_dir();
        assert!(dir.is_ok());

        let path = dir.unwrap();
        assert!(path.to_str().unwrap().contains("rhinolabs-claude"));
    }
}
