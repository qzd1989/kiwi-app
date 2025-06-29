<script setup lang="ts">
import { onMounted, ref, watchEffect } from "vue";
import { homeDir } from "@tauri-apps/api/path";
import { stateStore, localStore } from "@utils/store";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { invoke } from "@tauri-apps/api/core";
const isMounted = ref(false);

const init = async () => {
  stateStore.init();
  await localStoreInit();
};

const localStoreInit = async () => {
  await localStore.init();
  // projectRootDirectory
  const projectRootDirectory = await localStore.get("projectRootDirectory");
  const projectRootDirectoryExists = await invoke("path_exists", {
    path: projectRootDirectory,
  });
  const defaultProjectRootDirectory = await homeDir();
  if (projectRootDirectory == null || !projectRootDirectoryExists) {
    await localStore.set("projectRootDirectory", defaultProjectRootDirectory);
    return;
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
  if (!isMounted.value) return;
  await getCurrentWebview().setZoom(stateStore.zoom.factor);
});

onMounted(async () => {
  await init();
  window.addEventListener("keyup", shortcutZoom);
  isMounted.value = true;
});
</script>
<template>
  <router-view></router-view>
</template>
<style scoped></style>
