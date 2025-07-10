import { Sheet, SheetContent, SheetHeader, SheetTitle } from "../ui/sheet";
import PlugSetup from "../plug-setup/plug-setup";
import { usePlugContext } from "@/context/plug";
import { getPlugHandlerConfig, setPlugHandlerConfig } from "@/api/plugs";
import { Button } from "../ui/button";
import { useEffect } from "react";

export default function PlugSetting(props: {isOpen: boolean, setIsOpen: (open: boolean) => void, device_id: string, plug_index: number}) {

    const plugData = usePlugContext();

    async function setPlugHandler(){
      let res = await setPlugHandlerConfig(props.device_id, props.plug_index, plugData)
      if (res.error) {
        return
      }
      props.setIsOpen(false)
    }

    async function onOpenChange(){
      if (props.isOpen) {
        let {data} = await getPlugHandlerConfig(props.device_id, props.plug_index)
        if (data){
          console.log(data);
          plugData.setSensor(data.sensor);
          plugData.setCoolHolder(data.plug_config.cool_holder)
          plugData.setDeadAreas(data.plug_config.dead_areas)
          plugData.setCurvePoints(data.plug_config.curve);
        } else {
          plugData.setSensor(undefined);
          plugData.setCoolHolder(undefined);
          plugData.setDeadAreas([]);
          plugData.setCurvePoints([]);
        }
      }
    }

    useEffect(() => {
      onOpenChange()
    }, [props.isOpen])

    return (
      <Sheet open={props.isOpen} onOpenChange={props.setIsOpen}>
        <SheetContent
          side="left"
          className="w-full h-full pb-5 flex flex-col justify-between overflow-auto"
        >
          <SheetHeader>
            <SheetTitle>Plug Settings</SheetTitle>
          </SheetHeader>
          <PlugSetup/>
          <Button onClick={setPlugHandler} disabled={!plugData.sensor}>Set Plug</Button>
        </SheetContent>
      </Sheet>
    );
}