<script setup lang="ts">
import { EmitProgress, EmitProject, Language } from "@types";
import { ref, onMounted, onUnmounted, reactive } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { join, sep } from "@tauri-apps/api/path";
import { localStore, useStateStore } from "@utils/store";
import { msgError, msgSuccess } from "@utils/msg";
import { FormInstance, ElLoading, FormRules } from "element-plus";
import { listen } from "@tauri-apps/api/event";
import { useRouter } from "vue-router";
import { ProjectModel } from "@kiwi";
import { useI18n } from "vue-i18n";

interface Form {
  name: string;
  language: Language;
  path: string;
  fullPath: string;
  rootDirectory: string;
}

const { t } = useI18n();
const stateStore = useStateStore();
const router = useRouter();
const formRef = ref<FormInstance>();
const form = reactive<Form>({
  name: "",
  language: "python",
  path: "",
  fullPath: "",
  rootDirectory: "",
});
const rules = reactive<FormRules<Form>>({
  name: [
    {
      required: true,
      message: t("Project name is required."),
      trigger: "blur",
    },
    {
      pattern: /^[\u4e00-\u9fa5_a-zA-Z0-9]+$/,
      message: t(
        "Can only contain Chinese characters, English letters, digits, and underscores."
      ),
      trigger: "blur",
    },
  ],
  path: [
    {
      required: true,
      message: t("Project path is required."),
      trigger: "blur",
    },
  ],
});
const loading = ref<ReturnType<typeof ElLoading.service> | null>(null);
const languages: Language[] = ["python"];

const openSelector = async () => {
  try {
    const projectRootDirectory = (await localStore.get(
      "projectRootDirectory"
    )) as string;
    const selectedPath = await open({
      directory: true,
      multiple: false,
      defaultPath: projectRootDirectory ?? undefined,
    });
    if (selectedPath) {
      form.rootDirectory = selectedPath + (await sep());
      await localStore.set("projectRootDirectory", selectedPath);
    }
  } catch (e: unknown) {
    msgError(e);
  }
};

const save = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  try {
    await formEl.validate();
  } catch (e) {
    return;
  }
  loading.value = null;
  try {
    loading.value = ElLoading.service({
      lock: true,
      text: t("Project is initializing, please wait."),
      background: "rgba(0, 0, 0, 0.7)",
    });
    const path = await join(form.rootDirectory, form.path);
    const name = form.name;
    const language = form.language;
    const project = EmitProject.empty();
    project.name = name;
    project.language = language;
    project.path = path;
    const model = new ProjectModel(project);
    await model.save();
    await model.init(); //如果出错最晚在这一步,不会污染 stateStore.project
    stateStore.project = project;
    form.fullPath = path;
  } catch (e: unknown) {
    msgError(e);
  }
};

listen("msg:error", () => {
  loading.value?.close();
});

listen<EmitProgress>("progress:init_project", async (event) => {
  if (event.payload.percentage == 100) {
    loading.value?.close();
    msgSuccess(t("Project created successfully."));
    router.push({
      path: "/main/project/detail",
      query: { path: form.fullPath },
    });
  }
});

onMounted(async () => {
  form.rootDirectory = (await localStore.get("projectRootDirectory")) as string;
});

onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-header class="page-header">
      <el-row :gutter="0">
        <el-col :span="8" class="left">
          <router-link to="/">
            <el-icon :size="20" color="#fff"><ArrowLeft /></el-icon>
          </router-link>
        </el-col>
        <el-col :span="8" class="title">{{ t("Create Project") }}</el-col>
        <el-col :span="8" class="right"></el-col>
      </el-row>
    </el-header>
    <el-main
      ><el-form ref="formRef" :model="form" :rules="rules" label-position="top">
        <el-form-item :label="t('Project Name')" prop="name">
          <el-input
            placeholder=""
            v-model="form.name"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          ></el-input>
        </el-form-item>
        <el-form-item :label="t('Programming Language')" prop="description">
          <el-select v-model="form.language">
            <el-option
              v-for="item in languages"
              :key="item"
              :label="item"
              :value="item"
            ></el-option>
          </el-select>
        </el-form-item>
        <el-form-item :label="t('Project Path')" prop="path" :required="true">
          <el-input
            v-model="form.path"
            :placeholder="t('Project Folder Name')"
            autocapitalize="off"
            autocorrect="off"
            spellcheck="false"
          >
            <template #prepend>{{ form.rootDirectory }}</template>
            <template #append>
              <el-button @click="openSelector">
                <el-icon><FolderOpened /></el-icon>
              </el-button>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" class="save" @click="save(formRef)">
            {{ t("Create") }}
          </el-button>
        </el-form-item>
      </el-form>
    </el-main>
  </el-container>
</template>
<style scoped>
.el-container {
  .el-main {
    display: flex;
    justify-content: center;
    .el-form {
      width: 80vw;
      .save {
        margin-top: 10px;
        width: 100%;
      }
    }
  }
}
</style>
