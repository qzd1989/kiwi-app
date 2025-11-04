import { invoke } from "@tauri-apps/api/core";
import { u16 } from "@types";

class Server {
  constructor(
    public ip: string,
    public port: u16,
  ) {}

  address(): string {
    return `${this.ip}:${this.port}`;
  }

  static async startLocal(): Promise<Server> {
    try {
      return await invoke("start_local_server");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async startAny(): Promise<Server> {
    try {
      return await invoke("start_any_server");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async shutdown(): Promise<void> {
    try {
      await invoke("shutdown_server");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async isRemoteAlive(address: string): Promise<boolean> {
    try {
      return await invoke("is_remote_server_alive", { address });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async getLocalAddress(): Promise<string> {
    try {
      return await invoke("get_local_server_address");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async getLanAddress(): Promise<string> {
    try {
      return await invoke("get_lan_server_address");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async getRemoteAddress(): Promise<string> {
    try {
      return await invoke("get_remote_server_address");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async setRemoteAddress(address: string): Promise<string> {
    try {
      return await invoke("set_remote_server_address", { address });
    } catch (e: unknown) {
      throw e;
    }
  }
}

export { Server };
