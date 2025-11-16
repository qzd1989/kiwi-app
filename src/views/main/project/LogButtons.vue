<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
const { t } = useI18n();
const propos = defineProps(["clear"]);
onMounted(async () => {});
onUnmounted(async () => {});
</script>
<template>
  <el-button type="warning" @click="propos.clear" plain size="small">
    {{ t("clear") }}
  </el-button>
</template>
<style scoped></style>
