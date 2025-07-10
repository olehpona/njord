import { useEffect, useId, useState } from "react";
import { Input } from "../ui/input";
import { Switch } from "../ui/switch";
import { Label } from "../ui/label";
import { usePlugContext } from "@/context/plug";

export default function CoolingHolder() {
  const { cool_holder, setCoolHolder } = usePlugContext();

  const [enabled, setEnabled] = useState(Boolean(cool_holder));
  const [onDelta, setOnDelta] = useState(cool_holder?.on_delta ?? 0);
  const [offDelta, setOffDelta] = useState(cool_holder?.off_delta ?? 0);
  const [holdingTime, setHoldingTime] = useState(
    cool_holder?.holding_time ?? 0
  );

  const blockId = useId();
  const enableToggleId = useId();

  const onDeltaId = useId();
  const offDeltaId = useId();
  const holdingTimeId = useId();

  useEffect(() => {
    if (enabled) {
      setCoolHolder({
        on_delta: onDelta,
        off_delta: offDelta,
        holding_time: holdingTime,
      });
    } else {
      setCoolHolder(undefined);
    }
  }, [enabled, onDelta, offDelta, holdingTime]);

  return (
    <div className="w-full space-y-2">
      <Label className="fort-semibold text-lg" htmlFor={blockId}>
        Cooling Holder
      </Label>
      <div id={blockId} className="w-full">
        <div className="w-full flex space-x-2 items-center">
          <Label htmlFor={enableToggleId}>Enable Cool Holding</Label>
          <Switch
            checked={enabled}
            onCheckedChange={setEnabled}
            id={enableToggleId}
          />
        </div>
      </div>
      <div className="w-full flex space-x-2">
        <div className="flex-1">
          <Label htmlFor={onDeltaId}>On Delta (Δt °C)</Label>
          <Input
            min={0}
            value={onDelta}
            onChange={(e) => setOnDelta(Number(e.target.valueAsNumber))}
            disabled={!enabled}
            id={onDeltaId}
            type="number"
          />
        </div>
        <div className="flex-1">
          <Label htmlFor={offDeltaId}>Off Delta (Δt °C)</Label>
          <Input
            min={0}
            value={offDelta}
            onChange={(e) => setOffDelta(Number(e.target.valueAsNumber))}
            disabled={!enabled}
            id={offDeltaId}
            type="number"
          />
        </div>
      </div>
      <div>
        <Label htmlFor={holdingTimeId}>Holding Time (ms)</Label>
        <Input
          value={holdingTime}
          onChange={(e) => setHoldingTime(e.target.valueAsNumber)}
          disabled={!enabled}
          id={holdingTimeId}
          type="number"
        />
      </div>
    </div>
  );
}
