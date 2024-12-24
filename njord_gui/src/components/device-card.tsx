import { Settings, Trash } from "lucide-react";
import { Button } from "./ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Badge } from "./ui/badge";
import { Device } from "@/types/api";
import { removeDevice } from "@/api/device";

export default function DeviceCard({
  deviceId,
  device,
}: {
  deviceId: string;
  device: Device;
}) {

  function generateBadges() {
    let elements = [];
    for (let i = 0; i < device.device_info.max_ports; i++) {
      elements.push(
        <Badge variant={"outline"} className="space-x-1">
          <p className="font-bold">{i}:</p>
          <p className="font-semibold text-muted-foreground">38C / 80%</p>
        </Badge>
      );
    }
    return elements
  }

  function removeDeviceHandler() {
    removeDevice(deviceId);
  }

  return (
    <Card className="w-full">
      <CardHeader>
        <Badge variant={"outline"} className="bg-error text-white">
          Unreachable: Permission denied
        </Badge>
        <CardTitle>{device.device_info.board_name}</CardTitle>
        <CardDescription>On: {device.serial_info.com_port}</CardDescription>
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
          <Button variant="ghost">
            <Settings />
          </Button>
          <Button variant="ghost" onClick={removeDeviceHandler}>
            <Trash />
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
