import { invoke } from "@tauri-apps/api/core";
import { windowLabel } from "@types";

class Common {
  static async pathExists(path: string): Promise<boolean> {
    try {
      return await invoke("path_exists", { path });
    } catch (e) {
      throw e;
    }
  }

  static async xattrInterpreter(): Promise<void> {
    try {
      return await invoke("xattr_interpreter");
    } catch (e) {
      throw e;
    }
  }

  static async protectWindows(windows: windowLabel[]): Promise<void> {
    try {
      return await invoke("protect_windows", { windows });
    } catch (e) {
      throw e;
    }
  }

  static async unprotectWindows(windows: windowLabel[]): Promise<void> {
    try {
      return await invoke("unprotect_windows", { windows });
    } catch (e) {
      throw e;
    }
  }
}

export { Common };
