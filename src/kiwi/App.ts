import { invoke } from "@tauri-apps/api/core";
import { msgError } from "@utils/msg";

interface ConfigApp {
  websocket_port: number;
}

interface Config {
  app: ConfigApp;
}

class App {
  name: string;
  version: string;
  config: Config;
  relativeImageDataPath: string;

  constructor() {
    this.name = "";
    this.version = "";
    this.config = { app: { websocket_port: 0 } };
    this.relativeImageDataPath = "";
  }

  async init() {
    try {
      this.name = await invoke("get_app_name");
      this.version = await invoke("get_app_version");
      this.config = await invoke("get_app_config");
      this.relativeImageDataPath = await invoke("get_relative_image_data_path");
    } catch (e: unknown) {
      msgError(e);
    }
  }

  async save_config(): Promise<void> {
    try {
      await invoke("save_app_config", { config: this.config });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { App };
