import { DeviceConfig, DeviceInfo, PortInfo, SerialInfo } from "@/types/api";
import { errorWrapper } from "@/utils/errorWrapper";
import { invoke } from "@tauri-apps/api/core";
import {
  LOAD_DEVICE_INFO,
  GET_DEVICE_LIST,
  LOAD_DEVICE_CONFIG,
  LOAD_DEVICE_DEFAULT_CONFIG,
  ADD_DEVICE,
  REMOVE_DEVICE,
} from "./paths";
import { WrappedError } from "@/types/utils";

export async function getDevicesListApi(): Promise<WrappedError<PortInfo[]>> {
  return errorWrapper<PortInfo[]>(() => invoke(GET_DEVICE_LIST));
}

export async function loadDeviceInfoApi(
  serialInfo: SerialInfo
): Promise<WrappedError<DeviceInfo>> {
  return errorWrapper<DeviceInfo>(() =>
    invoke(LOAD_DEVICE_INFO, { serialInfo })
  );
}

export async function loadDeviceConfig(
  serialInfo: SerialInfo
): Promise<WrappedError<DeviceConfig>> {
  return errorWrapper<DeviceConfig>(() =>
    invoke(LOAD_DEVICE_CONFIG, { serialInfo })
  );
}

export async function loadDeviceDefaultConfig(
  serialInfo: SerialInfo
): Promise<WrappedError<DeviceConfig>> {
  return errorWrapper<DeviceConfig>(() =>
    invoke(LOAD_DEVICE_DEFAULT_CONFIG, { serialInfo })
  );
}

export async function addDevice(
  serialInfo: SerialInfo,
  deviceConfig: DeviceConfig
): Promise<WrappedError<unknown>> {
  return errorWrapper<unknown>(() =>
    invoke(ADD_DEVICE, { serialInfo, deviceConfig })
  );
}

export async function removeDevice(id: string): Promise<WrappedError<unknown>> {
  return errorWrapper<unknown>(() => invoke(REMOVE_DEVICE, { id }));
}