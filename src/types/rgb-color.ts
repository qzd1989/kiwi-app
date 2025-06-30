import { u8 } from ".";

interface RgbColor {
  r: u8;
  g: u8;
  b: u8;
}

namespace RgbColor {
  export const from = (r: number, g: number, b: number): RgbColor => {
    const rU8 = u8.from(r);
    const gU8 = u8.from(g);
    const bU8 = u8.from(b);
    return {
      r: rU8,
      g: gU8,
      b: bU8,
    };
  };
}

export { RgbColor };
