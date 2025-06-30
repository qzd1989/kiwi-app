type u32 = number;

namespace u32 {
  export const MIN = 0 as u32;

  export const MAX = 0xffffffff as u32;

  export const from = (value: number): u32 => {
    if (!u32.isValid(value)) {
      throw new Error(`Invalid u32: ${value}`);
    }
    return value as u32;
  };

  export const isValid = (value: number): value is u32 => {
    return (
      Number.isFinite(value) &&
      value >= u32.MIN &&
      value <= u32.MAX &&
      Math.floor(value) === value
    );
  };
}

export { u32 };
