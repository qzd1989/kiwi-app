type f64 = number;

namespace f64 {
  export const MIN = Number.MIN_VALUE as f64;

  export const MAX = Number.MAX_VALUE as f64;

  export const from = (value: number): f64 => {
    if (!f64.isValid(value)) {
      throw new Error(`Invalid f64: ${value}`);
    }
    return value as f64;
  };

  export const isValid = (value: number): value is f64 => {
    return Number.isFinite(value);
  };
}

export { f64 };
