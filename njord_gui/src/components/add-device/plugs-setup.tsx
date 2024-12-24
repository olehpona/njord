import { useEffect, useState } from "react";
import { Button } from "../ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Input } from "../ui/input";
import { Trash } from "lucide-react";
import { PlugSetting } from "@/types/device";
import { DEFAULT_PLUG_VALUE } from "@/const";
import { useAddDeviceContext } from "@/context/add-device";
import { deviceConfigToPlugSetting, plugSettingToDeviceConfig } from "@/utils/api";


export default function SetupPlugs() {
  const addDeviceContext = useAddDeviceContext()
  const {deviceInfo, deviceConfig} = addDeviceContext.data;
  const {setDeviceConfig} = addDeviceContext.updaters;
  const [plugs, setPlugs] = useState<PlugSetting[]>([])

  useEffect(() => {
    setPlugs(deviceConfigToPlugSetting(deviceConfig))
  }, [deviceConfig])



  function addPlug(){
    if (plugs.length < deviceInfo.max_ports){
      let newPlugs = [...plugs];
      newPlugs.push({port: 0, default_value: DEFAULT_PLUG_VALUE});
      setPlugs(newPlugs);
      setDeviceConfig(plugSettingToDeviceConfig(deviceConfig, plugs))
    }
  }

  function handleChangePlug(index: number, value: number){
    let newPlugs = [...plugs]
    newPlugs[index].port = value
    setPlugs(newPlugs)
    setDeviceConfig(plugSettingToDeviceConfig(deviceConfig, plugs))
  }
    function handleChangeDefault(index: number, value: number) {
      let newPlugs = [...plugs];
      newPlugs[index].default_value = value
      setPlugs(newPlugs);
      setDeviceConfig(plugSettingToDeviceConfig(deviceConfig, plugs));
    }

  function handlePlugDelete(elIndex: number){
    setPlugs(plugs.filter((_el, index) => index !== elIndex));
  }

  return (
    <div className="space-y-2 m-4">
      <div className="w-full">
        <Table>
          <TableHeader>
            <TableRow>
              <TableCell>Plug Id</TableCell>
              <TableCell>Plug GPIO</TableCell>
              <TableCell>Default Value %</TableCell>
            </TableRow>
          </TableHeader>
          <TableBody>
            {plugs.map((_el, index) => (
              <TableRow key={index}>
                <TableCell>{index}</TableCell>
                <TableCell>
                  <Input
                    min={0}
                    type="number"
                    value={plugs[index].port}
                    onChange={(e) =>
                      handleChangePlug(index, Number(e.target.value))
                    }
                  ></Input>
                </TableCell>
                <TableCell>
                  <Input
                    min={0}
                    max={100}
                    type="number"
                    value={plugs[index].default_value}
                    onChange={(e) =>
                      handleChangeDefault(index, Number(e.target.value))
                    }
                  ></Input>
                </TableCell>
                <TableCell>
                  <Button
                    variant="ghost"
                    onClick={() => handlePlugDelete(index)}
                  >
                    <Trash></Trash>
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
      <div className="w-full">
        <Button disabled={deviceInfo.max_ports === -1} onClick={addPlug}>
          Add plug
        </Button>
      </div>
    </div>
  );
}
