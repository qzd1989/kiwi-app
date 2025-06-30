<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { FormRules } from "element-plus";
import { useStateStore } from "@utils/store";
import { delay } from "@utils/common";
import { msgError, msgInfo, msgSuccess } from "@utils/msg";
import { AppModel, commonModel } from "@kiwi";

interface Form {
  originalWebsocketPort: number;
  websocketPort: number;
}

const stateStore = useStateStore();
const model = ref<AppModel | null>(null);
const shouldShowSaveSuccess = ref<boolean>(true);
const form = reactive<Form>({
  originalWebsocketPort: 0,
  websocketPort: 0,
});
const rules = reactive<FormRules<Form>>({
  websocketPort: [
    {
      required: true,
      message: "Websocket port is required.",
      trigger: "blur",
    },
  ],
});

const save = async () => {
  shouldShowSaveSuccess.value = true;
  try {
    await runWebsocket();
  } catch (e) {
    return;
  }
  try {
    await model.value?.save();
    if (shouldShowSaveSuccess.value) {
      msgSuccess("Settings saved successfully.");
    }
  } catch (e) {
    msgError(e);
  }
  stateStore.app.config!.app.websocket_port = form.websocketPort;
};

const runWebsocket = async () => {
  const isOriginalPortAlive = await commonModel.isWebsocketAlive(
    form.originalWebsocketPort
  );
  const portChanged = form.originalWebsocketPort !== form.websocketPort;
  if (!portChanged) return;
  stateStore.enable.isWebsocketAlive = false;
  try {
    await commonModel.shutdownWebsocket();
    await delay(200);
    await commonModel.openWebsocket(form.websocketPort);
    msgSuccess(
      `WebSocket started successfully on port: ${form.websocketPort}.`
    );
    form.originalWebsocketPort = form.websocketPort;
    stateStore.enable.isWebsocketAlive = true;
  } catch (e: unknown) {
    if (isOriginalPortAlive) {
      try {
        await delay(200);
        commonModel.shutdownWebsocket();
        await delay(200);
        commonModel.openWebsocket(form.originalWebsocketPort);
        stateStore.enable.isWebsocketAlive = true;
        const infoMsg = `WebSocket failed to start on port: ${form.websocketPort}. Reverted to the original port: ${form.originalWebsocketPort}, which started successfully.`;
        msgInfo(infoMsg);
        form.websocketPort = form.originalWebsocketPort;
        stateStore.enable.isWebsocketAlive = true;
        shouldShowSaveSuccess.value = false;
      } catch (rollbackError) {
        stateStore.enable.isWebsocketAlive = false;
        const errMsg = `WebSocket failed to start. Both the original port: ${form.originalWebsocketPort} and the new port: ${form.websocketPort} are unavailable.`;
        msgError(errMsg);
        throw new Error(errMsg);
      }
    } else {
      stateStore.enable.isWebsocketAlive = false;
      const errMsg = `WebSocket failed to start on port: ${form.websocketPort}, and original port: ${form.originalWebsocketPort} is also unavailable.`;
      msgError(errMsg);
      throw new Error(errMsg);
    }
  }
};

onMounted(async () => {
  model.value = new AppModel(stateStore.app);
  form.websocketPort = form.originalWebsocketPort =
    model.value.config!.app.websocket_port;
});
onUnmounted(async () => {});
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
        <el-col :span="8" class="title">Setting</el-col>
        <el-col :span="8" class="right"></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" label-position="top">
        <el-form-item label="Websocket Port" prop="websocketPort">
          <el-input-number
            :min="2"
            :max="65534"
            v-model="form.websocketPort"
            :step="1"
            :controls="false"
          ></el-input-number>
        </el-form-item>
        <el-form-item label="" prop="">
          <el-button type="primary" class="save" @click="save">
            Save
          </el-button>
        </el-form-item></el-form
      >
    </el-main>
  </el-container>
</template>
<style scoped>
.el-container {
  .el-main {
    display: flex;
    justify-content: center;
    .el-form {
      margin-top: 20px;
      width: 80vw;
      .tip {
        list-style: none;
        margin: 0px;
        padding: 0px;
        color: gray;
        line-height: 15px;
        li {
          margin-top: 5px;
          .success {
            color: green;
          }
        }
      }
      .save {
        width: 100%;
        margin-top: 10px;
      }
    }
  }
}
</style>
