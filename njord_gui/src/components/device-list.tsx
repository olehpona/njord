import { useDeviceStore } from "@/store/device";
import DeviceCard from "./device-card";

export default function DeviceList() {
    const {devices} = useDeviceStore();
    return devices.map((device) => (
      <DeviceCard deviceId={device.id} device={device.device} key={device.id} />
    ));
}