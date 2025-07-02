<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive, nextTick } from "vue";
import { useStateStore } from "@utils/store";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getAllWindows } from "@tauri-apps/api/window";
import { ElLoading, ElScrollbar } from "element-plus";
import { useRoute, useRouter } from "vue-router";
import { msgError, msgSuccess } from "@utils/msg";
import { platform } from "@tauri-apps/plugin-os";
import { delay, minimizeAll, unminimizeAll } from "@utils/common";
import { EmitLog, EmitProgress, EmitProject, Stack } from "@types";
import {
  register,
  ShortcutHandler,
  unregister,
} from "@tauri-apps/plugin-global-shortcut";
import { join } from "@tauri-apps/api/path";
import { listen } from "@tauri-apps/api/event";
import { ProjectModel } from "@kiwi";
import { useI18n } from "vue-i18n";

type LogType = "info" | "success" | "error";
interface HotKeys {
  recorder: string;
  runScript: string;
  runProject: string;
  stopAll: string;
}
interface Hiders {
  recorder: boolean;
  runProject: boolean;
  runScript: boolean;
}
interface Log {
  type: LogType;
  message: string;
  timestamp: number;
  formattedTime: string;
}
namespace Log {
  const formatLogTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const year = date.getFullYear().toString().slice(-2);
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    const seconds = (date.getSeconds() + (timestamp % 1))
      .toFixed(3)
      .padStart(6, "0");
    const formattedTime = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
    return formattedTime;
  };
  export const info = (message: string, timestamp: number): Log => ({
    type: "info",
    message,
    timestamp,
    formattedTime: formatLogTime(timestamp),
  });
  export const success = (message: string, timestamp: number): Log => ({
    type: "success",
    message,
    timestamp,
    formattedTime: formatLogTime(timestamp),
  });
  export const error = (message: string, timestamp: number): Log => ({
    type: "error",
    message,
    timestamp,
    formattedTime: formatLogTime(timestamp),
  });
}

const { t } = useI18n();
const model = ref<ProjectModel | null>(null);
const stateStore = useStateStore();
const route = useRoute();
const router = useRouter();
const reinitProgressLoading = ref<ReturnType<typeof ElLoading.service> | null>(
  null
);
const openProjectProgressLoading = ref<ReturnType<
  typeof ElLoading.service
> | null>(null);
const currentFile = ref<string | null>(null);
const hotKeys = reactive<HotKeys>({
  recorder: "F9",
  runScript: "F10",
  runProject: "F11",
  stopAll: "F12",
});
const hiders = reactive<Hiders>({
  recorder: false,
  runProject: false,
  runScript: false,
});
const logScrollbarRef = ref<InstanceType<typeof ElScrollbar> | null>(null);
const logs = ref<Stack<Log>>(new Stack(100));

const openMonitor = async () => {
  const monitor = new WebviewWindow("monitor", {
    url: "/monitor",
    title: t("Monitor"),
    width: 800,
    height: 600,
    minWidth: 800,
    minHeight: 600,
  });
  monitor.once("tauri://created", async () => {});
  monitor.once("tauri://error", async () => {
    const windows = await getAllWindows();
    let monitorExists = false;
    for (const window of windows) {
      if (window.label == "monitor") {
        monitorExists = true;
        window.unminimize().then(() => {
          return window.setFocus();
        });
      }
    }
    if (!monitorExists) {
      msgError(t("Open monitor failed."));
    }
  });
};

const openProjectLoadingBegin = () => {
  openProjectProgressLoading.value = ElLoading.service({
    lock: true,
    text: t("The project is opening, please wait."),
    background: "rgba(0, 0, 0, 0.7)",
  });
};

const openProjectLoadingFinished = () => {
  openProjectProgressLoading.value?.close();
  openProjectProgressLoading.value = null;
};

const openProject = async (path: string) => {
  reinitProgressLoading.value = null;
  try {
    let status = await ProjectModel.verify(path);
    switch (status) {
      case "valid":
        break;
      case "moved":
        stopOpenProjectProgressLoading();
        reinitProgressLoading.value = ElLoading.service({
          lock: true,
          text: t(
            "The project has been moved, and is now being reinitialized. Please wait."
          ),
          background: "rgba(0, 0, 0, 0.7)",
        });
        await ProjectModel.reinit(path);
        break;
      default:
        throw new Error(t("This is not a valid Kiwi project."));
    }
    const project = await ProjectModel.open(path); //如果出错最晚在这一步,不会污染 stateStore.project
    stateStore.project = project;
    stateStore.project.mainFileFullPath = await join(
      stateStore.project.path as string,
      stateStore.project.mainFile as string
    );
    model.value = new ProjectModel(stateStore.project);
    currentFile.value = stateStore.project.mainFile;
  } catch (e: unknown) {
    msgError(e);
    router.push({
      path: "/app/home",
    });
  }
};

const stopOpenProjectProgressLoading = () => {
  openProjectProgressLoading.value?.close();
  openProjectProgressLoading.value = null;
};

const initHotKeys = async () => {
  if ((await platform()) == "windows") {
    hotKeys.recorder = "Ctrl+F9";
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
  await safeRegisterHotkey(hotKeys.recorder, async (event) => {
    if (event.state === "Released") await record();
  });
  await safeRegisterHotkey(hotKeys.runScript, async (event) => {
    if (event.state === "Released") await runScript();
  });
  await safeRegisterHotkey(hotKeys.runProject, async (event) => {
    if (event.state === "Released") await runProject();
  });
  await safeRegisterHotkey(hotKeys.stopAll, async (event) => {
    if (event.state === "Released") await stopAll();
  });
};

const unregisterHotkeys = async () => {
  await safeUnregisterHotkey(hotKeys.recorder);
  await safeUnregisterHotkey(hotKeys.runScript);
  await safeUnregisterHotkey(hotKeys.runProject);
  await safeUnregisterHotkey(hotKeys.stopAll);
};

const record = async () => {
  const action = async () => {
    try {
      await model.value?.runRecorder();
    } catch (e: unknown) {
      msgError(e);
    }
  };
  if (hiders.recorder) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const runScript = async () => {
  const action = async () => {
    try {
      if (!currentFile.value) return;
      const scriptAbsolutePath = await join(
        route.query.path as string,
        currentFile.value
      );
      await model.value?.runScript(scriptAbsolutePath);
    } catch (e: unknown) {
      msgError(e);
    }
  };
  if (hiders.runScript) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const runProject = async () => {
  const action = async () => {
    try {
      const path = stateStore.project.mainFileFullPath as string;
      console.log(stateStore.project);
      await model.value?.runScript(path);
    } catch (e: unknown) {
      msgError(e);
    }
  };
  if (hiders.runProject) {
    minimizeAll().then(async () => {
      await action();
    });
    return;
  }
  await action();
};

const stopAll = async () => {
  try {
    await model.value?.stopAll();
  } catch (e: unknown) {
    msgError(e);
  }
};

const logScrollToBottom = () => {
  nextTick(() => {
    if (!logScrollbarRef.value) return;
    setTimeout(() => {
      const wrap = logScrollbarRef.value?.wrapRef;
      if (wrap) {
        wrap.scrollTop = wrap.scrollHeight;
      }
    }, 10);
  });
  1;
};

const clearLog = () => {
  logs.value.clear();
};

const openInEditor = async () => {
  try {
    await model.value?.openInEditor();
  } catch (e: unknown) {
    msgError(e);
  }
};

const reveal = async () => {
  try {
    await model.value?.reveal();
  } catch (e: unknown) {
    msgError(e);
  }
};

listen("msg:error", () => {
  reinitProgressLoading.value?.close();
});

listen<EmitProgress>("progress:reinit_project", async (event) => {
  if (event.payload.percentage == 100) {
    reinitProgressLoading.value?.close();
    msgSuccess(t("The project has been reinitialized successfully."));
  }
});

listen<string>("update:record_file", async (event) => {
  if (event.payload.length > 0) {
    currentFile.value = event.payload;
  }
});

listen<string>("run:status", async (event) => {
  if (event.payload == "running") {
    if (hiders.runProject || hiders.runScript) {
      await minimizeAll();
    }
  }
  if (event.payload == "stopped") {
    if (hiders.recorder || hiders.runProject || hiders.runScript) {
      await unminimizeAll();
    }
  }
});

listen<EmitLog>("log:info", (event) => {
  const log = Log.info(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitLog>("log:success", (event) => {
  const log = Log.success(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitLog>("log:error", (event) => {
  const log = Log.error(event.payload.data, event.payload.time);
  logs.value.push(log);
  logScrollToBottom();
});

listen<EmitProject>("backend:update:project", async (event) => {
  stateStore.project = event.payload;
  stateStore.project.mainFileFullPath = await join(
    stateStore.project.path as string,
    stateStore.project.mainFile as string
  );
  model.value = new ProjectModel(stateStore.project);
});

onMounted(async () => {
  openProjectLoadingBegin();
  await openProject(route.query.path as string);
  await initHotKeys();
  await delay(100);
  await registerHotkeys();
  openProjectLoadingFinished();
});

onUnmounted(async () => {
  await unregisterHotkeys();
});
</script>
<template>
  <el-container>
    <el-header class="page-header">
      <el-row :gutter="0">
        <el-col :span="8" class="left">
          <router-link to="/">
            <el-icon :size="20" color="#fff"><ArrowLeft /></el-icon>
          </router-link>
        </el-col>
        <el-col :span="8" class="title">{{ stateStore.project.name }}</el-col>
        <el-col :span="8" class="right">
          <el-icon class="monitor" :size="20" color="#fff" @click="openMonitor"
            ><Monitor
          /></el-icon>
        </el-col>
      </el-row>
    </el-header>
    <el-container class="main">
      <el-aside width="20vw" class="actions">
        <el-row :gutter="0">
          <el-col :span="24">
            <div class="run">
              <el-tooltip
                class="box-item"
                effect="dark"
                :content="t('Hide While Running')"
                placement="right-start"
              >
                <el-checkbox v-model="hiders.runProject" size="large"
              /></el-tooltip>
              <el-button type="success" size="large" @click="runProject">
                {{ t("Run Project") }} ({{ hotKeys.runProject }})
              </el-button>
            </div>
          </el-col>
        </el-row>
        <el-row :gutter="0">
          <el-col :span="24">
            <el-button type="danger" size="large" @click="stopAll">
              {{ t("Stop All") }} ({{ hotKeys.stopAll }})
            </el-button>
          </el-col>
        </el-row>
        <el-divider direction="horizontal" class="divider"></el-divider>
        <el-row :gutter="0">
          <el-col :span="24">
            <div class="run">
              <el-tooltip
                class="box-item"
                effect="dark"
                :content="t('Hide While Running')"
                placement="right-start"
              >
                <el-checkbox v-model="hiders.recorder" size="large" />
              </el-tooltip>
              <el-button
                type="primary"
                size="large"
                :plain="true"
                @click="record"
              >
                {{ t("Record") }} ({{ hotKeys.recorder }})
              </el-button>
            </div>
          </el-col>
        </el-row>
        <el-divider direction="horizontal" class="divider"></el-divider>
        <el-row :gutter="0">
          <el-col :span="24">
            <el-button type="info" size="large" @click="reveal" :plain="true">
              {{ t("Open") }}
            </el-button>
          </el-col>
        </el-row>
        <el-row :gutter="0">
          <el-col :span="24">
            <el-tooltip
              effect="dark"
              :content="
                t(
                  'If “Edit” doesn’t work, install VS Code or set “edit_command” manually in config.toml.'
                )
              "
              placement="bottom"
            >
              <el-button
                type="info"
                size="large"
                @click="openInEditor"
                :plain="true"
              >
                {{ t("Edit") }}
              </el-button>
            </el-tooltip>
          </el-col>
        </el-row>
      </el-aside>
      <el-container>
        <el-main>
          <div class="actions">
            <el-tooltip
              effect="dark"
              :content="t('Relative Path of Current File')"
              placement="bottom"
            >
              <el-input
                size="default"
                placeholder="main.py"
                v-model="currentFile"
              >
              </el-input>
            </el-tooltip>
            <div class="run">
              <el-tooltip
                class="box-item"
                effect="dark"
                :content="t('Hide While Running')"
                placement="right-start"
              >
                <el-checkbox v-model="hiders.runScript" size="large"
              /></el-tooltip>
              <el-button
                type="success"
                size="default"
                @click="runScript"
                :plain="true"
                >{{ t("Run File") }} ({{ hotKeys.runScript }})</el-button
              >
            </div>
          </div>
          <div class="log-content">
            <div class="clear">
              <el-button
                type="warning"
                size="small"
                @click="clearLog"
                :plain="true"
                >{{ t("Clear Log") }}</el-button
              >
            </div>
            <el-tabs type="border-card">
              <el-tab-pane label="Log">
                <el-scrollbar class="logs" ref="logScrollbarRef">
                  <ul>
                    <li class="log" :class="log.type" v-for="log in logs.stack">
                      <span class="time">{{ log.formattedTime }}</span>
                      <span class="data">{{ log.message }}</span>
                    </li>
                  </ul>
                </el-scrollbar>
              </el-tab-pane>
            </el-tabs>
          </div>
        </el-main>
      </el-container>
    </el-container>
  </el-container>
</template>
<style scoped>
.page-header {
  .monitor {
    cursor: pointer;
    :hover {
      color: gold;
    }
  }
}
.main {
  margin: 10px 10px 0px 10px;
  .run {
    position: relative;
    .el-button {
      position: relative;
      z-index: 1;
    }
    .el-checkbox {
      position: absolute;
      right: 0;
      bottom: 0;
      z-index: 2;
      height: 14px;
    }
  }
  .actions {
    .el-row {
      .el-button {
        width: 100%;
      }
      margin-bottom: 10px;
    }
    .divider {
      margin-top: 10px;
      margin-bottom: 10px;
    }
  }
  .el-main {
    padding: 0;
    margin-left: 10px;
    .actions {
      display: flex;
      .run {
        .el-checkbox {
          right: 2px;
        }
        margin-left: 10px;
      }
      margin-bottom: 10px;
    }
    .log-content {
      position: relative;
      .el-tabs {
        position: relative;
        z-index: 1;
      }
      .clear {
        right: 8px;
        top: 8px;
        position: absolute;
        z-index: 2;
        cursor: pointer;
        :hover {
          color: black;
        }
      }
      :deep(.el-tabs__content) {
        padding: 0px;
      }
      .logs {
        overflow: hidden;
        line-height: 20px;
        color: #999;
        font-size: 13px;
        overflow-y: auto;
        height: calc(100vh - 140px);
        ul {
          list-style: none;
          padding: 0px;
          margin: 10px;
          .log {
            .time {
              display: inline-block;
              min-width: 150px;
            }
          }
          .log.error {
            color: red;
          }
          .log.success {
            color: green;
          }
        }
      }
    }
  }
}
</style>
