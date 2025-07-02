<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { open } from "@tauri-apps/plugin-shell";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const props = defineProps(["release"]);
const visible = ref(true);
const title = computed(() => {
  return t("Discovered new version.", { version: props.release.version });
});

const close = () => {
  visible.value = false;
};

const download = async () => {
  const url = props.release.url;
  if (!url) {
    return;
  }
  await open(url);
};

onMounted(async () => {});

onUnmounted(async () => {});
</script>
<template>
  <el-dialog
    :title="title"
    width="50vw"
    v-model="visible"
    :show-close="false"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :align-center="true"
  >
    <el-scrollbar max-height="50vh">
      <ul class="notes">
        <li v-for="note in props.release.notes">{{ note }}</li>
      </ul>
    </el-scrollbar>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="close" v-if="!props.release.force_update">
          {{ t("Close") }}
        </el-button>
        <el-button type="primary" @click="download">
          {{ t("Download Now") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped>
.notes {
  list-style: none;
  padding: 0;
  margin: 0;
  padding: 0px 10px;
  li {
    line-height: 20px;
  }
}
</style>
