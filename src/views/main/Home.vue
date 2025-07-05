<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref } from "vue";
import { useRouter } from "vue-router";
import { useStateStore, localStore } from "@utils/store";
import { open } from "@tauri-apps/plugin-dialog";
import { msgError } from "@utils/msg";
import { AppModel, Release } from "@kiwi/App";
import ReleaseDialog from "@views/main/components/ReleaseDialog.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const stateStore = useStateStore();
const release = ref<Release | null>(null);
const isDev = import.meta.env.DEV;
const router = useRouter();
const isEnabled = computed(() => {
  return Object.values(stateStore.enable).every((v) => v === true);
});

const checkRelease = async () => {
  const app = new AppModel(stateStore.app);
  release.value = await app.checkRelease();
};

const selectProject = async () => {
  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
      defaultPath: (await localStore.get("projectRootDirectory")) ?? undefined,
    });
    if (selectedPath) {
      router.push({
        path: "/main/project/detail",
        query: { path: selectedPath },
      });
    }
  } catch (e: unknown) {
    msgError(e);
  }
};

const goToCreateProject = async () => {
  router.push({
    path: "/main/project/create",
  });
};

const goToSetting = () => {
  router.push({
    path: "/main/setting",
  });
};

const clearLocalStore = async () => {
  await localStore.clear();
};

onMounted(async () => {
  await checkRelease();
  // router.push({
  //   path: "/main/project/detail",
  //   query: { path: "/Users/kiwi/Downloads/project/2" },
  // });
});

onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-main>
      <el-row :gutter="0">
        <el-col :span="24">
          <div class="logo">
            <img src="/logo.png" alt="Logo" :width="100" />
          </div>
          <div class="title">{{ stateStore.app.name }}</div>
          <div class="sologan">
            {{ t("Hands-free, everything on autopilot.") }}
          </div>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            @click="goToCreateProject"
            :disabled="!isEnabled"
            >{{ t("Create Project") }}</el-button
          >
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            @click="selectProject"
            :disabled="!isEnabled"
            >{{ t("Open Project") }}</el-button
          >
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button type="primary" @click="goToSetting">{{
            t("Setting")
          }}</el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button type="primary" @click="clearLocalStore" v-if="isDev">
            ClearLocalStore
          </el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24" class="version">
          {{ t("Version") }}: {{ stateStore.app.version }}</el-col
        >
      </el-row>
    </el-main>
  </el-container>
  <ReleaseDialog v-if="release != null" :release="release" />
</template>
<style scoped>
.el-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  .el-main {
    max-width: 70vw;
    margin: auto;
    .el-row {
      .el-col {
        margin: 10px 0;
        text-align: center;
      }
      .logo {
        margin-bottom: 10px;
        margin-top: -20px;
      }
      .title {
        font-size: 35px;
        font-weight: bold;
        margin-bottom: 5px;
      }
      .sologan {
        font-size: 14px;
      }
      .el-button {
        width: 100%;
        min-height: 40px;
      }
      .version {
        font-size: 14px;
      }
    }
  }
}
</style>
