<script setup lang="ts">
import { f64, Png, Point, Size, u32, weightPoint } from "@types";
import { ref, onMounted, onUnmounted, reactive, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import { sep } from "@tauri-apps/api/path";
import { Code, Frame, Project, Server, Api } from "@api";
import { msgWarn, useLoading, hash } from "@utils/common";
import { useAppStore } from "@store";
import {
  copyText,
  cropAlphaEdgesFromCanvas,
  drawRect,
  drawText,
  msgError,
  msgSuccess
} from "@utils";
import { listen } from "@tauri-apps/api/event";

import Item from "@views/components/Item.vue";
import bgLight from "@assets/canvas-bg-light.png";
import bgDark from "@assets/canvas-bg-dark.png";

type MatchingInfo = {
  factor: f64;
  imageSize: Size;
  templateSize: Size;
};

type Form = {
  name: string;
  key: string;
  origin: Png;
  template: Png | null;
  start: Point;
  endPoint: Point;
  threshold: f64;
  minTemplateSide: u32;
};

const { t } = useI18n();
const appStore = useAppStore();
const { loading, startLoading, endLoading } = useLoading();
const props = defineProps(["params", "executionTime"]);
const emits = defineEmits(["close", "drawItems"]);
const result = ref<weightPoint | null | weightPoint[]>(null);
const matchingInfo = ref<MatchingInfo | null>(null);
const resultText = computed(() => {
  if (result.value) {
    return JSON.stringify(JSON.parse(JSON.stringify(result.value)), null, 2);
  }
  return null;
});
const scaledPngInfo = computed(() => {
  if (!matchingInfo.value) return;
  return (
    matchingInfo.value.imageSize.width +
    " x " +
    matchingInfo.value.imageSize.height
  );
});
const scaledTemplateInfo = computed(() => {
  if (!matchingInfo.value) return;
  return (
    matchingInfo.value.templateSize.width +
    " x " +
    matchingInfo.value.templateSize.height
  );
});
const code = ref<string | null>(null);
const bgUrl = ref(bgLight);
const mouseState = ref<"down" | "up">("up");
const canvasRef = ref<HTMLCanvasElement | null>(null);
const magnifyingGlassCanvasRef = ref<HTMLCanvasElement | null>(null);
const mousePoint = reactive(new Point(0, 0));
const relativePosition = reactive({ x: 0, y: 0 }); //相对于截图的位置
const eraserSmall = 5;
const eraserMedium = 10;
const eraserLarge = 20;
const eraserSideLength = ref(eraserLarge);
const shouldShowMagnifyingGlass = ref(false);
const magnifyingGlassSideLength = 50; //放大镜的实际边长
const extSideLength = 100; //额外扩展的画布长度,让图像居中方便擦除
const form = reactive<Form>({
  name: Date.now().toString(),
  key: Date.now().toString(),
  origin: props.params.png,
  template: null,
  start: new Point(0, 0),
  endPoint: new Point(0, 0),
  threshold: 0.99,
  minTemplateSide: 30
});
const templateMd5 = ref<string | null>(null);
const minTemplateSide = ref<number>(0);

const drawMagnifyingGlass = () => {
  if (!magnifyingGlassCanvasRef.value) return;
  if (!canvasRef.value) return;
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const imageData = ctx.getImageData(
    mousePoint.x - magnifyingGlassSideLength / 2,
    mousePoint.y - magnifyingGlassSideLength / 2,
    magnifyingGlassSideLength,
    magnifyingGlassSideLength
  );
  magnifyingGlassCanvasRef.value.width = magnifyingGlassSideLength;
  magnifyingGlassCanvasRef.value.height = magnifyingGlassSideLength;
  const magnifyingGlassCtx = magnifyingGlassCanvasRef.value.getContext("2d");
  if (!magnifyingGlassCtx) return;
  magnifyingGlassCtx.putImageData(imageData, 0, 0);
  magnifyingGlassCtx.fillStyle = "rgba(255, 0, 0, 0.8)";
  magnifyingGlassCtx.fillRect(
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
};

const onCanvasMouseDown = () => {
  mouseState.value = "down";
  erase();
};

const onCanvasMouseUp = () => {
  mouseState.value = "up";
};

const onCanvasMouseMove = (event: MouseEvent) => {
  shouldShowMagnifyingGlass.value = true;
  if (!canvasRef.value) return;
  const rect = canvasRef.value.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  mousePoint.x = event.clientX - left;
  mousePoint.y = event.clientY - top;
  relativePosition.x = event.clientX - left - extSideLength / 2;
  relativePosition.y = event.clientY - top - extSideLength / 2;
  drawMagnifyingGlass();
  if (mouseState.value == "down") {
    erase();
  }
};

const onCanvasMouseOut = () => {
  shouldShowMagnifyingGlass.value = false;
  mouseState.value = "up";
};

const setEraserSize = (length: number) => {
  eraserSideLength.value = length;
};

const erase = () => {
  if (!canvasRef.value) return;
  const ctx = canvasRef.value.getContext("2d");
  if (!ctx) return;
  var imageData = ctx.getImageData(
    mousePoint.x - eraserSideLength.value / 2,
    mousePoint.y - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
  for (var i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i + 3] = 0;
  }
  ctx.putImageData(
    imageData,
    mousePoint.x - eraserSideLength.value / 2,
    mousePoint.y - eraserSideLength.value / 2
  );
  drawMagnifyingGlass();
};

const reset = () => {
  loadData();
};

const loadData = () => {
  setTimeout(() => {
    if (!canvasRef.value) return;
    const template = props.params.selection.png.clone();
    template.draw(canvasRef.value, extSideLength / 2, extSideLength / 2);
    form.template = template;
    form.start = new Point(0, 0);
    form.endPoint = new Point(
      props.params.png.size.width,
      props.params.png.size.height
    );
    if (!form.template) return;
    minTemplateSide.value = Math.min(
      form.template.size.width,
      form.template.size.height
    );
  }, 100);
};

const formatName = (name: string | null): string => {
  if (name == null) {
    return "";
  }
  return name.replace(/[\u3000 ]/g, "").replace(/[/\\]+/g, sep());
};

const getTemplateFromCanvas = (canvas: HTMLCanvasElement): Png | null => {
  let template: Png;
  try {
    template = cropAlphaEdgesFromCanvas(canvas);
    return template;
  } catch (e: any) {
    const msg = e?.message || String(e);
    switch (msg) {
      case "error.canvas_context":
        msgError(t("Failed to get canvas context."));
        break;
      case "image.transparent":
        msgError(t("The template is fully transparent."));
        break;
      case "image.loadFailed":
        msgError(t("Failed to load image."));
        break;
      default:
        msgError(msg);
    }
    return null;
  }
};

const saveAndCopy = async () => {
  try {
    if (!code.value) return;
    if (!form.template) return;
    if (templateMd5.value == (await hash(form.template.base64))) {
      // 重复保存
    } else {
      // 新
      if (await Project.templateExists(form.name)) {
        msgWarn(
          t(
            "template already exists, current template name has been changed.",
            { name: form.name }
          )
        );
        form.name = Date.now().toString();
      }
    }
    const template = form.template.base64;
    const localAddress = await Server.getLocalAddress();
    const name = form.name;
    if (localAddress != appStore.remoteServerAddress) {
      const saveResult = await Api.request("save_template", {
        name,
        template
      });
      if (saveResult.status == "error") {
        throw saveResult.message;
      }
    }
    await Project.saveTemplate(name, template);
    await copyText(code.value);
    templateMd5.value = await hash(form.template.base64);
    msgSuccess(t("Copy succeeded."));
  } catch (e) {
    msgError(e);
  }
};

const findImage = async () => {
  if (!canvasRef.value) return;
  const template = getTemplateFromCanvas(canvasRef.value);
  if (!template) {
    return;
  }
  form.template = template;
  matchingInfo.value = result.value = code.value = null;
  try {
    startLoading();
    await Frame.findImage(
      form.key,
      props.params.png.base64,
      form.template.base64,
      form.start,
      form.endPoint,
      form.threshold,
      form.minTemplateSide
    );
  } catch (e) {
    msgError(e);
  }
};

const findImages = async () => {
  if (!canvasRef.value) return;
  const template = getTemplateFromCanvas(canvasRef.value);
  if (!template) {
    return;
  }
  form.template = template;
  matchingInfo.value = result.value = code.value = null;
  try {
    startLoading();
    await Frame.findImages(
      form.key,
      props.params.png.base64,
      form.template.base64,
      form.start,
      form.endPoint,
      form.threshold,
      form.minTemplateSide
    );
  } catch (e) {
    msgError(e);
  }
};

const drawItems = async () => {
  if (!form.template) return;
  const template = form.template;

  // null
  if (!result.value) {
    await emits("drawItems", {
      callback: (_ctx: CanvasRenderingContext2D) => {}
    });
    return;
  }

  // weightPoint[]
  if (Array.isArray(result.value)) {
    const items = result.value;
    await emits("drawItems", {
      callback: (ctx: CanvasRenderingContext2D) => {
        for (let item of items) {
          const point = Point.from(item.point).clone();
          const size = template.size;
          const textPoint = new Point(point.x, point.y - 5);
          drawRect(ctx, point, size);
          drawText(ctx, item.weight.toString(), textPoint);
        }
        return;
      }
    });
  } else {
    // weightPoint
    const item = result.value;
    await emits("drawItems", {
      callback: (ctx: CanvasRenderingContext2D) => {
        const point = Point.from(item.point).clone();
        const size = template.size;
        const textPoint = new Point(point.x, point.y - 5);
        drawRect(ctx, point, size);
        drawText(ctx, item.weight.toString(), textPoint);
        return;
      }
    });
  }
};

const updateCode = async () => {
  try {
    code.value = await Code.generateFindImageCode(
      formatName(form.name),
      form.start,
      form.endPoint,
      form.threshold,
      form.minTemplateSide
    );
  } catch (e) {
    msgError(e);
  }
};

const setToFit = () => {
  form.minTemplateSide = minTemplateSide.value;
};

const toggleBg = () => {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
};

listen<MatchingInfo>("backend:update:image_matching_info", async (event) => {
  matchingInfo.value = event.payload as MatchingInfo;
});

listen<MatchingInfo>("backend:update:images_matching_info", async (event) => {
  matchingInfo.value = event.payload as MatchingInfo;
});

listen<[string, weightPoint | null]>(
  `backend:update:image_matching_result`,
  async (event) => {
    endLoading();
    let [key, weightPoint] = event.payload;
    if (key != form.key) {
      return;
    }
    if (!weightPoint) return;
    result.value = weightPoint;
    await updateCode();
    await drawItems();
  }
);

listen<[string, weightPoint[]]>(
  `backend:update:images_matching_result`,
  async (event) => {
    endLoading();
    let [key, weightPoints] = event.payload;
    if (key != form.key) {
      return;
    }
    if (weightPoints.length == 0) return;
    result.value = weightPoints;
    code.value = await Code.generateFindImagesCode(
      formatName(form.name),
      form.start,
      form.endPoint,
      form.threshold,
      form.minTemplateSide
    );
    await drawItems();
  }
);

watch(
  form,
  async (_newVal) => {
    try {
      await updateCode();
    } catch (e) {
      msgError(e);
    }
  },
  { deep: true }
);

defineExpose({
  loadData
});

onMounted(async () => {
  loadData();
});

onUnmounted(async () => {});
</script>
<template>
  <el-container class="flex h-full flex-col">
    <el-header class="flex shrink-0 items-center">
      {{ $t("Find Image") }}
    </el-header>
    <el-main class="flex-1">
      <el-form>
        <!-- template workspace -->
        <div class="flex justify-center">
          <div class="relative">
            <div
              :style="{
                width:
                  props.params.selection.png.size.width + extSideLength + 'px',
                height:
                  props.params.selection.png.size.height + extSideLength + 'px',
                'background-image': `url(${bgUrl})`
              }"
            >
              <canvas
                ref="canvasRef"
                :width="props.params.selection.png.size.width + extSideLength"
                :height="props.params.selection.png.size.height + extSideLength"
                @mousemove="onCanvasMouseMove"
                @mouseout="onCanvasMouseOut"
                @mousedown="onCanvasMouseDown"
                @mouseup="onCanvasMouseUp"
              ></canvas>
            </div>
            <div
              class="magnifying-glass absolute"
              v-if="shouldShowMagnifyingGlass"
              :style="{
                left: mousePoint.x + magnifyingGlassSideLength + 'px',
                top: mousePoint.y - magnifyingGlassSideLength + 'px',
                'background-image': `url(${bgUrl})`
              }"
            >
              <canvas
                ref="magnifyingGlassCanvasRef"
                :width="eraserSideLength"
                :height="eraserSideLength"
                style="width: 100px; height: 100px"
              ></canvas>
            </div>
          </div>
        </div>
        <!-- mouse relative position -->
        <div class="my-2 flex justify-center">
          <el-button type="info" disabled>
            {{ relativePosition.x }} × {{ relativePosition.y }}
          </el-button>
        </div>
        <!-- toolbar -->
        <div class="grid grid-cols-5">
          <el-button
            type="primary"
            :plain="eraserSideLength == eraserSmall ? false : true"
            @click="setEraserSize(eraserSmall)"
          >
            {{ eraserSmall }}
          </el-button>
          <el-button
            type="primary"
            :plain="eraserSideLength == eraserMedium ? false : true"
            @click="setEraserSize(eraserMedium)"
          >
            {{ eraserMedium }}
          </el-button>
          <el-button
            type="primary"
            :plain="eraserSideLength == eraserLarge ? false : true"
            @click="setEraserSize(eraserLarge)"
          >
            {{ eraserLarge }}
          </el-button>
          <el-button type="danger" plain @click="reset">
            <el-icon><RefreshLeft /></el-icon>
          </el-button>
          <el-button
            type="primary"
            :plain="bgUrl == bgLight ? true : false"
            @click="toggleBg"
            style="min-width: 80px"
          >
            {{ bgUrl == bgDark ? t("Light Mode") : t("Dark Mode") }}
          </el-button>
        </div>
        <Item>
          <template #title>{{ $t("Template") }}</template>
          <template #content>
            <el-form-item prop="name" class="mb-0!">
              <el-tooltip
                effect="dark"
                :content="
                  $t!(
                    'The template name can be a string or a relative path, such as a/b.'
                  )
                "
                placement="left"
              >
                <el-input
                  v-model="form.name"
                  type="text"
                  autocapitalize="off"
                  autocorrect="off"
                  spellcheck="false"
                >
                  <template #prepend>{{ $t("Name") }}</template>
                  <template #append>.png</template>
                </el-input>
              </el-tooltip>
            </el-form-item>
          </template>
        </Item>
        <Item>
          <template #title>{{ $t("Find Area") }}</template>
          <template #extra>
            <div class="grid grid-cols-2">
              <el-button
                type="primary"
                @click="findImage"
                :disabled="loading != null"
              >
                {{ $t("Find One") }}
              </el-button>
              <el-button
                type="primary"
                @click="findImages"
                :disabled="loading != null"
              >
                {{ $t("Find Multiple") }}
              </el-button>
            </div>
          </template>
          <template #content>
            <el-row :gutter="10">
              <el-col :span="12">
                <el-form-item prop="start.x">
                  <el-input-number
                    v-model="form.start.x"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ $t("Start X") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item prop="start.y">
                  <el-input-number
                    v-model="form.start.y"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ $t("Start Y") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="10">
              <el-col :span="12">
                <el-form-item prop="end.x">
                  <el-input-number
                    v-model="form.endPoint.x"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ $t("End X") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item prop="end.y">
                  <el-input-number
                    v-model="form.endPoint.y"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ $t("End Y") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-form-item prop="threshold">
              <el-tooltip
                effect="dark"
                :content="$t('If no match is found, try lowering this value.')"
                placement="left-start"
              >
                <el-input-number
                  :controls="false"
                  :min="0.5"
                  :max="1.0"
                  v-model="form.threshold"
                  :precision="8"
                  class="w-full!"
                >
                  <template #prefix>
                    <span>{{ $t("Threshold") }}</span>
                  </template>
                </el-input-number>
              </el-tooltip>
            </el-form-item>
            <el-form-item prop="minTemplateLength" class="mb-0!">
              <el-input-number
                :controls="false"
                :min="1"
                v-model="form.minTemplateSide"
                class="w-full!"
              >
                <template #prefix>
                  <el-tooltip
                    effect="dark"
                    :content="
                      $t(
                        'If the template’s shortest side is larger than this value, it will be scaled down to this value, and the target image will be scaled by the same factor before matching. You can adjust it to improve matching performance if you understand its effect.'
                      )
                    "
                    placement="bottom"
                  >
                    <span>{{ $t("Min Template Side") }}</span>
                  </el-tooltip>
                </template>
                <template #suffix>
                  <el-tooltip
                    effect="dark"
                    :content="$t('set to fit')"
                    placement="bottom"
                  >
                    <el-icon
                      class="cursor-pointer"
                      @click="setToFit"
                      :color="
                        minTemplateSide == form.minTemplateSide
                          ? '#67C23A'
                          : '#909399'
                      "
                    >
                      <Select />
                    </el-icon>
                  </el-tooltip>
                </template>
              </el-input-number>
            </el-form-item>
          </template>
        </Item>
        <Item v-show="matchingInfo">
          <template #title>
            {{ $t("Info") }}
          </template>
          <template #content>
            <el-form-item>
              <el-input
                v-if="matchingInfo"
                v-model="matchingInfo.factor"
                class="w-full!"
                readonly
                disabled
                :autosize="true"
              >
                <template #prepend>{{ $t("Scale Factor") }}</template>
              </el-input>
            </el-form-item>
            <el-form-item>
              <el-input
                v-if="scaledPngInfo"
                v-model="scaledPngInfo"
                class="w-full!"
                readonly
                disabled
                :autosize="true"
              >
                <template #prepend>{{ $t("Scaled Image Size") }}</template>
              </el-input>
            </el-form-item>
            <el-form-item>
              <el-input
                v-if="scaledTemplateInfo"
                v-model="scaledTemplateInfo"
                class="w-full!"
                readonly
                disabled
                :autosize="true"
              >
                <template #prepend>{{ $t("Scaled Template Size") }}</template>
              </el-input>
            </el-form-item>
            <el-form-item class="mb-0!">
              <el-input
                v-if="props.executionTime"
                v-model="props.executionTime"
                class="w-full!"
                readonly
                disabled
                :autosize="true"
              >
                <template #prepend>{{ $t("Execution Time") }}</template>
                <template #append>{{ $t("Milliseconds") }}</template>
              </el-input>
            </el-form-item>
          </template>
        </Item>
        <Item v-show="!resultText">
          <template #content>
            <div class="flex items-center justify-center text-gray-400">
              {{ $t("No results...") }}
            </div>
          </template>
        </Item>
        <Item v-show="resultText">
          <template #title>{{ $t("Result") }}</template>
          <template #content>
            <el-input
              v-model="resultText"
              class="w-full!"
              :rows="2"
              type="textarea"
              :placeholder="$t('Result...')"
              readonly
              :autosize="true"
            />
          </template>
        </Item>
        <Item v-show="resultText">
          <template #title>{{ $t("Code") }}</template>
          <template #extra>
            <el-button type="primary" @click="saveAndCopy()">
              {{ $t("Save and Copy") }}
            </el-button>
          </template>
          <template #content>
            <el-input
              v-model="code"
              :rows="6"
              type="textarea"
              disabled
              :placeholder="t('Code...')"
            />
          </template>
        </Item>
      </el-form>
    </el-main>
    <el-footer class="flex shrink-0 items-center justify-end">
      <el-button @click="emits('close')">{{ $t("Close") }}</el-button>
    </el-footer>
  </el-container>
</template>
<style scoped></style>
