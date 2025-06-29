import { invoke } from "@tauri-apps/api/core";
import { getAllWindows } from "@tauri-apps/api/window";

type u32 = number;
type i32 = number;
type u8 = number;
type f64 = number;
type Base64Png = string;
type RgbaBuffer = Uint8Array;
type Language = "python" | "lua";
type HexColor = `#${string}`;
type WindowLabel = "main" | "monitor";

namespace u32 {
  export const MIN = 0 as u32;

  export const MAX = 0xffffffff as u32;

  export const from = (value: number): u32 => {
    if (!u32.isValid(value)) {
      throw new Error(`Invalid u32: ${value}`);
    }
    return value as u32;
  };

  export const isValid = (value: number): value is u32 => {
    return (
      Number.isFinite(value) &&
      value >= u32.MIN &&
      value <= u32.MAX &&
      Math.floor(value) === value
    );
  };
}

namespace i32 {
  export const MIN = -0x80000000 as i32;

  export const MAX = 0x7fffffff as i32;

  export const from = (value: number): i32 => {
    if (!i32.isValid(value)) {
      throw new Error(`Invalid i32: ${value}`);
    }
    return value as i32;
  };

  export const isValid = (value: number): value is i32 => {
    return (
      Number.isFinite(value) &&
      value >= MIN &&
      value <= MAX &&
      Math.floor(value) === value
    );
  };
}

namespace u8 {
  export const MIN = 0 as u8;

  export const MAX = 0xff as u8;

  export const from = (value: number): u8 => {
    if (!u8.isValid(value)) {
      throw new Error(`Invalid u8: ${value}`);
    }
    return value as u8;
  };

  export const isValid = (value: number): value is u8 => {
    return (
      Number.isFinite(value) &&
      value >= MIN &&
      value <= MAX &&
      Math.floor(value) === value
    );
  };
}

namespace f64 {
  export const MIN = Number.MIN_VALUE as f64;

  export const MAX = Number.MAX_VALUE as f64;

  export const from = (value: number): f64 => {
    if (!f64.isValid(value)) {
      throw new Error(`Invalid f64: ${value}`);
    }
    return value as f64;
  };

  export const isValid = (value: number): value is f64 => {
    return Number.isFinite(value);
  };
}

namespace Base64Png {
  export const PREFIX = "data:image/png;base64,";

  export const from = (value: string): Base64Png => {
    if (!isValid(value)) {
      throw new Error("Invalid Base64 PNG string");
    }
    return value as Base64Png;
  };

  export const isValid = (value: string): value is Base64Png => {
    return (
      typeof value === "string" &&
      value.startsWith(PREFIX) &&
      isBase64(value.slice(PREFIX.length))
    );
  };

  const isBase64 = (data: string): boolean => {
    return /^[A-Za-z0-9+/]+={0,2}$/.test(data);
  };
}

namespace RgbaBuffer {
  export const from = (data: ArrayLike<number> | ArrayBuffer): RgbaBuffer => {
    const buffer = new Uint8Array(data);
    if (!isValid(buffer)) {
      throw new Error(`Invalid RgbaBuffer: length must be a multiple of 4`);
    }
    return buffer as RgbaBuffer;
  };

  export const isValid = (buf: Uint8Array): buf is RgbaBuffer => {
    return buf instanceof Uint8Array && buf.length % 4 === 0;
  };

  export const pixelCount = (buf: RgbaBuffer): number => {
    return buf.length / 4;
  };

  export const getPixel = (
    buf: RgbaBuffer,
    index: number
  ): [r: u8, g: u8, b: u8, a: u8] => {
    const offset = index * 4;
    if (offset + 3 >= buf.length) {
      throw new Error(`Pixel index ${index} out of bounds`);
    }
    return [
      buf[offset] as u8,
      buf[offset + 1] as u8,
      buf[offset + 2] as u8,
      buf[offset + 3] as u8,
    ];
  };
}

interface Point {
  x: i32;
  y: i32;
}

namespace Point {
  export const from = (x: number, y: number): Point => ({
    x: i32.from(x),
    y: i32.from(y),
  });
}

interface WeightPoint {
  point: Point;
  weight: f64;
}

namespace WeightPoint {
  export const from = (point: Point, weight: number): WeightPoint => ({
    point,
    weight: f64.from(weight),
  });
}

interface ColoredPoint {
  point: Point;
  hex: HexColor;
}

namespace ColoredPoint {
  export const from = (point: Point, hexColor: string): ColoredPoint => ({
    point,
    hex: HexColor.from(hexColor),
  });
}

interface Size {
  width: u32;
  height: u32;
}

namespace Size {
  export const from = (width: number, height: number): Size => ({
    width: u32.from(width),
    height: u32.from(height),
  });
}

namespace HexColor {
  export const from = (value: string): HexColor => {
    if (!HexColor.isValid(value)) {
      throw new Error(`Invalid hex color format: ${value}`);
    }
    return value as HexColor;
  };

  export const isValid = (value: string): value is HexColor => {
    return /^#[0-9A-Fa-f]{6}([0-9A-Fa-f]{2})?$/.test(value);
  };

  export const random = (): HexColor => {
    const hex = Math.floor(Math.random() * 16777215)
      .toString(16)
      .padStart(6, "0");
    return `#${hex}` as HexColor;
  };
}

interface Progress {
  percentage: u32;
  message: string;
}

const delay = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const minimizeAll = async () => {
  const windows = await getAllWindows();
  for (const window of windows) {
    await window.minimize();
  }
};

const unminimizeAll = async () => {
  const windows = await getAllWindows();
  for (const window of windows) {
    window.unminimize().then(() => {
      return window.setFocus();
    });
  }
};

interface EmitData {
  data: string;
  time: number;
}

class Stack<T> {
  limit: number;
  stack: T[];

  constructor(limit: number) {
    this.stack = [];
    this.limit = limit;
  }

  push(element: T) {
    if (this.stack.length >= this.limit) {
      this.stack.shift();
    }
    this.stack.push(element);
  }

  clear(): void {
    this.stack = [];
  }
}

const openWebsocket = async (port: number) => {
  return await invoke("open_websocket", { port });
};

const shutdownWebsocket = async () => {
  return await invoke("shutdown_websocket");
};

const isWebsocketAlive = async (port: number): Promise<boolean> => {
  return await invoke("is_websocket_alive", { port });
};

export {
  u32,
  i32,
  u8,
  f64,
  Base64Png,
  RgbaBuffer,
  HexColor,
  Point,
  WeightPoint,
  ColoredPoint,
  Size,
  Stack,
  delay,
  minimizeAll,
  unminimizeAll,
  openWebsocket,
  shutdownWebsocket,
  isWebsocketAlive,
};
export type { Language, Progress, EmitData, WindowLabel };
