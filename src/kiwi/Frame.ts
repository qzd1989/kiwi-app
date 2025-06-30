import { invoke } from "@tauri-apps/api/core";
import {
  Base64Png,
  ColoredPoint,
  f64,
  HexColor,
  Point,
  RgbColor,
  Size,
  WeightPoint,
} from "@types";
import { msgError } from "@utils/msg";

class FrameModel {
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

  async findRelativeColors(
    origin: Base64Png,
    vertexHex: HexColor,
    relativePoints: ColoredPoint[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: RgbColor
  ): Promise<ColoredPoint | null> {
    try {
      return await invoke("find_relative_colors", {
        origin,
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

  async findColors(
    origin: Base64Png,
    hexColors: HexColor[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: RgbColor
  ): Promise<ColoredPoint[]> {
    try {
      return await invoke("find_colors", {
        origin,
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

  async recognizeText(
    origin: Base64Png,
    startPoint: Point,
    endPoint: Point
  ): Promise<string> {
    try {
      return await invoke("recognize_text", {
        origin,
        startPoint,
        endPoint,
      });
    } catch (e: unknown) {
      msgError(e);
      throw e;
    }
  }
}

const frameModel = new FrameModel();
export { frameModel };
