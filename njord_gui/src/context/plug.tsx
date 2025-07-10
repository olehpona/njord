import { createContext, ReactNode, useContext, useState } from "react";

export interface Sensor {
  sensor_type: string;
  identifier: string;
}

export interface CurvePoint {
  temp: number;
  value: number;
}

export interface DeadArea {
  min_value: number;
  max_value: number;
  variant: string;
}

export interface CoolHolderData {
  holding_time: number;
  on_delta: number;
  off_delta: number;
}

export interface PlugData {
  sensor: Sensor | undefined;
  curve: CurvePoint[];
  dead_areas: DeadArea[];
  cool_holder: CoolHolderData | undefined;
}

export interface PlugDataUpdaters {
  setSensor: (sensor: Sensor| undefined) => void;
  setCurvePoints: (curvePoints: CurvePoint[]) => void;
  setDeadAreas: (deadAreas: DeadArea[]) => void;
  setCoolHolder: (coolHandler: CoolHolderData| undefined) => void;
}

export type PlugConfig = PlugData & PlugDataUpdaters
const PlugContext = createContext<PlugConfig>({} as PlugConfig);

export default function PlugContextProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [sensor, setSensor] = useState<Sensor>()
  const [curvePoints, setCurvePoints] = useState<CurvePoint[]>([]);
  const [deadAreas, setDeadAreas] = useState<DeadArea[]>([]);
  const [coolHolder, setCoolHolder] = useState<CoolHolderData>();
  const updaters = {
    setSensor,
    setCurvePoints,
    setDeadAreas,
    setCoolHolder,
  };
  return (
    <PlugContext.Provider
      value={{
        sensor,
        curve: curvePoints,
        dead_areas: deadAreas,
        cool_holder: coolHolder,
        ...updaters,
      }}
    >
      {children}
    </PlugContext.Provider>
  );
}

export const usePlugContext = () => useContext(PlugContext);
