use crate::{Result, RhinolabsError};
use std::path::{Path, PathBuf};

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

    /// Get rhinolabs config directory: ~/.config/rhinolabs-ai/
    /// Used for storing profiles.json and other rhinolabs-specific config
    pub fn rhinolabs_config_dir() -> Result<PathBuf> {
        // Allow override for testing
        if let Ok(path) = std::env::var("RHINOLABS_CONFIG_PATH") {
            return Ok(PathBuf::from(path)
                .parent()
                .unwrap_or(Path::new(""))
                .to_path_buf());
        }

        let config_dir = dirs::config_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find config directory".into()))?
            .join("rhinolabs-ai");
        Ok(config_dir)
    }

    /// Get Claude user directory: ~/.claude/
    /// Used for user-level skills and configurations
    pub fn claude_user_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| RhinolabsError::Other("Could not find home directory".into()))?;
        Ok(home.join(".claude"))
    }

    /// Get Claude project directory for a given project path: /project/.claude/
    /// Used for project-level skills and configurations
    pub fn claude_project_dir(project_path: &Path) -> PathBuf {
        project_path.join(".claude")
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
                || which::which("claude").is_ok()
        } else if cfg!(target_os = "windows") {
            // Check desktop app installation paths
            let program_files = std::env::var("ProgramFiles").unwrap_or_default();
            let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();

            std::path::Path::new(&format!("{}/Claude Code/Claude Code.exe", program_files)).exists()
                || std::path::Path::new(&format!(
                    "{}/Programs/Claude Code/Claude Code.exe",
                    local_appdata
                ))
                .exists()
                || which::which("claude").is_ok()
        } else {
            // Linux - check if command exists
            which::which("claude").is_ok()
        }
    }

    /// Check if plugin is installed
    pub fn is_plugin_installed() -> bool {
        Self::plugin_dir().map(|p| p.exists()).unwrap_or(false)
    }

    /// Get the skills directory for a specific deploy target (user level)
    pub fn target_skills_dir(target: crate::DeployTarget) -> Result<PathBuf> {
        crate::TargetPaths::user_skills_dir(target)
    }

    /// Get the project skills directory for a specific deploy target
    pub fn target_project_skills_dir(target: crate::DeployTarget, project_path: &Path) -> PathBuf {
        crate::TargetPaths::project_skills_dir(target, project_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::ENV_MUTEX;

    #[test]
    fn test_claude_code_plugins_dir() {
        let dir = Paths::claude_code_plugins_dir();
        assert!(dir.is_ok());

        let path = dir.unwrap();
        assert!(
            path.to_str().unwrap().contains("Claude Code")
                || path.to_str().unwrap().contains("claude-code")
        );
    }

    #[test]
    fn test_plugin_dir() {
        // Acquire mutex to ensure no other test has RHINOLABS_DEV_PATH set
        let _lock = ENV_MUTEX.lock().unwrap();

        // Save and clear any existing dev path
        let original = std::env::var("RHINOLABS_DEV_PATH").ok();
        std::env::remove_var("RHINOLABS_DEV_PATH");

        let dir = Paths::plugin_dir();
        assert!(dir.is_ok());

        let path = dir.unwrap();
        assert!(path.to_str().unwrap().contains("rhinolabs-claude"));

        // Restore original if any
        if let Some(val) = original {
            std::env::set_var("RHINOLABS_DEV_PATH", val);
        }
    }

    #[test]
    fn test_plugin_dir_with_dev_path() {
        let _lock = ENV_MUTEX.lock().unwrap();

        // Set a custom dev path
        let test_path = "/tmp/test-rhinolabs-dev";
        std::env::set_var("RHINOLABS_DEV_PATH", test_path);

        let dir = Paths::plugin_dir();
        assert!(dir.is_ok());

        let path = dir.unwrap();
        assert_eq!(path.to_str().unwrap(), test_path);

        // Clean up
        std::env::remove_var("RHINOLABS_DEV_PATH");
    }

    #[test]
    fn test_is_dev_mode() {
        let _lock = ENV_MUTEX.lock().unwrap();

        // Clear dev path
        std::env::remove_var("RHINOLABS_DEV_PATH");
        assert!(!Paths::is_dev_mode());

        // Set dev path
        std::env::set_var("RHINOLABS_DEV_PATH", "/tmp/test");
        assert!(Paths::is_dev_mode());

        // Clean up
        std::env::remove_var("RHINOLABS_DEV_PATH");
    }

    #[test]
    fn test_target_skills_dir_delegates_correctly() {
        let target = crate::DeployTarget::ClaudeCode;
        let direct = crate::TargetPaths::user_skills_dir(target).unwrap();
        let via_paths = Paths::target_skills_dir(target).unwrap();
        assert_eq!(direct, via_paths);
    }

    #[test]
    fn test_target_skills_dir_works_for_all_targets() {
        for target in crate::DeployTarget::all() {
            let result = Paths::target_skills_dir(*target);
            assert!(
                result.is_ok(),
                "target_skills_dir should work for {:?}",
                target
            );
        }
    }

    #[test]
    fn test_target_project_skills_dir_delegates_correctly() {
        let project = Path::new("/home/user/project");
        let target = crate::DeployTarget::ClaudeCode;

        let direct = crate::TargetPaths::project_skills_dir(target, project);
        let via_paths = Paths::target_project_skills_dir(target, project);
        assert_eq!(direct, via_paths);
    }

    #[test]
    fn test_target_project_skills_dir_works_for_all_targets() {
        let project = Path::new("/home/user/project");
        for target in crate::DeployTarget::all() {
            let path = Paths::target_project_skills_dir(*target, project);
            assert!(
                path.starts_with(project),
                "Project skills dir for {:?} should be under project: {:?}",
                target,
                path
            );
        }
    }
}
