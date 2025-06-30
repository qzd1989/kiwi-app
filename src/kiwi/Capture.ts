import { invoke } from "@tauri-apps/api/core";
import { Size } from "@types";
import { msgError } from "@utils/msg";

class Capture {
  async getMonitorSize(): Promise<Size> {
    try {
      return await invoke("get_monitor_size");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async requestFrameData(): Promise<void> {
    try {
      await invoke("request_frame_data");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { Capture };
