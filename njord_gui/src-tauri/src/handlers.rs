use crate::state::{AppState, CoreMessage};
use njord_backend::controller::{PlugConfig, PlugState};
use njord_backend::device::{Device, DeviceConfig, DeviceInfo, DeviceState, PortInfo, SerialInfo};
use njord_backend::sensors::{SensorId, SensorType};
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use crate::storage::Storage;

#[tauri::command]
pub async fn get_core_messages(state: State<'_, Mutex<AppState>>) -> Result<Vec<CoreMessage>, ()> {
    let app_state = state.lock().await;
    Ok(app_state.core_messages.clone())
}


#[tauri::command]
pub fn get_device_list() -> Result<Vec<PortInfo>, ()> {
    Device::get_device_list()
}

#[tauri::command]
pub async fn load_device_info(serial_info: SerialInfo) -> Result<DeviceInfo, String> {
    let mut device = Device::new(serial_info);
    Ok(device.get_board_info().await?)
}

#[tauri::command]
pub async fn load_device_config(serial_info: SerialInfo) -> Result<DeviceConfig, String> {
    let mut device = Device::new(serial_info);
    Ok(device.get_device_config().await?)
}

#[tauri::command]
pub async fn load_connected_device_config(state: State<'_, Mutex<AppState>>, id: String) -> Result<DeviceConfig, String> {
    let device = {
        let state_lock = state.lock().await;
        state_lock.devices.get(&id).ok_or("No such device")?.clone()
    };
    let mut device_lock = device.lock().await;
    let device_config = device_lock.get_device_config().await?;
    Ok(device_config)
}

#[tauri::command]
pub async fn load_device_default_config(serial_info: SerialInfo) -> Result<DeviceConfig, String> {
    let mut device = Device::new(serial_info);
    Ok(device.get_device_default_config().await?)
}

#[tauri::command]
pub async fn load_connected_device_default_config(state: State<'_, Mutex<AppState>>, id: String) -> Result<DeviceConfig, String> {
    let device = {
        let state_lock = state.lock().await;
        state_lock.devices.get(&id).ok_or("No such device")?.clone()
    };
    let mut device_lock = device.lock().await;
    let device_config = device_lock.get_device_default_config().await?;
    Ok(device_config)
}

pub async fn send_device_summary(app: AppHandle, devices: &HashMap<String, Arc<Mutex<Device>>>) {
    let mut summaries = HashMap::new();
    for (id, device) in devices {
        let device = device.lock().await;
        let summary = device.create_summary();
        summaries.insert(id.clone(), summary);
    }
    let _ = app.emit("devices_update", summaries);
}

#[tauri::command]
pub async fn add_device(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    serial_info: SerialInfo,
    device_config: DeviceConfig,
) -> Result<(), String> {
    {
        let mut state_lock = state.lock().await;
        state_lock.add_device(serial_info, Some(device_config)).await?;
    }

    save_settings(app, state).await?;
    Ok(())
}

#[tauri::command]
pub async fn fetch_devices(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().await;
    send_device_summary(app, &state.devices).await;
    Ok(())
}

#[tauri::command]
pub async fn remove_device(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    id: String,
) -> Result<(), String> {
    {
        let mut state_lock = state.lock().await;
        state_lock.remove_device(id).await?;
    }

    save_settings(app, state).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_device_config(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    id: String,
    device_config: DeviceConfig,
) -> Result<(), String> {
    let mut state = state.lock().await;

    state.update_device_config(id, device_config).await?;

    send_device_summary(app, &state.devices).await;
    Ok(())
}

#[tauri::command]
pub async fn get_sensors(
    state: State<'_, Mutex<AppState>>,
) -> Result<HashMap<SensorType, Vec<String>>, ()> {
    let state_lock = state.lock().await;
    let mut res: HashMap<SensorType, Vec<String>> = HashMap::new();
    for (key, value) in state_lock.sensors.iter() {
        res.insert(
            key.clone(),
            value
                .iter()
                .map(|(k, _val)| k.clone())
                .collect::<Vec<String>>(),
        );
    }
    Ok(res)
}

#[tauri::command]
pub async fn set_plug_handler_config(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    device_id: String,
    plug_index: u8,
    plug_config: PlugConfig,
    sensor_id: SensorId,
) -> Result<(), String> {
    {
        let mut state_lock = state.lock().await;
        state_lock.set_plug_handler(device_id, plug_index, sensor_id, plug_config).await?;
    }
    save_settings(app, state).await?;
    Ok(())
}

#[derive(Serialize)]
pub struct PlugHandlerData {
    sensor: SensorId,
    plug_config: PlugConfig,
}

#[tauri::command]
pub async fn get_plug_handler_config(state: State<'_, Mutex<AppState>>, device_id: String, plug_index: u8) -> Result<Option<PlugHandlerData>, String> {
    let mut state_lock = state.lock().await;
    let handler_vec = state_lock.plug_handlers.get(&device_id).ok_or("No such device".to_string())?;
    let handler_vec_lock = handler_vec.lock().await;
    let handler_option = handler_vec_lock.get(plug_index as usize).ok_or("No such plug".to_string())?;
    match handler_option {
        Some(handler) => {
            Ok(Some(PlugHandlerData {
                sensor: handler.plug_externals.sensor.get_sensor_id(),
                plug_config: handler.plug_config.clone(),
            }))
        }
        _ => Ok(None)
    }
}

#[tauri::command]
pub async fn get_plug_states(state: State<'_, Mutex<AppState>>, device_id: String) -> Result<Vec<Option<PlugState>>, String> {
    let plug_handlers_arc = {
        let mut state_lock = state.lock().await;
        state_lock.plug_handlers.get(&device_id).ok_or("No such device".to_string())?.clone()
    };
    let plug_handlers_lock = plug_handlers_arc.lock().await;

    let mut result = Vec::new();
    for plug_handler in plug_handlers_lock.iter() {
        match plug_handler {
            Some(handler) => result.push(Some(handler.get_state())),
            None => result.push(None)
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_device_status(state: State<'_, Mutex<AppState>>, device_id: String) -> Result<DeviceState, String> {
    let state_lock = state.lock().await;
    let device_arc = state_lock.devices.get(&device_id).ok_or("No such device".to_string())?;
    let device_lock = device_arc.lock().await;

    Ok(device_lock.device_state.clone())
}

#[tauri::command]
pub async fn load_settings(app: AppHandle,
                           state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut path = app.path().app_data_dir().unwrap();
    path.push("settings.json");
    let path_str = path.to_string_lossy();
    let mut state_lock = state.lock().await;
    Storage::load_data(&path_str, &mut state_lock).await?;

    send_device_summary(app, &state_lock.devices).await;
    Ok(())
}

#[tauri::command]
pub async fn save_settings(app: AppHandle,
                           state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut path = app.path().app_data_dir().unwrap();
    path.push("settings.json");
    let path_str = path.to_string_lossy();
    let mut state_lock = state.lock().await;
    Storage::dump_data(&path_str, &state_lock).await?;

    send_device_summary(app, &state_lock.devices).await;
    Ok(())
}