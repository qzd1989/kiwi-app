import { invoke } from "@tauri-apps/api/core";
import { f64, Point } from "@utils/common";
import { msgError } from "@utils/msg";

class Code {
  async generateFindImageCode(
    subpath: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64
  ): Promise<string> {
    try {
      return await invoke("generate_find_image_code", {
        subpath,
        startPoint,
        endPoint,
        threshold,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async generateFindImagesCode(
    subpath: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64
  ): Promise<string> {
    try {
      return await invoke("generate_find_images_code", {
        subpath,
        startPoint,
        endPoint,
        threshold,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { Code };
