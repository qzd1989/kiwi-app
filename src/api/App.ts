import { invoke } from "@tauri-apps/api/core";

class App {
  static async toListener(): Promise<void> {
    try {
      return await invoke("set_role_to_listener");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async toUser(): Promise<void> {
    try {
      return await invoke("set_role_to_user");
    } catch (e: unknown) {
      throw e;
    }
  }
}

export { App };
