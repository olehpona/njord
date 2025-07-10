import { Plus, RefreshCcw } from "lucide-react";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "../ui/sheet";
import { Button } from "../ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { useEffect, useState } from "react";
import { PortInfo } from "@/types/api";
import { BAUD_RATES } from "@/const";
import { loadDeviceInfoApi, getDevicesListApi, addDevice } from "@/api/device";
import DeviceSetup from "../device-setup/device-setup";
import { useDeviceContext } from "@/context/device";

export default function AddDevice() {
  const deviceContext = useDeviceContext();
  const { deviceInfo, serialInfo, deviceConfig } = deviceContext.data;
  const { setDeviceInfo, setSerialInfo, clear } = deviceContext.updaters;

  const [devices, setDevices] = useState<PortInfo[]>([]);
  const [isConnecting, setIsConnecting] = useState<boolean>(false);

  const [isSheetOpen, setIsSheetOpen] = useState(false);

  async function getDevicesList() {
    const deviceList = await getDevicesListApi();
    deviceList.data ? setDevices(deviceList.data) : null;
  }

  function changeComPort(value: string) {
    setDeviceInfo({ board_name: "", max_ports: -1 });
    setSerialInfo({ com_port: value, baud_rate: serialInfo.baud_rate });
  }

  function changeBaudPort(value: string) {
    setDeviceInfo({ board_name: "", max_ports: -1 });
    setSerialInfo({
      com_port: serialInfo.com_port,
      baud_rate: parseInt(value),
    });
  }

  async function getDeviceInfo() {
    setIsConnecting(true);
    const info = await loadDeviceInfoApi(serialInfo);
    setIsConnecting(false);
    info.data ? setDeviceInfo(info.data) : null;
  }

  async function addDeviceHandler() {
    const response = await addDevice(serialInfo, deviceConfig);
    response.error ? null : setIsSheetOpen(false);
  }

  useEffect(() => {
    if (!isSheetOpen) clear()
  }, [isSheetOpen])

  useEffect(() => {
    getDevicesList();
  }, []);

  return (
    <Sheet open={isSheetOpen} onOpenChange={() => setIsSheetOpen(!isSheetOpen)}>
      <SheetTrigger className="flex-1">
        <Button className="w-full">
          <Plus />
        </Button>
      </SheetTrigger>
      <SheetContent side="left">
        <SheetHeader>
          <SheetTitle>Add Device</SheetTitle>
        </SheetHeader>
        <div className="w-full h-full pb-5 flex flex-col justify-between overflow-auto">
          <div className="w-full space-y-5 mb-5">
            <div className="w-full space-y-1">
              <div className="w-full flex space-x-2">
                <Select
                  onValueChange={changeComPort}
                  value={serialInfo.com_port}
                >
                  <SelectTrigger className="w-[40%]">
                    <SelectValue placeholder="Port" />
                  </SelectTrigger>
                  <SelectContent>
                    {devices.map((e) => (
                      <SelectItem key={e.name} value={e.name}>
                        <span className="font-bold">{e.name}</span>{" "}
                        <span className="font-light text-sm">
                          {e.port_type} {e.device_data}
                        </span>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Select
                  onValueChange={changeBaudPort}
                  value={serialInfo.baud_rate.toString()}
                >
                  <SelectTrigger className="w-[40%]">
                    <SelectValue placeholder="Baud" />
                  </SelectTrigger>
                  <SelectContent>
                    {BAUD_RATES.map((e) => (
                      <SelectItem key={e} value={e.toString()}>
                        {e}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button className="w-[20%]" onClick={() => getDevicesList()}>
                  <RefreshCcw></RefreshCcw>
                </Button>
              </div>
              <Button disabled={isConnecting} onClick={() => getDeviceInfo()}>
                Connect
              </Button>
            </div>
            <div>
              <p>
                <span className="font-bold">Device:</span>{" "}
                {deviceInfo.board_name}
              </p>
              <p>
                <span className="font-bold">Plugs:</span>{" "}
                {deviceInfo.max_ports <= 0 ? "" : deviceInfo.max_ports}
              </p>
            </div>
          </div>
          <DeviceSetup></DeviceSetup>
          <Button
            disabled={deviceInfo.max_ports <= 0}
            onClick={addDeviceHandler}
            className="w-full"
          >
            Add
          </Button>
        </div>
      </SheetContent>
    </Sheet>
  );
}
