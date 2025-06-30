type u8 = number;
namespace u8 {
  export const MIN = 0 as u8;

  export const MAX = 0xff as u8;

  export const from = (value: number): u8 => {
    if (!u8.isValid(value)) {
      throw new Error(`Invalid u8: ${value}`);
    }
    return value as u8;
  };

  export const isValid = (value: number): value is u8 => {
    return (
      Number.isFinite(value) &&
      value >= MIN &&
      value <= MAX &&
      Math.floor(value) === value
    );
  };
}

export { u8 };
