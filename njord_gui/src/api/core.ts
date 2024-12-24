import { CoreMessage } from "@/types/api";
import { invoke } from "@tauri-apps/api/core";
import { GET_CORE_MESSAGES } from "./paths";

export async function get_core_messages(): Promise<CoreMessage[]> {
  return invoke<CoreMessage[]>(GET_CORE_MESSAGES);
}
