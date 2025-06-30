import { invoke } from "@tauri-apps/api/core";
import { ColoredPoint, f64, HexColor, Point, RgbColor } from "@utils/common";
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

  async generateFindRelativeColorsCode(
    vertexHex: HexColor,
    relativePoints: ColoredPoint[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: RgbColor
  ): Promise<string> {
    try {
      return await invoke("generate_find_relative_colors_code", {
        vertexHex,
        relativePoints,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async generateFindColorsCode(
    hexColors: HexColor[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: RgbColor
  ): Promise<string> {
    try {
      return await invoke("generate_find_colors_code", {
        hexColors,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }

  async generateRecognizeTextCode(
    startPoint: Point,
    endPoint: Point
  ): Promise<string> {
    try {
      return await invoke("generate_recognize_text_code", {
        startPoint,
        endPoint,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

export { Code };
