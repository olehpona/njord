import { Settings, Trash } from "lucide-react";
import { Button } from "../ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { Badge } from "../ui/badge";
import { DeviceState, removeDevice } from "@/api/device";
import { useDeviceContext } from "@/context/device";
import DeviceSettings from "./device-settings";
import { ReactNode, useEffect, useState } from "react";
import PlugSetting from "./plug-settings";
import PlugContextProvider from "@/context/plug";
import { PlugState } from "@/api/plugs";
import { listen } from "@tauri-apps/api/event";

export default function DeviceCard({ deviceId }: { deviceId: string }) {
  const deviceContext = useDeviceContext();
  const { serialInfo, deviceInfo } = deviceContext.data;
  const [plugIndex, setPlugIndex] = useState(0);
  const [deviceError, setDeviceError] = useState("");
  const [plugsStates, setPlugsStates] = useState<(PlugState | undefined)[]>([]);

  const [isSheetOpen, setIsSheetOpen] = useState(false);

  function generateBadges() {
    let elements: ReactNode[] = [];
    plugsStates.forEach((state, index) => {
      elements.push(
        <Badge
          onClick={() => {
            setIsSheetOpen(true);
            setPlugIndex(index);
          }}
          variant={"outline"}
          className="space-x-1"
        >
          <p className="font-bold">{index}:</p>
          <p className="font-semibold text-muted-foreground">
            {state ? `${state.last_temp}°C/${state.plug_value}%` : " -/-"}
          </p>
        </Badge>
      );
    });
    return elements;
  }

  function removeDeviceHandler() {
    removeDevice(deviceId);
  }

  useEffect(() => {
    listen<Record<string,DeviceState>>("device_state_update", (data) => {
      setDeviceError("")
      console.log(data)
      if (
        data.payload[deviceId] &&
        typeof data.payload[deviceId] === "object"
      ) {
        setDeviceError(data.payload[deviceId].Error);
      }
      
    })
    listen<Record<string,(PlugState | undefined)[]>>("plugs_states_update", (data) => {
      if (data.payload[deviceId]) {
        setPlugsStates(data.payload[deviceId]);
      } else {
        setDeviceError("Failed fetching plug states")
      }
    });
  }, []);

  return (
    <Card className="w-full">
      <CardHeader>
        <Badge variant={"outline"} className={`bg-error text-white ${deviceError? "": "hidden"}`}>
          {deviceError}
        </Badge>
        <CardTitle>{deviceInfo.board_name}</CardTitle>
        <CardDescription>On: {serialInfo.com_port}</CardDescription>
      </CardHeader>
      <CardContent className="flex w-full items-center">
        <div className="flex-1">
          <div className="flex flex-col">
            <div className="w-fit h-16 flex flex-col flex-wrap gap-2">
              {generateBadges()}
            </div>
          </div>
        </div>
        <div className="flex flex-col space-y-1">
          <DeviceSettings id={deviceId}>
            <Button variant="ghost">
              <Settings />
            </Button>
          </DeviceSettings>
          <Button variant="ghost" onClick={removeDeviceHandler}>
            <Trash />
          </Button>
        </div>
      </CardContent>
      <PlugContextProvider>
        <PlugSetting
          isOpen={isSheetOpen}
          setIsOpen={setIsSheetOpen}
          device_id={serialInfo.com_port}
          plug_index={plugIndex}
        />
      </PlugContextProvider>
    </Card>
  );
}
