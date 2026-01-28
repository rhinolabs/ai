mod commands;

use commands::*;

/// Set up development mode by detecting the project directory
fn setup_dev_mode() {
    // Skip if already set
    if std::env::var("RHINOLABS_DEV_PATH").is_ok() {
        return;
    }

    // Try to find rhinolabs-claude directory relative to current working directory
    if let Ok(cwd) = std::env::current_dir() {
        // Check parent directories for rhinolabs-claude
        let mut dir = cwd.as_path();
        for _ in 0..5 {
            let plugin_dir = dir.join("rhinolabs-claude");
            if plugin_dir.exists() && plugin_dir.join("settings.json").exists() {
                std::env::set_var("RHINOLABS_DEV_PATH", &plugin_dir);
                eprintln!("[DEV MODE] Using local plugin directory: {}", plugin_dir.display());
                return;
            }
            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                break;
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_dev_mode();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Status & Installation (for CLI, not GUI primary use)
            get_status,
            install_plugin,
            update_plugin,
            uninstall_plugin,
            // Diagnostics
            run_diagnostics,
            // Manifest
            get_manifest,
            update_manifest,
            // Settings
            get_settings,
            update_settings,
            get_permissions,
            update_permissions,
            add_permission,
            remove_permission,
            get_env_vars,
            set_env_var,
            remove_env_var,
            get_status_line,
            update_status_line,
            // MCP Configuration
            get_mcp_config,
            update_mcp_config,
            list_mcp_servers,
            get_mcp_server,
            add_mcp_server,
            update_mcp_server,
            remove_mcp_server,
            get_mcp_settings,
            update_mcp_settings,
            sync_mcp_config,
            // Output Styles
            list_output_styles,
            get_output_style,
            get_active_output_style,
            set_active_output_style,
            create_output_style,
            update_output_style,
            delete_output_style,
            // Skills
            list_skills,
            get_skill,
            create_skill,
            update_skill,
            toggle_skill,
            delete_skill,
            // Skill Sources
            list_skill_sources,
            add_skill_source,
            update_skill_source,
            remove_skill_source,
            install_skill_from_source,
            install_skill_from_remote,
            get_installed_skill_ids,
            fetch_remote_skills,
            fetch_skill_content,
            fetch_remote_skill_files,
            // Instructions
            get_instructions,
            update_instructions,
            // Project & Release
            get_project_config,
            update_project_config,
            get_project_status,
            fetch_latest_release,
            bump_version,
            create_release,
            // IDE & File Operations
            list_available_ides,
            open_skill_in_ide,
            open_instructions_in_ide,
            open_output_style_in_ide,
            get_skill_files,
            // Profile Instructions
            get_profile_instructions,
            update_profile_instructions,
            open_profile_instructions_in_ide,
            // Profiles
            list_profiles,
            get_profile,
            create_profile,
            update_profile,
            delete_profile,
            assign_skills_to_profile,
            get_profile_skills,
            get_profiles_for_skill,
            get_default_user_profile,
            set_default_user_profile,
            install_profile,
            update_installed_profile,
            uninstall_profile,
            // Auto-invoke Rules
            get_auto_invoke_rules,
            update_auto_invoke_rules,
            // Deploy & Sync
            export_config,
            deploy_config,
            sync_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
