<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useAppStore } from "@store";
import { Locale } from "@api";
import { useI18n } from "vue-i18n";

const { locale } = useI18n();
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

const initLocal = async () => {
  locale.value = await Locale.get();
};

onMounted(async () => {
  await watchWindow();
  await initLocal();
});

onUnmounted(async () => {});
</script>
<template>
  <router-view></router-view>
</template>
<style scoped></style>
