use std::sync::Arc;
use sysinfo::Components;
use crate::sensors::{Sensor, SensorsProvidersStates};

pub struct SysInfoSensor {
    identifier: String,
}

impl SysInfoSensor {
    pub fn new(_sensors_providers_state: &SensorsProvidersStates, identifier: String) -> Result<Arc<Self>, String> {
        Ok(Arc::new(Self {
            identifier,
        }))
    }

    pub fn get_sensors(_sensors_providers_states: &SensorsProvidersStates) -> Result<Vec<String>, String> {
        let components = Components::new_with_refreshed_list();

        let response = components.iter().map(|component| component.label().to_string()
        ).collect::<Vec<String>>();

        Ok(response)
    }
}

impl Sensor for SysInfoSensor {
    fn get_temperature(&self) -> Result<f32, String> {
        let binding = Components::new_with_refreshed_list();
        let found_component = binding.list().iter().find(|component| component.label() == self.identifier);
        match found_component{
            Some(component) => {
                match component.temperature() {
                    Some(temperature) => Ok(temperature),
                    _ => Err(format!("{} Failed to get temperature", self.identifier))
                }
            },
            _ => Err("Component not found".to_string())
        }
    }
}