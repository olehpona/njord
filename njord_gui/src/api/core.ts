import { CoreMessage } from "@/types/api";
import { invoke } from "@tauri-apps/api/core";
import {GET_CORE_MESSAGES, LOAD_SETTINGS, SAVE_SETTINGS} from "./paths";

export async function get_core_messages(): Promise<CoreMessage[]> {
  return invoke<CoreMessage[]>(GET_CORE_MESSAGES);
}

export async function load_settings(){
  console.log(await invoke(LOAD_SETTINGS));
}

export async function save_settings(){
  await invoke(SAVE_SETTINGS);
}