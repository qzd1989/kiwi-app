<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { join, sep } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { FormInstance, FormRules } from "element-plus";
import { msgError } from "@utils";
import { Project } from "@api";
import { useRouter } from "vue-router";
import { useAppStore } from "@store";
import { documentDir } from "@tauri-apps/api/path";
import { useI18n } from "vue-i18n";

const appStore = useAppStore();
const router = useRouter();

type Form = {
  name: string;
  folder: string;
  rootDir: string;
  rootPath: string;
};

const { t } = useI18n();
const props = defineProps(["close"]);
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  name: "",
  folder: "",
  rootDir: "", //没有/结尾
  rootPath: "", //有/结尾
});
const rules = reactive<FormRules<Form>>({
  name: [
    {
      pattern: /^[\u4e00-\u9fa5_a-zA-Z0-9]+$/,
      message: t(
        "Can only contain Chinese characters, English letters, digits, and underscores.",
      ),
      trigger: "blur",
    },
  ],
  folder: [
    {
      pattern: /^[\u4e00-\u9fa5_a-zA-Z0-9]+$/,
      message: t(
        "Can only contain Chinese characters, English letters, digits, and underscores.",
      ),
      trigger: "blur",
    },
    {
      validator: (_rule, _value, callback) => {
        if (!form.rootPath || form.rootPath.trim() === "") {
          callback(
            new Error(
              t("Select the root path by clicking the icon on the right."),
            ),
          );
        } else {
          callback(); // 校验通过
        }
      },
      trigger: "blur",
    },
  ],
});
const fullPath = ref<string>("");
const visible = ref<boolean>(true);

const openSelector = async () => {
  try {
    const path = await open({
      directory: true,
      multiple: false,
      defaultPath: form.rootDir,
    });
    if (path) {
      form.rootDir = path;
      form.rootPath = path + (await sep());
      fullPath.value = await join(form.rootPath, form.folder);
    }
  } catch (e: unknown) {
    msgError(e);
  }
};

const save = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
    const path = await join(form.rootPath, form.folder);
    await Project.create(form.name, "python", path);
    const project = await Project.open(path);
    appStore.project = project;
    router.push({
      path: "/project",
      query: { path },
    });
  } catch (e) {
    msgError(e);
    return;
  }
  props.close();
};

onMounted(async () => {
  form.rootDir = await documentDir();
  form.rootPath = form.rootDir + (await sep());
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
    <template #header>{{ t("Create Project") }}</template>
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item
        :label="t('Project Name')"
        prop="name"
        :required="true"
        label-position="top"
      >
        <el-input
          v-model="form.name"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
        ></el-input>
      </el-form-item>
      <el-form-item
        :label="t('Project Folder Name')"
        prop="folder"
        :required="true"
        label-position="top"
      >
        <el-input
          v-model="form.folder"
          autocapitalize="off"
          autocorrect="off"
          spellcheck="false"
        >
          <template #prepend v-if="form.rootPath">{{ form.rootPath }}</template>
          <template #append>
            <el-button @click="openSelector">
              <el-icon><FolderOpened /></el-icon>
            </el-button>
          </template>
        </el-input>
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="info" plain @click="props.close()">
          {{ t("Close") }}
        </el-button>
        <el-button type="primary" @click="save(formRef)">
          {{ t("Create") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
<style scoped></style>
