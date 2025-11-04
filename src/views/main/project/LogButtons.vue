<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
const propos = defineProps(["clear"]);
onMounted(async () => {});
onUnmounted(async () => {});
</script>
<template>
  <el-button type="warning" @click="propos.clear" plain size="small">
    clear
  </el-button>
</template>
<style scoped></style>
