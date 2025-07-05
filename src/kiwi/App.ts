import { invoke } from "@tauri-apps/api/core";
import { msgError } from "@utils/msg";
import { getVersion, getName } from "@tauri-apps/api/app";
import { apiFetch } from "@utils/api";
import { type, arch } from "@tauri-apps/plugin-os";
import { Locale, u32 } from "@types";

// 定义 Release 接口
interface Release {
  signature: string;
  version: string;
  pub_date: string; // RFC 3339 格式的字符串 "2025-07-02T15:43:00+08:00"
  force_update: boolean;
  notes: string[];
  url: string;
  size: u32;
}

type Platforms = Record<string, Release>;

interface PlatformData {
  platforms: Platforms;
}

interface ConfigApp {
  websocket_port: number;
  locale: Locale;
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

  // 静态方法获取应用信息
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

  // 保存应用配置
  async save(): Promise<void> {
    try {
      await invoke("save_app_config", { config: this.config });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  // 检查平台的 Release 更新
  async checkRelease(): Promise<Release | null> {
    const locale = this.app.config?.app.locale as string;
    const platformData = (await apiFetch("/version.json", {
      params: {
        locale,
      },
    })) as PlatformData;
    const platforms = platformData.platforms;
    const osName = await type();
    const archName = await arch();
    const platformKey = osName + "-" + archName;
    if (!(platformKey in platforms)) return null;
    let release = platforms[platformKey];
    if (release.version === this.version) {
      return null; // 没有更新
    }
    return release;
  }
}

export { App, AppModel };
export type { Release };
