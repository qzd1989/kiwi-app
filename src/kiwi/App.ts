import { invoke } from "@tauri-apps/api/core";
import { msgError } from "@utils/msg";
import { getVersion, getName } from "@tauri-apps/api/app";
import { apiFetch } from "@utils/api";
import { type, arch } from "@tauri-apps/plugin-os";

interface PlatformInfo {
  signature: string;
  force_update: boolean;
  url: string;
}

interface ReleaseInfo {
  version: string;
  notes: string[];
  pub_date: string; // RFC 3339 格式的字符串
  platforms: {
    [platform: string]: PlatformInfo;
  };
}

interface Release {
  version: string;
  notes: string[];
  force_update: boolean;
  pub_date: string;
  url: string;
}

interface ConfigApp {
  websocket_port: number;
  locale: string;
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

  async checkRelease(): Promise<Release | null> {
    const release = (await apiFetch("/version.json")) as ReleaseInfo;
    const osName = await type();
    const archName = await arch();
    if (this.version == release.version) return null;
    const platformKey = osName + "-" + archName;
    if (!(platformKey in release.platforms)) return null;
    const platformInfo = release.platforms[platformKey];
    if (platformInfo.url == "") return null;
    return {
      version: release.version,
      notes: release.notes,
      force_update: platformInfo.force_update,
      pub_date: release.pub_date,
      url: platformInfo.url,
    };
  }
}

export { App, AppModel };
export type { Release };
