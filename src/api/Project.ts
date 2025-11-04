import { invoke } from "@tauri-apps/api/core";
class Project {
  constructor(
    public name: string,
    public version: string,
    public description: string,
    public path: string,
  ) {}

  static async create(
    name: string,
    language: string,
    path: string,
  ): Promise<void> {
    try {
      return await invoke("create_project", {
        name,
        language,
        path,
      });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async open(path: string): Promise<Project> {
    try {
      return await invoke("open_project", {
        path,
      });
    } catch (e: unknown) {
      throw e;
    }
  }

  async openFolder(): Promise<void> {
    try {
      return await invoke("open_project_folder");
    } catch (e: unknown) {
      throw e;
    }
  }

  async openInEditor(): Promise<void> {
    try {
      return await invoke("open_project_in_editor");
    } catch (e: unknown) {
      throw e;
    }
  }

  static async saveTemplate(name: string, template: string): Promise<void> {
    try {
      return await invoke("save_template", {
        name,
        template,
      });
    } catch (e) {
      throw e;
    }
  }

  static async runScript(file?: string) {
    try {
      if (!file) {
        file = await invoke("get_project_entry_file");
      }
      await invoke("run_script", {
        file,
      });
    } catch (e) {
      await Project.setPid(0);
      throw e;
    }
  }

  static async stopRunScript() {
    try {
      await invoke("stop_run_script");
    } catch (e) {
      throw e;
    }
  }

  static async entryFile(): Promise<string> {
    try {
      return await invoke("get_project_entry_file");
    } catch (e) {
      throw e;
    }
  }

  static async setPid(pid: number): Promise<void> {
    try {
      return await invoke("set_project_pid", { pid });
    } catch (e) {
      throw e;
    }
  }

  static async is_running(): Promise<number> {
    try {
      return await invoke("is_project_running");
    } catch (e) {
      throw e;
    }
  }
}

export { Project };
