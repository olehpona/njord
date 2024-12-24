use std::collections::HashMap;
use std::sync::{Arc};

#[cfg(target_os = "windows")]
use crate::sensors_providers::lhm_sensor::{LhmSensor, LhmState};
use crate::sensors_providers::sys_info_sensor::SysInfoSensor;
use crate::sensors_providers::nvml_sensor::{NvmlSensor, NvmlState};

pub enum SensorType {
    #[cfg(target_os = "windows")]
    LhmSensor,
    SysInfoSensor,
    NvmlSensor,
}

pub struct SensorFactory{}

impl SensorFactory {
    pub fn create_sensor(sensor_type: SensorType ,sensors_providers_state: &SensorsProvidersStates, identifier: String) -> Result<Arc<dyn Sensor>, String>{
        match sensor_type {
            #[cfg(target_os = "windows")]
            SensorType::LhmSensor => {
                let sensor = LhmSensor::new(sensors_providers_state, identifier)?;
                Ok(sensor as Arc<dyn Sensor>)
            },
            SensorType::SysInfoSensor => {
                let sensor = SysInfoSensor::new(sensors_providers_state, identifier)?;
                Ok(sensor as Arc<dyn Sensor>)
            },
            SensorType::NvmlSensor => {
                let sensor = NvmlSensor::new(sensors_providers_state, identifier)?;
                Ok(sensor as Arc<dyn Sensor>)
            }
        }
    }
    pub fn get_sensors_names_by_type(sensor_type: SensorType, sensors_providers_state: &SensorsProvidersStates) -> Result<Vec<String>, String>{
        match sensor_type {
            SensorType::LhmSensor => LhmSensor::get_sensors(sensors_providers_state),
            SensorType::SysInfoSensor => SysInfoSensor::get_sensors(sensors_providers_state),
            SensorType::NvmlSensor => NvmlSensor::get_sensors(sensors_providers_state),
        }
    }
    pub fn get_all_sensors(sensors_providers_state: &SensorsProvidersStates) -> HashMap<String, HashMap<String, Arc<dyn Sensor>>>{
        let mut sensors:HashMap<String, HashMap<String, Arc<dyn Sensor>>> = HashMap::new();

        {
            let mut nvml_sensors: HashMap<String, Arc<dyn Sensor>> = HashMap::new();
            Self::get_sensors_names_by_type(SensorType::NvmlSensor, sensors_providers_state).unwrap_or_default().iter().for_each(|name| {
                let sensor_result = NvmlSensor::new(sensors_providers_state, name.clone());
                if let Ok(sensor) = sensor_result {
                    nvml_sensors.insert(name.clone(), sensor);
                }
            });
            if !nvml_sensors.is_empty() {
                sensors.insert("Nvml".to_owned(), nvml_sensors);
            }
        };
        {
            let mut sys_info_sensors: HashMap<String, Arc<dyn Sensor>> = HashMap::new();
            Self::get_sensors_names_by_type(SensorType::SysInfoSensor, sensors_providers_state).unwrap_or_default().iter().for_each(|name| {
                let sensor_result = SysInfoSensor::new(sensors_providers_state, name.clone());
                if let Ok(sensor) = sensor_result {
                    sys_info_sensors.insert(name.clone(), sensor);
                }
            });
            if !sys_info_sensors.is_empty() {
                sensors.insert("SysInfo".to_owned(), sys_info_sensors);
            }
        };
        {
            let mut lhm_sys_sensors: HashMap<String, Arc<dyn Sensor>> = HashMap::new();
            Self::get_sensors_names_by_type(SensorType::LhmSensor, sensors_providers_state).unwrap_or_default().iter().for_each(|name| {
                let sensor_result = LhmSensor::new(sensors_providers_state, name.clone());
                if let Ok(sensor) = sensor_result {
                    lhm_sys_sensors.insert(name.clone(), sensor);
                }
            });
            if !lhm_sys_sensors.is_empty() {
                sensors.insert("Lhm".to_owned(), lhm_sys_sensors);
            }
        };
        sensors
    }
}

pub trait Sensor: Send + Sync {
    fn get_temperature(&self) -> Result<f32, String>;
}

pub struct SensorsProvidersStates {
    #[cfg(target_os = "windows")]
    pub lhm_state: Option<Arc<LhmState>>,
    pub nvml_state: Option<Arc<NvmlState>>
}
