<script setup lang="ts">
import { onMounted, watchEffect } from "vue";
import { homeDir } from "@tauri-apps/api/path";
import { useStateStore, localStore } from "@utils/store";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { isWebsocketAlive, openWebsocket } from "@utils/common";
import { msgError } from "@utils/msg";
import { listen } from "@tauri-apps/api/event";
import { onUnmounted } from "vue";
import { AppModel, commonModel } from "@kiwi";
const stateStore = useStateStore();

const init = async () => {
  await focus();
  stateStore.app = await AppModel.getApp();
  await localStoreInit();
  if (!(await websocketInit())) {
    return;
  }
};

const focus = async () => {
  await getCurrentWebview().setFocus();
};

const websocketInit = async () => {
  try {
    const port = stateStore.app.config!.app.websocket_port;
    await openWebsocket(port);
    stateStore.enable.isWebsocketAlive = await isWebsocketAlive(port);
  } catch (e: unknown) {
    msgError(e);
    return false;
  }
  return true;
};

const localStoreInit = async () => {
  await localStore.init();
  const savedPath = await localStore.get("projectRootDirectory");
  const defaultPath = await homeDir();
  const exists =
    savedPath && (await commonModel.pathExists(savedPath as string));
  if (!exists) {
    await localStore.set("projectRootDirectory", defaultPath);
  }
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

watchEffect(async () => {
  await getCurrentWebview().setZoom(stateStore.zoom.factor);
});

listen("msg:error", (event: any) => {
  msgError(event.payload.data);
});

onMounted(async () => {
  await init();
  window.addEventListener("keyup", shortcutZoom);
});

onUnmounted(async () => {
  window.removeEventListener("keyup", shortcutZoom);
});
</script>
<template>
  <router-view></router-view>
</template>
<style scoped></style>
