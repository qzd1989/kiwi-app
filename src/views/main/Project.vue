<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "@store";
import { Server } from "@api/Server";
import Log from "@views/main/project/Log.vue";
import Monitor from "@views/main/project/Monitor.vue";
import MonitorButtons from "@views/main/project/MonitorButtons.vue";
import ConnectToServerModal from "./project/ConnectToServerModal.vue";
import Item from "@views/components/Item.vue";
import { Png } from "@types";
import { delay, msgError, msgWarn } from "@utils";
import { Capturer, Project } from "@api";
import LogButtons from "./project/LogButtons.vue";
import { platform } from "@tauri-apps/plugin-os";
import {
  register,
  ShortcutHandler,
  unregister,
} from "@tauri-apps/plugin-global-shortcut";

interface HotKeys {
  runScript: string;
  runProject: string;
  stopAll: string;
}

const appStore = useAppStore();
const showConnectToServerModal = ref(false);
const router = useRouter();
const isFullScreen = ref<boolean>(false);
const toggleFullScreen = () => {
  isFullScreen.value = !isFullScreen.value;
};
const monitorRef = ref<InstanceType<typeof Monitor> | null>(null);
const logRef = ref<InstanceType<typeof Log> | null>(null);
const activeTab = ref<"log" | "monitor">("monitor");
const remoteAliveTimer = ref<number | undefined>(undefined);
const capturerStatusTimer = ref<number | undefined>(undefined);
const isRemoteServerAlive = ref<boolean>(false);
const isCapturerRunning = ref<boolean>(false);
const shouldShowCapturer = ref<boolean>(false);
const entryFile = ref<string | null>(null);
const hotKeys = reactive<HotKeys>({
  runScript: "F10",
  runProject: "F11",
  stopAll: "F12",
});

const closeConnectToServerModal = async () => {
  showConnectToServerModal.value = false;
  await updateCapturerVisibility();
};

const loadPngFromFile = async (png: Png) => {
  monitorRef.value?.loadPngFromFile(png);
};

const capture = async () => {
  await monitorRef.value?.capture();
};

const resetPng = async () => {
  await monitorRef.value?.resetPng();
};

const exportPng = async () => {
  await monitorRef.value?.exportPng();
};

const clearLog = async () => {
  logRef.value?.clear();
};

const updateEntryFile = async () => {
  try {
    entryFile.value = await Project.entryFile();
  } catch (e) {
    msgError(e);
  }
};

const startCapturer = async () => {
  try {
    await Capturer.start();
  } catch (e) {
    msgError(e);
  }
};

const stopCapturer = async () => {
  try {
    // if project is running, don't stop.
    let isRunning = await Project.is_running();
    if (isRunning) {
      msgWarn("Script is running, stop it first.");
      return;
    }
    await Capturer.stop();
  } catch (e) {
    msgError(e);
  }
};

const runProject = async () => {
  try {
    await Project.runScript();
  } catch (e) {
    msgError(e);
  }
};

const runScript = async () => {
  if (!entryFile.value) return;
  try {
    await Project.runScript(entryFile.value);
  } catch (e) {
    msgError(e);
  }
};

const stopRunScript = async () => {
  try {
    await Project.stopRunScript();
  } catch (e) {
    msgError(e);
  }
};

const openProjectFolder = async () => {
  await appStore.project?.openFolder();
};

const editProjectInEditor = async () => {
  await appStore.project?.openInEditor();
};

const updateCapturerVisibility = async (): Promise<boolean> => {
  try {
    const localAddress = await Server.getLocalAddress();
    shouldShowCapturer.value = localAddress == appStore.remoteServerAddress;
  } catch (e) {
    msgError(e);
  }
  return false;
};

const initRemoteAliveTimer = async () => {
  remoteAliveTimer.value = window.setInterval(async () => {
    try {
      if (!appStore.remoteServerAddress) return;
      isRemoteServerAlive.value = await Server.isRemoteAlive(
        appStore.remoteServerAddress,
      );
    } catch (e) {
      msgError(e);
    }
  }, 200);
};

const initCapturerStatusTimer = async () => {
  capturerStatusTimer.value = window.setInterval(async () => {
    try {
      isCapturerRunning.value = await Capturer.is_running();
    } catch (e) {
      msgError(e);
    }
  }, 200);
};

const removeRemoteAliveTimer = () => {
  if (remoteAliveTimer.value !== undefined) {
    clearInterval(remoteAliveTimer.value);
    remoteAliveTimer.value = undefined;
  }
};

const removeCapturerStatusTimer = () => {
  if (capturerStatusTimer.value != undefined) {
    clearInterval(capturerStatusTimer.value);
    capturerStatusTimer.value = undefined;
  }
};

const initHotKeys = async () => {
  if ((await platform()) == "windows") {
    hotKeys.runScript = "Ctrl+F10";
    hotKeys.runProject = "Ctrl+F11";
    hotKeys.stopAll = "Ctrl+F12";
  }
};

const safeUnregisterHotkey = async (key: string) => {
  try {
    await unregister(key);
  } catch (error) {
    // unregister失败会报错,并且用isRegistered也会误判,所以此处就不显示错误了
  }
};

const safeRegisterHotkey = async (key: string, handler: ShortcutHandler) => {
  await safeUnregisterHotkey(key);
  try {
    await register(key, handler);
  } catch (e: unknown) {
    msgError(e);
  }
};

const registerHotkeys = async () => {
  await safeRegisterHotkey(hotKeys.runScript, async (event) => {
    if (event.state === "Released") await runScript();
  });
  await safeRegisterHotkey(hotKeys.runProject, async (event) => {
    if (event.state === "Released") await runProject();
  });
  await safeRegisterHotkey(hotKeys.stopAll, async (event) => {
    if (event.state === "Released") await stopRunScript();
  });
};

const unregisterHotkeys = async () => {
  await safeUnregisterHotkey(hotKeys.runScript);
  await safeUnregisterHotkey(hotKeys.runProject);
  await safeUnregisterHotkey(hotKeys.stopAll);
};

onMounted(async () => {
  appStore.initProject();
  await updateCapturerVisibility();
  await updateEntryFile();
  await initRemoteAliveTimer();
  await initCapturerStatusTimer();

  // register hotkeys
  await initHotKeys();
  await delay(100);
  await registerHotkeys();
});

onUnmounted(async () => {
  removeRemoteAliveTimer();
  removeCapturerStatusTimer();

  await unregisterHotkeys();
});
</script>
<template>
  <el-container
    class="h-screen w-screen outline select-none"
    :class="isFullScreen ? 'p-0' : 'p-2'"
  >
    <el-aside class="space-y-2 pr-2" v-show="!isFullScreen">
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            plain
            size="large"
            class="w-full"
            @click="showConnectToServerModal = true"
            :disabled="isCapturerRunning"
          >
            <div class="flex h-full w-full items-center">
              {{ appStore.remoteServerAddress }}
              <div
                class="ml-2 h-3 w-3 rounded-full bg-green-400"
                v-if="isRemoteServerAlive"
              ></div>
              <div
                class="ml-2 h-3 w-3 rounded-full bg-red-400"
                v-if="!isRemoteServerAlive"
              ></div>
            </div>
          </el-button>
        </el-col>
      </el-row>
      <el-divider direction="horizontal" class="my-0! mb-2!"></el-divider>
      <el-row :gutter="0" v-if="shouldShowCapturer">
        <el-col :span="24">
          <el-button
            v-if="isCapturerRunning"
            class="w-full"
            type="info"
            size="large"
            @click="stopCapturer"
          >
            Stop Capturer
          </el-button>

          <el-button
            v-if="!isCapturerRunning"
            class="group w-full"
            size="large"
            type="primary"
            @click="startCapturer"
          >
            <div class="flex items-center">
              <div>Only Start Capturer</div>
              <div class="ml-2">
                <el-tooltip
                  effect="dark"
                  content="You can run file in vscode."
                  placement="right-start"
                >
                  <el-icon
                    class="mt-0.5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                  >
                    <QuestionFilled />
                  </el-icon>
                </el-tooltip>
              </div>
            </div>
          </el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="primary"
            size="large"
            class="group w-full"
            @click="runProject"
          >
            <div class="flex items-center">
              <div>Run Project</div>
              <div class="ml-2">
                <el-tooltip
                  effect="dark"
                  :content="hotKeys.runProject"
                  placement="right-start"
                >
                  <el-icon
                    class="mt-0.5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                  >
                    <QuestionFilled />
                  </el-icon>
                </el-tooltip>
              </div>
            </div>
          </el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="danger"
            size="large"
            class="group w-full"
            @click="stopRunScript"
          >
            <div class="flex items-center">
              <div>Stop All</div>
              <div class="ml-2">
                <el-tooltip
                  effect="dark"
                  :content="hotKeys.stopAll"
                  placement="right-start"
                >
                  <el-icon
                    class="mt-0.5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                  >
                    <QuestionFilled />
                  </el-icon>
                </el-tooltip>
              </div>
            </div>
          </el-button>
        </el-col>
      </el-row>
      <el-divider direction="horizontal" class="my-0! mb-2!"></el-divider>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="info"
            plain
            size="large"
            class="w-full"
            @click="editProjectInEditor"
          >
            Edit
          </el-button>
        </el-col>
      </el-row>
      <el-row :gutter="0">
        <el-col :span="24">
          <el-button
            type="info"
            plain
            size="large"
            class="w-full"
            @click="router.push({ path: '/home' })"
          >
            Back
          </el-button>
        </el-col>
      </el-row>
      <Item class="mt-2">
        <template #title>
          <span class="font-bold">{{ $t("Project Info") }}</span>
        </template>
        <template #content>
          <div class="text-sm text-gray-600">
            <div class="flex border-b border-b-gray-300 py-1 pt-0">
              <div class="mr-2 shrink-0 font-semibold">Name</div>
              <div>{{ appStore.project?.name }}</div>
            </div>
            <div class="flex border-b border-b-gray-300 py-1 pt-0">
              <div class="mr-2 shrink-0 font-semibold">Version</div>
              <div>{{ appStore.project?.version }}</div>
            </div>
            <div class="flex border-b border-b-gray-300 py-1">
              <div class="mr-2 shrink-0 font-semibold">Path</div>
              <div
                class="cursor-pointer text-blue-400 underline hover:decoration-0"
                @click="openProjectFolder"
              >
                {{ appStore.project?.path }}
              </div>
            </div>
            <div class="flex border-b border-b-gray-300 py-1 pt-0">
              <div class="mr-2 shrink-0 font-semibold">Description</div>
              <div>{{ appStore.project?.description }}</div>
            </div>
          </div>
        </template>
      </Item>
    </el-aside>
    <el-container>
      <el-header
        class="flex h-auto! items-center gap-2 p-0! pb-2!"
        v-show="!isFullScreen"
      >
        <el-input v-model="entryFile" size="large"></el-input>
        <el-button class="group" type="primary" @click="runScript" size="large">
          <div class="flex items-center">
            <div>Run File</div>
            <div class="ml-2">
              <el-tooltip
                effect="dark"
                :content="hotKeys.runScript"
                placement="left"
              >
                <el-icon
                  class="mt-0.5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                >
                  <QuestionFilled />
                </el-icon>
              </el-tooltip>
            </div>
          </div>
        </el-button>
      </el-header>
      <el-main class="relative p-0!">
        <el-tabs
          type="border-card"
          class="relative h-full w-full"
          v-model="activeTab"
        >
          <el-tab-pane :label="$t('Log')" name="log" class="h-full w-full">
            <Log ref="logRef" />
          </el-tab-pane>
          <el-tab-pane
            :label="$t('Monitor')"
            name="monitor"
            class="h-full w-full"
          >
            <Monitor ref="monitorRef" class="h-full w-full" />
          </el-tab-pane>
        </el-tabs>
        <div class="absolute top-1.5 right-2">
          <div v-show="activeTab == 'log'">
            <el-button
              type="primary"
              @click="toggleFullScreen"
              plain
              size="small"
              class="-mr-1 px-1.5!"
            >
              <el-icon><FullScreen /></el-icon>
            </el-button>
            <LogButtons :clear="clearLog" />
          </div>
          <div v-show="activeTab == 'monitor'">
            <el-button
              type="primary"
              @click="toggleFullScreen"
              plain
              size="small"
              class="-mr-1 px-1.5!"
            >
              <el-icon><FullScreen /></el-icon>
            </el-button>
            <MonitorButtons
              :loadPngFromFile="loadPngFromFile"
              :capture="capture"
              :resetPng="resetPng"
              :exportPng="exportPng"
            />
          </div>
        </div>
      </el-main>
    </el-container>
  </el-container>
  <ConnectToServerModal
    v-if="showConnectToServerModal"
    :close="closeConnectToServerModal"
  />
</template>
<style scoped>
:deep(.el-tabs--border-card > .el-tabs__content) {
  padding: 0;
}
</style>
