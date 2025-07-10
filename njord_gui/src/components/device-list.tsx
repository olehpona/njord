import { useDeviceStore } from "@/store/device";
import DeviceCard from "./device/device-card";
import { DeviceDataContextProvider } from "@/context/device";

export default function DeviceList() {
  const { devices } = useDeviceStore();
  return devices.map((device) => (
    <DeviceDataContextProvider
      key={device.id}
      defaultValue={{
        deviceInfo: device.device.device_info,
        deviceConfig: device.device.device_config,
        serialInfo: device.device.serial_info,
      }}
    >
      <DeviceCard deviceId={device.id} />
    </DeviceDataContextProvider>
  ));
}
