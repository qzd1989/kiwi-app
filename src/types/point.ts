import { i32 } from ".";

interface Point {
  x: i32;
  y: i32;
}

class Point {
  constructor(public x: i32, public y: i32) {}
  static from(x: i32, y: i32): Point {
    return new Point(x, y);
  }
  clone(): Point {
    return Point.from(this.x, this.y);
  }
}

export { Point };
