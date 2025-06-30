import { invoke } from "@tauri-apps/api/core";
import { Base64Png, f64, Point, Size, WeightPoint } from "@utils/common";
import { msgError } from "@utils/msg";

class Frame {
  async findImage(
    origin: Base64Png,
    template: Base64Png,
    startPoint: Point,
    endPoint: Point,
    threshold: f64
  ): Promise<WeightPoint | null> {
    try {
      return await invoke("find_image", {
        origin,
        template,
        startPoint,
        endPoint,
        threshold,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async findImages(
    origin: Base64Png,
    template: Base64Png,
    templateSize: Size,
    startPoint: Point,
    endPoint: Point,
    threshold: f64
  ): Promise<WeightPoint[]> {
    try {
      return await invoke("find_images", {
        origin,
        template,
        templateSize,
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

export { Frame };
