mod handlers;
mod state;
mod utils;
mod storage;

use tauri::async_runtime::Mutex;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::Manager;
use crate::state::AppState;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     tauri::Builder::default()
         .setup(|app| {
             AppState::new(app.app_handle().clone());
             Ok(())
         })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![handlers::get_device_list, handlers::load_device_info, handlers::get_core_messages, handlers::load_device_config, handlers::load_device_default_config, handlers::add_device, handlers::remove_device, handlers::update_device_config, handlers::get_sensors, handlers::set_plug_handler_config, handlers::get_plug_handler_config, handlers::load_connected_device_default_config, handlers::get_plug_states, handlers::load_connected_device_config, handlers::load_settings, handlers::get_device_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
