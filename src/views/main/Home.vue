<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { Project, App, Common } from "@api";
import { msgError } from "@utils";
import { useAppStore, useLocalStore } from "@store";
import CreateModal from "@views/main/project/CreateModal.vue";
import { Server } from "@api/Server";
import { delay, msgSuccess } from "@utils/common";
import { Release } from "@types";
import { open as openUrl } from "@tauri-apps/plugin-shell";
import ReleaseModal from "./ReleaseModal.vue";
import SettingModal from "./SettingModal.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const isDev = import.meta.env.DEV;
const appStore = useAppStore();
const localStore = useLocalStore();
const router = useRouter();
const showCreateModal = ref(false);
const showSettingModal = ref(false);
const app_version = ref<string | null>(null);
const release = ref<Release | null>(null);

const closeCreateModal = () => {
  showCreateModal.value = false;
};

const closeSettingModal = () => {
  showSettingModal.value = false;
};

const selectProject = async () => {
  try {
    const path = await open({
      directory: true,
      multiple: false,
      defaultPath: undefined,
    });
    if (path) {
      appStore.project = await Project.open(path);
      router.push({
        path: "/project",
        query: { path },
      });
    }
  } catch (e: unknown) {
    msgError(e);
  }
};

const openLocalServer = async () => {
  try {
    await Server.shutdown();
    await delay(200);
    await Server.startLocal();
    await Server.setRemoteAddress(await Server.getLocalAddress());
    appStore.remoteServerAddress = await Server.getRemoteAddress();
    await App.toUser();
  } catch (e) {
    appStore.remoteServerAddress = null;
    msgError(e);
  }
};

const initApp = async () => {
  try {
    app_version.value = await App.version();
  } catch (e) {
    msgError(e);
  }
};
const checkRelease = async () => {
  release.value = await App.checkRelease();
};

const clearLocalStore = async () => {
  await localStore.clear();
  msgSuccess(t("Local storage cleared."));
};

const goOfficialWebsite = async () => {
  const url = "https://kiwi.biexi.com";
  await openUrl(url);
};

const xattrInterpreter = async () => {
  try {
    await Common.xattrInterpreter();
  } catch (e) {
    msgError(e);
  }
};

onMounted(async () => {
  await openLocalServer();
  await initApp();
  await xattrInterpreter();
  await checkRelease();
});
onUnmounted(async () => {});
</script>
<template>
  <div
    class="relative flex h-screen w-screen flex-col items-center justify-center"
  >
    <div class="mb-6 flex flex-col items-center gap-2">
      <div class="h-30 w-30 rounded-full">
        <img src="./../../assets/logo.png" />
      </div>
      <div class="text-4xl font-extrabold text-gray-700">
        {{ t("Kiwi Assistant") }}
      </div>
      <div class="text-gray-500">
        {{ t("Simplifying your tasks, one click at a time.") }}
      </div>
    </div>
    <ul class="flex flex-col items-center gap-4">
      <li class="min-w-md">
        <el-button
          type="primary"
          plain
          @click="router.push({ path: '/listening' })"
          size="large"
          class="w-full"
        >
          {{ t("Start Remote Assistance") }}
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button
          type="primary"
          @click="showCreateModal = true"
          size="large"
          class="w-full"
        >
          {{ t("Create Project") }}
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button
          type="primary"
          @click="selectProject"
          size="large"
          class="w-full"
        >
          {{ t("Open Project") }}
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button
          type="info"
          plain
          size="large"
          class="w-full"
          @click="goOfficialWebsite"
        >
          {{ t("Official Website") }}
        </el-button>
      </li>
      <li class="min-w-md" v-show="isDev">
        <el-button
          type="info"
          plain
          @click="clearLocalStore"
          size="large"
          class="w-full"
        >
          {{ t("ClearLocalStore") }}
        </el-button>
      </li>
    </ul>

    <div class="mt-4 text-sm text-gray-400 underline">
      {{ t("Version") }}: {{ app_version }}
    </div>
    <div
      class="absolute top-2 right-2 text-2xl"
      @click="showSettingModal = true"
    >
      <el-icon
        class="transform cursor-pointer transition duration-300 hover:rotate-45"
      >
        <Setting class="text-gray-400 hover:text-blue-500" />
      </el-icon>
    </div>
  </div>
  <ReleaseModal v-if="release != null" :release="release" />
  <CreateModal v-if="showCreateModal" :close="closeCreateModal" />
  <SettingModal v-if="showSettingModal" :close="closeSettingModal" />
</template>
<style scoped></style>
