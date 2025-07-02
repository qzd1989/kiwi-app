<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { FormRules } from "element-plus";
import { useStateStore } from "@utils/store";
import { delay } from "@utils/common";
import { msgError, msgInfo, msgSuccess } from "@utils/msg";
import { AppModel, commonModel } from "@kiwi";
import { Locale, locales } from "@types";
import { useI18n } from "vue-i18n";

interface Form {
  originalWebsocketPort: number;
  websocketPort: number;
  locale: Locale;
}

const { t, locale } = useI18n();
const stateStore = useStateStore();
const shouldShowSaveSuccess = ref<boolean>(true);
const form = reactive<Form>({
  originalWebsocketPort: 0,
  websocketPort: 0,
  locale: "en-US",
});
const rules = reactive<FormRules<Form>>({
  websocketPort: [
    {
      required: true,
      message: t("WebSocket port is required."),
      trigger: "blur",
    },
  ],
  locale: [
    {
      required: true,
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
    const model: AppModel = new AppModel(stateStore.app);
    if (!model.config) {
      throw new Error(t("App configuration not found."));
    }
    model.config.app.websocket_port = form.websocketPort;
    model.config.app.locale = form.locale;
    await model.save();
    if (shouldShowSaveSuccess.value) {
      msgSuccess(t("Settings saved successfully."));
    }
  } catch (e) {
    msgError(e);
  }
  stateStore.app.config!.app.websocket_port = form.websocketPort;
  stateStore.app.config!.app.locale = form.locale;
  await changeLocale();
};

const changeLocale = async () => {
  if (!stateStore.app.config) return;
  locale.value = form.locale;
  stateStore.app.config.app.locale = form.locale;
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
      t("WebSocket started successfully.", { port: form.websocketPort })
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
        const infoMsg = t(
          "WebSocket failed to start, revert to original port.",
          {
            port: form.websocketPort,
            originalPort: form.originalWebsocketPort,
          }
        );
        msgInfo(infoMsg);
        form.websocketPort = form.originalWebsocketPort;
        stateStore.enable.isWebsocketAlive = true;
        shouldShowSaveSuccess.value = false;
      } catch (rollbackError) {
        stateStore.enable.isWebsocketAlive = false;
        const errMsg = t(
          "WebSocket failed to start on both port and original port.",
          {
            port: form.websocketPort,
            originalPort: form.originalWebsocketPort,
          }
        );
        msgError(errMsg);
        throw new Error(errMsg);
      }
    } else {
      stateStore.enable.isWebsocketAlive = false;
      const errMsg = t(
        "WebSocket failed to start on both port and original port.",
        {
          port: form.websocketPort,
          originalPort: form.originalWebsocketPort,
        }
      );
      msgError(errMsg);
      throw new Error(errMsg);
    }
  }
};

onMounted(async () => {
  form.websocketPort = form.originalWebsocketPort =
    stateStore.app.config!.app.websocket_port;
  form.locale = stateStore.app.config!.app.locale;
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
        <el-col :span="8" class="title">{{ t("Setting") }}</el-col>
        <el-col :span="8" class="right"></el-col>
      </el-row>
    </el-header>
    <el-main>
      <el-form ref="formRef" :model="form" :rules="rules" label-position="top">
        <el-form-item :label="t('WebSocket Port')" prop="websocketPort">
          <el-input-number
            :min="2"
            :max="65534"
            v-model="form.websocketPort"
            :step="1"
            :controls="false"
            style="width: 100%"
          ></el-input-number>
        </el-form-item>
        <el-form-item :label="t('Language')" prop="locale">
          <el-select v-model="form.locale">
            <el-option
              v-for="appLocale in locales"
              :key="appLocale.key"
              :label="appLocale.name"
              :value="appLocale.key"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="" prop="">
          <el-button type="primary" class="save" @click="save">
            {{ t("Save") }}
          </el-button>
        </el-form-item>
      </el-form>
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
