import { useAppStore } from "@store";

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
  args: Record<string, any> | null;
};

type Response<T = any> = {
  status: "success" | "error";
  message: string;
  data: T | null;
};

class Api {
  private static instance: Api | null = null;
  private ws: WebSocket;
  private pending: Array<(res: Response) => void> = [];
  private connected = false;
  private readyPromise: Promise<void>;
  private readyResolve!: () => void;

  private constructor(url: string) {
    this.ws = new WebSocket(url);

    this.readyPromise = new Promise((resolve) => {
      this.readyResolve = resolve;
    });

    this.ws.addEventListener("open", () => {
      this.connected = true;
      this.readyResolve();
    });

    this.ws.addEventListener("message", (event) => {
      try {
        const msg = JSON.parse(event.data);
        if (this.pending.length > 0) {
          const resolver = this.pending.shift()!;
          resolver(msg);
        }
      } catch (err) {
        console.error("[Api] Invalid message:", event.data);
      }
    });

    this.ws.addEventListener("close", () => {
      this.connected = false;
      console.warn("[Api] WebSocket disconnected");
    });

    this.ws.addEventListener("error", (err) => {
      console.error("[Api] WebSocket error:", err);
    });
  }

  private async ready() {
    if (this.connected) {
      return;
    }
    await this.readyPromise;
  }

  static getInstance(): Api {
    if (!Api.instance) {
      const appStore = useAppStore();
      Api.instance = new Api(`ws://${appStore.remoteServerAddress}`);
    }
    return Api.instance;
  }

  static async request<T = any>(
    method: Method,
    args: Record<string, any> | null = null,
  ): Promise<Response<T>> {
    const api = Api.getInstance();
    return api._request<T>(method, args);
  }

  async _request<T = any>(
    method: Method,
    args: Record<string, any> | null = null,
  ): Promise<Response<T>> {
    await this.ready();

    const payload = { method, args } satisfies RequestPayload;

    return new Promise((resolve, reject) => {
      this.pending.push(resolve);

      try {
        this.ws.send(JSON.stringify(payload));
      } catch (err) {
        this.pending.pop();
        reject(err);
        return;
      }

      const timeout = setTimeout(() => {
        const index = this.pending.indexOf(resolve);
        if (index !== -1) {
          this.pending.splice(index, 1);
        }
        reject(new Error("Request timed out after 5 seconds"));
      }, 5000);

      const wrappedResolve = (res: Response) => {
        clearTimeout(timeout);
        resolve(res);
      };

      // Replace the original resolve with wrappedResolve to clear timeout on response
      this.pending[this.pending.length - 1] = wrappedResolve;
    });
  }

  close() {
    this.ws.close();
  }
}

export { Api };
export type { Response };
