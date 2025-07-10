import { usePlugContext } from "@/context/plug";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { useEffect, useId, useState } from "react";
import { SensorsData } from "@/types/plugs";
import { getSensors } from "@/api/plugs";
import { Label } from "../ui/label";

export default function SensorSelect() {
  const { sensor, setSensor } = usePlugContext();

  const [sensors, setSensors] = useState<SensorsData>({});

  useEffect(() => {
    getSensors().then((data) => {
      data.data ? setSensors(data.data) : {};
    });
  }, []);

  const selectId = useId();

  return (
    <div className="w-full">
      <Label className="font-semibold text-lg" htmlFor={selectId}>
        Sensor
      </Label>
      <Select
        onValueChange={(value) => setSensor(JSON.parse(value))}
        value={JSON.stringify(sensor)}
      >
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select Sensor" />
        </SelectTrigger>
        <SelectContent id={selectId}>
          {Object.entries(sensors).map(([sensor_type, value]) => (
            <SelectGroup key={sensor_type}>
              <SelectLabel>{sensor_type}</SelectLabel>
              {value.map((identifier) => (
                <SelectItem
                  key={identifier}
                  value={JSON.stringify({ sensor_type, identifier })}
                >
                  {identifier}
                </SelectItem>
              ))}
            </SelectGroup>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
