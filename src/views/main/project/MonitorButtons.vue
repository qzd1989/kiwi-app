<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { Png } from "@types";
import { msgError } from "@utils";

const propos = defineProps([
  "loadPngFromFile",
  "resetPng",
  "capture",
  "exportPng",
]);

const selectMonitorImage = async () => {
  try {
    const path = await open({
      directory: false,
      multiple: false,
      defaultPath: undefined,
      filters: [
        {
          name: "Image",
          extensions: ["png"],
        },
      ],
    });
    if (path) {
      draw(path);
    }
  } catch (e) {
    msgError(e);
  }
};

const capture = async () => {
  await propos.resetPng();
  await propos.capture();
};

const draw = async (path: string) => {
  const png = await Png.fromFile(path);
  await propos.loadPngFromFile(png);
};

onMounted(async () => {});
onUnmounted(async () => {});
</script>
<template>
  <el-button type="primary" plain size="small" class="-mr-1" @click="capture">
    capture
  </el-button>
  <el-button
    type="primary"
    @click="selectMonitorImage"
    plain
    size="small"
    class="-mr-1"
  >
    import
  </el-button>
  <el-button
    type="primary"
    @click="propos.exportPng"
    plain
    size="small"
    class="-mr-1"
  >
    export
  </el-button>
  <el-button type="warning" @click="propos.resetPng" plain size="small">
    reset
  </el-button>
</template>
<style scoped></style>
