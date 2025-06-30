<script setup lang="ts">
import { ref, onMounted, watch, reactive } from "vue";
import { msgError, msgSuccess, msgWarn } from "@utils/msg";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useStateStore } from "@utils/store";
import { base64PngToRgbPixels, drawArc, drawText } from "@utils/common";
import { Base64Png, ColoredPoint, RgbColor, HexColor, Point } from "@types";
import { FormInstance, FormRules } from "element-plus";

interface RelativeColorPoint {
  key: string;
  hex: HexColor;
  point: Point;
  relativePoint: Point;
}
class RelativeColorPoint {
  key: string;
  hex: HexColor;
  point: Point;
  relativePoint: Point;
  constructor(hex: HexColor, point: Point, relativePoint: Point) {
    this.key = point.x + "," + point.y;
    this.hex = hex;
    this.point = point;
    this.relativePoint = relativePoint;
  }
  clone(): RelativeColorPoint {
    return RelativeColorPoint.from(this.hex, this.point, this.relativePoint);
  }
}
namespace RelativeColorPoint {
  export const from = (
    hex: HexColor,
    point: Point,
    relativePoint: Point
  ): RelativeColorPoint => {
    return new RelativeColorPoint(hex, point, relativePoint);
  };
}
interface Form {
  points: RelativeColorPoint[];
  offset: RgbColor;
  base64Png: Base64Png | null;
  findArea: {
    start: Point;
    end: Point;
  };
}

const props = defineProps(["params", "target"]);
const emits = defineEmits(["close", "drawItems", "clearAllItems"]);
const stateStore = useStateStore();
const result = ref<string | null>(null);
const code = ref<string | null>(null);
const points = ref<RelativeColorPoint[]>([]);
const loading = ref(false);
const pixelValue = 10;
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  points: [],
  offset: RgbColor.from(0, 0, 0),
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
  "findArea.start.x": [{ required: true, trigger: "blur" }],
  "findArea.start.y": [{ required: true, trigger: "blur" }],
  "findArea.end.x": [{ required: true, trigger: "blur" }],
  "findArea.end.y": [{ required: true, trigger: "blur" }],
  "offset.r": [{ required: true, trigger: "blur" }],
  "offset.g": [{ required: true, trigger: "blur" }],
  "offset.b": [{ required: true, trigger: "blur" }],
});

const close = () => {
  emits("close");
};

const drawImage = async () => {
  try {
    const data = await base64PngToRgbPixels(props.params.base64Png);
    let arr: RelativeColorPoint[] = [];
    let index = 0;
    for (let y = 0; y < props.params.size.height; y++) {
      for (let x = 0; x < props.params.size.width; x++) {
        const hex = HexColor.fromRgbColor(data[index]);
        const point = Point.from(x, y);
        const relativePoint = Point.from(-1, -1);
        let row = RelativeColorPoint.from(hex, point, relativePoint);
        arr.push(row);
        index++;
      }
    }
    points.value = arr;
  } catch (e: unknown) {
    msgError(e);
  }
};

const pushColor = async (relativeColorPoint: RelativeColorPoint) => {
  if (
    form.points
      .map((item) => {
        return item.key;
      })
      .includes(relativeColorPoint.key)
  ) {
    msgError("The point is already exist!");
    return;
  }
  result.value = code.value = null;
  relativeColorPoint.relativePoint = Point.from(-1, -1);
  form.points.push(relativeColorPoint);
  caculateRelativePoints();
};

const unAdd = async () => {
  result.value = code.value = null;
  form.points.pop();
  caculateRelativePoints();
};

const removeColor = (relativeColorPoint: RelativeColorPoint) => {
  result.value = code.value = null;
  form.points = form.points.filter((item) => {
    const compareItem = item.point.x + "," + item.point.y;
    const compare =
      relativeColorPoint.point.x + "," + relativeColorPoint.point.y;
    return compareItem !== compare;
  });
  caculateRelativePoints();
};

const caculateRelativePoints = () => {
  if (form.points.length == 0) {
    return;
  }
  if (form.points.length == 1) {
    form.points[0].relativePoint.x = 0;
    form.points[0].relativePoint.y = 0;
    return;
  }
  const points: RelativeColorPoint[] = [];
  form.points.forEach((item) => {
    points.push(item.clone());
  });
  points.sort((a, b) => {
    if (a.point.y == b.point.y) {
      return a.point.x - b.point.x;
    } else {
      return a.point.y - b.point.y;
    }
  });
  points.forEach((item, index, origin) => {
    //the first element is the peak point
    if (index == 0) {
      origin[index].relativePoint.x = 0;
      origin[index].relativePoint.y = 0;
    }
    if (index > 0) {
      origin[index].relativePoint.x = item.point.x - origin[0].point.x;
      origin[index].relativePoint.y = item.point.y - origin[0].point.y;
    }
  });
  //assign points' relativePoint into points
  form.points.forEach((item, index, arr) => {
    let point = points
      .filter((c) => {
        return c.key == item.key;
      })
      .pop();
    if (!point) return;
    arr[index].relativePoint.x = point.relativePoint.x;
    arr[index].relativePoint.y = point.relativePoint.y;
  });
  return;
};

const getRelativePoint = (key: string): Point => {
  if (form.points.length == 0) {
    return Point.from(-1, -1);
  }
  return form.points
    .filter((item) => {
      return item.key == key;
    })
    .pop()!.relativePoint;
};

const getPeakKey = () => {
  if (form.points.length == 0) {
    return "";
  }
  return form.points
    .filter((item) => {
      return item.relativePoint.x == 0 && item.relativePoint.y == 0;
    })
    .pop()!.key;
};

const getRelativePoints = (): ColoredPoint[] => {
  if (form.points.length <= 1) {
    return [];
  }
  return form.points
    .filter((item) => item.relativePoint.x != 0 || item.relativePoint.y != 0)
    .map((item) => {
      return ColoredPoint.from(item.relativePoint, item.hex);
    });
};

const generateCode = async () => {
  const vertexHex = getVertexHex();
  const relativePoints = getRelativePoints();
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const rgbOffset = form.offset;
  try {
    code.value = await stateStore.code.generateFindRelativeColorsCode(
      vertexHex,
      relativePoints,
      startPoint,
      endPoint,
      rgbOffset
    );
  } catch (e: unknown) {
    msgError(e);
  }
};

const getVertexHex = (): HexColor => {
  if (form.points.length == 0) {
    return HexColor.from("#ffffff");
  }
  return form.points
    .filter((item) => {
      return item.relativePoint.x == 0 && item.relativePoint.y == 0;
    })
    .pop()!.hex;
};

const findRelativeColors = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
  } catch (e: unknown) {
    return;
  }
  if (loading.value) return;
  result.value = code.value = null;
  if (form.points.length == 0) {
    clearAllItems();
    msgWarn("The colors must not be empty.");
    return;
  }
  const vertexHex = getVertexHex();
  const relativePoints = getRelativePoints();
  const origin = props.target.originalBase64Png;
  const startPoint = form.findArea.start;
  const endPoint = form.findArea.end;
  const rgbOffset = form.offset;
  try {
    loading.value = true;
    const peak = await stateStore.frame.findRelativeColors(
      origin,
      vertexHex,
      relativePoints,
      startPoint,
      endPoint,
      rgbOffset
    );
    if (peak == null) {
      clearAllItems();
    } else {
      result.value = JSON.stringify(peak);
      drawItems(peak);
    }
    await generateCode();
  } catch (e: unknown) {
    clearAllItems();
    result.value = code.value = null;
    msgError(e);
  } finally {
    loading.value = false;
  }
};

const drawItems = (peak: ColoredPoint) => {
  emits("drawItems", {
    callback: (ctx: CanvasRenderingContext2D) => {
      const title = `peak point(${peak.point.x}, ${peak.point.y})`;
      const titlePoint = Point.from(peak.point.x - 5, peak.point.y - 10);
      drawArc(ctx, peak.point, 5);
      drawText(ctx, title, titlePoint);
    },
  });
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

const clearAllItems = () => {
  emits("clearAllItems");
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
  form.points = [];
  form.base64Png = props.params.base64Png;
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
</script>
<template>
  <el-container>
    <el-header>Find Relative Colors</el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" status-icon>
        <div class="work-area">
          <div class="canvas-work">
            <div class="pixels-box">
              <div
                class="pixels"
                :style="{
                  width:
                    (props.params.size.width * pixelValue) /
                      stateStore.zoom.factor +
                    'px',
                  height:
                    (props.params.size.height * pixelValue) /
                      stateStore.zoom.factor +
                    'px',
                  transformOrigin: 'top left',
                  transform: `scale(${stateStore.zoom.factor})`,
                  gridTemplateColumns: `repeat(${props.params.size.width}, ${pixelValue}px)`,
                }"
              >
                <div
                  class="pixel"
                  v-for="item in points"
                  :style="{
                    'background-color': item.hex,
                    width: pixelValue + 'px',
                    height: pixelValue + 'px',
                  }"
                  @click="pushColor(item)"
                  :class="{
                    selected: form.points
                      .map((row) => {
                        return row.key;
                      })
                      .includes(item.key),
                    peak: item.key == getPeakKey(),
                  }"
                ></div>
              </div>
            </div>
          </div>
          <div class="actions">
            <el-button size="small" type="danger" @click="unAdd">
              <el-icon><Back /></el-icon>
            </el-button>
          </div>
          <div class="item">
            <div class="title">Colors</div>
            <el-form-item
              prop="points"
              style="margin-bottom: 0px"
              v-show="form.points.length > 0"
            >
              <el-input
                class="color"
                style="margin-bottom: 2px"
                v-for="item in form.points"
                :value="
                  item.hex +
                  ' ' +
                  '(' +
                  item.point.x +
                  ',' +
                  item.point.y +
                  ')' +
                  '(' +
                  getRelativePoint(item.key).x +
                  ',' +
                  getRelativePoint(item.key).y +
                  ')'
                "
                disabled
              >
                <template #prepend>
                  <div
                    style="height: 10px; width: 10px; border-radius: 5px"
                    :style="{ backgroundColor: item.hex }"
                  ></div>
                </template>
                <template #append>
                  <el-button @click="removeColor(item)">×</el-button>
                </template>
              </el-input>
            </el-form-item>
          </div>
          <div class="item">
            <div class="title">
              <span>Find Area</span>
              <el-button
                type="primary"
                @click="findRelativeColors(formRef)"
                :disabled="loading"
              >
                findOne
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
              <el-row :gutter="10">
                <el-col :span="8">
                  <el-form-item style="margin-bottom: 0px" prop="offset.r">
                    <el-input-number
                      v-model="form.offset.r"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :max="50"
                      :min="0"
                      ><template #prefix>
                        <span>offset r</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offset.g" style="margin-bottom: 0px">
                    <el-input-number
                      v-model="form.offset.g"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :max="50"
                      :min="0"
                      ><template #prefix>
                        <span>offset g</span>
                      </template>
                    </el-input-number>
                  </el-form-item>
                </el-col>
                <el-col :span="8">
                  <el-form-item prop="offset.b" style="margin-bottom: 0px">
                    <el-input-number
                      v-model="form.offset.b"
                      :controls="false"
                      :style="{ width: '100%' }"
                      :max="50"
                      :min="0"
                      ><template #prefix>
                        <span>offset b</span>
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
                :rows="9"
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
      display: grid;
      gap: 0;
    }
    .pixel {
      box-sizing: border-box;
    }
    .pixel:hover,
    .selected {
      border: 2px solid rgb(0, 0, 119);
      box-shadow: inset 0 0 5px white;
    }

    .peak {
      border-color: red;
      box-shadow: inset 0 0 5px #ffff00;
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
.canvas-bg {
  background-repeat: repeat;
  overflow: hidden;
}
.actions {
  margin-top: 10px;
  display: flex;
  justify-content: space-around;
  margin-bottom: 10px;
}
.el-button.current {
  background-color: rgb(97, 97, 97);
}
</style>
