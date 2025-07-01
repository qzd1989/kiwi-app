import { invoke } from "@tauri-apps/api/core";
import { msgError } from "@utils/msg";
import { getVersion, getName } from "@tauri-apps/api/app";

interface ConfigApp {
  websocket_port: number;
}

interface Config {
  app: ConfigApp;
}

interface App {
  name: string | null;
  version: string | null;
  config: Config | null;
  relativeImageDataPath: string | null;
}

namespace App {
  export const empty = (): App => ({
    name: null,
    version: null,
    config: null,
    relativeImageDataPath: null,
  });

  export const from = (
    name: string,
    version: string,
    config: Config,
    relativeImageDataPath: string
  ): App => ({
    name,
    version,
    config,
    relativeImageDataPath,
  });
}

class AppModel {
  constructor(private app: App) {}

  get name() {
    return this.app.name;
  }

  get version() {
    return this.app.version;
  }

  get config() {
    return this.app.config;
  }

  get relativeImageDataPath() {
    return this.app.relativeImageDataPath;
  }

  static async getApp(): Promise<App> {
    try {
      const name = await getName();
      const version = await getVersion();
      const config = (await invoke("get_app_config")) as Config;
      const relativeImageDataPath = (await invoke(
        "get_relative_image_data_path"
      )) as string;
      return App.from(name, version, config, relativeImageDataPath);
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async save(): Promise<void> {
    try {
      await invoke("save_app_config", { config: this.config });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { App, AppModel };
