import { invoke } from "@tauri-apps/api/core";
import {
  f64,
  hexColor,
  Point,
  RelativeColoredPoints,
  rgbColor,
  u32,
} from "@types";

class Code {
  static async generateFindImageCode(
    subpath: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64,
    minTemplateSide: u32,
  ): Promise<string> {
    try {
      return await invoke("generate_find_image_code", {
        subpath,
        startPoint,
        endPoint,
        threshold,
        minTemplateSide,
      });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async generateFindImagesCode(
    subpath: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64,
    minTemplateSide: u32,
  ): Promise<string> {
    try {
      return await invoke("generate_find_images_code", {
        subpath,
        startPoint,
        endPoint,
        threshold,
        minTemplateSide,
      });
    } catch (e) {
      throw e;
    }
  }

  static async generateFindRelativeColorsCode(
    points: RelativeColoredPoints,
    startPoint: Point,
    endPoint: Point,
    rgbOffset: rgbColor,
  ): Promise<string> {
    try {
      const serializedPoints = points.map((p) => ({
        colored_point: p.coloredPoint.clone(),
        relative_point: p.relativePoint.clone(),
      }));
      return await invoke("generate_find_relative_colors_code", {
        points: serializedPoints,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e) {
      throw e;
    }
  }

  static async generateFindColorsCode(
    hexColors: hexColor[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: rgbColor,
  ): Promise<string> {
    try {
      return await invoke("generate_find_colors_code", {
        hexColors,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e) {
      throw e;
    }
  }

  static async generateRecognizeTextCode(
    startPoint: Point,
    endPoint: Point,
  ): Promise<string> {
    try {
      return await invoke("generate_recognize_text_code", {
        startPoint,
        endPoint,
      });
    } catch (e) {
      throw e;
    }
  }
  static async generateMoveToAbsolutePositionCode(
    point: Point,
  ): Promise<string> {
    try {
      return await invoke("generate_move_absolute_code", {
        point,
      });
    } catch (e) {
      throw e;
    }
  }
}

export { Code };
