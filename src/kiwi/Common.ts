import { invoke } from "@tauri-apps/api/core";
import { WindowLabel } from "@utils/common";
import { msgError } from "@utils/msg";

class Common {
  async pathExists(path: string): Promise<boolean> {
    try {
      return await invoke("path_exists", { path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
    return false;
  }

  async xattrPython(): Promise<void> {
    try {
      return await invoke("xattr_python");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async protectWindows(windows: WindowLabel[]): Promise<void> {
    try {
      return await invoke("protect_windows", { windows });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async unprotectWindows(windows: WindowLabel[]): Promise<void> {
    try {
      return await invoke("unprotect_windows", { windows });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async openWebsocket(port: number): Promise<void> {
    try {
      await invoke("open_websocket", { port });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async shutdownWebsocket(): Promise<void> {
    try {
      await invoke("shutdown_websocket");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async isWebsocketAlive(port: number): Promise<boolean> {
    try {
      return await invoke("is_websocket_alive", { port });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
    return false;
  }
}

export { Common };
