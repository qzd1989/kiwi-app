import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { Png, Point, Size } from "@types";
import { ElLoading, ElMessage } from "element-plus";
import { ref } from "vue";
import { useI18n } from "vue-i18n";

const copyText = async (text: string) => {
  await writeText(text);
};

const delay = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const drawText = (
  ctx: CanvasRenderingContext2D,
  text: string,
  point: Point,
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
  radius: number,
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

const msgInfo = (msg: string, duration?: number) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "info",
    message: `${msg}`,
    duration,
  });
};

const msgWarn = (msg: string, duration?: number) => {
  ElMessage({
    showClose: true,
    grouping: true,
    type: "warning",
    message: `${msg}`,
    duration,
  });
};

const msgSuccess = (msg: string, duration?: number) => {
  ElMessage({
    showClose: true,
    message: `${msg}`,
    type: "success",
    grouping: true,
    duration,
  });
};

const msgError = (e: unknown, duration?: number) => {
  let message: string;

  if (e instanceof Error) {
    message = e.message;
  } else if (typeof e === "object" && e !== null) {
    if ("message" in e) {
      message = e.message as string;
    } else {
      try {
        message = JSON.stringify(e, null, 2);
      } catch {
        message = "Can't parse error object.";
      }
    }
  } else {
    message = String(e);
  }

  ElMessage({
    showClose: true,
    message: `${message}`,
    type: "error",
    grouping: true,
    duration,
  });
};

const cropAlphaEdgesFromCanvas = (canvas: HTMLCanvasElement): Png => {
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
  return Png.fromBase64(outCanvas.toDataURL("image/png"));
};

const useLoading = () => {
  const { t } = useI18n();
  const loading = ref<ReturnType<typeof ElLoading.service> | null>(null);
  const startLoading = (text?: string) => {
    loading.value = ElLoading.service({
      lock: true,
      text: text ?? t("Please wait."),
      background: "rgba(0, 0, 0, 0.7)",
    });
  };
  const endLoading = () => {
    if (loading.value) {
      loading.value.close();
    }
    loading.value = null;
  };
  return { loading, startLoading, endLoading };
};

export {
  delay,
  msgError,
  msgSuccess,
  msgWarn,
  msgInfo,
  copyText,
  drawArc,
  drawRect,
  drawText,
  cropAlphaEdgesFromCanvas,
  useLoading,
};
