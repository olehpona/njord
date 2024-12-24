import { toast, Toaster } from "sonner";
import NavBar from "./components/navbar";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { get_core_messages } from "./api/core";
import { CoreMessage, Device } from "./types/api";
import CoreMessageCard from "./components/core-message-card";
import { useDeviceStore } from "./store/device";
import DeviceList from "./components/device-list";

interface DeviceUpdateResponse {
  [key: string]: Device;
}

function App() {
  const [coreMessages, setCoreMessages] = useState<CoreMessage[]>([]);
  const { setDevices } = useDeviceStore();

  let coreMessageHandler = async () => {
    let messages = await get_core_messages();
    console.log(messages);
    setCoreMessages(messages);
  };

  useEffect(() => {
    listen<string>("errors", (error) => {
      console.log(error);
      toast.error(error.payload);
    });
    listen<DeviceUpdateResponse>("devices_update", (data) =>
      setDevices(
        Object.entries(data.payload).map(([key, value]) => ({
          id: key,
          device: value,
        }))
      )
    );
    coreMessageHandler();
  }, []);

  return (
    <div className="bg-background h-dvh flex items-center flex-col p-2 space-y-4">
      <NavBar></NavBar>
      {coreMessages.map((e) => (
        <CoreMessageCard message={e} key={e.message} />
      ))}
      <DeviceList />
      <Toaster></Toaster>
    </div>
  );
}

export default App;
