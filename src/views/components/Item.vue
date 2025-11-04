<script setup lang="ts">
import { useSlots } from "vue";
const slots = useSlots();
</script>
<template>
  <div class="item my-4 rounded-xl bg-gray-100 p-4 shadow-md">
    <div v-if="slots.title" class="title mb-4 flex justify-between">
      <span class="text-gray-600"><slot name="title"></slot></span>
      <slot name="extra"></slot>
    </div>
    <slot name="content"></slot>
  </div>
</template>
<style scoped></style>
