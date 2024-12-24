use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;
use wmi::{COMLibrary, Variant, WMIConnection};
use crate::sensors::{Sensor, SensorsProvidersStates};

#[cfg(target_os = "windows")]
pub enum LhmStateQuery {
    WMIQuery(mpsc::Sender<Result<Vec<HashMap<String, LhmVariant>>, String>>, String)
}

#[cfg(target_os = "windows")]
pub enum LhmVariant {
    Empty,
    Null,
    String(String),
    I1(i8),
    I2(i16),
    I4(i32),
    I8(i64),
    R4(f32),
    R8(f64),
    Bool(bool),
    UI1(u8),
    UI2(u16),
    UI4(u32),
    UI8(u64),
    Array(Vec<LhmVariant>),
}

#[cfg(target_os = "windows")]
impl LhmVariant {
    pub fn from_wmi_variant(variant: &Variant) -> Self {
        match variant {
            Variant::Empty => Self::Empty,
            Variant::Null => Self::Null,
            Variant::String(data) => Self::String(data.to_owned()),
            Variant::I1(data) => Self::I1(data.to_owned()),
            Variant::I2(data) => Self::I2(data.to_owned()),
            Variant::I4(data) => Self::I4(data.to_owned()),
            Variant::I8(data) => Self::I8(data.to_owned()),
            Variant::R4(data) => Self::R4(data.to_owned()),
            Variant::R8(data) => Self::R8(data.to_owned()),
            Variant::Bool(data) => Self::Bool(data.to_owned()),
            Variant::UI1(data) => Self::UI1(data.to_owned()),
            Variant::UI2(data) => Self::UI2(data.to_owned()),
            Variant::UI4(data) => Self::UI4(data.to_owned()),
            Variant::UI8(data) => Self::UI8(data.to_owned()),
            Variant::Array(data) => {
                let new_data = data.iter().map(LhmVariant::from_wmi_variant).collect();
                Self::Array(new_data)
            },
            _ => Self::Null
        }
    }
}

#[cfg(target_os = "windows")]
pub struct LhmState {
    pub wmi_thread: mpsc::Sender<LhmStateQuery>,
}

#[cfg(target_os = "windows")]
impl LhmState {
    pub fn new() -> Result<Arc<LhmState>, String> {
        let (tx, rx) = mpsc::channel();
        let (init_tx, init_rx) = mpsc::channel();

        thread::spawn(move || {
            let com_con = if let Ok(com) = COMLibrary::new() { com } else {
                let _ = init_tx.send(Err("Failed opening COM".to_string()));
                return;
            };

            let wmi_con = if let Ok(con) = WMIConnection::with_namespace_path("ROOT\\LibreHardwareMonitor", com_con) { con } else {
                let _ = init_tx.send(Err("Failed to establish WMI connection".to_string()));
                return;
            };

            let _ = init_tx.send(Ok(()));

            for command in rx {
                match command {
                    LhmStateQuery::WMIQuery(response_tx, query) => {
                        let wmi_response: Result<Vec<HashMap<String, LhmVariant>>, String> = match wmi_con
                            .raw_query(&query)
                            .map_err(|_| "Failed to query WMI for temperature sensors".to_string()) as Result<Vec<HashMap<String, Variant>>, String>
                        {
                            Ok(data) => {
                                let vec_of_lhm_hashmaps = data
                                    .into_iter()
                                    .map(|hashmap| {
                                        hashmap
                                            .into_iter()
                                            .map(|(key, variant)| {
                                                let lhm_variant = LhmVariant::from_wmi_variant(&variant);
                                                (key, lhm_variant)
                                            })
                                            .collect::<HashMap<String, LhmVariant>>()
                                    })
                                    .collect();
                                Ok(vec_of_lhm_hashmaps)
                            }
                            Err(e) => Err(e),
                        };

                        let _ = response_tx.send(wmi_response);
                    }
                }
            }
        });

        match init_rx.recv() {
            Ok(Ok(())) => Ok(Arc::new(Self { wmi_thread: tx })),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Failed to receive initialization status".to_string()),
        }
    }
}

#[cfg(target_os = "windows")]
pub struct LhmSensor {
    identifier: String,
    lhm_state: Arc<LhmState>,
}

#[cfg(target_os = "windows")]
impl LhmSensor{
    pub fn new(sensors_providers_state: &SensorsProvidersStates, identifier: String) -> Result<Arc<Self>, String> {
        Ok(Arc::new(Self {
            identifier,
            lhm_state: sensors_providers_state.lhm_state.clone().ok_or("No lhm state")?.clone(),
        }))
    }

    pub fn get_sensors(sensors_providers_state: &SensorsProvidersStates) -> Result<Vec<String>, String> {

        let (response_tx, response_rx) = mpsc::channel();
        sensors_providers_state.lhm_state.clone().ok_or("No lhm state")?.clone()
            .wmi_thread.send(LhmStateQuery::WMIQuery(response_tx, "SELECT Identifier FROM Sensor WHERE SensorType = 'Temperature'".to_string()))
            .map_err(|_| "Failed to send WMI request".to_string())?;

        let results: Vec<HashMap<String, LhmVariant>> = response_rx.recv().map_err(|_| "Failed to receive WMI response".to_string())??;

        let mut sensors = Vec::new();

        for result in results {
            if let Some(sensor_id) = result.get("Identifier") {
                if let LhmVariant::String(identifier) = sensor_id {
                    sensors.push(identifier.clone());
                } else {
                    return Err("Unexpected WMI response for Identifier".to_string());
                }
            }
        }

        Ok(sensors)
    }
}

#[cfg(target_os = "windows")]
impl Sensor for LhmSensor {
    fn get_temperature(&self) -> Result<f32, String> {
        let (response_tx, response_rx) = mpsc::channel();
        self.lhm_state
            .wmi_thread.send(LhmStateQuery::WMIQuery(response_tx, format!("SELECT * FROM Sensor WHERE Identifier = '{}'", self.identifier)))
            .map_err(|_| "Failed to send WMI request".to_string())?;

        let result: Vec<HashMap<String, LhmVariant>> = response_rx.recv().map_err(|_| "Failed to receive WMI response".to_string())??;

        if result.is_empty() {
            Err("Sensor not found".to_string())
        } else if let Some(value) = result[0].get("Value") {
            if let LhmVariant::R4(value) = value{
                Ok(*value)
            } else {
                Err("Unexpected value".to_string())
            }
        } else  {
            Err("Unexpected value".to_string())
        }
    }
}
