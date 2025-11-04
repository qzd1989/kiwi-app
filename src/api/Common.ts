import { invoke } from "@tauri-apps/api/core";
import { windowLabel } from "@types";

class Common {
  static async pathExists(path: string): Promise<boolean> {
    try {
      return await invoke("path_exists", { path });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async xattrPython(): Promise<void> {
    try {
      return await invoke("xattr_python");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async protectWindows(windows: windowLabel[]): Promise<void> {
    try {
      return await invoke("protect_windows", { windows });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async unprotectWindows(windows: windowLabel[]): Promise<void> {
    try {
      return await invoke("unprotect_windows", { windows });
    } catch (e: unknown) {
      throw e;
    }
  }
}

export { Common };
