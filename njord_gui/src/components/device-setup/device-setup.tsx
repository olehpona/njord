import { SerialInfo } from "@/types/api";
import SetupPlugs from "./plugs-setup";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { loadConnectedDeviceConfig, loadConnectedDeviceDefaultConfig, loadDeviceConfig, loadDeviceDefaultConfig } from "@/api/device";
import { useId } from "react";
import { useDeviceContext } from "@/context/device";


export default function DeviceSetup({deviceId}: {deviceId?: string}) {
  const deviceContext = useDeviceContext();
  const { deviceInfo, deviceConfig, serialInfo } = deviceContext.data;
  const { setDeviceConfig } = deviceContext.updaters;
  const isDisabled = deviceInfo.max_ports <= 0;

  const updateTimeId = useId();

  const loadCurrentConfig = async () => {
    let response;
    if (deviceId) {
      response = await loadConnectedDeviceConfig(deviceId);
    } else {
      response = await loadDeviceConfig(serialInfo as SerialInfo);
    }
    if (response.data) {
      setDeviceConfig(response.data);
    }
  };

  const loadDefaultConfig = async () => {
    let response;
    if (deviceId) {
      response = await loadConnectedDeviceDefaultConfig(deviceId);
    } else {
      response = await loadDeviceDefaultConfig(serialInfo as SerialInfo);
    }
    if (response.data) {
      setDeviceConfig(response.data);
    }
  };

  return (
    <div className="w-full h-full space-y-4 mb-8">
      <div className="w-full flex flex-row space-x-2">
        <Button
          className="flex-grow"
          onClick={loadCurrentConfig}
          disabled={isDisabled}
        >
          Load current config
        </Button>
        <Button
          className="flex-grow"
          onClick={loadDefaultConfig}
          disabled={isDisabled}
        >
          Load default config
        </Button>
      </div>
      <div>
        <Label htmlFor={updateTimeId}>Update Time</Label>
        <Input
          id={updateTimeId}
          value={deviceConfig.update_time}
          onChange={(event) => {
            setDeviceConfig({
              ports: deviceConfig.ports,
              default_values: deviceConfig.default_values,
              update_time: parseInt(event.target.value),
            });
          }}
          disabled={isDisabled}
          type="number"
        ></Input>
      </div>
      <SetupPlugs></SetupPlugs>
    </div>
  );
}
