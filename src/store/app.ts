import { defineStore } from "pinia";
import {  Project } from "@api";

export const useAppStore = defineStore("app", {
  state: () => ({
    window: {
      width: 0,
      height: 0,
      scaleFactor: 0,
    },
    project: null as Project | null,
    remoteServerAddress: null as string | null,
  }),
  actions: {
    initProject() {
      if (!this.project) return;
      this.project = new Project(
        this.project.name,
        this.project.version,
        this.project.description,
        this.project.path,
      );
    },
  },
  persist: {
    storage: sessionStorage,
    //页面在刷新的情况下，会自动执行这个方法
    afterHydrate: (context) => {
      context.store.project = new Project(
        context.store.project.name,
        context.store.project.version,
        context.store.project.description,
        context.store.project.path,
      );
    },
  },
});
