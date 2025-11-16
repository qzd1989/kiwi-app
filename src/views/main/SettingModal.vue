<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, reactive } from "vue";
import { useI18n } from "vue-i18n";
import { Locale } from "@api";
import { msgSuccess } from "@utils";
const { t, locale } = useI18n();
const props = defineProps(["close"]);
const visible = ref(true);
const title = computed(() => {
  return t("Settings");
});

const form = reactive({
  locale: "en-US",
});

const close = () => {
  props.close();
};

const confirm = () => {
  Locale.set(form.locale);
  locale.value = form.locale;
  msgSuccess(t("Setting changed successfully!"));
  close();
};

onMounted(async () => {
  form.locale = await Locale.get();
});
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
      <el-form ref="formRef" :model="form">
        <el-form-item :label="$t('Language')" prop="locale">
          <el-select v-model="form.locale">
            <el-option
              v-for="item in Locale.all()"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            ></el-option>
          </el-select>
        </el-form-item>
      </el-form>
    </el-scrollbar>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="close">
          {{ t("Close") }}
        </el-button>
        <el-button type="primary" @click="confirm">
          {{ t("Confirm") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped></style>
