import { i32 } from ".";

interface Point {
  x: i32;
  y: i32;
}

class Point {
  constructor(public x: i32, public y: i32) {}

  clone(): Point {
    return Point.from(this.x, this.y);
  }
}

namespace Point {
  export const from = (x: number, y: number): Point => new Point(x, y);
}

export { Point };
