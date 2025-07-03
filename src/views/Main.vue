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
import { useI18n } from "vue-i18n";
import { EmitMsg, Locale } from "@types";

const { locale } = useI18n();
const stateStore = useStateStore();
const init = async () => {
  // init app
  const app = await AppModel.getApp();
  if (!app.config) return;
  await focus();
  stateStore.app = app;

  // set locale
  setLocale(app.config.app.locale);

  // init local store
  await localStoreInit();

  // init websocket
  if (!(await websocketInit())) {
    return;
  }
};

const setLocale = (newLocale: Locale) => {
  if (!stateStore.app.config) return;
  locale.value = stateStore.app.config.app.locale = newLocale;
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

listen<EmitMsg>("msg:error", (event) => {
  msgError(event.payload.data);
});

listen<string>("backend:update:locale", (event) => {
  setLocale(event.payload as Locale);
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
