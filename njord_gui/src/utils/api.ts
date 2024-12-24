import { DeviceConfig } from "@/types/api";
import { PlugSetting } from "@/types/device";
import { DEFAULT_PLUG_VALUE } from "@/const";

export function deviceConfigToPlugSetting(
  deviceConfig: DeviceConfig
): PlugSetting[] {
  const plugs = deviceConfig.ports;
  const defaultValues = deviceConfig.default_values;
  return plugs.map((el, index) => ({
    port: el,
    default_value: defaultValues[index] !== undefined
      ? defaultValues[index]
      : DEFAULT_PLUG_VALUE,
  }));
}

export function plugSettingToDeviceConfig(
  deviceConfig: DeviceConfig,
  plugSettings: PlugSetting[]
): DeviceConfig {
  let ports = [];
  let default_values = [];

  for (let plugSetting of plugSettings) {
    ports.push(plugSetting.port);
    default_values.push(plugSetting.default_value);
  }

  return {
    ...deviceConfig,
    ports,
    default_values,
  };
}
