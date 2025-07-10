use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use njord_backend::controller::PlugConfig;
use njord_backend::device::SerialInfo;
use njord_backend::sensors::SensorId;
use crate::state::{AppState, CoreMessage, CoreMessageKind};

#[derive(Serialize, Deserialize)]
pub struct PlugHandlerStore {
    plug_index: u8,
    sensor_id: SensorId,
    plug_config: PlugConfig
}

#[derive(Serialize, Deserialize)]
pub struct DeviceStore {
    device_id: String,
    serial_info: SerialInfo,
    plug_handlers: Vec<PlugHandlerStore>
}

#[derive(Serialize, Deserialize)]

pub struct Storage {
    devices: Vec<DeviceStore>
}

impl Storage {
    pub async fn load_data(location: &str, state: &mut AppState) -> Result<(), String> {
        let content = fs::read_to_string(location).map_err(|_| "Failed to read storage".to_string())?;
        let self_data: Storage = serde_json::from_str(&content).map_err(|_| "Failed to parse storage".to_string())?;

        for device_store in self_data.devices {
            let serial_info = device_store.serial_info;
            let plug_handlers = device_store.plug_handlers;

            match state.add_device(serial_info.clone(), None).await{
                Err(e) => state.core_messages.push(CoreMessage {
                    kind: CoreMessageKind::Error,
                    message: format!("Failed loading device {} ({})", serial_info.com_port, e)
                }),
                _ => {}
            };

            for plug_handler in plug_handlers {
                state.set_plug_handler(device_store.device_id.clone(), plug_handler.plug_index, plug_handler.sensor_id, plug_handler.plug_config).await?;
            }
        }

        Ok(())
    }

    pub async fn dump_data(location: &str, state: &AppState) -> Result<(), String> {
        let mut self_data = Self {
            devices: Vec::new()
        };

        for (device_id, device_arc) in state.devices.clone(){
            let summary = {
                let device_lock = device_arc.lock().await;
                device_lock.create_summary()
            };

            let mut plug_handlers = Vec::new();

            {
                let plug_handlers_arc = state.plug_handlers.get(&device_id).ok_or("Plug handler not found")?;
                let plug_handlers_lock = plug_handlers_arc.lock().await;
                for plug_handler_option in plug_handlers_lock.iter() {
                    if let Some(plug_handler) = plug_handler_option {
                        plug_handlers.push(PlugHandlerStore {
                            plug_index: plug_handler.plug_externals.plug_index,
                            plug_config: plug_handler.plug_config.clone(),
                            sensor_id: plug_handler.plug_externals.sensor.get_sensor_id()
                        })
                    }
                }
            }

            self_data.devices.push(DeviceStore {
                device_id,
                serial_info: summary.serial_info,
                plug_handlers
            })
        }

        let json = serde_json::to_string_pretty(&self_data).map_err(|_| "Failed to dump storage".to_string())?; // prettified JSON
        println!("{location}");
        fs::create_dir_all(Path::new(location).parent().ok_or("Failed to get parrent path".to_string())?).map_err(|e| e.to_string())?;
        fs::write(location, json).map_err(|e| e.to_string())?; // запис у файл

        Ok(())
    }
}