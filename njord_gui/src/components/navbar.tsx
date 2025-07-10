import { SlidersHorizontal } from "lucide-react";
import { Button } from "./ui/button";
import AddDevice from "./add-device/add-device";
import { DeviceDataContextProvider } from "@/context/device";

export default function NavBar() {
  return (
    <nav className="flex w-full space-x-2">
      <Button>
        <SlidersHorizontal />
      </Button>
      <DeviceDataContextProvider>
        <AddDevice></AddDevice>
      </DeviceDataContextProvider>
    </nav>
  );
}
