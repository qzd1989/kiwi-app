// commands::frontend::project::save_project,
// commands::frontend::project::verify_project,
// commands::frontend::project::init_project,
// commands::frontend::project::reinit_project,
// commands::frontend::project::open_project,
// commands::frontend::project::get_project,
// commands::frontend::project::open_project_in_editor,
// commands::frontend::project::reveal_project_folder,
// commands::frontend::project::save_image,
// commands::frontend::project::get_image,
// commands::frontend::project::get_image_size,
// commands::frontend::project::run,
// commands::frontend::project::run_recorder,
// commands::frontend::project::stop_all,
// commands::frontend::code::generate_find_image_code,
// commands::frontend::code::generate_find_images_code,
// commands::frontend::code::generate_find_relative_colors_code,
// commands::frontend::code::generate_find_colors_code,
// commands::frontend::code::generate_recognize_text_code,

import { invoke } from "@tauri-apps/api/core";
import { Base64Png, Language } from "@utils/common";
import { msgError } from "@utils/msg";

type VerifyStatus = "valid" | "invalid" | "moved";

interface ProjectInfo {
  name: string | null;
  language: Language;
  mainFile: string | null;
  path: string | null;
  kiwiVersion: string | null;
  mainFileFullPath?: string;
}

class Project implements ProjectInfo {
  name: string | null;
  language: Language;
  mainFile: string | null;
  path: string | null;
  kiwiVersion: string | null;
  mainFileFullPath?: string | undefined;

  constructor() {
    this.name = null;
    this.language = "python";
    this.mainFile = null;
    this.path = null;
    this.kiwiVersion = null;
  }

  async saveImage(name: string, base64Png: Base64Png): Promise<void> {
    try {
      return await invoke("save_image", {
        name,
        data: base64Png,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { Project };
export type { VerifyStatus };
