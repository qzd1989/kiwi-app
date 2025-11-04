<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { msgError } from "@utils";
import { delay, copyText, msgSuccess } from "@utils/common";
import { Server } from "@api/Server";
import { App, Capturer } from "@api";

const router = useRouter();
const address = ref<string>("");
const isConnected = ref<boolean>(false);
const intervalId = ref<number | undefined>(undefined);

const copy = async () => {
  try {
    await copyText(address.value);
    msgSuccess("Copy Successded.");
  } catch (e) {
    msgError(e);
  }
};

const openAnyServer = async () => {
  try {
    await Server.shutdown();
    await delay(200);
    await Server.startAny();
    address.value = await Server.getLanAddress();
    await App.toListener();
  } catch (e) {
    msgError(e);
  }
};

const isAlive = async () => {
  try {
    const result = await Server.isRemoteAlive(address.value);
    if (result) {
      isConnected.value = true;
    } else {
      isConnected.value = false;
    }
  } catch (e) {
    isConnected.value = false;
    msgError(e);
  }
};

onMounted(async () => {
  await openAnyServer();
  await Capturer.start();
  intervalId.value = window.setInterval(async () => {
    await isAlive();
  }, 200);
});

onUnmounted(async () => {
  await Capturer.stop();
  if (intervalId.value !== undefined) {
    clearInterval(intervalId.value);
    intervalId.value = undefined;
  }
});
</script>
<template>
  <div class="mx-auto flex h-screen w-1/2 items-center justify-center">
    <el-form>
      <div>
        <el-form-item
          label="Server Address"
          prop="address"
          label-position="top"
        >
          <el-input
            v-model="address"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          >
            <template #append>
              <div
                class="h-3 w-3 rounded-full bg-gray-400"
                v-show="!isConnected"
              ></div>
              <div
                class="h-3 w-3 rounded-full bg-green-400"
                v-show="isConnected"
              ></div>
            </template>
          </el-input>
        </el-form-item>
        <el-button type="info" plain @click="router.push({ path: '/home' })">
          Back
        </el-button>
        <el-button type="primary" @click="copy">Copy</el-button>
      </div>
    </el-form>
  </div>
</template>
<style scoped></style>
