import { Size } from "./size";
import { readFile } from "@tauri-apps/plugin-fs";
import { u32 } from "./u32";
import { Point } from "./point";
import { ColoredPoint } from "./colored-point";
import { rgbColor } from "./rgb-color";
import { hexColor, rgbToHex } from "./hex-color";

class Png {
  constructor(
    public size: Size,
    public bytes: Uint8Array,
    public base64: string,
  ) {}

  clone(): Png {
    return new Png(this.size, this.bytes, this.base64);
  }

  draw(
    canvas: HTMLCanvasElement,
    drawX = 0,
    drawY = 0,
  ): Promise<CanvasRenderingContext2D> {
    return new Promise((resolve, reject) => {
      const ctx = canvas.getContext("2d", { willReadFrequently: true });
      if (!ctx) return reject(new Error("Canvas context not available"));
      const img = new window.Image();
      img.src = this.base64;
      img.onload = () => {
        ctx.drawImage(img, drawX, drawY);
        resolve(ctx);
      };
      img.onerror = reject;
    });
  }

  crop(start: Point, size: Size): Promise<Png> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = function () {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        if (!ctx) return reject(new Error("Canvas context not available"));
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
          size.height,
        );
        const cropped = canvas.toDataURL("image/png");
        resolve(Png.fromBase64(cropped));
      };
      img.onerror = function () {
        reject(new Error("Could not load image."));
      };
      img.src = this.base64;
    });
  }

  toRgbColors(): Promise<rgbColor[]> {
    let img = new Image();
    img.src = this.base64;
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
        let imageData = ctx.getImageData(
          0,
          0,
          canvas.width,
          canvas.height,
        ).data;

        // 创建一个二维数组来存储像素点颜色
        let pixels = [];
        for (let i = 0; i < imageData.length; i += 4) {
          // 每四个元素代表一个像素点的RGB值
          let pixel: rgbColor = {
            r: imageData[i],
            g: imageData[i + 1],
            b: imageData[i + 2],
          };
          pixels.push(pixel);
        }

        resolve(pixels); // 解析Promise，返回像素数组
      };

      img.onerror = function () {
        reject(
          new Error("Failed to load image from the provided Base64Png string."),
        );
      };
    });
  }

  async toPixels(): Promise<ColoredPoint[]> {
    const colors = await this.toRgbColors();
    let points: ColoredPoint[] = [];
    let index = 0;
    for (let y = 0; y < this.size.height; y++) {
      for (let x = 0; x < this.size.width; x++) {
        let hex: hexColor = rgbToHex(colors[index]);
        let row = new ColoredPoint(new Point(x, y), hex);
        points.push(row);
        index++;
      }
    }
    return points;
  }

  static async fromFile(path: string): Promise<Png> {
    const bytes = await readFile(path);

    // 转成 base64 字符串
    let binary = "";
    const chunkSize = 0x8000;
    for (let i = 0; i < bytes.length; i += chunkSize) {
      const chunk = bytes.subarray(i, i + chunkSize);
      binary += String.fromCharCode(...chunk);
    }
    const base64Raw = btoa(binary);
    const base64 = `data:image/png;base64,${base64Raw}`;

    // 验证 PNG 文件头
    if (
      bytes[0] !== 0x89 ||
      bytes[1] !== 0x50 ||
      bytes[2] !== 0x4e ||
      bytes[3] !== 0x47 ||
      bytes[4] !== 0x0d ||
      bytes[5] !== 0x0a ||
      bytes[6] !== 0x1a ||
      bytes[7] !== 0x0a
    ) {
      throw new Error("Not a valid PNG file");
    }

    // 解析宽高
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const width = view.getUint32(16, false) as u32;
    const height = view.getUint32(20, false) as u32;

    return new Png(new Size(width, height), bytes, base64);
  }

  static fromBase64(base64: string): Png {
    const data = base64;
    const cleanBase64 = data.replace(/^data:image\/png;base64,/, "");
    const binary = atob(cleanBase64);
    const len = binary.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
      bytes[i] = binary.charCodeAt(i);
    }
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const width = view.getUint32(16, false) as u32;
    const height = view.getUint32(20, false) as u32;
    return new Png(new Size(width, height), bytes, base64);
  }

  static fromTrimmedCanvas(canvas: HTMLCanvasElement): Png {
    const width = canvas.width;
    const height = canvas.height;
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("error.canvas_context");

    const imageData = ctx.getImageData(0, 0, width, height);
    const data = imageData.data;
    const getAlpha = (x: number, y: number) => data[(y * width + x) * 4 + 3];

    let top = 0,
      bottom = height - 1,
      left = 0,
      right = width - 1;
    let found = false;

    // Find top
    outer_top: for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        if (getAlpha(x, y) !== 0) {
          top = y;
          found = true;
          break outer_top;
        }
      }
    }

    if (!found) {
      throw new Error("image.transparent");
    }

    // Find bottom
    outer_bottom: for (let y = height - 1; y >= 0; y--) {
      for (let x = 0; x < width; x++) {
        if (getAlpha(x, y) !== 0) {
          bottom = y;
          break outer_bottom;
        }
      }
    }

    // Find left
    outer_left: for (let x = 0; x < width; x++) {
      for (let y = top; y <= bottom; y++) {
        if (getAlpha(x, y) !== 0) {
          left = x;
          break outer_left;
        }
      }
    }

    // Find right
    outer_right: for (let x = width - 1; x >= 0; x--) {
      for (let y = top; y <= bottom; y++) {
        if (getAlpha(x, y) !== 0) {
          right = x;
          break outer_right;
        }
      }
    }

    const cropWidth = right - left + 1;
    const cropHeight = bottom - top + 1;

    const outCanvas = document.createElement("canvas");
    outCanvas.width = cropWidth;
    outCanvas.height = cropHeight;
    const outCtx = outCanvas.getContext("2d");
    if (!outCtx) throw new Error("error.canvas_context");

    outCtx.drawImage(
      canvas,
      left,
      top,
      cropWidth,
      cropHeight,
      0,
      0,
      cropWidth,
      cropHeight,
    );
    const base64 = outCanvas.toDataURL("image/png");
    return Png.fromBase64(base64);
  }
}

export { Png };
