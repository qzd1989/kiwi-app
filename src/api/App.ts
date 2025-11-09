import { invoke } from "@tauri-apps/api/core";
import { Release } from "@types";

class App {
  static async toListener(): Promise<void> {
    try {
      return await invoke("set_role_to_listener");
    } catch (e) {
      throw e;
    }
  }

  static async toUser(): Promise<void> {
    try {
      return await invoke("set_role_to_user");
    } catch (e) {
      throw e;
    }
  }

  static async version(): Promise<string> {
    try {
      return await invoke("get_app_version");
    } catch (e) {
      throw e;
    }
  }

  static async checkRelease(): Promise<Release | null> {
    try {
      return await invoke("get_release");
    } catch (e) {
      throw e;
    }
  }
}

export { App };
