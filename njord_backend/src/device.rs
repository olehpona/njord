use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{BufRead, Write};
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio_serial::{ClearBuffer, FlowControl, SerialPort, SerialPortType};

const PING_API: &str = "ping";
const PONG_API: &str = "pong";
const GET_BOARD_INFO_API: &str = "board_info";
const GET_PLUGS_VALUES_API: &str = "get_value";
const GET_DEFAULT_CONFIG_API: &str = "get_default_config";
const GET_CONFIG_API: &str = "get_config";
const SET_UPDATE_TIME_API: &str = "set_update_time";
const SET_PLUG_DEFAULT_VALUE_API: &str = "set_default_value";
const SET_PLUG_VALUE_API: &str = "set_value";
const SET_DEVICE_CONFIG_API: &str = "set_config";
const SET_PLUS_CONFIG_API: &str = "ports_setup";
const LOAD_DEFAULT_CONFIG_API: &str = "load_default_config";

pub type PortValue = u8;

#[derive(Serialize, Deserialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
    pub device_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPlugsResponse {
    pub values: Vec<PortValue>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DeviceConfig {
    pub ports: Vec<u8>,
    pub default_values: Vec<PortValue>,
    pub update_time: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceCode {
    Ok,
    Err,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceResponse<T> {
    pub code: DeviceCode,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Serial error: {0}")]
    SerialPortError(#[from] tokio_serial::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    CustomError(String),
}

impl From<DeviceError> for String {
    fn from(error: DeviceError) -> Self {
        error.to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DeviceInfo {
    pub board_name: String,
    pub max_ports: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SerialInfo {
    pub com_port: String,
    pub baud_rate: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceSummary {
    pub serial_info: SerialInfo,
    pub device_info: DeviceInfo,
    pub device_config: DeviceConfig,
}

#[derive(Serialize, Clone)]
pub enum DeviceState {
    Ok,
    Error(String),
}

pub struct Device {
    serial_port_connection: Option<Box<dyn SerialPort>>,
    serial_info: SerialInfo,
    pub device_info: DeviceInfo,
    plugs_values: Vec<PortValue>,
    pub device_config: DeviceConfig,
    pub device_state: DeviceState,
}

impl Device {
    pub fn get_device_list() -> Result<Vec<PortInfo>, ()> {
        let ports = tokio_serial::available_ports().map_err(|_| ())?;
        Ok(ports
            .iter()
            .map(|port| {
                let (port_type, device_data) = match port.clone().port_type {
                    SerialPortType::UsbPort(dev_info) => {
                        let mut info = String::new();
                        if let Some(data) = dev_info.product {
                            info.push_str(&data)
                        };
                        info.push(' ');
                        if let Some(data) = dev_info.manufacturer {
                            info.push_str(&data)
                        }
                        ("USB".to_string(), info)
                    }
                    SerialPortType::BluetoothPort => ("Bluetooth".to_string(), String::new()),
                    SerialPortType::PciPort => ("Pci".to_string(), String::new()),
                    SerialPortType::Unknown => ("Unknown".to_string(), String::new()),
                };
                PortInfo {
                    name: port.port_name.clone(),
                    port_type,
                    device_data,
                }
            })
            .collect())
    }

    pub fn create_summary(&self) -> DeviceSummary {
        DeviceSummary {
            serial_info: self.serial_info.clone(),
            device_info: self.device_info.clone(),
            device_config: self.device_config.clone(),
        }
    }

    pub fn new(serial_info: SerialInfo) -> Self {
        let mut self_struct = Self {
            serial_port_connection: None,
            serial_info,
            device_info: Default::default(),
            plugs_values: Vec::new(),
            device_config: Default::default(),
            device_state: DeviceState::Ok,
        };

        let _ = self_struct.open_connection();

        self_struct
    }

    pub fn open_connection(&mut self) -> Result<(), DeviceError> {
        if self.serial_port_connection.is_some() {
            self.serial_port_connection.take();
        };
        let mut connection = tokio_serial::new(
            self.serial_info.com_port.clone(),
            self.serial_info.baud_rate,
        )
        .flow_control(FlowControl::Software)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .timeout(Duration::from_millis(1500))
        .open()?;
        connection.write_data_terminal_ready(true)?;
        self.serial_port_connection = Some(connection);

        Ok(())
    }

    async fn ensure_connection(&mut self) -> Result<(), DeviceError> {
        if self.serial_port_connection.is_none() {
            if let DeviceState::Error(_) = self.device_state{
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            self.open_connection().map_err(|err| {
                self.device_state = DeviceState::Error(err.to_string());
                err
            })?;
        };
        self.device_state = DeviceState::Ok;
        Ok(())
    }

    pub async fn fetch_data(&mut self) -> Result<(), DeviceError> {
        self.device_info = self.get_board_info().await?;
        self.plugs_values.resize(self.device_config.ports.len(), 0);
        self.plugs_values = self.get_plugs_values().await?;
        self.device_config = self.get_device_config().await?;
        Ok(())
    }

    async fn write(&mut self, value: &Value) -> Result<(), DeviceError> {
        self.ensure_connection().await?;
        let serial_port_connection =
            self.serial_port_connection
                .as_mut()
                .ok_or(DeviceError::CustomError(
                    "Device connection isn't created".to_string(),
                ))?;
        if let Err(err) = serial_port_connection.clear(ClearBuffer::All) {
            self.device_state = DeviceState::Error(err.to_string());
            self.serial_port_connection.take();
            return Err(DeviceError::from(err));
        }

        let mut to_write = serde_json::to_string(value)?;
        to_write += "\n";
        serial_port_connection
            .write_all(to_write.as_bytes())
            .map_err(|err| {
                self.device_state = DeviceState::Error(err.to_string());
                self.serial_port_connection.take();
                err
            })?;
        Ok(())
    }

    async fn read<T>(&mut self) -> Result<DeviceResponse<T>, DeviceError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.ensure_connection().await?;
        let serial_port_connection =
            self.serial_port_connection
                .as_mut()
                .ok_or(DeviceError::CustomError(
                    "Device connection isn't created".to_string(),
                ))?;

        let mut reader = std::io::BufReader::new(serial_port_connection);

        let mut buf_str = String::new();
        reader
            .read_line(&mut buf_str)
            .map_err(|e| DeviceError::CustomError(e.to_string()))
            .map_err(|err| {
                self.device_state = DeviceState::Error(err.to_string());
                self.serial_port_connection.take();
                err
            })?;
        let device_response: DeviceResponse<T> = serde_json::from_str(buf_str.trim_end())
            .map_err(|e| DeviceError::CustomError(format!("JSON parse error: {}", e)))?;

        match device_response.code {
            DeviceCode::Ok => Ok(device_response),
            DeviceCode::Err => Err(DeviceError::CustomError(
                device_response
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string()),
            )),
        }
    }

    async fn ping(&mut self) -> Result<bool, DeviceError> {
        let json_command = json!({
            "command": PING_API
        });
        self.write(&json_command).await?;
        let res = self.read::<String>().await?.message;
        if let Some(pong) = res {
            if pong == PONG_API {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn test_connection(&mut self, timeout: Duration, interval: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            let response = self.ping().await;
            if let Ok(ping) = response {
                if ping {
                    return true;
                }
            }
            tokio::time::sleep(interval).await;
        }
        false
    }

    pub async fn get_board_info(&mut self) -> Result<DeviceInfo, DeviceError> {
        let json_command = json!({
            "command": GET_BOARD_INFO_API
        });
        self.write(&json_command).await?;
        let device_info = self.read::<DeviceInfo>().await?.data;

        if device_info.is_none() {
            Err(DeviceError::CustomError("Empty data".into()))
        } else {
            Ok(device_info.unwrap())
        }
    }

    pub async fn get_plugs_values(&mut self) -> Result<Vec<u8>, DeviceError> {
        let json_command = json!({
            "command": GET_PLUGS_VALUES_API
        });
        self.write(&json_command).await?;
        let response = self.read::<GetPlugsResponse>().await?.data;

        if response.is_none() {
            Err(DeviceError::CustomError("Empty data".into()))
        } else {
            Ok(response.unwrap().values)
        }
    }

    pub async fn get_device_default_config(&mut self) -> Result<DeviceConfig, DeviceError> {
        let json_command = json!({
            "command": GET_DEFAULT_CONFIG_API
        });
        self.write(&json_command).await?;
        let config = self.read::<DeviceConfig>().await?.data;

        if config.is_none() {
            Err(DeviceError::CustomError("Empty data".into()))
        } else {
            Ok(config.unwrap())
        }
    }

    pub async fn get_device_config(&mut self) -> Result<DeviceConfig, DeviceError> {
        let json_command = json!({
            "command": GET_CONFIG_API
        });
        self.write(&json_command).await?;

        let config = self.read::<DeviceConfig>().await?.data;

        if config.is_none() {
            Err(DeviceError::CustomError("Empty data".into()))
        } else {
            Ok(config.unwrap())
        }
    }

    pub async fn set_update_time(&mut self, time: u64) -> Result<(), DeviceError> {
        let json_command = json!({
            "command": SET_UPDATE_TIME_API,
            "data": vec![time]
        });
        self.write(&json_command).await?;
        self.read::<()>().await?;
        self.device_config.update_time = time;
        Ok(())
    }

    pub async fn set_default_value(
        &mut self,
        index: u8,
        value: PortValue,
    ) -> Result<(), DeviceError> {
        if index < self.device_config.ports.len() as u8 {
            let json_command = json!({
                "command": SET_PLUG_DEFAULT_VALUE_API,
                "data": vec![index, value]
            });
            self.write(&json_command).await?;
            self.read::<()>().await?;
            Ok(())
        } else {
            Err(DeviceError::CustomError("Incorrect index".into()))
        }
    }

    pub async fn set_plug_value(&mut self, index: u8, value: PortValue) -> Result<(), DeviceError> {
        if index < self.device_config.ports.len() as u8 {
            self.plugs_values[index as usize] = value;
            let json_command = json!({
                "command": SET_PLUG_VALUE_API,
                "data": vec![index, value]
            });

            self.write(&json_command).await?;
            self.read::<()>().await?;
            Ok(())
        } else {
            Err(DeviceError::CustomError("Incorrect index".into()))
        }
    }

    pub async fn set_device_config(&mut self, config: &DeviceConfig) -> Result<(), DeviceError> {
        let json_command = json!({
            "command": SET_DEVICE_CONFIG_API,
            "data": vec![serde_json::to_string(&config)?]
        });
        self.write(&json_command).await?;
        self.read::<()>().await?;
        Ok(())
    }

    pub async fn set_plugs_config(&mut self, plugs: &[u8]) -> Result<(), DeviceError> {
        let json_command = json!({
            "command": SET_PLUS_CONFIG_API,
            "data": plugs
        });
        self.write(&json_command).await?;
        self.read::<()>().await?;
        self.device_config.ports = plugs.to_vec();
        self.plugs_values = self.get_plugs_values().await?;

        Ok(())
    }

    pub async fn load_default_config(&mut self) -> Result<(), DeviceError> {
        let json_command = json!({
            "command": LOAD_DEFAULT_CONFIG_API
        });
        self.write(&json_command).await?;
        self.read::<()>().await?;
        self.device_config = self.get_device_config().await?;
        self.plugs_values = self.get_plugs_values().await?;

        Ok(())
    }
}
