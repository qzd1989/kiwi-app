import { load, Store } from "@tauri-apps/plugin-store";

class LocalStore {
  private store!: Store;
  private ready: Promise<void>;

  /**
   * @param path 存储文件名，默认 "store.json"
   * @param autoSave 是否自动保存，默认 true
   */
  constructor(path: string = "store.json", autoSave: boolean = true) {
    this.ready = this.init(path, autoSave);
  }

  // 初始化 store
  private async init(path: string, autoSave: boolean) {
    this.store = await load(path, {
      autoSave,
      defaults: {},
    });
  }

  /** 等待 store 初始化完成 */
  private async ensureReady() {
    await this.ready;
  }

  /** 获取值 */
  public async get<T>(key: string): Promise<T | undefined> {
    await this.ensureReady();
    return this.store.get<T>(key);
  }

  /** 设置值 */
  public async set<T>(key: string, value: T): Promise<void> {
    await this.ensureReady();
    await this.store.set(key, value);
    await this.save();
  }

  /** 手动保存 */
  public async save(): Promise<void> {
    await this.ensureReady();
    await this.store.save();
  }
}

const useLocalStore = (): LocalStore => {
  const store = new LocalStore();
  return store;
};

export { useLocalStore };
