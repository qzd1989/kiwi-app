import { invoke } from "@tauri-apps/api/core";
import { getAllWindows } from "@tauri-apps/api/window";
import { Base64Png, Point, RgbColor, Size } from "@types";

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

export {
  cropBase64Png,
  drawBase64PngImageOnCanvas,
  base64PngToRgbPixels,
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
