<script setup lang="ts">
import { onMounted, onUnmounted, reactive } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { Png } from "@types";
import {
  delay,
  msgError,
  safeRegisterHotkey,
  safeUnregisterHotkey,
} from "@utils";
import { platform } from "@tauri-apps/plugin-os";

interface Hotkeys {
  capture: string;
  captureAndExport: string;
}

const hotkeySetting = {
  windows: { capture: "Ctrl+F8", captureAndExport: "Ctrl+F12" },
  macos: { capture: "F8", captureAndExport: "F12" },
};
const hotkeys = reactive<Hotkeys>(hotkeySetting.macos);
const propos = defineProps([
  "loadPngFromFile",
  "resetPng",
  "capture",
  "exportPng",
  "captureAndExportPng",
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

const captureAndExport = async () => {
  await propos.resetPng();
  await propos.captureAndExportPng();
};

const draw = async (path: string) => {
  const png = await Png.fromFile(path);
  await propos.loadPngFromFile(png);
};

const initHotkeys = async () => {
  if ((await platform()) == "windows") {
    hotkeys.capture = hotkeySetting.windows.capture;
    hotkeys.captureAndExport = hotkeySetting.windows.captureAndExport;
  }
};

const registerHotkeys = async () => {
  await safeRegisterHotkey(hotkeys.capture, async (event) => {
    if (event.state === "Released") await capture();
  });
  await safeRegisterHotkey(hotkeys.captureAndExport, async (event) => {
    if (event.state === "Released") await captureAndExport();
  });
};

const unregisterHotkeys = async () => {
  await safeUnregisterHotkey(hotkeys.capture);
  await safeUnregisterHotkey(hotkeys.captureAndExport);
};

onMounted(async () => {
  // register hotkeys
  await initHotkeys();
  await delay(100);
  await registerHotkeys();
});
onUnmounted(async () => {
  await unregisterHotkeys();
});
</script>
<template>
  <el-tooltip effect="dark" :content="hotkeys.capture" placement="bottom">
    <el-button type="primary" plain size="small" class="-mr-1" @click="capture">
      capture
    </el-button>
  </el-tooltip>
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
  <el-tooltip
    effect="dark"
    :content="hotkeys.captureAndExport"
    placement="bottom"
  >
    <el-button
      type="primary"
      plain
      size="small"
      class="-mr-1"
      @click="captureAndExport"
    >
      screenshot
    </el-button>
  </el-tooltip>
  <el-button type="warning" @click="propos.resetPng" plain size="small">
    reset
  </el-button>
</template>
<style scoped></style>
