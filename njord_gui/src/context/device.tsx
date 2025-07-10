import { BAUD_RATES, DEFAULT_UPDATE_TIME } from "@/const";
import { DeviceConfig, DeviceInfo, SerialInfo } from "@/types/api";
import { createContext, ReactNode, useContext, useState } from "react";

export interface DeviceData {
  serialInfo: SerialInfo;
  deviceInfo: DeviceInfo;
  deviceConfig: DeviceConfig;
}

export interface DeviceUpdaters {
  setSerialInfo: (serialInfo: SerialInfo) => void;
  setDeviceInfo: (deviceInfo: DeviceInfo) => void;
  setDeviceConfig: (deviceConfig: DeviceConfig) => void;
  clear: () => void;
}

export interface DeviceContextType {
  data: DeviceData;
  updaters: DeviceUpdaters;
}

const DeviceContext = createContext<DeviceContextType>(
  {} as DeviceContextType
);

export function DeviceDataContextProvider({
  children,
  defaultValue,
}: {
  children: ReactNode;
  defaultValue?: DeviceData;
}) {
  const [serialInfo, setSerialInfo] = useState<SerialInfo>(
    defaultValue?.serialInfo || {
      com_port: "",
      baud_rate: BAUD_RATES[0],
    }
  );
  const [deviceInfo, setDeviceInfo] = useState<DeviceInfo>(
    defaultValue?.deviceInfo || {
      board_name: "",
      max_ports: -1,
    }
  );
  const [deviceConfig, setDeviceConfig] = useState<DeviceConfig>(
    defaultValue?.deviceConfig || {
      ports: [],
      default_values: [],
      update_time: DEFAULT_UPDATE_TIME,
    }
  );

  const clear = () => {
    setSerialInfo({
      com_port: "",
      baud_rate: BAUD_RATES[0],
    });
    setDeviceInfo({
      board_name: "",
      max_ports: -1,
    });
    setDeviceConfig({
      ports: [],
      default_values: [],
      update_time: DEFAULT_UPDATE_TIME,
    });
  };

  const data: DeviceData = { serialInfo, deviceInfo, deviceConfig };
  const updaters: DeviceUpdaters = {
    setSerialInfo,
    setDeviceInfo,
    setDeviceConfig,
    clear,
  };

  return (
    <DeviceContext.Provider value={{ data, updaters }}>
      {children}
    </DeviceContext.Provider>
  );
}

export const useDeviceContext = () => useContext(DeviceContext)