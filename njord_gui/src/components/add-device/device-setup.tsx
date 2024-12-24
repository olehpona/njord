import { SerialInfo } from "@/types/api";
import { deviceConfigToPlugSetting } from "@/utils/api";
import { useEffect, useId } from "react";
import SetupPlugs from "./plugs-setup";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { loadDeviceConfig, loadDeviceDefaultConfig } from "@/api/device";
import { useAddDeviceContext } from "@/context/add-device";

export default function DeviceSetup() {
  const addDeviceContext = useAddDeviceContext();
  const { deviceInfo, deviceConfig, serialInfo } = addDeviceContext.data;
  const { setDeviceConfig } = addDeviceContext.updaters;
  const isDisabled = deviceInfo.max_ports <= 0;

  const updateTimeId = useId();

  const loadCurrentConfig = async () => {
    const response = await loadDeviceConfig(serialInfo as SerialInfo);
    if (response.data) {
      setDeviceConfig(response.data);
    }
  };

  const loadDefaultConfig = async () => {
    const response = await loadDeviceDefaultConfig(serialInfo as SerialInfo);
    if (response.data) {
      setDeviceConfig(response.data);
    }
  };


  return (
    <div className="w-full h-full space-y-4 mb-8">
      <div className="w-full flex flex-row space-x-2">
        <Button className="flex-grow" onClick={loadCurrentConfig} disabled={isDisabled}>
          Load current config
        </Button>
        <Button className="flex-grow" onClick={loadDefaultConfig} disabled={isDisabled}>
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
              update_time: parseInt(event.target.value)
            });
          }}
          disabled={isDisabled}
          type="number"
        ></Input>
      </div>
      <SetupPlugs
      ></SetupPlugs>
    </div>
  );
}
