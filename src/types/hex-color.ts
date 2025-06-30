import { RgbColor, u8 } from ".";

type HexColor = `#${string}`;

namespace HexColor {
  export const from = (value: string): HexColor => {
    if (!HexColor.isValid(value)) {
      throw new Error(`Invalid hex color format: ${value}`);
    }
    return value as HexColor;
  };

  export const isValid = (value: string): value is HexColor => {
    return /^#[0-9A-Fa-f]{6}([0-9A-Fa-f]{2})?$/.test(value);
  };

  export const random = (): HexColor => {
    const hex = Math.floor(Math.random() * 16777215)
      .toString(16)
      .padStart(6, "0");
    return `#${hex}` as HexColor;
  };

  export const fromRgbColor = (rgbPixelColor: RgbColor): HexColor => {
    function toHex(colorValue: u8) {
      var hex = colorValue.toString(16);
      return hex.length === 1 ? "0" + hex : hex;
    }
    const hexString =
      "#" +
      toHex(rgbPixelColor.r) +
      toHex(rgbPixelColor.g) +
      toHex(rgbPixelColor.b);
    return HexColor.from(hexString);
  };
}

export { HexColor };
