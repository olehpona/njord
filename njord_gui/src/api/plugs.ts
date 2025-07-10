import { SensorsData } from "@/types/plugs";
import { errorWrapper } from "@/utils/errorWrapper";
import { invoke } from "@tauri-apps/api/core";
import {
  GET_PLUG_HANDLER_CONFIG,
  GET_PLUG_STATES,
  GET_SENSORS,
  SET_PLUG_HANDLER_CONFIG,
} from "./paths";
import { WrappedError } from "@/types/utils";
import {
  CoolHolderData,
  CurvePoint,
  DeadArea,
  PlugData,
  Sensor,
} from "@/context/plug";

export async function getSensors(): Promise<WrappedError<SensorsData>> {
  return errorWrapper<SensorsData>(() => invoke(GET_SENSORS));
}

export async function setPlugHandlerConfig(
  deviceId: string,
  plugIndex: number,
  data: PlugData
): Promise<WrappedError<unknown>> {
  return errorWrapper<unknown>(() =>
    invoke(SET_PLUG_HANDLER_CONFIG, {
      deviceId,
      plugIndex,
      plugConfig: {
        curve: data.curve,
        dead_areas: data.dead_areas,
        cool_holder: data.cool_holder,
      },
      sensorId: data.sensor,
    })
  );
}

export interface PlugHandlerData {
  sensor: Sensor;
  plug_config: {
    curve: CurvePoint[];
    dead_areas: DeadArea[];
    cool_holder: CoolHolderData | undefined;
  };
}

export async function getPlugHandlerConfig(
  deviceId: string,
  plugIndex: number
): Promise<WrappedError<PlugHandlerData>> {
  return errorWrapper<PlugHandlerData>(() =>
    invoke(GET_PLUG_HANDLER_CONFIG, {
      deviceId,
      plugIndex,
    })
  );
}

export interface PlugState {
  last_temp: number;
  plug_value: number;
}

export async function getPlugsStates(
  deviceId: string
): Promise<WrappedError<(PlugState | undefined)[]>> {
  return errorWrapper<(PlugState | undefined)[]>(() =>
    invoke(GET_PLUG_STATES, { deviceId })
  );
}
