import { defineStore } from "pinia";
import { load, Store } from "@tauri-apps/plugin-store";
import { Size } from "@types";
import { App, Project } from "@kiwi";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

type LocalStoreKey = "projectRootDirectory" | "isPythonAttributed";
interface Enable {
  isWebsocketAlive: boolean;
}
namespace Enable {
  export const init = (): Enable => ({
    isWebsocketAlive: false,
  });
}
interface Zoom {
  factor: number;
  min: number;
  max: number;
}
namespace Zoom {
  export const init = (): Zoom => ({
    factor: 1,
    min: 0.5,
    max: 1.5,
  });
}

const useStateStore = defineStore("store", {
  state: () => ({
    enable: Enable.init(),
    zoom: Zoom.init(),
    monitorSize: Size.from(0, 0),

    //kiwi object
    app: App.empty(),
    project: Project.empty(),
  }),
  persist: true,
});

class LocalStore {
  instance: Store | null = null;
  storeFile = "kiwi.json";
  readonly keys: LocalStoreKey[] = [
    "projectRootDirectory",
    "isPythonAttributed",
  ];

  async init() {
    if (this.instance == null) {
      this.instance = await load(this.storeFile, { autoSave: true });
    }
  }

  async get<T = unknown>(key: LocalStoreKey): Promise<T | null> {
    if (!this.keys.includes(key)) {
      return null;
    }
    if (!this.instance) await this.init();
    const value = await this.instance!.get<T>(key);
    return value === undefined ? null : value;
  }

  async set<T = unknown>(key: LocalStoreKey, value: T) {
    if (!this.keys.includes(key)) {
      throw new Error(t("LocalStore Key is not registered.", { key }));
    }
    if (!this.instance) await this.init();
    await this.instance?.set(key, value);
  }

  async clear() {
    await this.instance?.clear();
  }
}

const localStore = new LocalStore();
export { useStateStore, localStore };
