use rhinolabs_core::{
    Installer, Updater, McpSync, Paths, Version, Doctor,
    Manifest, PluginManifest,
    Settings, PluginSettings, PermissionConfig, StatusLineConfig,
    OutputStyles, OutputStyle,
    Skills, Skill, CreateSkillInput, UpdateSkillInput, SkillSource, SkillSourceType, SkillSchema, RemoteSkill, RemoteSkillFile,
    InstructionsManager, Instructions,
    McpConfigManager, McpConfig, McpServer, McpSettings,
    Project, ProjectConfig, ProjectStatus,
    Profiles, Profile, CreateProfileInput, UpdateProfileInput, ProfileInstallResult,
    Deploy, ConfigManifest, DeployResult, SyncResult,
};
use rhinolabs_core::diagnostics::DiagnosticReport;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

// ============================================
// Status Types
// ============================================

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfo {
    is_installed: bool,
    version: Option<String>,
    installed_at: Option<String>,
    plugin_path: Option<String>,
    claude_code_installed: bool,
    mcp_configured: bool,
}

// ============================================
// Status & Installation Commands
// ============================================

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

// ============================================
// Diagnostics Commands
// ============================================

#[tauri::command]
pub async fn run_diagnostics() -> Result<DiagnosticReport, String> {
    Doctor::run().await.map_err(|e| e.to_string())
}

// ============================================
// Manifest Commands
// ============================================

#[tauri::command]
pub fn get_manifest() -> Result<PluginManifest, String> {
    Manifest::get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_manifest(manifest: PluginManifest) -> Result<(), String> {
    Manifest::update(&manifest).map_err(|e| e.to_string())
}

// ============================================
// Settings Commands
// ============================================

#[tauri::command]
pub fn get_settings() -> Result<PluginSettings, String> {
    Settings::get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(settings: PluginSettings) -> Result<(), String> {
    Settings::update(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_permissions() -> Result<PermissionConfig, String> {
    Settings::get_permissions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_permissions(permissions: PermissionConfig) -> Result<(), String> {
    Settings::update_permissions(permissions).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_permission(permission_type: String, permission: String) -> Result<(), String> {
    Settings::add_permission(&permission_type, &permission).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_permission(permission_type: String, permission: String) -> Result<(), String> {
    Settings::remove_permission(&permission_type, &permission).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_env_vars() -> Result<HashMap<String, String>, String> {
    Settings::get_env_vars().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_env_var(key: String, value: String) -> Result<(), String> {
    Settings::set_env_var(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_env_var(key: String) -> Result<(), String> {
    Settings::remove_env_var(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_status_line() -> Result<StatusLineConfig, String> {
    Settings::get_status_line().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_status_line(config: StatusLineConfig) -> Result<(), String> {
    Settings::update_status_line(config).map_err(|e| e.to_string())
}

// ============================================
// MCP Configuration Commands
// ============================================

#[tauri::command]
pub fn get_mcp_config() -> Result<McpConfig, String> {
    McpConfigManager::get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_mcp_config(config: McpConfig) -> Result<(), String> {
    McpConfigManager::update(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_mcp_servers() -> Result<HashMap<String, McpServer>, String> {
    McpConfigManager::list_servers().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mcp_server(name: String) -> Result<Option<McpServer>, String> {
    McpConfigManager::get_server(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_mcp_server(name: String, server: McpServer) -> Result<(), String> {
    McpConfigManager::add_server(&name, server).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_mcp_server(name: String, server: McpServer) -> Result<(), String> {
    McpConfigManager::update_server(&name, server).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_mcp_server(name: String) -> Result<(), String> {
    McpConfigManager::remove_server(&name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mcp_settings() -> Result<McpSettings, String> {
    McpConfigManager::get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_mcp_settings(settings: McpSettings) -> Result<(), String> {
    McpConfigManager::update_settings(settings).map_err(|e| e.to_string())
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

// ============================================
// Output Styles Commands
// ============================================

#[tauri::command]
pub fn list_output_styles() -> Result<Vec<OutputStyle>, String> {
    OutputStyles::list().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_output_style(id: String) -> Result<Option<OutputStyle>, String> {
    OutputStyles::get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_active_output_style() -> Result<Option<OutputStyle>, String> {
    OutputStyles::get_active().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_active_output_style(id: String) -> Result<(), String> {
    OutputStyles::set_active(&id).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOutputStyleInput {
    name: String,
    description: String,
    keep_coding_instructions: bool,
    content: String,
}

#[tauri::command]
pub fn create_output_style(style: CreateOutputStyleInput) -> Result<OutputStyle, String> {
    OutputStyles::create(
        &style.name,
        &style.description,
        style.keep_coding_instructions,
        &style.content,
    ).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOutputStyleInput {
    name: Option<String>,
    description: Option<String>,
    keep_coding_instructions: Option<bool>,
    content: Option<String>,
}

#[tauri::command]
pub fn update_output_style(id: String, style: UpdateOutputStyleInput) -> Result<(), String> {
    OutputStyles::update(
        &id,
        style.name.as_deref(),
        style.description.as_deref(),
        style.keep_coding_instructions,
        style.content.as_deref(),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_output_style(id: String) -> Result<(), String> {
    OutputStyles::delete(&id).map_err(|e| e.to_string())
}

// ============================================
// Skills Commands
// ============================================

#[tauri::command]
pub fn list_skills() -> Result<Vec<Skill>, String> {
    Skills::list().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_skill(id: String) -> Result<Option<Skill>, String> {
    Skills::get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_skill(input: CreateSkillInput) -> Result<Skill, String> {
    Skills::create(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_skill(id: String, input: UpdateSkillInput) -> Result<(), String> {
    Skills::update(&id, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_skill(id: String, enabled: bool) -> Result<(), String> {
    Skills::toggle(&id, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_skill(id: String) -> Result<(), String> {
    Skills::delete(&id).map_err(|e| e.to_string())
}

// ============================================
// Skill Sources Commands
// ============================================

#[tauri::command]
pub fn list_skill_sources() -> Result<Vec<SkillSource>, String> {
    Skills::list_sources().map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSkillSourceInput {
    id: String,
    name: String,
    source_type: String,
    url: String,
    description: String,
    #[serde(default)]
    fetchable: bool,
    #[serde(default)]
    schema: String,
}

#[tauri::command]
pub fn add_skill_source(input: AddSkillSourceInput) -> Result<(), String> {
    let source_type = match input.source_type.as_str() {
        "official" => SkillSourceType::Official,
        "marketplace" => SkillSourceType::Marketplace,
        "community" => SkillSourceType::Community,
        _ => SkillSourceType::Local,
    };

    let schema = match input.schema.as_str() {
        "standard" => SkillSchema::Standard,
        _ => SkillSchema::Custom,
    };

    let source = SkillSource {
        id: input.id,
        name: input.name,
        source_type,
        url: input.url,
        description: input.description,
        enabled: true,
        fetchable: input.fetchable,
        schema,
        skill_count: None,
    };

    Skills::add_source(source).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_skill_source(
    id: String,
    enabled: Option<bool>,
    name: Option<String>,
    url: Option<String>,
    description: Option<String>,
    fetchable: Option<bool>,
    schema: Option<String>,
) -> Result<(), String> {
    let schema = schema.map(|s| match s.as_str() {
        "standard" => SkillSchema::Standard,
        _ => SkillSchema::Custom,
    });
    Skills::update_source(&id, enabled, name, url, description, fetchable, schema).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_skill_source(id: String) -> Result<(), String> {
    Skills::remove_source(&id).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallSkillFromSourceInput {
    skill_id: String,
    skill_content: String,
    source_id: String,
    source_name: String,
}

#[tauri::command]
pub fn install_skill_from_source(input: InstallSkillFromSourceInput) -> Result<Skill, String> {
    Skills::install_from_source(
        &input.skill_id,
        &input.skill_content,
        &input.source_id,
        &input.source_name,
    ).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallSkillFromRemoteInput {
    source_url: String,
    skill_id: String,
    source_id: String,
    source_name: String,
}

#[tauri::command]
pub async fn install_skill_from_remote(input: InstallSkillFromRemoteInput) -> Result<Skill, String> {
    Skills::install_from_remote(
        &input.source_url,
        &input.skill_id,
        &input.source_id,
        &input.source_name,
    ).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_installed_skill_ids() -> Result<Vec<String>, String> {
    Skills::installed_ids().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_remote_skills(source_id: String) -> Result<Vec<RemoteSkill>, String> {
    let sources = Skills::list_sources().map_err(|e| e.to_string())?;

    let source = sources
        .into_iter()
        .find(|s| s.id == source_id)
        .ok_or_else(|| format!("Source '{}' not found", source_id))?;

    Skills::fetch_from_github(&source).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_skill_content(url: String) -> Result<String, String> {
    Skills::fetch_skill_by_url(&url).await.map_err(|e| e.to_string())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchRemoteSkillFilesInput {
    source_url: String,
    skill_id: String,
}

#[tauri::command]
pub async fn fetch_remote_skill_files(input: FetchRemoteSkillFilesInput) -> Result<Vec<RemoteSkillFile>, String> {
    Skills::fetch_remote_skill_files(&input.source_url, &input.skill_id).await.map_err(|e| e.to_string())
}

// ============================================
// Instructions Commands
// ============================================

#[tauri::command]
pub fn get_instructions() -> Result<Instructions, String> {
    InstructionsManager::get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_instructions(content: String) -> Result<(), String> {
    InstructionsManager::update(&content).map_err(|e| e.to_string())
}

// ============================================
// Project Commands
// ============================================

#[tauri::command]
pub fn get_project_config() -> Result<ProjectConfig, String> {
    Project::get_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project_config(config: ProjectConfig) -> Result<(), String> {
    Project::update_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project_status() -> Result<ProjectStatus, String> {
    Project::get_status().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_latest_release() -> Result<Option<String>, String> {
    Project::fetch_latest_release().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn bump_version(bump_type: String) -> Result<String, String> {
    Project::bump_version(&bump_type).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_release(version: String, changelog: String, prerelease: bool) -> Result<String, String> {
    Project::create_release(&version, &changelog, prerelease).await.map_err(|e| e.to_string())
}

// ============================================
// IDE Commands
// ============================================

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeInfo {
    id: String,
    name: String,
    command: String,
    available: bool,
}

#[tauri::command]
pub fn list_available_ides() -> Vec<IdeInfo> {
    let ides = vec![
        ("code", "VS Code", "code"),
        ("cursor", "Cursor", "cursor"),
        ("zed", "Zed", "zed"),
        ("windsurf", "Windsurf", "windsurf"),
        ("neovim", "Neovim (Terminal)", "nvim"),
        ("vim", "Vim (Terminal)", "vim"),
    ];

    ides.into_iter()
        .map(|(id, name, cmd)| {
            let available = Command::new("which")
                .arg(cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            IdeInfo {
                id: id.to_string(),
                name: name.to_string(),
                command: cmd.to_string(),
                available,
            }
        })
        .collect()
}

#[tauri::command]
pub fn open_skill_in_ide(skill_id: String, ide_command: String) -> Result<(), String> {
    let skill_path = Skills::get_skill_path(&skill_id).map_err(|e| e.to_string())?;

    Command::new(&ide_command)
        .arg(&skill_path)
        .spawn()
        .map_err(|e| format!("Failed to open IDE: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn open_instructions_in_ide(ide_command: String) -> Result<(), String> {
    let path = InstructionsManager::get_path().map_err(|e| e.to_string())?;

    Command::new(&ide_command)
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open IDE: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn open_output_style_in_ide(style_id: String, ide_command: String) -> Result<(), String> {
    let path = OutputStyles::get_style_path(&style_id).map_err(|e| e.to_string())?;

    Command::new(&ide_command)
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open IDE: {}", e))?;

    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillFile {
    name: String,
    path: String,
    relative_path: String,
    is_directory: bool,
    content: Option<String>,
    language: Option<String>,
}

#[tauri::command]
pub fn get_skill_files(skill_id: String) -> Result<Vec<SkillFile>, String> {
    let skill_path = Skills::get_skill_path(&skill_id).map_err(|e| e.to_string())?;
    let mut files = Vec::new();

    collect_skill_files(&skill_path, &skill_path, &mut files)
        .map_err(|e| format!("Failed to read skill files: {}", e))?;

    Ok(files)
}

fn collect_skill_files(base_path: &PathBuf, current_path: &PathBuf, files: &mut Vec<SkillFile>) -> std::io::Result<()> {
    if current_path.is_dir() {
        for entry in std::fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(base_path).unwrap_or(&path);

            if path.is_dir() {
                files.push(SkillFile {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: path.display().to_string(),
                    relative_path: relative.display().to_string(),
                    is_directory: true,
                    content: None,
                    language: None,
                });
                collect_skill_files(base_path, &path, files)?;
            } else {
                let content = std::fs::read_to_string(&path).ok();
                let language = detect_language(&path);

                files.push(SkillFile {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: path.display().to_string(),
                    relative_path: relative.display().to_string(),
                    is_directory: false,
                    content,
                    language,
                });
            }
        }
    }
    Ok(())
}

fn detect_language(path: &PathBuf) -> Option<String> {
    let ext = path.extension()?.to_str()?;
    let lang = match ext {
        "md" => "markdown",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "rs" => "rust",
        "py" => "python",
        "go" => "go",
        "sh" | "bash" => "bash",
        "css" => "css",
        "html" => "html",
        "sql" => "sql",
        _ => return None,
    };
    Some(lang.to_string())
}

// ============================================
// Profile Commands
// ============================================

#[tauri::command]
pub fn list_profiles() -> Result<Vec<Profile>, String> {
    Profiles::list().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_profile(id: String) -> Result<Option<Profile>, String> {
    Profiles::get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_profile(input: CreateProfileInput) -> Result<Profile, String> {
    Profiles::create(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_profile(id: String, input: UpdateProfileInput) -> Result<Profile, String> {
    Profiles::update(&id, input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_profile(id: String) -> Result<(), String> {
    Profiles::delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn assign_skills_to_profile(profile_id: String, skill_ids: Vec<String>) -> Result<Profile, String> {
    Profiles::assign_skills(&profile_id, skill_ids).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_profile_skills(profile_id: String) -> Result<Vec<Skill>, String> {
    Profiles::get_profile_skills(&profile_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_profiles_for_skill(skill_id: String) -> Result<Vec<Profile>, String> {
    Profiles::get_profiles_for_skill(&skill_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_default_user_profile() -> Result<Option<Profile>, String> {
    Profiles::get_default_user_profile().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_default_user_profile(profile_id: String) -> Result<(), String> {
    Profiles::set_default_user_profile(&profile_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn install_profile(profile_id: String, target_path: Option<String>) -> Result<ProfileInstallResult, String> {
    let path = target_path.as_deref().map(std::path::Path::new);
    Profiles::install(&profile_id, path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_installed_profile(profile_id: String, target_path: Option<String>) -> Result<ProfileInstallResult, String> {
    let path = target_path.as_deref().map(std::path::Path::new);
    Profiles::update_installed(&profile_id, path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn uninstall_profile(target_path: String) -> Result<(), String> {
    Profiles::uninstall(std::path::Path::new(&target_path)).map_err(|e| e.to_string())
}

// ============================================
// Deploy Commands
// ============================================

#[tauri::command]
pub fn export_config(output_path: String) -> Result<(String, ConfigManifest), String> {
    let (path, manifest) = Deploy::export_config(std::path::Path::new(&output_path))
        .map_err(|e| e.to_string())?;
    Ok((path.display().to_string(), manifest))
}

#[tauri::command]
pub async fn deploy_config(version: String, changelog: String) -> Result<DeployResult, String> {
    Deploy::deploy(&version, &changelog)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_config() -> Result<SyncResult, String> {
    Deploy::sync()
        .await
        .map_err(|e| e.to_string())
}
