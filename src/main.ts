import { createApp } from "vue";
import Main from "@views/Main.vue";
import Home from "@views/main/Home.vue";
import Setting from "@views/main/Setting.vue";
import ProjectCreate from "@views/main/project/Create.vue";
import ProjectDetail from "@views/main/project/Detail.vue";

import Monitor from "@views/Monitor.vue";
import "./css/default.css";
const app = createApp(Main);

// element plust
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
app.use(ElementPlus);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

// router
import { createRouter, createWebHistory } from "vue-router";
const routes = [
  { path: "/", redirect: "/main/home" },
  { path: "/main/home", component: Home },
  { path: "/main/setting", component: Setting },
  { path: "/main/project/create", component: ProjectCreate },
  { path: "/main/project/detail", component: ProjectDetail },
  { path: "/monitor", component: Monitor },
];
const router = createRouter({
  history: createWebHistory(),
  routes,
});
app.use(router);

// persist pinia
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
app.use(pinia);

app.mount("#app");
