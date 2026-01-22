// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_plugin,
            update_plugin,
            uninstall_plugin,
            sync_mcp_config,
            get_status,
            run_diagnostics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
