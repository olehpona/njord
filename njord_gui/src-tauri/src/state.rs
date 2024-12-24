use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use serde::Serialize;
use njord_backend::device::Device;
use njord_backend::sensors::{Sensor, SensorFactory, SensorsProvidersStates};
use njord_backend::sensors_providers::lhm_sensor::LhmState;
use njord_backend::sensors_providers::nvml_sensor::NvmlState;

#[derive(Serialize, Clone)]
pub enum CoreMessageKind {
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Clone)]
pub struct CoreMessage {
    pub kind: CoreMessageKind,
    pub message: String,
}

pub struct AppState {
    pub devices: HashMap<String, Arc<Mutex<Device>>>,
    pub sensors_providers_states: SensorsProvidersStates,
    pub sensors: HashMap<String, HashMap<String, Arc<dyn Sensor>>>,
    pub core_messages: Vec<CoreMessage>
}

impl AppState {
    pub fn new() -> Self {
        let mut core_messages: Vec<CoreMessage> = Vec::new();

        let () = core_messages.push(
            CoreMessage {
                kind: CoreMessageKind::Warning,
                message: "Failed to initialize Lhm".to_string()
            }
        );

        let sensors_providers_states = SensorsProvidersStates {
            lhm_state: if let Ok(lhm_state) = LhmState::new() { Some(lhm_state) } else {
                let () = core_messages.push(
                    CoreMessage {
                        kind: CoreMessageKind::Error,
                        message: "Failed to initialize Lhm".to_string()
                    }
                );
                None
            },
            nvml_state: if let Ok(nvml_state) = NvmlState::new() { Some(Arc::new(nvml_state)) } else {
                let () = core_messages.push(
                    CoreMessage {
                        kind: CoreMessageKind::Error,
                        message: "Failed to initialize NVML".to_string()
                    }
                );
                None
            }
        };
        let sensors = SensorFactory::get_all_sensors(&sensors_providers_states);
        Self {
            devices: HashMap::new(),
            sensors_providers_states,
            sensors,
            core_messages
        }
    }
}