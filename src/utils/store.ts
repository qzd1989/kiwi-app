import { defineStore } from "pinia";
import { load, Store } from "@tauri-apps/plugin-store";
import { App } from "@kiwi/App";
import { Project } from "@kiwi/Project";
import { Common } from "@kiwi/Common";
import { Capture } from "@kiwi/Capture";
import { Size } from "./common";
import { Frame } from "@kiwi/Frame";
import { Code } from "@kiwi/Code";

type LocalStoreKey = "projectRootDirectory" | "isPythonAttributed";

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

interface Enable {
  isWebsocketAlive: boolean;
}

namespace Enable {
  export const init = (): Enable => ({
    isWebsocketAlive: false,
  });
}

const useStateStore = defineStore("store", {
  state: () => ({
    enable: Enable.init(),
    zoom: Zoom.init(),
    monitorSize: Size.from(0, 0),

    //kiwi object
    app: new App(),
    common: new Common(),
    project: new Project(),
    capture: new Capture(),
    frame: new Frame(),
    code: new Code(),
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
      throw new Error(`LocalStore key ${key} is not registered.`);
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
