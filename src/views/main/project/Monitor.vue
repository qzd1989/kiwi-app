<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive, computed } from "vue";
import { useAppStore } from "@store";
import { Api, Code, Common } from "@api";
import bgUrl from "@assets/canvas-bg-light.png";
import { Png, Point, Size } from "@types";
import { copyText, msgError } from "@utils";
import FindImage from "./monitor/FindImage.vue";
import FindColor from "./monitor/FindColor.vue";
import FindRelativeColor from "./monitor/FindRelativeColor.vue";
import RecognizeText from "./monitor/RecognizeText.vue";
import { listen } from "@tauri-apps/api/event";
import { delay, msgSuccess } from "@utils";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { useI18n } from "vue-i18n";

type Selection = {
  start: Point;
  end: Point;
  png: Png | null;
};

type Params = {
  png: Png;
  selection: Selection;
};

const { t } = useI18n();
const appStore = useAppStore();
const findImageRef = ref<InstanceType<typeof FindImage> | null>(null);
const findRelativeColorRef = ref<InstanceType<typeof FindRelativeColor> | null>(
  null,
);
const findColorRef = ref<InstanceType<typeof FindColor> | null>(null);
const recognizeTextRef = ref<InstanceType<typeof RecognizeText> | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const png = ref<Png | null>(null);
const mouseState = ref<"down" | "up">("up");
const mousePoint = reactive(new Point(0, 0));
const selection = reactive<Selection>({
  start: new Point(0, 0),
  end: new Point(0, 0),
  png: null,
});
const params = ref<Params | null>(null);
const selectionToolbarPosition = computed<Point>(() => {
  const x = Math.min(selection.start.x, selection.end.x);
  const y = Math.max(selection.start.y, selection.end.y);
  return new Point(x, y);
});
const minimumSelectionLength = 10;
const shouldDrawSelection = ref(false);
const shouldShowSelectionToolbar = ref(false);
const isDraggingAside = ref(false);
const minimumAsideWidth = 550;
const asideWidth = ref(0);
const asideTag = ref<
  "None" | "FindImage" | "FindColor" | "FindRelativeColor" | "RecognizeText"
>("None");
const executionTime = ref(0);
const onDrawItems = ref<((ctx: CanvasRenderingContext2D) => void) | null>(null);

const loadPngFromFile = async (pngTarget: Png) => {
  png.value = pngTarget;
  await draw();
};

const exportPng = async () => {
  if (!png.value) {
    msgError("No image to export.");
    return;
  }

  try {
    // 弹出系统保存对话框，默认文件名 {Ymdhis}.png
    const defaultFileName =
      new Date().toISOString().replace(/[-:T]/g, "").split(".")[0] + ".png";

    const path = await save({
      defaultPath: defaultFileName,
      filters: [
        {
          name: "PNG Image",
          extensions: ["png"],
        },
      ],
    });

    if (!path) return; // 用户取消保存

    // 从 base64 获取二进制数据
    const base64Data = png.value.base64.replace(/^data:image\/png;base64,/, "");
    const bytes = Uint8Array.from(atob(base64Data), (c) => c.charCodeAt(0));

    // 写入文件
    await writeFile(path, bytes);

    msgSuccess("Image exported successfully!");
  } catch (e) {
    msgError(e?.toString() || "Failed to export image.");
  }
};

const capture = async () => {
  try {
    await Common.protectWindows(["main"]);
    const data = await Api.request("capture");
    if (data.status == "success") {
      png.value = Png.fromBase64(data.data);
      if (params.value) {
        params.value.png = png.value;
      }
      await draw();
    } else {
      msgError(data.message);
    }
  } catch (e) {
    msgError(e);
  } finally {
    try {
      await Common.unprotectWindows(["main"]);
    } catch (e) {
      msgError(e);
    }
  }
};

const resetPng = async () => {
  mouseState.value = "up";
  mousePoint.reset();
  shouldDrawSelection.value = false;
  shouldShowSelectionToolbar.value = false;
  selection.start.reset();
  selection.end.reset();
  selection.png = null;
  onDrawItems.value = null;
  await draw();
};

const hideSelectionToolbar = async () => {
  shouldShowSelectionToolbar.value = false;
  await draw();
};

const hideSelectionRect = async () => {
  shouldDrawSelection.value = false;
  await draw();
};

const hideSelection = async () => {
  await hideSelectionRect();
  await hideSelectionToolbar();
};

const draw = async () => {
  if (!canvasRef.value || !png.value) return null;
  const ctx = await png.value.draw(canvasRef.value, 0, 0);
  drawSelectionRect(ctx);
  if (onDrawItems.value) {
    onDrawItems.value(ctx);
  }
};

const drawSelectionRect = (ctx: CanvasRenderingContext2D) => {
  if (!shouldDrawSelection.value) return;
  const x = selection.start.x;
  const y = selection.start.y;
  const width = selection.end.x - selection.start.x;
  const height = selection.end.y - selection.start.y;
  ctx.beginPath();
  ctx.rect(x, y, width, height);
  ctx.fillStyle = "rgba(0, 0, 0, 0.5)";
  ctx.fill();
  ctx.strokeStyle = "#ffffff";
  ctx.lineWidth = 0.5;
  ctx.stroke();
};

const onCanvasMouseDown = (event: MouseEvent) => {
  if (event.button !== 0) {
    copyText(`(${mousePoint.x}, ${mousePoint.y})`);
    return;
  }
  selection.start = mousePoint.clone();
  mouseState.value = "down";
  shouldDrawSelection.value = true;
  shouldShowSelectionToolbar.value = false;
};

const copyMoveToAbsolutePositionCode = async () => {
  try {
    const result = await Code.generateMoveToAbsolutePositionCode(mousePoint);
    copyText(result);
    msgSuccess(t("Copy succeeded."));
  } catch (e) {
    msgError(e);
  }
};

const onCanvasMouseUp = async (event: MouseEvent) => {
  if (event.button !== 0) return;
  if (!png.value) return;
  mouseState.value = "up";
  selection.end = mousePoint.clone();
  if (
    Math.abs(selection.start.x - selection.end.x) < minimumSelectionLength ||
    Math.abs(selection.start.y - selection.end.y) < minimumSelectionLength
  ) {
    await copyMoveToAbsolutePositionCode();
    await hideSelectionRect();
    await hideSelectionToolbar();
    return;
  }
  shouldShowSelectionToolbar.value = true;
  selection.png = await png.value.crop(
    selection.start,
    Size.fromPoints(selection.start, selection.end),
  );
};

const onCanvasMouseMove = async (event: MouseEvent) => {
  if (!canvasRef.value) return;
  const rect = canvasRef.value.getBoundingClientRect();
  mousePoint.x = event.clientX - Math.round(rect.left);
  mousePoint.y = event.clientY - Math.round(rect.top);
  if (mouseState.value === "down") {
    selection.end = mousePoint.clone();
    await draw();
  }
};

const onCanvasMouseOut = () => {
  mousePoint.reset();
};

const openAside = () => {
  asideWidth.value = minimumAsideWidth;
};

const closeAside = () => {
  asideWidth.value = 0;
  asideTag.value = "None";
};

const findImage = async () => {
  if (!selection.png) return;
  if (!png.value) return;
  params.value = {
    png: png.value,
    selection: {
      png: selection.png.clone(),
      start: selection.start.clone(),
      end: selection.end.clone(),
    },
  };
  openAside();
  await hideSelectionRect();
  await hideSelectionToolbar();
  asideTag.value = "FindImage";
  findImageRef.value?.loadData();
};

const findRelativeColor = async () => {
  if (!selection.png) return;
  if (!png.value) return;
  params.value = {
    png: png.value,
    selection: {
      png: selection.png.clone(),
      start: selection.start.clone(),
      end: selection.end.clone(),
    },
  };
  openAside();
  await hideSelectionRect();
  await hideSelectionToolbar();
  asideTag.value = "FindRelativeColor";
  findRelativeColorRef.value?.loadData();
};

const findColor = async () => {
  if (!selection.png) return;
  if (!png.value) return;
  params.value = {
    png: png.value,
    selection: {
      png: selection.png.clone(),
      start: selection.start.clone(),
      end: selection.end.clone(),
    },
  };
  openAside();
  await hideSelectionRect();
  await hideSelectionToolbar();
  asideTag.value = "FindColor";
  findColorRef.value?.loadData();
};

const recognizeText = async () => {
  if (!selection.png) return;
  if (!png.value) return;
  params.value = {
    png: png.value,
    selection: {
      png: selection.png.clone(),
      start: selection.start.clone(),
      end: selection.end.clone(),
    },
  };
  openAside();
  await hideSelectionRect();
  await hideSelectionToolbar();
  asideTag.value = "RecognizeText";
  recognizeTextRef.value?.loadData();
};

const moveListener = (event: MouseEvent) => {
  if (isDraggingAside.value == false) return;
  asideWidth.value = Math.max(
    appStore.window.width / appStore.window.scaleFactor - event.clientX,
    minimumAsideWidth,
  );
};

const drawItems = async (data: any) => {
  onDrawItems.value = data.callback;
  await draw();
};

const upListener = () => {
  isDraggingAside.value = false;
};

listen<number>("backend:update:execution_time", async (event) => {
  executionTime.value = event.payload;
});

defineExpose({
  loadPngFromFile,
  resetPng,
  capture,
  exportPng,
});

onMounted(async () => {
  document.addEventListener("mousemove", moveListener);
  document.addEventListener("mouseup", upListener);

  await delay(200);
  await capture();

  // test code
  // setTimeout(async () => {
  //   if (!png.value) return;
  //   selection.start = new Point(68, 108);
  //   selection.end = new Point(133, 171);
  //   selection.png = await png.value.crop(
  //     selection.start,
  //     Size.fromPoints(selection.start, selection.end),
  //   );
  //   await findRelativeColor();
  // }, 200);
  // test code end
});

onUnmounted(() => {});
</script>
<template>
  <div class="flex overflow-hidden outline">
    <el-container>
      <el-container>
        <el-main
          class="h-full w-full p-0!"
          :style="{
            'background-image': `url(${bgUrl})`,
          }"
        >
          <div class="workspace relative">
            <!-- toolbar -->
            <div
              class="absolute hidden gap-2 rounded-sm bg-green-700 p-2 text-xl text-white"
              :style="{
                display: shouldShowSelectionToolbar ? 'inline-flex' : 'none',
                top: selectionToolbarPosition.y + 2 + 'px',
                left: selectionToolbarPosition.x + 'px',
              }"
            >
              <!-- find image -->
              <el-icon :title="$t('Find Image')" @click="findImage()">
                <Picture
                  class="cursor-pointer fill-current hover:text-amber-200"
                />
              </el-icon>
              <!-- find relative colors -->
              <el-icon
                :title="$t('Find Relative Colors')"
                @click="findRelativeColor()"
              >
                <Orange
                  class="cursor-pointer fill-current hover:text-amber-200"
                />
              </el-icon>
              <!-- find colors-->
              <el-icon :title="$t('Find Colors')" @click="findColor()">
                <Pointer
                  class="cursor-pointer fill-current hover:text-amber-200"
                />
              </el-icon>
              <!-- recognize text -->
              <el-icon :title="$t('Recognize Text')" @click="recognizeText()">
                <View
                  class="cursor-pointer fill-current hover:text-amber-200"
                />
              </el-icon>
              <!-- close -->
              <el-icon :title="$t('Close')" @click="hideSelection">
                <CircleClose
                  class="cursor-pointer fill-current hover:text-amber-200"
                />
              </el-icon>
            </div>
            <!-- canvas -->
            <canvas
              class="canvas"
              ref="canvasRef"
              :width="png?.size.width"
              :height="png?.size.height"
              @mousedown="onCanvasMouseDown"
              @mouseup="onCanvasMouseUp"
              @mousemove="onCanvasMouseMove"
              @mouseout="onCanvasMouseOut"
            ></canvas>
          </div>
        </el-main>
        <el-footer
          class="flex h-5! items-center gap-4 p-0! px-1! text-xs text-gray-500"
        >
          <div class="min-w-40">
            {{ $t("Monitor Size") }}: ({{ png?.size.width }},
            {{ png?.size.height }})
          </div>
          <div class="min-w-40">
            {{ $t("Mouse Position") }}: ({{ mousePoint.x }}, {{ mousePoint.y }})
          </div>
          <div class="min-w-40">
            {{ $t("Selection Start") }}: ({{ selection.start.x }},
            {{ selection.start.y }})
          </div>
          <div class="min-w-40">
            {{ $t("Selection End") }}: ({{ selection.end.x }},
            {{ selection.end.y }})
          </div>
          <div class="min-w-40">
            {{ $t("Selection Size") }}: ({{
              Size.fromPoints(selection.start, selection.end).width
            }}, {{ Size.fromPoints(selection.start, selection.end).height }})
          </div>
        </el-footer>
      </el-container>
      <el-aside
        :style="{
          width: asideWidth + 'px',
        }"
        class="flex"
        ref="asideRef"
      >
        <div
          class="h-full w-1 shrink-0 cursor-col-resize"
          @mousedown="isDraggingAside = true"
        ></div>
        <div class="flex-1">
          <FindImage
            ref="findImageRef"
            v-if="asideTag == 'FindImage'"
            :params="params"
            :executionTime="executionTime"
            @drawItems="drawItems"
            @close="closeAside"
          />
          <FindColor
            ref="findColorRef"
            v-if="asideTag == 'FindColor'"
            :params="params"
            :executionTime="executionTime"
            @drawItems="drawItems"
            @close="closeAside"
          />
          <FindRelativeColor
            ref="findRelativeColorRef"
            v-if="asideTag == 'FindRelativeColor'"
            :params="params"
            :executionTime="executionTime"
            @drawItems="drawItems"
            @close="closeAside"
          />
          <RecognizeText
            ref="recognizeTextRef"
            v-if="asideTag == 'RecognizeText'"
            :params="params"
            :executionTime="executionTime"
            @drawItems="drawItems"
            @close="closeAside"
          />
        </div>
      </el-aside>
    </el-container>
  </div>
</template>
<style scoped></style>
