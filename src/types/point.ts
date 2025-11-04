import { i32 } from "./i32";
class Point {
  constructor(
    public x: i32,
    public y: i32,
  ) {}
  clone(): Point {
    return new Point(this.x, this.y);
  }
  reset(): void {
    this.x = 0;
    this.y = 0;
  }
  toString(): string {
    return `${this.x}, ${this.y}`;
  }
  toPythonCode(): string {
    return `(${this.x}, ${this.y})`;
  }
  static arrayToPythonCode(points: Point[]): string {
    if (points.length === 0) return "[]";
    let rows = points.map((p) => `  Point(x=${p.x}, y=${p.y}),`);
    return `[\n${rows.join("\n")}\n]`;
  }

  static from(obj: { x: number; y: number }) {
    return new Point(obj.x as i32, obj.y as i32);
  }
}
export { Point };
