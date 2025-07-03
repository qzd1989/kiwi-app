<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { join } from "@tauri-apps/api/path";
import { msgError } from "@utils/msg";
import { useStateStore } from "@utils/store";
import { ref, onMounted, onUnmounted, reactive, watch } from "vue";
import { ElLoading, ElContainer } from "element-plus";
import {
  Base64Png,
  EmitMsg,
  EmitProject,
  HexColor,
  Point,
  Size,
  Stack,
} from "@types";
import { cropBase64Png } from "@utils/common";
import { useResizeObserver } from "@vueuse/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useI18n } from "vue-i18n";
import { Locale } from "@types";

import FindImage from "@views/monitor/components/Image.vue";
import FindRelativeColor from "@views/monitor/components/RelativeColor.vue";
import FindColor from "@views/monitor/components/Color.vue";
import FindText from "@views/monitor/components/Text.vue";
import { captureModel, commonModel } from "@kiwi";

interface Target {
  name: string;
  key: string;
  size: Size;
  base64Png: Base64Png | null;
  originalBase64Png: Base64Png | null;
}
interface Params {
  start: Point;
  size: Size;
  base64Png: Base64Png | null;
}
namespace Params {
  export const init = (): Params => {
    return {
      start: Point.from(0, 0),
      size: Size.from(0, 0),
      base64Png: null,
    };
  };
}
interface Form {
  target: Target | null;
  findArea: {
    start: Point;
    end: Point;
  };
}

const { t, locale } = useI18n();
const stateStore = useStateStore();
const loading = ref<ReturnType<typeof ElLoading.service> | null>(null);
const targets = ref<Target[]>([]);
const containerRef = ref<InstanceType<typeof ElContainer> | null>(null);
const bgLight = "/src/assets/canvas-bg-light.png";
const bgUrl = ref(bgLight);
const gapLength = ref(10);
const leftWidth = ref(0);
const rightWidth = ref(0);
const shouldDrawCapture = ref(false);
const isCaptured = ref(false);
const isCapturing = ref(false);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const rightRef = ref<HTMLElement | null>(null);
const showImage = ref(false);
const showRelativeColor = ref(false);
const showColor = ref(false);
const showText = ref(false);
const drawItemsCallback = ref<((ctx: CanvasRenderingContext2D) => void) | null>(
  null
);
const hoveredPixelHexColor = ref<HexColor | null>(null);
const hoveredPixelPoint = reactive<Point>(Point.from(0, 0));
const capturedSize = reactive<Size>(Size.from(0, 0));
const draggingRight = ref(false);
const pagePoint = reactive<Point>(Point.from(0, 0));
const mouseVerticalStack = new Stack(2);
const form = reactive<Form>({
  target: null,
  findArea: {
    start: Point.from(0, 0),
    end: Point.from(0, 0),
  },
});
const params = reactive<Params>(Params.init());

const capture = async () => {
  const action = async () => {
    if (!form.target) return;
    loading.value = null;
    try {
      form.target.size = await captureModel.getMonitorSize();
      loading.value = ElLoading.service({
        lock: true,
        text: t("Capturing, please wait."),
        background: "rgba(0, 0, 0, 0.7)",
      });
      await commonModel.protectWindows(["main", "monitor"]);
      await captureModel.requestFrameData();
    } catch (e: unknown) {
      msgError(e);
    }
  };
  if (!form.target) return;
  cancelCapture();
  await action();
};

const onCanvasMouseMove = (event: MouseEvent) => {
  if (!canvasRef.value) return;
  const rect = canvasRef.value.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  hoveredPixelPoint.x = event.clientX - left;
  hoveredPixelPoint.y = event.clientY - top;
  if (isCapturing.value == true) {
    form.findArea.end = Point.clone(hoveredPixelPoint);
  }
  draw();
};

const onCanvasMouseOut = () => {};

const onCanvasMouseDown = (event: MouseEvent) => {
  if (event.button != 0) {
    return;
  }
  draw();
  form.findArea.start.x = hoveredPixelPoint.x;
  form.findArea.start.y = hoveredPixelPoint.y;
  isCaptured.value = false;
  isCapturing.value = true;
  shouldDrawCapture.value = true;
};
const onCanvasMouseUp = (event: MouseEvent) => {
  if (event.button != 0) {
    return;
  }
  form.findArea.end = Point.clone(hoveredPixelPoint);
  isCaptured.value = true;
  isCapturing.value = false;
  shouldDrawCapture.value = true;
  if (
    Math.abs(form.findArea.start.x - form.findArea.end.x) < 10 ||
    Math.abs(form.findArea.start.y - form.findArea.end.y) < 10
  ) {
    shouldDrawCapture.value = false;
    cancelCapture();
  }
  draw();
};

const cancelCapture = () => {
  shouldDrawCapture.value = isCaptured.value = isCapturing.value = false;
  form.findArea.start = form.findArea.end = Point.from(0, 0);
  draw();
};

const draw = () => {
  if (!form.target) return;
  if (!form.target.base64Png) return;
  if (!canvasRef.value) return;

  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  const img = new window.Image();
  img.src = form.target.base64Png;
  img.onload = () => {
    if (!ctx) return;
    if (!form.target) return;
    if (!form.target.size) return;
    canvas.width = form.target.size.width;
    canvas.height = form.target.size.height;
    ctx.drawImage(img, 0, 0);

    if (drawItemsCallback.value) {
      drawItemsCallback.value(ctx);
    }

    hoveredPixelHexColor.value = HexColor.from(
      getPixelHex(ctx, hoveredPixelPoint)
    );

    if (shouldDrawCapture.value) {
      drawCapturedRect();
    }
  };
};

const drawCapturedRect = () => {
  if (!canvasRef.value) return;
  const ctx = canvasRef.value.getContext("2d");
  if (!ctx) return;
  ctx.beginPath();
  const x = form.findArea.start.x;
  const y = form.findArea.start.y;
  let width = form.findArea.end.x - form.findArea.start.x;
  let height = form.findArea.end.y - form.findArea.start.y;
  if (isCapturing.value) {
    width = hoveredPixelPoint.x - form.findArea.start.x;
    height = hoveredPixelPoint.y - form.findArea.start.y;
  }
  ctx.rect(x, y, width, height);
  capturedSize.width = Math.abs(width);
  capturedSize.height = Math.abs(height);
  ctx.strokeStyle = "#489029";
  ctx.stroke();
};

const reset = () => {
  if (!form.target) return;
  form.target.base64Png = form.target.originalBase64Png;
  drawItemsCallback.value = null;
  draw();
};

const getPixelHex = (ctx: CanvasRenderingContext2D, point: Point) => {
  var imageData = ctx.getImageData(point.x, point.y, 1, 1);
  var data = imageData.data;
  var r = data[0];
  var g = data[1];
  var b = data[2];
  var hex =
    "#" +
    ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
  return hex;
};

const actionPosition = (): Point => {
  const x = Math.min(form.findArea.start.x, form.findArea.end.x);
  const y = Math.max(form.findArea.start.y, form.findArea.end.y);
  return Point.from(x, y);
};

const shortcutZoom = async (event: KeyboardEvent) => {
  if (
    (event.key === "=" && event.ctrlKey) ||
    (event.key === "=" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.min(
      stateStore.zoom.factor + 0.1,
      stateStore.zoom.max
    );
  }
  if (
    (event.key === "-" && event.ctrlKey) ||
    (event.key === "-" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = Math.max(
      stateStore.zoom.factor - 0.1,
      stateStore.zoom.min
    );
  }
  if (
    (event.key === "0" && event.ctrlKey) ||
    (event.key === "0" && event.metaKey)
  ) {
    event.preventDefault();
    stateStore.zoom.factor = 1;
  }
};

const moveListener = (event: MouseEvent) => {
  if (!containerRef.value) return;
  const containerRect = containerRef.value.$el.getBoundingClientRect();
  pagePoint.x = event.clientX - containerRect.left;
  pagePoint.y = event.clientY - containerRect.top;
  mouseVerticalStack.push(pagePoint.y);
  //to left
  if (draggingRight.value) {
    const remain = stateStore.monitorSize.width;
    rightWidth.value = Math.max(stateStore.monitorSize.width - pagePoint.x, 0);
    rightWidth.value = Math.min(rightWidth.value, remain - gapLength.value);
    leftWidth.value = remain - rightWidth.value;
  }
};

const upListener = () => {
  draggingRight.value = false;
};

const drag = (event: MouseEvent, area: string) => {
  event.preventDefault();
  switch (area) {
    case "right":
      draggingRight.value = true;
      break;
  }
};

const initParams = () => {
  const inited = Params.init();
  params.start = inited.start;
  params.base64Png = inited.base64Png;
  params.size = inited.size;
};

const findImage = async () => {
  // guard
  if (!form.target) return null;
  // init
  closeFind();
  initParams();
  rightWidth.value = 420;
  drawItemsCallback.value = null;
  // set params
  const startX = Math.min(form.findArea.start.x, form.findArea.end.x);
  const startY = Math.min(form.findArea.start.y, form.findArea.end.y);
  const width = Math.abs(form.findArea.start.x - form.findArea.end.x);
  const height = Math.abs(form.findArea.start.y - form.findArea.end.y);
  const start = Point.from(startX, startY);
  const size = Size.from(width, height);
  params.start = start;
  params.size = size;
  params.base64Png = (await cropBase64Png(
    form.target.originalBase64Png as Base64Png,
    start,
    size
  )) as Base64Png;
  cancelCapture();
  showImage.value = true;
};

const findRelativeColor = async () => {
  // guard
  if (!form.target) return null;
  // init
  closeFind();
  initParams();
  rightWidth.value = 420;
  drawItemsCallback.value = null;
  // set params
  const startX = Math.min(form.findArea.start.x, form.findArea.end.x);
  const startY = Math.min(form.findArea.start.y, form.findArea.end.y);
  const width = Math.abs(form.findArea.start.x - form.findArea.end.x);
  const height = Math.abs(form.findArea.start.y - form.findArea.end.y);
  const start = Point.from(startX, startY);
  const size = Size.from(width, height);
  params.start = start;
  params.size = size;
  params.base64Png = (await cropBase64Png(
    form.target.originalBase64Png as Base64Png,
    start,
    size
  )) as Base64Png;
  cancelCapture();
  showRelativeColor.value = true;
};

const findColor = async () => {
  // guard
  if (!form.target) return null;
  // init
  closeFind();
  initParams();
  rightWidth.value = 420;
  drawItemsCallback.value = null;
  // set params
  const startX = Math.min(form.findArea.start.x, form.findArea.end.x);
  const startY = Math.min(form.findArea.start.y, form.findArea.end.y);
  const width = Math.abs(form.findArea.start.x - form.findArea.end.x);
  const height = Math.abs(form.findArea.start.y - form.findArea.end.y);
  const start = Point.from(startX, startY);
  const size = Size.from(width, height);
  params.start = start;
  params.size = size;
  params.base64Png = (await cropBase64Png(
    form.target.originalBase64Png as Base64Png,
    start,
    size
  )) as Base64Png;
  cancelCapture();
  showColor.value = true;
};

const findText = async () => {
  // guard
  if (!form.target) return null;
  // init
  closeFind();
  initParams();
  rightWidth.value = 420;
  drawItemsCallback.value = null;
  // set params
  const startX = Math.min(form.findArea.start.x, form.findArea.end.x);
  const startY = Math.min(form.findArea.start.y, form.findArea.end.y);
  const width = Math.abs(form.findArea.start.x - form.findArea.end.x);
  const height = Math.abs(form.findArea.start.y - form.findArea.end.y);
  const start = Point.from(startX, startY);
  const size = Size.from(width, height);
  params.start = start;
  params.size = size;
  params.base64Png = (await cropBase64Png(
    form.target.originalBase64Png as Base64Png,
    start,
    size
  )) as Base64Png;
  cancelCapture();
  showText.value = true;
};

const closeFind = () => {
  showImage.value = false;
  showRelativeColor.value = false;
  showColor.value = false;
  showText.value = false;
  rightWidth.value = 0;
};

const drawItems = (data: any) => {
  drawItemsCallback.value = data.callback;
  draw();
};

const clearAllItems = () => {
  drawItemsCallback.value = null;
  draw();
};

const setLocale = (newLocale: Locale) => {
  if (!stateStore.app.config) return;
  locale.value = stateStore.app.config.app.locale = newLocale;
};

const loadTargets = async () => {
  targets.value = [
    {
      name: "Primary Monitor",
      key: "primary_monitor",
      size: Size.from(0, 0),
      base64Png: null,
      originalBase64Png: null,
    },
  ];
};

useResizeObserver(containerRef, (entries) => {
  if (!rightRef.value) return;
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  stateStore.monitorSize.width = width;
  stateStore.monitorSize.height = height;
  leftWidth.value = stateStore.monitorSize.width - rightRef.value.offsetWidth;
});

listen<EmitProject>("backend:update:project", async (event) => {
  stateStore.project = event.payload;
  if (!stateStore.project.path || !stateStore.project.mainFile) return;
  stateStore.project.mainFileFullPath = await join(
    stateStore.project.path,
    stateStore.project.mainFile
  );
});

listen<EmitMsg>("msg:error", (event) => {
  msgError(event.payload.data);
});

listen<Base64Png>("backend:update:frame", async (event) => {
  if (!form.target) return;
  form.target.base64Png = form.target.originalBase64Png = event.payload;
  drawItemsCallback.value = null;
  draw();
  loading.value?.close();
  commonModel.unprotectWindows(["main", "monitor"]);
});

listen<Locale>("backend:update:locale", async (event) => {
  setLocale(event.payload as Locale);
});

// test todo
watch(
  () => {
    return form.target?.base64Png;
  },
  async (newVal) => {
    if (newVal) {
      // form.findArea.start = Point.from(697, 1045);
      // form.findArea.end = Point.from(747, 1098);
      // findRelativeColor();
    }
  }
);

onMounted(async () => {
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);
  //zoom
  window.addEventListener("keyup", shortcutZoom);

  await loadTargets();
  if ((await getCurrentWindow().label) == "monitor") {
    form.target = targets.value[0];
    await capture();
  }
});

onUnmounted(async () => {
  window.removeEventListener("keyup", shortcutZoom);
});
</script>
<template>
  <el-container
    class="container"
    ref="containerRef"
    :style="{
      '--gap-width': gapLength + 'px',
    }"
  >
    <el-container
      ref="leftRef"
      class="left"
      :style="{
        width: leftWidth + 'px',
      }"
    >
      <el-header ref="headerRef">
        <el-select v-model="form.target" class="monitors" disabled>
          <el-option
            v-for="target in targets"
            :key="target.key"
            :label="target.name"
            :value="target.key"
          />
        </el-select>
        <div class="capture">
          <el-button
            type="primary"
            plain
            @click="capture"
            :disabled="form.target == null"
          >
            <el-text>{{ t("Capture") }}</el-text>
          </el-button>
        </div>
        <el-button
          type="primary"
          plain
          @click="reset"
          :disabled="form.target == null"
          style="margin-right: 5px"
        >
          <el-text>{{ t("Reset") }}</el-text>
        </el-button>
      </el-header>
      <el-main
        ref="mainRef"
        :style="{
          'background-image': `url(${bgUrl})`,
        }"
      >
        <!-- work -->
        <div class="work">
          <!-- actions -->
          <div
            class="actions"
            v-if="shouldDrawCapture && isCaptured"
            :style="{
              top: actionPosition().y + 2 + 'px',
              left: actionPosition().x + 'px',
            }"
          >
            <!-- find image -->
            <el-icon :title="t('Find Image')" @click="findImage()">
              <Picture />
            </el-icon>
            <!-- find locating colors -->
            <el-icon
              :title="t('Find Relative Colors')"
              @click="findRelativeColor()"
            >
              <Orange />
            </el-icon>
            <!-- find colors-->
            <el-icon :title="t('Find Colors')" @click="findColor()">
              <Pointer />
            </el-icon>
            <!-- recognize text -->
            <el-icon :title="t('Recognize Text')" @click="findText()">
              <View />
            </el-icon>
            <!-- close -->
            <el-icon :title="t('Close')" @click="cancelCapture">
              <CircleClose />
            </el-icon>
          </div>
          <!-- canvas -->
          <canvas
            class="canvas"
            ref="canvasRef"
            :width="form.target?.size.width"
            :height="form.target?.size.height"
            @mousedown="onCanvasMouseDown"
            @mouseup="onCanvasMouseUp"
            @mousemove="onCanvasMouseMove"
            @mouseout="onCanvasMouseOut"
          ></canvas>
          <!-- canvas end -->
        </div>
        <!-- work end -->
      </el-main>
      <el-footer>
        <span v-if="form.target?.base64Png">
          {{ t("Monitor Size") }}: ({{ form.target?.size.width }},
          {{ form.target?.size.height }})
        </span>
        <span>
          {{ t("Position") }}: ({{ hoveredPixelPoint.x }},
          {{ hoveredPixelPoint.y }})
        </span>
        <span>{{ t("Hex Color") }}: {{ hoveredPixelHexColor }}</span>
        <span>
          {{ t("Begin Point") }}: ({{ form.findArea.start.x }},
          {{ form.findArea.start.y }})
        </span>
        <span>
          {{ t("End Point") }}: ({{ form.findArea.end.x }},
          {{ form.findArea.end.y }})
        </span>
        <span>
          {{ t("Captured Rect Size") }}: ({{ capturedSize.width }},
          {{ capturedSize.height }})
        </span>
      </el-footer>
    </el-container>
    <el-aside
      ref="rightRef"
      class="right"
      :style="{
        width: rightWidth + 'px',
      }"
    >
      <div
        class="gap-vertical"
        @mousedown="drag($event, 'right')"
        :class="{ selected: draggingRight }"
      ></div>
      <div class="find-area">
        <FindImage
          v-if="showImage"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :target="form.target"
          :imageDataPath="stateStore.app.relativeImageDataPath"
        />
        <FindRelativeColor
          v-if="showRelativeColor"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :target="form.target"
        />
        <FindColor
          v-if="showColor"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :target="form.target"
        />
        <FindText
          v-if="showText"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :target="form.target"
        />
      </div>
    </el-aside>
  </el-container>
</template>
<style scoped>
.container {
  height: 100vh;
  .left {
    .el-header {
      padding: 0px;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .monitors {
        margin: 0px 5px;
      }
      .capture {
        margin-right: 5px;
        position: relative;
        .el-checkbox {
          height: 14px;
          position: absolute;
          right: 0;
          bottom: 0;
          z-index: 2;
        }
        .el-button {
          position: relative;
          z-index: 1;
        }
      }
      .others {
        display: flex;
      }
    }
    .el-main {
      overflow: auto;
      padding: 0px;
      .work {
        color: white;
        position: relative;
        display: inline-block;
        .actions {
          display: inline-flex;
          overflow: hidden;
          border-radius: 5px;
          background-color: var(--el-color-success-dark-2);
          color: var(--Basic-White);
          position: absolute;
        }
        .actions .el-icon {
          padding: 5px;
          cursor: pointer;
        }
        .actions .el-icon:hover {
          color: var(--el-color-warning-light-5);
        }
      }
    }
    .el-footer {
      font-size: 12px;
      overflow: hidden;
      display: flex;
      align-items: center;
      height: 20px;
      padding: 0px 10px;
      color: #666;
      span {
        margin-right: 10px;
      }
    }
  }
  .right {
    position: relative;
    .gap-vertical {
      position: absolute;
      left: 0px;
      top: 0px;
      width: var(--gap-width);
      height: 100%;
    }
    .gap-vertical:hover,
    .gap-vertical.selected {
      cursor: col-resize;
    }
    .find-area {
      position: relative;
      margin-left: var(--gap-width);
      margin-right: var(--gap-width);
    }
  }
}
</style>
