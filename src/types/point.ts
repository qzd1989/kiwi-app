import { i32 } from ".";

interface Point {
  x: i32;
  y: i32;
}

namespace Point {
  export const from = (x: i32, y: i32): Point => {
    return { x, y };
  };
  export const clone = (point: Point): Point => {
    return Point.from(point.x, point.y);
  };
}

export { Point };
