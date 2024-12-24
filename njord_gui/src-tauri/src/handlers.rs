use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use njord_backend::device::{Device, DeviceConfig, DeviceInfo, PortInfo, SerialInfo};
use crate::state::{AppState, CoreMessage};

#[tauri::command]
pub fn get_core_messages(state: State<'_, Mutex<AppState>>) -> Result<Vec<CoreMessage>, ()>{
    let app_state = state.lock().map_err(|_| ())?;
    Ok(app_state.core_messages.clone())
}

#[tauri::command]
pub fn get_device_list() -> Result<Vec<PortInfo>, ()> {
    Device::get_device_list()
}

#[tauri::command]
pub fn load_device_info(serial_info: SerialInfo) -> Result<DeviceInfo, String> {
    let mut device = Device::new(serial_info).map_err(|e| e.to_string())?;
    Ok(device.get_board_info()?)
}

#[tauri::command]
pub fn load_device_config(serial_info: SerialInfo) -> Result<DeviceConfig, String> {
    let mut device = Device::new(serial_info).map_err(|e| e.to_string())?;
    Ok(device.get_device_config()?)
}

#[tauri::command]
pub fn load_device_default_config(serial_info: SerialInfo) -> Result<DeviceConfig, String> {
    let mut device = Device::new(serial_info).map_err(|e| e.to_string())?;
    Ok(device.get_device_default_config()?)
}

#[tauri::command]
pub async fn add_device(app: AppHandle, state: State<'_, Mutex<AppState>>, serial_info: SerialInfo, device_config: DeviceConfig) -> Result<(), String> {
    let id = serial_info.com_port.clone();
    let mut device = Device::new(serial_info)?;
    device.set_device_config(&device_config)?;
    let ping = device.test_connection(Duration::from_millis(800), Duration::from_millis(150)).await;
    if !ping {
        device.open_connection()?;
        if !device.test_connection(Duration::from_millis(800), Duration::from_millis(150)).await {
            return Err("Failed connecting device after setting config".to_string())
        }
    }

    device.fetch_data()?;
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.devices.insert(id.to_string(), Arc::new(Mutex::new(device)));
    let _ = app.emit("devices_update", &state.devices);
    Ok(())
}

#[tauri::command]
pub fn fetch_devices(app: AppHandle, state: State<'_, Mutex<AppState>>) -> Result<(), String>{
    let state = state.lock().map_err(|e| e.to_string())?;
    let _ = app.emit("devices_update", &state.devices);
    Ok(())
}

#[tauri::command]
pub fn remove_device(app: AppHandle, state: State<'_, Mutex<AppState>>, id: String) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.devices.remove(&id);
    let _ = app.emit("devices_update", &state.devices);
    Ok(())
}

#[tauri::command]
pub async fn update_device_config(app: AppHandle, state: State<'_, Mutex<AppState>>, id: String, device_config: DeviceConfig) -> Result<(), String> {
    let device_option = {
        let state = state.lock().map_err(|e| e.to_string())?;
        match state.devices.get(&id) {
            Some(value) => Some(value.clone()),
            _ => None
        }
    };

    let device_arc = match device_option {
        Some(device_lock) => device_lock,
        _ => return Err("Device does not exist".to_string())
    };
    let mut device_lock = device_arc.lock().map_err(|e| e.to_string())?;
    device_lock.set_device_config(&device_config)?;
    let ping = device_lock.test_connection(Duration::from_secs(10), Duration::from_millis(50)).await;
    if !ping {
        device_lock.open_connection()?;
        if !device_lock.test_connection(Duration::from_millis(800), Duration::from_millis(150)).await {
            return Err("Failed connecting device after setting config".to_string())
        }
    }
    device_lock.fetch_data()?;
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let _ = app.emit("devices_update", &state.devices);
    Ok(())
}