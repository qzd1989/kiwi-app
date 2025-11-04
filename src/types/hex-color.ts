import { rgbColor } from "./rgb-color";
import { u8 } from "./u8";

type hexColor = `#${string}`;

export const rgbToHex = (color: rgbColor): hexColor => {
  function toHex(colorValue: u8) {
    var hex = colorValue.toString(16);
    return hex.length === 1 ? "0" + hex : hex;
  }
  const hex = "#" + toHex(color.r) + toHex(color.g) + toHex(color.b);
  return hex as hexColor;
};

export type { hexColor };
