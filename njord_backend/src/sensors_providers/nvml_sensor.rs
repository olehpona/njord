use std::sync::Arc;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;
use crate::sensors::{Sensor, SensorId, SensorType, SensorsProvidersStates};

pub struct NvmlState {
    nvml: Nvml,
}

impl NvmlState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            nvml: Nvml::init().map_err(|e| e.to_string())?,
        })
    }
}

pub struct NvmlSensor {
    sensor_type: SensorType,
    identifier: String,
    state: Arc<NvmlState>
}

impl NvmlSensor {
    pub fn new(sensors_providers_state: &SensorsProvidersStates, identifier: String) -> Result<Arc<Self>, String> {
        Ok(Arc::new(Self {
            sensor_type: SensorType::NvmlSensor,
            identifier,
            state: sensors_providers_state.nvml_state.clone().ok_or("NVML not inited")?.clone()
        }))
    }

    pub fn get_sensors(sensors_providers_states: &SensorsProvidersStates) -> Result<Vec<String>, String> {
        let state = sensors_providers_states.nvml_state.clone().ok_or("NVML not inited")?.clone();
        let device_count = state.nvml.device_count().map_err(|e| e.to_string())?;
        let mut devices = Vec::new();

        for i in 0..device_count {
            let device = state.nvml.device_by_index(i).map_err(|e| e.to_string())?;
            devices.push(device.pci_info().map_err(|e| e.to_string())?.bus_id);
        };
        Ok(devices)
    }
}

impl Sensor for NvmlSensor {
    fn get_temperature(&self) -> Result<f32, String> {
        let device = self.state.nvml.device_by_pci_bus_id(self.identifier.as_str()).map_err(|e| e.to_string())?;
        Ok(device.temperature(TemperatureSensor::Gpu).map_err(|e| e.to_string())? as f32)
    }
    fn get_sensor_id(&self) -> SensorId {
        SensorId {
            sensor_type: self.sensor_type.clone(),
            identifier: self.identifier.clone()
        }
    }
}