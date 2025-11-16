<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { FormInstance, FormRules } from "element-plus";
import { Server } from "@api/Server";
import { useAppStore, useLocalStore } from "@store";
import { msgError, msgSuccess } from "@utils";
import { Delete } from "@element-plus/icons-vue";
import { useI18n } from "vue-i18n";

type Form = {
  address: string;
};
const { t } = useI18n();
const appStore = useAppStore();
const localStore = useLocalStore();
const props = defineProps(["close", "capture"]);
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  address: ""
});
const rules = reactive<FormRules<Form>>({
  address: [
    {
      pattern:
        /^(?:(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)\.){3}(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d):(?:\d{1,4}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])$/,
      message: t("Invalid Address format."),
      trigger: "blur"
    }
  ]
});
const visible = ref<boolean>(true);
const recentAddresses = ref<string[]>([]);

const connect = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
    const isAlive = await Server.isRemoteAlive(form.address);
    if (isAlive) {
      await Server.setRemoteAddress(form.address);
      appStore.remoteServerAddress = form.address;
      msgSuccess(t("Server is alive.", { address: form.address }));
      await addAddress(form.address);
      await props.capture();
      await props.close();
    } else {
      msgError(t("Remote Server is not alive."));
    }
  } catch (e) {
    msgError(e);
    return;
  }
};

const loadServerAddress = async () => {
  form.address = await Server.getRemoteAddress();
};

const setToLocal = async () => {
  form.address = await Server.getLocalAddress();
  connect(formRef.value);
};

const setAddress = async (address: string) => {
  form.address = address;
};

const initAddresses = async () => {
  recentAddresses.value = (await localStore.get("recentAddresses")) ?? [];
};

const deleteAddress = async (address: string) => {
  // 过滤掉要删除的地址
  recentAddresses.value = recentAddresses.value.filter((a) => a !== address);

  // 保存到 localStore
  await localStore.set("recentAddresses", recentAddresses.value);
};

const addAddress = async (address: string) => {
  if (!address) return;

  // 如果数组还没初始化或 undefined，先赋空数组
  if (!recentAddresses.value) recentAddresses.value = [];

  // 避免重复添加
  if (!recentAddresses.value.includes(address)) {
    recentAddresses.value.unshift(address); // 插入到最前面
  }

  // 可限制数组长度，例如保留最近 20 条
  recentAddresses.value = recentAddresses.value.slice(0, 20);

  // 保存到 localStore
  await localStore.set("recentAddresses", recentAddresses.value);
};

onMounted(async () => {
  await loadServerAddress();
  await initAddresses();
});

onUnmounted(async () => {});
</script>
<template>
  <el-dialog
    v-model="visible"
    width="70vw"
    :show-close="false"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :align-center="true"
  >
    <template #header>{{ t("Connect to Remote Server") }}</template>
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item
        :label="t('Address')"
        prop="address"
        :required="true"
        label-position="top"
      >
        <el-input
          v-model="form.address"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
        ></el-input>
      </el-form-item>
      <div class="flex flex-wrap gap-1">
        <div v-for="address in recentAddresses" class="flex">
          <el-button-group>
            <el-button
              type="info"
              size="small"
              plain
              @click="setAddress(address)"
            >
              {{ address }}
            </el-button>
            <el-button
              type="info"
              size="small"
              plain
              :icon="Delete"
              @click="deleteAddress(address)"
            />
          </el-button-group>
        </div>
      </div>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="info" plain @click="props.close()">
          {{ t("Close") }}
        </el-button>
        <el-button type="primary" @click="setToLocal()">
          {{ t("Use Local Server") }}
        </el-button>
        <el-button type="primary" @click="connect(formRef)">
          {{ t("Connect") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped></style>
