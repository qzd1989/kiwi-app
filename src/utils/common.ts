import { invoke } from "@tauri-apps/api/core";
import { getAllWindows } from "@tauri-apps/api/window";

type u32 = number;
type i32 = number;
type f64 = number;
type Base64Png = string;
type RgbaBuffer = Uint8Array;
type Language = "python" | "lua";
type HexColor = `#${string}`;
type WindowLabel = "main" | "monitor";
type u8 = number;
type RgbColor = {
  r: u8;
  g: u8;
  b: u8;
};

namespace RgbColor {
  export const from = (r: number, g: number, b: number): RgbColor => {
    const rU8 = u8.from(r);
    const gU8 = u8.from(g);
    const bU8 = u8.from(b);
    return {
      r: rU8,
      g: gU8,
      b: bU8,
    };
  };
}

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

  export const fromRgbColor = (rgbPixelColor: RgbColor): HexColor => {
    function toHex(colorValue: u8) {
      var hex = colorValue.toString(16);
      return hex.length === 1 ? "0" + hex : hex;
    }
    const hexString =
      "#" +
      toHex(rgbPixelColor.r) +
      toHex(rgbPixelColor.g) +
      toHex(rgbPixelColor.b);
    return HexColor.from(hexString);
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

const cropBase64Png = (
  data: Base64Png,
  start: Point,
  size: Size
): Promise<Base64Png> => {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = function () {
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");
      if (!ctx) return null;
      canvas.width = size.width;
      canvas.height = size.height;
      ctx.drawImage(
        img,
        start.x,
        start.y,
        size.width,
        size.height,
        0,
        0,
        size.width,
        size.height
      );
      const croppedBase64 = canvas.toDataURL("image/png");
      resolve(croppedBase64);
    };
    img.onerror = function () {
      reject(new Error("Could not load image."));
    };
    img.src = data;
  });
};

const drawBase64PngImageOnCanvas = (
  canvas: HTMLCanvasElement,
  data: Base64Png,
  start: Point,
  size: Size
) => {
  return new Promise((resolve) => {
    const imageData = new Image();
    imageData.onload = function () {
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.drawImage(imageData, start.x, start.y, size.width, size.height);
      resolve(ctx);
    };
    imageData.src = data;
  });
};

const drawText = (
  ctx: CanvasRenderingContext2D,
  text: string,
  point: Point
) => {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.font = "12px system-ui";
  ctx.strokeText(text, point.x, point.y);
  ctx.fillStyle = "#ff0000";
  ctx.fillText(text, point.x, point.y);
};

const drawArc = (
  ctx: CanvasRenderingContext2D,
  point: Point,
  radius: number
) => {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.beginPath();
  ctx.arc(point.x, point.y, radius, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.beginPath();
  ctx.arc(point.x, point.y, radius, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.beginPath();
  ctx.arc(point.x, point.y, 1, 0, Math.PI * 2);
  ctx.stroke();
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.beginPath();
  ctx.arc(point.x, point.y, 1, 0, Math.PI * 2);
  ctx.stroke();
};

const drawRect = (ctx: CanvasRenderingContext2D, point: Point, size: Size) => {
  ctx.lineWidth = 3;
  ctx.strokeStyle = "#ffffff";
  ctx.strokeRect(point.x, point.y, size.width, size.height);
  ctx.lineWidth = 1;
  ctx.strokeStyle = "#ff0000";
  ctx.strokeRect(point.x, point.y, size.width, size.height);
};

const base64PngToRgbPixels = (base64Png: Base64Png): Promise<RgbColor[]> => {
  // 创建一个HTML图像对象
  let img = new Image();
  // 解码Base64字符串并设置为图像源
  img.src = base64Png;

  return new Promise((resolve, reject) => {
    // 确保图像加载完成后再处理
    img.onload = function () {
      // 创建一个canvas元素
      let canvas = document.createElement("canvas");
      // 设置canvas大小与图像一致
      canvas.width = img.width;
      canvas.height = img.height;
      // 获取2D渲染上下文
      let ctx = canvas.getContext("2d");
      if (!ctx) return;
      // 将图像绘制到canvas上
      ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
      // 获取图像数据
      let imageData = ctx.getImageData(0, 0, canvas.width, canvas.height).data;

      // 创建一个二维数组来存储像素点颜色
      let pixels = [];
      for (let i = 0; i < imageData.length; i += 4) {
        // 每四个元素代表一个像素点的RGB值
        let pixel = RgbColor.from(
          imageData[i],
          imageData[i + 1],
          imageData[i + 2]
        );
        pixels.push(pixel);
      }

      resolve(pixels); // 解析Promise，返回像素数组
    };

    img.onerror = function () {
      reject(
        new Error("Failed to load image from the provided Base64Png String.")
      );
    };
  });
};

const rgbToHex = (rgbPixelColor: RgbColor) => {
  /**
   * 将单个RGB颜色值转换为两位的十六进制字符串
   * @param {number} colorValue - 单个RGB颜色值（0-255之间的整数）
   * @returns {string} - 两位的十六进制颜色值
   */
  function toHex(colorValue: u8) {
    var hex = colorValue.toString(16); // 将数字转换为十六进制字符串
    return hex.length === 1 ? "0" + hex : hex; // 确保两位字符，如果只有一位则前面加0
  }

  // 构建最终的十六进制颜色值
  var hexColor =
    "#" +
    toHex(rgbPixelColor.r) +
    toHex(rgbPixelColor.g) +
    toHex(rgbPixelColor.b);
  return hexColor; // 返回转换后的十六进制颜色值
};

export {
  u32,
  i32,
  u8,
  f64,
  Base64Png,
  RgbaBuffer,
  HexColor,
  RgbColor,
  Point,
  WeightPoint,
  ColoredPoint,
  Size,
  Stack,
  cropBase64Png,
  drawBase64PngImageOnCanvas,
  base64PngToRgbPixels,
  rgbToHex,
  delay,
  minimizeAll,
  unminimizeAll,
  openWebsocket,
  shutdownWebsocket,
  isWebsocketAlive,
  drawText,
  drawArc,
  drawRect,
};
export type { Language, Progress, EmitData, WindowLabel };
