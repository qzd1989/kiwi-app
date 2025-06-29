import { invoke } from "@tauri-apps/api/core";
import { msgErrorObject } from "@utils/msg";

type WindowLabel = "main" | "monitor";

interface ConfigApp {
  websocket_port: number;
}

interface Config {
  app: ConfigApp;
}

class App {
  name: string = "";
  version: string = "";
  config: Config = { app: { websocket_port: 0 } };
  relative_image_data_path: string = "";

  async init() {
    try {
      this.name = await invoke("get_app_name");
      this.version = await invoke("get_app_version");
      this.config = await invoke("get_app_config");
      this.relative_image_data_path = await invoke(
        "get_relative_image_data_path"
      );
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async xattr_python(): Promise<void> {
    try {
      await invoke("xattr_python");
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async save_config(): Promise<void> {
    try {
      await invoke("save_app_config", { config: this.config });
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async open_websocket(port: number): Promise<void> {
    try {
      await invoke("open_websocket", { port });
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async shutdown_websocket(): Promise<void> {
    try {
      await invoke("shutdown_websocket");
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async is_websocket_alive(): Promise<boolean> {
    try {
      return await invoke("is_websocket_alive");
    } catch (e: any) {
      msgErrorObject(e);
    }
    return false;
  }

  async show_windows_from_capture(windows: WindowLabel[]): Promise<void> {
    try {
      await invoke("show_windows_from_capture", { windows });
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async hide_windows_from_capture(windows: WindowLabel[]): Promise<void> {
    try {
      await invoke("hide_windows_from_capture", { windows });
    } catch (e: any) {
      msgErrorObject(e);
    }
  }

  async path_exists(path: string): Promise<boolean> {
    try {
      return await invoke("path_exists", { path });
    } catch (e: any) {
      msgErrorObject(e);
    }
    return false;
  }
}

export { App };
