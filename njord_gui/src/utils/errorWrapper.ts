import { WrappedError } from "@/types/utils";
import { toast } from "sonner";

export async function errorWrapper<T>(
  fn: () => Promise<T>
): Promise<WrappedError<T>> {
  try {
    const data = await fn();
    return { data, error: null };
  } catch (error) {
    const error_msg = error instanceof Error ? error.message : error as string;

    toast.error(error_msg)
    console.error(error_msg)

    return {
      data: null,
      error: error_msg,
    };
  }
}
