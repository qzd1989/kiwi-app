type i32 = number;

namespace i32 {
  export const MIN = -0x80000000 as i32;

  export const MAX = 0x7fffffff as i32;

  export const from = (value: number): i32 => {
    if (!i32.isValid(value)) {
      throw new Error(`Invalid i32: ${value}`);
    }
    return value as i32;
  };

  export const isValid = (value: number): value is i32 => {
    return (
      Number.isFinite(value) &&
      value >= MIN &&
      value <= MAX &&
      Math.floor(value) === value
    );
  };
}

export { i32 };
