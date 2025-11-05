<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { Project } from "@api";
import { msgError } from "@utils";
import { useAppStore } from "@store";
import CreateModal from "@views/main/project/CreateModal.vue";
import { Server } from "@api/Server";
import { delay } from "@utils/common";
import { App } from "@api";

const appStore = useAppStore();
const router = useRouter();
const showCreateModal = ref(false);

const closeCreateModal = () => {
  showCreateModal.value = false;
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

// test code
// const openAnyServer = async () => {
//   try {
//     await Server.shutdown();
//     await delay(200);
//     await Server.startAny();
//     await Server.setRemoteAddress(await Server.getLanAddress());
//     appStore.remoteServerAddress = await Server.getRemoteAddress();
//   } catch (e) {
//     appStore.remoteServerAddress = null;
//     msgError(e);
//   }
// };
// test code end

onMounted(async () => {
  await openLocalServer();
  // test code
  // await openAnyServer();
  // await App.toListener(); //模拟listener用户，这样websocket::find_image/find_images就会去resource_dir/.cache/data/templates目录里找模板
  // setTimeout(async () => {
  //   try {
  //     const project = await Project.open("/Users/kiwi/Desktop/god");
  //     appStore.project = project;
  //     router.push({
  //       path: "/project",
  //       query: { path: "/Users/kiwi/Desktop/god" },
  //     });
  //   } catch (e) {
  //     msgError(e);
  //   }
  // }, 100);
  // test code end
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
      <div class="text-4xl font-extrabold text-gray-700">Kiwi</div>
      <div class="text-gray-500">
        Simplifying your tasks, one click at a time.
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
          Start Listening
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button
          type="primary"
          @click="showCreateModal = true"
          size="large"
          class="w-full"
        >
          Create Project
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button
          type="primary"
          @click="selectProject"
          size="large"
          class="w-full"
        >
          Open Project
        </el-button>
      </li>
      <li class="min-w-md">
        <el-button type="info" plain @click="" size="large" class="w-full">
          Official Website
        </el-button>
      </li>
      <li class="hidden min-w-md">
        <el-button type="info" plain @click="" size="large" class="w-full">
          ClearLocalStore
        </el-button>
      </li>
    </ul>

    <div class="mt-4 text-sm text-gray-400 underline">Version: 1.0.0</div>
    <div class="absolute top-2 right-2 text-2xl">
      <el-icon
        class="transform cursor-pointer transition duration-300 hover:rotate-45"
      >
        <Setting class="text-gray-400 hover:text-blue-500" />
      </el-icon>
    </div>
  </div>
  <CreateModal v-if="showCreateModal" :close="closeCreateModal" />
</template>
<style scoped></style>
