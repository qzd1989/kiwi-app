<script setup lang="ts">
import { Project } from "@kiwi/Project";
import { listen } from "@tauri-apps/api/event";
import { join } from "@tauri-apps/api/path";
import { msgError } from "@utils/msg";
import { useStateStore } from "@utils/store";
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { ElLoading, ElContainer } from "element-plus";
import { Base64Png, HexColor, Point, Size, Stack } from "@utils/common";
import { useResizeObserver } from "@vueuse/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface CaptureTarget {
  name: string;
  key: string;
  size: Size;
  base64Png: Base64Png | null;
  originalBase64Png: Base64Png | null;
}
interface Form {
  capturedTarget: CaptureTarget | null;
  findArea: {
    start: Point;
    end: Point;
  };
}

const stateStore = useStateStore();
const loading = ref<ReturnType<typeof ElLoading.service> | null>(null);
const capturedTargets = ref<CaptureTarget[]>([
  {
    name: "Primary Monitor",
    key: "primary_monitor",
    size: Size.from(0, 0),
    base64Png: null,
    originalBase64Png: null,
  },
]);
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
  capturedTarget: null,
  findArea: {
    start: Point.from(0, 0),
    end: Point.from(0, 0),
  },
});

const capture = async () => {
  const action = async () => {
    if (!form.capturedTarget) return;
    loading.value = null;
    try {
      form.capturedTarget.size = await stateStore.capture.getMonitorSize();
      loading.value = ElLoading.service({
        lock: true,
        text: "Capturing, please wait.",
        background: "rgba(0, 0, 0, 0.7)",
      });
      await stateStore.common.protectWindows(["main", "monitor"]);
      await stateStore.capture.requestFrameData();
    } catch (e: unknown) {
      msgError(e);
    }
  };
  if (!form.capturedTarget) return;
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
    form.findArea.end = {
      x: hoveredPixelPoint.x,
      y: hoveredPixelPoint.y,
    };
  }
  draw();
};
const onCanvasMouseOut = () => {
  // hoveredPixelPoint.x = -1;
  // hoveredPixelPoint.y = -1;
};
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
  form.findArea.end = {
    x: hoveredPixelPoint.x,
    y: hoveredPixelPoint.y,
  };
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
  if (!form.capturedTarget) return;
  if (!form.capturedTarget.base64Png) return;
  if (!canvasRef.value) return;

  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  const img = new window.Image();
  img.src = form.capturedTarget.base64Png;
  img.onload = () => {
    if (!ctx) return;
    if (!form.capturedTarget) return;
    if (!form.capturedTarget.size) return;
    canvas.width = form.capturedTarget.size.width;
    canvas.height = form.capturedTarget.size.height;
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
  if (!form.capturedTarget) return;
  form.capturedTarget.base64Png = form.capturedTarget.originalBase64Png;
  drawItemsCallback.value = null;
  // items.value = []; todo
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

const findImage = async () => {};
const findRelativeColors = async () => {};
const findColors = async () => {};
const recognizeText = async () => {};

useResizeObserver(containerRef, (entries) => {
  if (!rightRef.value) return;
  const entry = entries[0];
  const { width, height } = entry.contentRect;
  stateStore.monitorSize.width = width;
  stateStore.monitorSize.height = height;
  leftWidth.value = stateStore.monitorSize.width - rightRef.value.offsetWidth;
});

listen<Project>("backend:update:project", async (event) => {
  stateStore.project = event.payload;
  if (!stateStore.project.path || !stateStore.project.mainFile) return;
  stateStore.project.mainFileFullPath = await join(
    stateStore.project.path,
    stateStore.project.mainFile
  );
});

listen("msg:error", (event: any) => {
  msgError(event.payload.data);
});

listen<Base64Png>("backend:update:frame", async (event) => {
  if (!form.capturedTarget) return;
  form.capturedTarget.base64Png = form.capturedTarget.originalBase64Png =
    event.payload;
  drawItemsCallback.value = null;
  draw();
  loading.value?.close();
  stateStore.common.unprotectWindows(["main", "monitor"]);
});

onMounted(async () => {
  // init
  form.capturedTarget = capturedTargets.value[0];
  // gap
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);
  //zoom
  window.addEventListener("keyup", shortcutZoom);
  // init
  if ((await getCurrentWindow().label) == "monitor") {
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
        <el-select
          v-model="form.capturedTarget"
          placeholder="Select"
          class="monitors"
          disabled
        >
          <el-option
            v-for="target in capturedTargets"
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
            :disabled="form.capturedTarget == null"
          >
            <el-text>capture</el-text>
          </el-button>
        </div>
        <el-button
          type="primary"
          plain
          @click="reset"
          :disabled="form.capturedTarget == null"
          style="margin-right: 5px"
        >
          <el-text>reset</el-text>
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
            <el-icon title="find image" @click="findImage()">
              <Picture />
            </el-icon>
            <!-- find locating colors -->
            <el-icon title="find locating colors" @click="findRelativeColors()">
              <Orange />
            </el-icon>
            <!-- find colors-->
            <el-icon title="find color" @click="findColors()">
              <Pointer />
            </el-icon>
            <!-- recognize text -->
            <el-icon title="recognize text" @click="recognizeText()">
              <View />
            </el-icon>
            <!-- close -->
            <el-icon title="close" @click="cancelCapture">
              <CircleClose />
            </el-icon>
          </div>
          <!-- canvas -->
          <canvas
            class="canvas"
            ref="canvasRef"
            :width="form.capturedTarget?.size.width"
            :height="form.capturedTarget?.size.height"
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
        <span v-if="form.capturedTarget?.base64Png">
          monitor size: ({{ form.capturedTarget?.size.width }},
          {{ form.capturedTarget?.size.height }})
        </span>
        <span
          >position: ({{ hoveredPixelPoint.x }},
          {{ hoveredPixelPoint.y }})</span
        >
        <span>hex: {{ hoveredPixelHexColor }}</span>
        <span
          >beginAt: ({{ form.findArea.start.x }},
          {{ form.findArea.start.y }})</span
        >
        <span
          >endAt: ({{ form.findArea.end.x }}, {{ form.findArea.end.y }})</span
        >
        <span
          >captured Rect Size: ({{ capturedSize.width }},
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
        <!-- <Image
          v-if="showImage"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
          :imageDataPath="imageDataPath"
        />
        <RelativeColors
          v-if="showLocatingColors"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        />
        <Colors
          v-if="showColors"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        />
        <Text
          v-if="showTexts"
          @close="closeFind"
          @drawItems="drawItems"
          @clearAllItems="clearAllItems"
          :params="params"
          :monitor="monitor"
        /> -->
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
