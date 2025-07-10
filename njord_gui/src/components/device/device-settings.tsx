import { ReactNode, useState } from "react";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "../ui/sheet";
import DeviceSetup from "../device-setup/device-setup";
import { Button } from "../ui/button";
import { useDeviceContext } from "@/context/device";
import { updateDeviceConfig } from "@/api/device";

export default function DeviceSettings(props: {children: ReactNode, id: string}) {
    const DeviceContext = useDeviceContext();
    const [isOpen, setIsOpen] = useState(false);

    async function saveDeviceSettings() {
        let response = await updateDeviceConfig(props.id, DeviceContext.data.deviceConfig);
        response.error ? null : setIsOpen(false);
    }

    return (
      <Sheet open={isOpen} onOpenChange={() => setIsOpen(!isOpen)}>
        <SheetTrigger>{props.children}</SheetTrigger>
        <SheetContent
          side="left"
          className="w-full h-full pb-5 flex flex-col justify-between overflow-auto"
        >
          <SheetHeader>
            <SheetTitle>Device Settings</SheetTitle>
          </SheetHeader>
          <DeviceSetup deviceId={props.id} />
          <Button onClick={saveDeviceSettings}>Save</Button>
        </SheetContent>
      </Sheet>
    );
}