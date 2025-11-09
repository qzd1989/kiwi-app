import { createApp } from "vue";
import "./styles/common.css";
import App from "./App.vue";
const app = createApp(App);

// router
import { createRouter, createWebHistory } from "vue-router";
import Home from "./views/main/Home.vue";
import Project from "./views/main/Project.vue";
import Listening from "./views/main/Listening.vue";
const routes = [
  { path: "/", redirect: "/home" },
  { path: "/home", component: Home },
  { path: "/project", component: Project },
  { path: "/listening", component: Listening },
];
const router = createRouter({
  history: createWebHistory(),
  routes,
});
app.use(router);

// i18n
import { createI18n } from "vue-i18n";
import zhCN from "./locales/zh-CN.json";
import enUS from "./locales/en-US.json";
const messages = {
  "zh-CN": zhCN,
  "en-US": enUS,
};
const i18n = createI18n({
  legacy: false,
  locale: "en-US", // set locale
  fallbackLocale: "en-US", // set fallback locale
  messages,
  missingWarn: false, //临时关闭翻译缺失警告，test code
});
app.use(i18n);

// element plus
import ElementPlus from "element-plus";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import "element-plus/dist/index.css";
app.use(ElementPlus);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

// pinia
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
app.use(pinia);

// firebase
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
const firebaseConfig = {
  apiKey: "AIzaSyDR_xgBK2-HdpNAaF_92CxSwezZxAOUnNs",
  authDomain: "kiwi-app-7dbb6.firebaseapp.com",
  projectId: "kiwi-app-7dbb6",
  storageBucket: "kiwi-app-7dbb6.firebasestorage.app",
  messagingSenderId: "596016300611",
  appId: "1:596016300611:web:2f10ef1c8293d5995e4af6",
  measurementId: "G-WKPJWZ2EF2",
};
const firebaseApp = initializeApp(firebaseConfig);
const analytics = getAnalytics(firebaseApp);

app.mount("#app");

// disable right-click in production environment
if (!import.meta.env.DEV) {
  window.addEventListener("contextmenu", (e) => {
    e.preventDefault();
  });
}
