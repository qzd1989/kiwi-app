import { useAppStore } from "@store";
import WebSocket from "@tauri-apps/plugin-websocket";

type Method =
  | "save_template"
  | "capture"
  | "health_check"
  | "find_image"
  | "find_images"
  | "find_relative_colors"
  | "find_colors"
  | "recognize_text"
  | "save_frame"
  | "click_left"
  | "click_right"
  | "press_left"
  | "press_right"
  | "release_left"
  | "release_right"
  | "move_absolute"
  | "move_relative"
  | "get_mouse_location"
  | "scroll_vertical"
  | "scroll_horizontal"
  | "press_key"
  | "release_key"
  | "click_key"
  | "input_text";

type RequestPayload = {
  method: string;
  args?: Record<string, any>;
};

type Response<T = any> = {
  status: "success" | "error";
  message: string;
  data: T | null;
};

class Api {
  static async request<T = any>(
    method: Method,
    args: Record<string, any> | null = null,
  ): Promise<T | null> {
    const appStore = useAppStore();
    const ws = await WebSocket.connect(`ws://${appStore.remoteServerAddress}`);

    const payload: RequestPayload = { method };
    if (args) payload.args = args;

    return new Promise<T | null>((resolve, reject) => {
      const listener = (msg: any) => {
        try {
          // msg.data 是 JSON 字符串，需要解析
          const data =
            typeof msg.data === "string" ? JSON.parse(msg.data) : msg.data;
          resolve(data as T);
        } catch (err) {
          reject(err);
        } finally {
          ws.disconnect().catch(() => {});
        }
      };

      ws.addListener(listener);

      ws.send(JSON.stringify(payload)).catch((err) => {
        ws.disconnect().catch(() => {});
        reject(err);
      });
    });
  }
}

export { Api };
export type { Response };
