import { invoke } from "@tauri-apps/api/core";
import { Base64Png, Language } from "@types";
import { msgError } from "@utils/msg";

type VerifyStatus = "valid" | "invalid" | "moved";

interface Project {
  name: string | null;
  language: Language;
  mainFile: string | null;
  path: string | null;
  kiwiVersion: string | null;
  mainFileFullPath?: string;
}

namespace Project {
  export const init = (): Project => ({
    name: null,
    language: "python",
    mainFile: null,
    path: null,
    kiwiVersion: null,
  });
}

class ProjectModel {
  constructor(private project: Project) {}

  get name() {
    return this.project.name;
  }

  get language() {
    return this.project.language;
  }

  get mainFile() {
    return this.project.mainFile;
  }

  get path() {
    return this.project.path;
  }

  get kiwiVersion() {
    return this.project.kiwiVersion;
  }

  get mainFileFullPath() {
    return this.project.mainFileFullPath;
  }

  private ensureExists() {
    if (!this.project.path) {
      const e = "Project path is required.";
      msgError(e);
      throw new Error(e);
    }
  }

  async save(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("save_project", {
        name: this.project.name,
        language: this.project.language,
        path: this.project.path,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async init(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("init_project", { path: this.project.path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  static async verify(path: string): Promise<VerifyStatus> {
    try {
      return await invoke("verify_project", { path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  static async open(path: string): Promise<Project> {
    try {
      return await invoke("open_project", { path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  static async reinit(path: string): Promise<void> {
    try {
      return await invoke("reinit_project", { path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async saveImage(name: string, data: Base64Png): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("save_image", {
        name,
        data,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async runRecorder(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("run_recorder");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async runScript(path: string): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("run_script", { path });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async stopAll(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("stop_all");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async openInEditor(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("open_project_in_editor");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async reveal(): Promise<void> {
    this.ensureExists();
    try {
      return await invoke("reveal_project_folder");
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { ProjectModel, Project };
export type { VerifyStatus };
