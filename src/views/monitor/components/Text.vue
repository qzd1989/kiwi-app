<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, reactive } from "vue";
import { drawRect, drawText, drawBase64PngImageOnCanvas } from "@utils/common";
import { Base64Png, Point } from "@types";
import { msgError, msgSuccess } from "@utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { FormInstance, FormRules } from "element-plus";
import { codeModel, frameModel } from "@kiwi";

interface Form {
  base64Png: Base64Png | null;
  findArea: {
    start: Point;
    end: Point;
  };
}

const props = defineProps(["params", "target"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const result = ref<string | null>(null);
const code = ref<string | null>(null);
const loading = ref(false);
const formRef = ref<FormInstance>();
const canvasRef = ref<HTMLCanvasElement | null>(null);
const form = reactive<Form>({
  base64Png: null,
  findArea: {
    start: Point.from(0, 0),
    end: Point.from(0, 0),
  },
});
const rules = reactive<FormRules<Form>>({});

const close = () => {
  emits("close");
};

const drawImage = async () => {
  if (!canvasRef.value) return;
  try {
    drawBase64PngImageOnCanvas(
      canvasRef.value,
      form.base64Png as Base64Png,
      Point.from(0, 0),
      props.params.size
    );
  } catch (e: unknown) {
    msgError(e);
  }
};

const generateCode = async () => {
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  code.value = await codeModel.generateRecognizeTextCode(startPoint, endPoint);
};

const recognize = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
  } catch (e: unknown) {
    return;
  }
  if (loading.value) return;

  result.value = code.value = null;

  const origin = props.target.originalBase64Png;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  try {
    loading.value = true;
    const text = await frameModel.recognizeText(origin, startPoint, endPoint);
    drawItem(text);
    result.value = text;
    await generateCode();
  } catch (e: unknown) {
    clearAllItems();
    result.value = code.value = null;
    msgError(e);
  } finally {
    loading.value = false;
  }
};

const drawItem = (text: string) => {
  emits("drawItems", {
    callback: (ctx: CanvasRenderingContext2D) => {
      const areaPoint = form.findArea.start;
      const areaSize = {
        width: form.findArea.end.x - form.findArea.start.x,
        height: form.findArea.end.y - form.findArea.start.y,
      };
      const textPoint = Point.from(areaPoint.x, areaPoint.y - 10);
      if (text != null) {
        drawText(ctx, text, textPoint);
      }
      drawRect(ctx, areaPoint, areaSize);
    },
  });
};

const clearAllItems = () => {
  emits("clearAllItems");
};

const copy = async () => {
  if (!code.value) return;
  try {
    await writeText(code.value);
    msgSuccess("copy successed");
  } catch (e: any) {
    msgError(`copy failed: ${e.message}`);
  }
};

const loadData = () => {
  form.findArea.start = props.params.start.clone();
  form.findArea.end = Point.from(
    props.params.start.x + props.params.size.width,
    props.params.start.y + props.params.size.height
  );
  form.base64Png = props.params.base64Png;
  result.value = code.value = null;
  setTimeout(drawImage, 100);
};

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
    <el-header>Recognize Text</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="canvas-area">
              <canvas
                ref="canvasRef"
                :width="props.params.size.width"
                :height="props.params.size.height"
              ></canvas>
            </div>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button
                type="primary"
                @click="recognize(formRef)"
                :disabled="loading"
              >
                Recognize
              </el-button>
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
                      disabled
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
                      disabled
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
                      disabled
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
                      disabled
                      ><template #prefix>
                        <span>end y</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
              </el-row>
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
              <el-button type="primary" @click="copy"> copy </el-button>
            </div>
            <div>
              <el-input
                v-model="code"
                style="width: 100%"
                :rows="4"
                type="textarea"
                placeholder="code"
                readonly
              />
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
    .pixels-box {
      max-width: 300px;
      max-height: 300px;
      overflow: scroll;
    }
    .pixels {
      border: 1px solid #000;
      display: flex;
      flex-wrap: wrap;
    }
    .pixel {
      width: 5px;
      height: 5px;
      border: 1px solid #000;
    }
    .pixel:hover,
    .selected {
      border-color: white;
    }
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
.actions {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
  margin-bottom: 10px;
}
</style>
