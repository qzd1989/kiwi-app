import { u8 } from ".";

type RgbaBuffer = Uint8Array;

namespace RgbaBuffer {
  export const from = (data: ArrayLike<number> | ArrayBuffer): RgbaBuffer => {
    const buffer = new Uint8Array(data);
    if (!isValid(buffer)) {
      throw new Error(`Invalid RgbaBuffer: length must be a multiple of 4`);
    }
    return buffer as RgbaBuffer;
  };

  export const isValid = (buf: Uint8Array): buf is RgbaBuffer => {
    return buf instanceof Uint8Array && buf.length % 4 === 0;
  };

  export const pixelCount = (buf: RgbaBuffer): number => {
    return buf.length / 4;
  };

  export const getPixel = (
    buf: RgbaBuffer,
    index: number
  ): [r: u8, g: u8, b: u8, a: u8] => {
    const offset = index * 4;
    if (offset + 3 >= buf.length) {
      throw new Error(`Pixel index ${index} out of bounds`);
    }
    return [
      buf[offset] as u8,
      buf[offset + 1] as u8,
      buf[offset + 2] as u8,
      buf[offset + 3] as u8,
    ];
  };
}

export { RgbaBuffer };
