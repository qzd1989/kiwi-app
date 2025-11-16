<script setup lang="ts">
import {
  ColoredPoint,
  Point,
  rgbColor,
  RelativeColoredPoint,
  RelativeColoredPoints,
  Png
} from "@types";
import { copyText, msgError, msgSuccess, msgWarn } from "@utils";
import { ref, onMounted, onUnmounted, reactive, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import Item from "@views/components/Item.vue";
import { Code, Frame } from "@api";
import { drawArc, drawText, useLoading } from "@utils/common";

type Form = {
  origin: Png;
  points: RelativeColoredPoints;
  offset: rgbColor;
  start: Point;
  end: Point;
};

const { t } = useI18n();
const { loading, startLoading, endLoading } = useLoading();
const props = defineProps(["params", "executionTime"]);
const emits = defineEmits(["close", "drawItems"]);
const pixelSideLength = ref(10);
const pixels = ref<ColoredPoint[] | null>(null);
const result = ref<ColoredPoint | null>(null);
const resultText = computed(() => {
  if (result.value) {
    return JSON.stringify(JSON.parse(JSON.stringify(result.value)), null, 2);
  }
  return null;
});
const code = ref<string | null>(null);
const form = reactive<Form>({
  origin: props.params.png,
  points: new RelativeColoredPoints(),
  offset: { r: 10, g: 10, b: 10 },
  start: new Point(0, 0),
  end: new Point(0, 0)
});

const pushPoint = (coloredPoint: ColoredPoint) => {
  let point: RelativeColoredPoint = new RelativeColoredPoint(
    coloredPoint,
    new Point(-1, -1)
  );
  if (
    form.points
      .map((item) => {
        return item.key();
      })
      .includes(point.key())
  ) {
    msgError(t("The color is already exists."));
    return;
  }
  form.points.push(point);
  form.points.caculateRelativePoint();
};

const removePoint = (point: RelativeColoredPoint) => {
  form.points = new RelativeColoredPoints(
    ...form.points.filter((item) => item.key() !== point.key())
  );
  form.points.caculateRelativePoint();
};

const isVertex = (coloredPoint: ColoredPoint): boolean => {
  for (const point of form.points) {
    if (point.key() == coloredPoint.key() && point.isVertex()) {
      return true;
    }
  }
  return false;
};

const loadData = () => {
  setTimeout(async () => {
    const template = props.params.selection.png.clone();
    pixels.value = await template.toPixels();
    form.start = new Point(0, 0);
    form.end = new Point(
      props.params.png.size.width,
      props.params.png.size.height
    );
  }, 100);
};

const unAdd = () => {
  form.points.pop();
  form.points.caculateRelativePoint();
};

const drawItems = async () => {
  // null
  if (!result.value) {
    await emits("drawItems", {
      callback: (_ctx: CanvasRenderingContext2D) => {}
    });
    return;
  }

  // ColoredPoint
  const item = result.value;
  await emits("drawItems", {
    callback: (ctx: CanvasRenderingContext2D) => {
      const title = `(${item.point.x}, ${item.point.y})`;
      const fullTitle = t("Vertex") + title;
      const titlePoint = new Point(item.point.x - 5, item.point.y - 10);
      drawArc(ctx, item.point, 5);
      drawText(ctx, fullTitle, titlePoint);
    }
  });
};

const findRelativeColor = async () => {
  if (form.points.length == 0) {
    msgWarn(t("The points must not be empty."));
    return;
  }

  if (!form.points.vertexHex()) {
    return;
  }

  result.value = code.value = null;

  try {
    startLoading();
    result.value = await Frame.findRelativeColors(
      props.params.png.base64,
      form.points,
      form.start,
      form.end,
      form.offset
    );
    await updateCode();
    await drawItems();
  } catch (e) {
    msgError(e);
  } finally {
    endLoading();
  }
};

const copy = async () => {
  if (!code.value) return;
  await copyText(code.value);
  msgSuccess(t("Copy succeeded."));
};

const updateCode = async () => {
  try {
    code.value = await Code.generateFindRelativeColorsCode(
      form.points,
      form.start,
      form.end,
      form.offset
    );
  } catch (e) {
    msgError(e);
  }
};

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
      {{ $t("Find Relative Color") }}
    </el-header>
    <el-main class="flex-1">
      <el-form>
        <!-- template workspace -->
        <div class="flex justify-center">
          <div
            class="flex justify-center overflow-scroll"
            style="width: 400px; height: 400px"
          >
            <div
              class="pixels grid"
              :style="{
                width:
                  props.params.selection.png.size.width * pixelSideLength +
                  'px',
                height:
                  props.params.selection.png.size.height * pixelSideLength +
                  'px',
                transformOrigin: 'top left',
                gridTemplateColumns: `repeat(${props.params.selection.png.size.width}, ${pixelSideLength}px)`
              }"
            >
              <div
                class="pixel"
                :class="{
                  'box-border border-2 shadow-[inset_0_0_5px_white]':
                    form.points
                      .map((item) => {
                        return item.key();
                      })
                      .includes(item.key()),
                  'border-red-600': isVertex(item)
                }"
                v-for="item in pixels"
                :style="{
                  'background-color': item.hex,
                  width: pixelSideLength + 'px',
                  height: pixelSideLength + 'px'
                }"
                @click="pushPoint(item)"
              ></div>
            </div>
          </div>
        </div>
        <!-- pixelSideLength -->
        <div class="mt-2 flex items-center justify-center gap-5">
          <el-slider
            v-model="pixelSideLength"
            :min="5"
            :max="20"
            show-tooltip
          />
          <el-button
            size="small"
            type="danger"
            @click="unAdd"
            v-show="form.points.length > 0"
          >
            <el-icon><Back /></el-icon>
          </el-button>
        </div>
        <Item v-show="form.points.length > 0">
          <template #title>{{ $t("Colors") }}</template>
          <template #content>
            <el-input
              v-show="form.points.length > 0"
              class="mb-2"
              v-for="item in form.points"
              :value="
                item.coloredPoint.hex +
                ' ' +
                '(' +
                item.coloredPoint.point.x +
                ',' +
                item.coloredPoint.point.y +
                ')' +
                '(' +
                item.relativePoint.x +
                ',' +
                item.relativePoint.y +
                ')'
              "
              disabled
            >
              <template #prepend>
                <div
                  class="h-3 w-3 rounded-full outline"
                  :style="{ backgroundColor: item.coloredPoint.hex }"
                ></div>
              </template>
              <template #append>
                <el-button @click="removePoint(item)">×</el-button>
              </template>
            </el-input>
            <el-button type="primary" plain @click="" class="mt-2 w-full">
              {{ t("Copy All") }}
            </el-button>
          </template>
        </Item>
        <Item>
          <template #title>{{ $t("Find Area") }}</template>
          <template #extra>
            <el-button
              type="primary"
              @click="findRelativeColor"
              :disabled="loading != null"
            >
              {{ $t("Find") }}
            </el-button>
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
                      <span>{{ t("Start X") }}</span>
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
                      <span>{{ t("Start Y") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="10">
              <el-col :span="12">
                <el-form-item prop="end.x">
                  <el-input-number
                    v-model="form.end.x"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ t("End X") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item prop="end.y">
                  <el-input-number
                    v-model="form.end.y"
                    :controls="false"
                    class="w-full!"
                  >
                    <template #prefix>
                      <span>{{ t("End Y") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="10">
              <el-col :span="8">
                <el-form-item prop="offset.r" class="mb-0!">
                  <el-input-number
                    v-model="form.offset.r"
                    :controls="false"
                    class="w-full!"
                    :max="50"
                    :min="0"
                  >
                    <template #prefix>
                      <span>{{ t("Offset R") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item prop="offset.g" class="mb-0!">
                  <el-input-number
                    v-model="form.offset.g"
                    :controls="false"
                    class="w-full!"
                    :max="50"
                    :min="0"
                  >
                    <template #prefix>
                      <span>{{ t("Offset G") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item prop="offset.b" class="mb-0!">
                  <el-input-number
                    v-model="form.offset.b"
                    :controls="false"
                    class="w-full!"
                    :max="50"
                    :min="0"
                  >
                    <template #prefix>
                      <span>{{ t("Offset B") }}</span>
                    </template>
                  </el-input-number>
                </el-form-item>
              </el-col>
            </el-row>
          </template>
        </Item>
        <Item v-show="resultText">
          <template #title>
            {{ $t("Info") }}
          </template>
          <template #content>
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
        <Item v-show="!resultText">
          <template #content>
            <div class="flex items-center justify-center text-gray-400">
              {{ $t("No results...") }}
            </div>
          </template>
        </Item>
        <Item v-show="resultText">
          <template #title>
            {{ $t("Code") }}
          </template>
          <template #extra>
            <el-button type="success" @click="copy">
              {{ $t("Copy") }}
            </el-button>
          </template>
          <template #content>
            <el-form-item label="" prop="" class="mb-0!">
              <el-input
                v-model="code"
                style="width: 100%"
                :rows="9"
                type="textarea"
                :placeholder="t!('Code...')"
                readonly
              />
            </el-form-item>
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
