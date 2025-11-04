import { invoke } from "@tauri-apps/api/core";
class Capturer {
  static async start() {
    try {
      await invoke("run_capturer");
    } catch (e) {
      throw e;
    }
  }

  static async stop() {
    try {
      await invoke("stop_capturer");
    } catch (e) {
      throw e;
    }
  }

  static async is_running(): Promise<boolean> {
    try {
      return await invoke("is_capturer_running");
    } catch (e) {
      throw e;
    }
  }
}

export { Capturer };
