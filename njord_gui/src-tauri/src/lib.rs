mod handlers;
mod state;

use std::sync::Mutex;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::Manager;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     tauri::Builder::default()
         .setup(|app| {
             let app_state = AppState::new();
             app.manage(Mutex::new(app_state));
             Ok(())
         })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![handlers::get_device_list, handlers::load_device_info, handlers::get_core_messages, handlers::load_device_config, handlers::load_device_default_config, handlers::add_device, handlers::remove_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
