<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useAppStore } from "@store";

const appStore = useAppStore();

const watchWindow = async () => {
  const window = await getCurrentWindow();
  const size = await window.innerSize();
  const scaleFactor = await window.scaleFactor();
  appStore.window = {
    width: size.width,
    height: size.height,
    scaleFactor: scaleFactor,
  };
  window.onResized(async ({ payload: size }) => {
    appStore.window.width = size.width;
    appStore.window.height = size.height;
    appStore.window.scaleFactor = await window.scaleFactor();
  });
};

onMounted(async () => {
  await watchWindow();
});

onUnmounted(async () => {});
</script>
<template>
  <router-view></router-view>
</template>
<style scoped></style>
