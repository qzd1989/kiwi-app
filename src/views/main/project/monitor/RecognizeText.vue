<script setup lang="ts">
import { Point } from "@types";
import { copyText, drawRect, msgError, msgSuccess, useLoading } from "@utils";
import { Size } from "@types";
import { ref, onMounted, onUnmounted, reactive, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Code, Frame } from "@api";
import Item from "@views/components/Item.vue";

type Form = {
  start: Point;
  end: Point;
};

const props = defineProps(["params", "executionTime"]);
const emits = defineEmits(["close", "drawItems"]);
const { t } = useI18n();
const { loading, startLoading, endLoading } = useLoading();
const result = ref<string | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const code = ref<string | null>(null);
const resultText = computed(() => {
  return result.value;
});
const resultLength = computed((): number => {
  return result.value ? result.value.length : 0;
});
const form = reactive<Form>({
  start: new Point(0, 0),
  end: new Point(0, 0),
});

const loadData = () => {
  setTimeout(async () => {
    form.start = props.params.selection.start.clone();
    form.end = props.params.selection.end.clone();
    await draw();
  }, 100);
};

const draw = async () => {
  if (!canvasRef.value) return;
  const size = Size.fromPoints(form.start, form.end);
  (await props.params.png.crop(form.start, size)).draw(canvasRef.value);
};

const copy = async () => {
  if (!code.value) return;
  await copyText(code.value);
  msgSuccess(t("Copy succeeded."));
};

const drawItems = async () => {
  await emits("drawItems", {
    callback: (ctx: CanvasRenderingContext2D) => {
      const size = Size.fromPoints(form.start, form.end);
      drawRect(ctx, form.start, size);
    },
  });
};

const recognizeText = async () => {
  result.value = code.value = null;
  try {
    startLoading();
    result.value = await Frame.recognizeText(
      props.params.png.base64,
      form.start,
      form.end,
    );
    await updateCode();
    await drawItems();
  } catch (e) {
    msgError(e);
  } finally {
    endLoading();
  }
};

const updateCode = async () => {
  try {
    code.value = await Code.generateRecognizeTextCode(form.start, form.end);
  } catch (e) {
    msgError(e);
  }
};

watch(
  () => props.params?.png?.base64,
  async () => {
    await draw();
  },
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
  { deep: true },
);

defineExpose({
  loadData,
});

onMounted(async () => {
  loadData();
});

onUnmounted(async () => {});
</script>
<template>
  <el-container class="flex h-full flex-col">
    <el-header class="flex shrink-0 items-center">
      {{ $t("Recognize Text") }}
    </el-header>
    <el-main class="flex-1">
      <el-form>
        <!-- template workspace -->
        <div class="flex justify-center">
          <canvas
            ref="canvasRef"
            :width="props.params.selection.png.size.width"
            :height="props.params.selection.png.size.height"
          ></canvas>
        </div>
        <Item>
          <template #title>{{ $t("Find Area") }}</template>
          <template #extra>
            <el-button
              type="primary"
              @click="recognizeText"
              :disabled="loading != null"
            >
              {{ $t("Recognize") }}
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
                <el-form-item prop="end.x" class="mb-0!">
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
                <el-form-item prop="end.y" class="mb-0!">
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
          </template>
        </Item>
        <Item v-show="resultText">
          <template #title>
            {{ $t("Info") }}
          </template>
          <template #content>
            <el-form-item class="mb-0!">
              <el-input
                v-model="resultLength"
                class="w-full!"
                readonly
                disabled
                :autosize="true"
              >
                <template #prepend>{{ $t("Text Length") }}</template>
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
            ></el-input>
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
