use rhinolabs_core::{Installer, Updater, McpSync, Paths, Version, Doctor};
use rhinolabs_core::diagnostics::DiagnosticReport;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StatusInfo {
    is_installed: bool,
    version: Option<String>,
    installed_at: Option<String>,
    plugin_path: Option<String>,
    claude_code_installed: bool,
    mcp_configured: bool,
}

#[tauri::command]
pub async fn install_plugin(local_path: Option<String>) -> Result<(), String> {
    let installer = Installer::new();

    if let Some(path) = local_path {
        installer
            .install_from_local(std::path::Path::new(&path))
            .map_err(|e| e.to_string())
    } else {
        installer.install().await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn update_plugin() -> Result<(), String> {
    let updater = Updater::new();
    updater.update().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn uninstall_plugin() -> Result<(), String> {
    let installer = Installer::new();
    installer.uninstall().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_mcp_config(url: Option<String>, file_path: Option<String>) -> Result<(), String> {
    let sync = match (url, file_path) {
        (Some(url), None) => McpSync::from_remote(url),
        (None, Some(file)) => McpSync::from_local(file),
        _ => return Err("Must specify either url or file_path".into()),
    };

    sync.sync().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_status() -> Result<StatusInfo, String> {
    let is_installed = Paths::is_plugin_installed();
    let version_info = Version::installed().ok().flatten();

    let status = StatusInfo {
        is_installed,
        version: version_info.as_ref().map(|v| v.version.clone()),
        installed_at: version_info.map(|v| v.installed_at.to_rfc3339()),
        plugin_path: Paths::plugin_dir().ok().map(|p| p.display().to_string()),
        claude_code_installed: Paths::is_claude_code_installed(),
        mcp_configured: Paths::mcp_config_path()
            .map(|p| p.exists())
            .unwrap_or(false),
    };

    Ok(status)
}

#[tauri::command]
pub async fn run_diagnostics() -> Result<DiagnosticReport, String> {
    Doctor::run().await.map_err(|e| e.to_string())
}
