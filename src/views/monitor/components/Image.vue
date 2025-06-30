<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive, computed, watch } from "vue";
import { msgError, msgSuccess, msgWarn } from "@utils/msg";
import { useStateStore } from "@utils/store";
import { FormInstance, FormRules } from "element-plus";
import {
  drawBase64PngImageOnCanvas,
  drawRect,
  drawText,
  cropBase64Png,
} from "@utils/common";
import { Base64Png, f64, Point, WeightPoint, Size } from "@types";
import { sep } from "@tauri-apps/api/path";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

interface Form {
  name: string | null;
  threshold: f64;
  base64Png: Base64Png | null;
  findArea: {
    start: Point;
    end: Point;
  };
}
interface Item {
  start: Point;
  size: Size;
  title: string | null;
}
namespace Item {
  export const from = (
    start: Point,
    size: Size,
    title: string | null
  ): Item => {
    return {
      start,
      size,
      title,
    };
  };
}

const props = defineProps(["params", "target", "imageDataPath"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const stateStore = useStateStore();
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  name: null,
  threshold: f64.from(0.99),
  base64Png: null,
  findArea: {
    start: Point.from(0, 0),
    end: Point.from(0, 0),
  },
});
// 动态计算并限制用户能输入的值
const findArea = reactive({
  start: {
    x: { min: 0, max: 0 },
    y: { min: 0, max: 0 },
  },
  end: {
    x: { min: 0, max: 0 },
    y: { min: 0, max: 0 },
  },
});
const rules = reactive<FormRules<Form>>({
  name: [
    {
      required: true,
      trigger: "blur",
    },
    {
      pattern: /^[^\u3000\s\\]+$/,
      message: "Spaces and backslashes are not allowed.",
      trigger: "blur",
    },
  ],
  threshold: [
    {
      required: true,
      trigger: "blur",
    },
  ],
});
const originalBase64Png = ref("");
const dataExtSideLength = 100; //额外扩展的画布长度,让图像居中方便擦除
const bgLight = "/src/assets/canvas-bg-light.png";
const bgDark = "/src/assets/canvas-bg-dark.png";
const bgUrl = ref(bgLight);
const showmagnifyingGlass = ref(false);
const point = reactive({ x: 0, y: 0 });
const magnifyingGlassSideLength = 50; //放大镜的实际边长
const isErasing = ref(false);
const eraserSmall = 5;
const eraserMedium = 10;
const eraserLarge = 20;
const eraserSideLength = ref(eraserLarge);
const relativePosition = reactive({ x: 0, y: 0 }); //相对于截图的位置
const result = ref<string | null>(null);
const code = ref<string | null>(null);
const findType = ref("findImage");
const loading = ref(false);
const hiddenCanvasRef = ref<HTMLCanvasElement | null>(null);
const magnifyingGlassCanvasRef = ref<HTMLCanvasElement | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const filePath = computed(() => {
  if (form.name == null) {
    return "";
  }
  const relativeFilePath = formatImageName(form.name);
  return relativeFilePath + ".png";
});
const fullFilePath = computed(() => {
  if (!form.name?.trim()) {
    return "";
  }
  return (
    stateStore.project.path +
    sep() +
    props.imageDataPath +
    sep() +
    filePath.value
  );
});

const formatImageName = (name: string | null): string => {
  if (name == null) {
    return "";
  }
  return name.replace(/[\u3000 ]/g, "").replace(/[/\\]+/g, sep());
};

const eraseFromOuter = () => {
  if (showmagnifyingGlass.value == true) {
    erase();
  }
};

const erase = () => {
  if (!canvasRef.value) return;
  const ctx = canvasRef.value.getContext("2d");
  if (!ctx) return;
  var imageData = ctx.getImageData(
    point.x - eraserSideLength.value / 2,
    point.y - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
  for (var i = 0; i < imageData.data.length; i += 4) {
    imageData.data[i + 3] = 0;
  }
  ctx.putImageData(
    imageData,
    point.x - eraserSideLength.value / 2,
    point.y - eraserSideLength.value / 2
  );
  drawMagnifyingGlass();
};

const drawMagnifyingGlass = () => {
  const magnifyingGlassCanvas = magnifyingGlassCanvasRef.value;
  if (magnifyingGlassCanvas == null) {
    return;
  }
  if (!canvasRef.value) return;
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const imageData = ctx.getImageData(
    point.x - magnifyingGlassSideLength / 2,
    point.y - magnifyingGlassSideLength / 2,
    magnifyingGlassSideLength,
    magnifyingGlassSideLength
  );
  magnifyingGlassCanvas.width = magnifyingGlassSideLength;
  magnifyingGlassCanvas.height = magnifyingGlassSideLength;
  const magnifyingGlassCtx = magnifyingGlassCanvas.getContext("2d");
  if (!magnifyingGlassCtx) return;
  magnifyingGlassCtx.putImageData(imageData, 0, 0);
  //draw erase rect
  magnifyingGlassCtx.fillStyle = "rgba(255, 0, 0, 0.8)";
  magnifyingGlassCtx.fillRect(
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    magnifyingGlassSideLength / 2 - eraserSideLength.value / 2,
    eraserSideLength.value,
    eraserSideLength.value
  );
};

const close = () => {
  emits("close");
};

const onImageMouseMove = (event: MouseEvent) => {
  showmagnifyingGlass.value = true;
  const canvas = canvasRef.value;
  if (!canvas) return;
  const rect = canvas.getBoundingClientRect();
  const left = Math.round(rect.left);
  const top = Math.round(rect.top);
  point.x = event.clientX - left;
  point.y = event.clientY - top;
  relativePosition.x = event.clientX - left - dataExtSideLength / 2;
  relativePosition.y = event.clientY - top - dataExtSideLength / 2;
  drawMagnifyingGlass();
  if (isErasing.value == true) {
    erase();
  }
};

const onImageMouseOut = () => {
  showmagnifyingGlass.value = false;
};

const onImageMouseDown = () => {
  isErasing.value = true;
  erase();
};

const onDataMouseUp = () => {
  isErasing.value = false;
};

const setEraserSize = (length: number) => {
  eraserSideLength.value = length;
};

const resetImage = () => {
  form.base64Png = originalBase64Png.value;
  drawImage();
};

const drawImage = () => {
  if (canvasRef.value == null) {
    return;
  }
  const start = Point.from(dataExtSideLength / 2, dataExtSideLength / 2);
  drawBase64PngImageOnCanvas(
    canvasRef.value,
    form.base64Png as Base64Png,
    start,
    props.params.size
  );
};

const findImage = async () => {
  if (loading.value) return;
  findType.value = "findImage";
  result.value = code.value = null;
  const origin = props.target.base64Png;
  const template = form.base64Png as Base64Png;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = form.threshold;
  try {
    loading.value = true;
    const weightPoint = await stateStore.frame.findImage(
      origin,
      template,
      startPoint,
      endPoint,
      threshold
    );
    if (weightPoint == null) {
      clearAllItems();
    } else {
      result.value = JSON.stringify(weightPoint);
      drawItems(formatItems([weightPoint]));
    }
    code.value = await stateStore.code.generateFindImageCode(
      formatImageName(form.name),
      startPoint,
      endPoint,
      threshold
    );
  } catch (e: unknown) {
    clearAllItems();
    result.value = code.value = null;
    msgError(e);
  } finally {
    loading.value = false;
  }
};

const findImages = async () => {
  if (loading.value) return;
  findType.value = "findImages";
  result.value = code.value = null;
  const origin = props.target.base64Png;
  const template = form.base64Png as Base64Png;
  const templateSize = props.params.size;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const threshold = form.threshold;
  try {
    loading.value = true;
    const weightPoints = await stateStore.frame.findImages(
      origin,
      template,
      templateSize,
      startPoint,
      endPoint,
      threshold
    );
    if (weightPoints.length == 0) {
      clearAllItems();
    } else {
      result.value = JSON.stringify(weightPoints);
      drawItems(formatItems(weightPoints));
    }
    code.value = await stateStore.code.generateFindImagesCode(
      formatImageName(form.name),
      startPoint,
      endPoint,
      threshold
    );
  } catch (e: unknown) {
    clearAllItems();
    result.value = code.value = null;
    msgError(e);
  } finally {
    loading.value = false;
  }
};

const drawItems = (items: Item[]) => {
  emits("drawItems", {
    callback: (ctx: CanvasRenderingContext2D) => {
      for (let item of items) {
        const point = item.start.clone();
        const size = {
          width: item.size.width,
          height: item.size.height,
        };
        const textPoint = Point.from(item.start.x, item.start.y - 5);
        drawRect(ctx, point, size);
        drawText(ctx, item.title as string, textPoint);
      }
    },
  });
};

const formatItems = (items: WeightPoint[]): Item[] => {
  const data = [];
  for (const item of items) {
    const row = Item.from(
      item.point,
      props.params.size,
      item.weight.toString()
    );
    data.push(row);
  }
  return data;
};

const clearAllItems = () => {
  emits("clearAllItems");
};

const toggleDataSuppliedBg = () => {
  bgUrl.value = bgUrl.value == bgDark ? bgLight : bgDark;
};

const saveAndCopy = async (formEl: FormInstance | undefined) => {
  if (!canvasRef.value) return;
  if (!formEl) return;
  try {
    await formEl.validate();
  } catch (e: unknown) {
    return;
  }
  if (!code.value?.trim()) {
    msgWarn("Result is null.");
    return;
  }
  const data = await cropBase64Png(
    canvasRef.value.toDataURL("image/png"),
    Point.from(dataExtSideLength / 2, dataExtSideLength / 2),
    props.params.size
  );
  try {
    await stateStore.project.saveImage(form.name as string, data);
    await writeText(code.value);
    msgSuccess("Copy successed.");
  } catch (e: unknown) {
    msgError(e);
  }
};

const copyFullFilePath = async () => {
  try {
    await writeText(fullFilePath.value);
    msgSuccess("Copy successed.");
  } catch (e: any) {
    msgError(`Copy failed: ${e.message}`);
  }
};

const loadData = () => {
  findArea.start.x.max = props.target.size.width - 1;
  findArea.start.y.max = props.target.size.height - 1;
  findArea.end.x.max = props.target.size.width;
  findArea.end.y.max = props.target.size.height;

  form.findArea.start = Point.from(0, 0);
  form.findArea.end = Point.from(
    props.target.size.width,
    props.target.size.height
  );
  form.base64Png = originalBase64Png.value = props.params.base64Png;
  result.value = code.value = null;
  setTimeout(drawImage, 100);
};

watch(
  () => {
    return form.findArea.start.x;
  },
  (newVal) => {
    if (newVal) {
      findArea.end.x.min = Math.max(form.findArea.start.x, 0) + 1;
      findArea.end.x.max = props.target.size.width;
      if (form.findArea.end.x <= form.findArea.start.x) {
        form.findArea.end.x = form.findArea.start.x + 1;
      }
    }
  }
);

watch(
  () => {
    return form.findArea.start.y;
  },
  (newVal) => {
    if (newVal) {
      findArea.end.y.min = Math.max(form.findArea.start.y, 0) + 1;
      findArea.end.y.max = props.target.size.height;
      if (form.findArea.end.y <= form.findArea.start.y) {
        form.findArea.end.y = form.findArea.start.y + 1;
      }
    }
  }
);

watch(props.params, async () => {
  loadData();
});

onMounted(async () => {
  loadData();
});
onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-header>Find Image</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work" @click="eraseFromOuter">
            <div class="canvas-area">
              <div
                class="canvas-bg"
                :style="{
                  width: props.params.size.width + dataExtSideLength + 'px',
                  height: props.params.size.height + dataExtSideLength + 'px',
                  'background-image': `url(${bgUrl})`,
                }"
              >
                <canvas
                  ref="canvasRef"
                  :width="props.params.size.width + dataExtSideLength"
                  :height="props.params.size.height + dataExtSideLength"
                  @mousemove="onImageMouseMove"
                  @mouseout="onImageMouseOut"
                  @mousedown="onImageMouseDown"
                  @mouseup="onDataMouseUp"
                ></canvas>
                <canvas
                  style="display: none"
                  ref="hiddenCanvasRef"
                  :width="props.params.size.width"
                  :height="props.params.size.height"
                ></canvas>
              </div>
              <div
                v-if="showmagnifyingGlass"
                class="magnifying-glass"
                :style="{
                  left: point.x + magnifyingGlassSideLength + 'px',
                  top: point.y - magnifyingGlassSideLength + 'px',
                  'background-image': `url(${bgUrl})`,
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
          <div style="display: flex; justify-content: center; margin-top: 10px">
            <el-button type="info" disabled>
              {{ relativePosition.x }} × {{ relativePosition.y }}
            </el-button>
          </div>
          <div class="actions">
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
            <el-button type="danger" plain @click="resetImage">
              <el-icon><RefreshLeft /></el-icon>
            </el-button>
            <el-button
              type="primary"
              :plain="bgUrl == bgLight ? true : false"
              @click="toggleDataSuppliedBg"
              style="min-width: 80px"
            >
              {{ bgUrl == bgDark ? "Light" : "Dark" }}
            </el-button>
          </div>
          <div class="item">
            <el-form-item prop="name" style="margin-bottom: 0px">
              <el-input
                v-model="form.name"
                type="text"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
              >
                <template #prepend>image name</template>
                <template #append>.png</template>
              </el-input>
            </el-form-item>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <div>
                <el-button
                  type="primary"
                  @click="findImage"
                  :disabled="loading"
                >
                  findOne
                </el-button>
                <el-button
                  type="primary"
                  @click="findImages"
                  :disabled="loading"
                >
                  findMultiple
                </el-button>
              </div>
            </div>
            <div style="margin-bottom: -10px">
              <el-row :gutter="10">
                <el-col :span="12">
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.start.x"
                  >
                    <el-input-number
                      v-model="form.findArea.start.x"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :min="findArea.start.x.min"
                      :max="findArea.start.x.max"
                      ><template #prefix>
                        <span>start x</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.start.y"
                  >
                    <el-input-number
                      v-model="form.findArea.start.y"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :min="findArea.start.y.min"
                      :max="findArea.start.y.max"
                      ><template #prefix>
                        <span>start y</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
              </el-row>
              <el-row :gutter="10">
                <el-col :span="12">
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.end.x"
                  >
                    <el-input-number
                      v-model="form.findArea.end.x"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :min="findArea.end.x.min"
                      :max="findArea.end.x.max"
                      ><template #prefix>
                        <span>end x</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item
                    style="margin-bottom: 10px"
                    prop="findArea.end.y"
                  >
                    <el-input-number
                      v-model="form.findArea.end.y"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :min="findArea.end.y.min"
                      :max="findArea.end.y.max"
                      ><template #prefix>
                        <span>end y</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
              </el-row>
            </div>
            <div>
              <el-form-item style="margin-bottom: 0px" prop="threshold">
                <el-tooltip
                  effect="dark"
                  content="min: 0.5, max:1"
                  placement="left-start"
                >
                  <el-input-number
                    :controls="false"
                    :min="0.5"
                    :max="1.0"
                    v-model="form.threshold"
                    :precision="8"
                    :style="{ width: '100%' }"
                  >
                    <template #prefix>
                      <span>threshold</span>
                    </template>
                  </el-input-number>
                </el-tooltip>
              </el-form-item>
            </div>
            <div>
              <el-input
                v-model="result"
                style="width: 100%"
                :rows="2"
                type="textarea"
                placeholder="result"
                readonly
                :autosize="true"
              />
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Code</span>
              <el-button type="primary" @click="saveAndCopy(formRef)">
                save and copy
              </el-button>
            </div>
            <div>
              <el-input
                v-model="code"
                style="width: 100%"
                :rows="6"
                type="textarea"
                readonly
                :disabled="!form.name?.trim()"
              />
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Full path of Image</span>
            </div>
            <div>
              <el-form-item style="margin-bottom: 0px">
                <el-input
                  v-model="fullFilePath"
                  autocapitalize="off"
                  autocorrect="off"
                  spellcheck="false"
                  readonly
                  :disabled="!form.name?.trim()"
                >
                  <template #append>
                    <el-button
                      type="primary"
                      @click="copyFullFilePath"
                      :disabled="!form.name?.trim()"
                    >
                      copy
                    </el-button>
                  </template>
                </el-input>
              </el-form-item>
            </div>
          </div>
        </div>
      </el-form>
    </el-main>
    <el-footer>
      <el-button @click="close">Close</el-button>
    </el-footer>
  </el-container>
</template>
<style scoped>
.el-container {
  height: 100vh;
  .el-header {
    padding: 10px 0px;
  }
  .el-main {
    overflow-x: hidden;
    overflow-y: auto;
    .item {
      background-color: var(--LightFill);
      margin: 10px 0px;
      border-radius: 5px;
      padding: 10px;
      display: flex;
      flex-direction: column;
      align-items: stretch;
      gap: 10px;
      .title {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }
    }
  }
  .el-footer {
    text-align: right;
    padding: 10px 0px;
  }
}
.canvas-work {
  display: flex;
  justify-content: center;
}
.canvas-area {
  position: relative;
}
.canvas-bg {
  background-repeat: repeat;
  overflow: hidden;
}
.actions {
  margin: 0px;
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
  margin-top: 15px;
  margin-bottom: -20px;
  padding: 0px 10px;
  .el-button {
    margin-bottom: 10px;
  }
}
.el-button.current {
  background-color: rgb(97, 97, 97);
}
.magnifying-glass {
  position: absolute;
  left: 0;
  top: 0;
  color: wheat;
  width: 100px;
  height: 100px;
  border: 1px solid #000;
  background-repeat: repeat;
}
</style>
