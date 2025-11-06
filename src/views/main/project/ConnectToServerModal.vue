<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { FormInstance, FormRules } from "element-plus";
import { Server } from "@api/Server";
import { useAppStore } from "@store";
import { msgError, msgSuccess } from "@utils";

type Form = {
  address: string;
};

const appStore = useAppStore();
const props = defineProps(["close", "capture"]);
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  address: "",
});
const rules = reactive<FormRules<Form>>({
  address: [
    {
      pattern:
        /^(?:(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)\.){3}(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d):(?:\d{1,4}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])$/,
      message: "Invalid Address format.",
      trigger: "blur",
    },
  ],
});
const visible = ref<boolean>(true);
const connect = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
    const isAlive = await Server.isRemoteAlive(form.address);
    if (isAlive) {
      await Server.setRemoteAddress(form.address);
      appStore.remoteServerAddress = form.address;
      msgSuccess(`Server ${form.address} is alive.`);
      await props.capture();
      await props.close();
    } else {
      msgError("Remote Server is not alive.");
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

const setToLan = async () => {
  form.address = await Server.getLanAddress();
  connect(formRef.value);
};

const setToHW = async () => {
  form.address = "192.168.5.100:9927";
  connect(formRef.value);
};

onMounted(async () => {
  await loadServerAddress();
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
    <template #header>Connect to Remote Server</template>
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item
        label="Address"
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
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="info" plain @click="props.close()">Close</el-button>
        <el-button type="primary" @click="setToLocal()">
          Use Local Server
        </el-button>
        <el-button type="primary" @click="setToHW()">
          Use 192.168.5.100 Server
        </el-button>
        <el-button type="primary" @click="setToLan()" v-show="false">
          Use Lan Server
        </el-button>
        <el-button type="primary" @click="connect(formRef)">Connect</el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped></style>
