import { u32 } from ".";

interface Size {
  width: u32;
  height: u32;
}

namespace Size {
  export const from = (width: number, height: number): Size => ({
    width: u32.from(width),
    height: u32.from(height),
  });
}

export { Size };
