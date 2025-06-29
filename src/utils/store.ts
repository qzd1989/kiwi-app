import { defineStore } from "pinia";
import { load, Store } from "@tauri-apps/plugin-store";
import { App } from "@kiwi/App";

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
    app: new App(),
    zoom: Zoom.init(),
  }),
  persist: true,
});

class StateStore {
  data: ReturnType<typeof useStateStore> | null = null;

  init() {
    if (this.data == null) {
      this.data = useStateStore();
    }
  }

  get app(): App {
    if (!this.data) {
      throw new Error("StateStore not initialized");
    }
    return this.data.app;
  }

  get zoom(): Zoom {
    if (!this.data) {
      throw new Error("StateStore not initialized");
    }
    return this.data.zoom;
  }
}

type LocalStoreKey = "projectRootDirectory" | "isPythonAttributed";

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
      throw new Error(`LocalStore key ${key} is not registered.`);
    }
    if (!this.instance) await this.init();
    await this.instance?.set(key, value);
  }

  async clear() {
    if (!this.instance) await this.init();
    await this.instance?.clear();
  }
}

const localStore = new LocalStore();
const stateStore = new StateStore();

export { stateStore, localStore };
