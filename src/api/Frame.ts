import { invoke } from "@tauri-apps/api/core";
import {
  ColoredPoint,
  f64,
  hexColor,
  Point,
  RelativeColoredPoints,
  rgbColor,
  u32,
  weightPoint,
} from "@types";

class Frame {
  static async findImage(
    key: string,
    origin: string,
    template: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64,
    minTemplateSide: u32,
  ): Promise<weightPoint | null> {
    try {
      return await invoke("find_image", {
        key,
        origin,
        template,
        startPoint,
        endPoint,
        threshold,
        minTemplateSide,
      });
    } catch (e) {
      throw e;
    }
  }

  static async findImages(
    key: string,
    origin: string,
    template: string,
    startPoint: Point,
    endPoint: Point,
    threshold: f64,
    minTemplateSide: u32,
  ): Promise<weightPoint[]> {
    try {
      return await invoke("find_images", {
        key,
        origin,
        template,
        startPoint,
        endPoint,
        threshold,
        minTemplateSide,
      });
    } catch (e) {
      throw e;
    }
  }

  static async findRelativeColors(
    origin: string,
    points: RelativeColoredPoints,
    startPoint: Point,
    endPoint: Point,
    rgbOffset: rgbColor,
  ): Promise<ColoredPoint | null> {
    try {
      const serializedPoints = points.map((p) => ({
        colored_point: p.coloredPoint.clone(),
        relative_point: p.relativePoint.clone(),
      }));
      return await invoke("find_relative_colors", {
        origin,
        points: serializedPoints,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e: unknown) {
      throw e;
    }
  }

  static async findColors(
    origin: string,
    hexColors: hexColor[],
    startPoint: Point,
    endPoint: Point,
    rgbOffset: rgbColor,
  ): Promise<ColoredPoint[]> {
    try {
      return await invoke("find_colors", {
        origin,
        hexColors,
        startPoint,
        endPoint,
        rgbOffset,
      });
    } catch (e) {
      throw e;
    }
  }

  static async recognizeText(
    origin: string,
    startPoint: Point,
    endPoint: Point,
  ): Promise<string> {
    try {
      return await invoke("recognize_text", {
        origin,
        startPoint,
        endPoint,
      });
    } catch (e: unknown) {
      throw e;
    }
  }
}

export { Frame };
