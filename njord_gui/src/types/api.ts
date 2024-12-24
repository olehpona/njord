export interface DeviceInfo {
  board_name: string,
  max_ports: number
}

export interface PortInfo {
  name: string;
  port_type?: string;
  device_data?: string;
}

export interface SerialInfo {
  com_port: string;
  baud_rate: number;
}

export interface DeviceConfig {
    ports: number[],
    default_values: number[],
    update_time: number
}

export enum CoreMessageKind {
  Info = "Info",
  Warning = "Warning",
  Error = "Error"
}

export interface CoreMessage {
  kind: CoreMessageKind,
  message: string
}

export interface Device {
  serial_info: SerialInfo,
  device_info: DeviceInfo,
  device_config: DeviceConfig
}