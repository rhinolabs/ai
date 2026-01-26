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

    #[error("{0}")]
    Other(String),
}
