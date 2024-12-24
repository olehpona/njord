import { BAUD_RATES, DEFAULT_UPDATE_TIME } from "@/const";
import { DeviceConfig, DeviceInfo, SerialInfo } from "@/types/api";
import { createContext, ReactNode, useContext, useState } from "react";

export interface AddDeviceData {
  serialInfo: SerialInfo;
  deviceInfo: DeviceInfo;
  deviceConfig: DeviceConfig;
}

export interface AddDeviceUpdaters {
  setSerialInfo: (serialInfo: SerialInfo) => void;
  setDeviceInfo: (deviceInfo: DeviceInfo) => void;
  setDeviceConfig: (deviceConfig: DeviceConfig) => void;
}

export interface AddDeviceContextType {
  data: AddDeviceData;
  updaters: AddDeviceUpdaters;
}

const AddDeviceContext = createContext<AddDeviceContextType>(
  {} as AddDeviceContextType
);

export function AddDeviceDataContextProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [serialInfo, setSerialInfo] = useState<SerialInfo>({
    com_port: "",
    baud_rate: BAUD_RATES[0]
  });
  const [deviceInfo, setDeviceInfo] = useState<DeviceInfo>({
    board_name: "",
    max_ports: -1
  });
  const [deviceConfig, setDeviceConfig] = useState<DeviceConfig>({
    ports: [],
    default_values: [],
    update_time: DEFAULT_UPDATE_TIME
  });

  const data: AddDeviceData = { serialInfo, deviceInfo, deviceConfig };
  const updaters: AddDeviceUpdaters = {
    setSerialInfo,
    setDeviceInfo,
    setDeviceConfig,
  };

  return (
    <AddDeviceContext.Provider value={{ data, updaters }}>
      {children}
    </AddDeviceContext.Provider>
  );
}

export const useAddDeviceContext = () => useContext(AddDeviceContext)