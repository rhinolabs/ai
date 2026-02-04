use thiserror::Error;

pub type Result<T> = std::result::Result<T, RhinolabsError>;

#[derive(Error, Debug)]
pub enum RhinolabsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Claude Code not found. Please install Claude Code first.")]
    ClaudeCodeNotFound,

    #[error("Plugin not installed")]
    PluginNotInstalled,

    #[error("Plugin already installed at: {0}")]
    PluginAlreadyInstalled(String),

    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Installation failed: {0}")]
    InstallationFailed(String),

    #[error("Update failed: {0}")]
    UpdateFailed(String),

    #[error("MCP sync failed: {0}")]
    McpSyncFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Target '{0}' is not yet supported for this operation")]
    TargetNotSupported(String),

    #[error("{0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_not_supported_message() {
        let err = RhinolabsError::TargetNotSupported("Amp".into());
        let msg = err.to_string();
        assert!(
            msg.contains("Amp"),
            "Error message should contain target name: {}",
            msg
        );
        assert!(
            msg.contains("not yet supported"),
            "Error message should mention not supported: {}",
            msg
        );
    }

    #[test]
    fn test_target_not_supported_with_different_targets() {
        let targets = vec!["Amp", "Antigravity", "OpenCode"];
        for name in targets {
            let err = RhinolabsError::TargetNotSupported(name.into());
            assert!(
                err.to_string().contains(name),
                "Should contain '{}' in: {}",
                name,
                err
            );
        }
    }

    #[test]
    fn test_target_not_supported_is_debug() {
        let err = RhinolabsError::TargetNotSupported("Amp".into());
        let debug = format!("{:?}", err);
        assert!(debug.contains("TargetNotSupported"));
        assert!(debug.contains("Amp"));
    }
}
