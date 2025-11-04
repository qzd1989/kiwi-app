import { Point } from "./point";
import { u32 } from "./u32";
class Size {
  constructor(
    public width: u32,
    public height: u32,
  ) {}
  clone(): Size {
    return new Size(this.width, this.height);
  }
  public toCode(): string {
    return `Size(width=${this.width}, height=${this.height})`;
  }
  static fromPoints(start: Point, end: Point): Size {
    const width = Math.abs(end.x - start.x);
    const height = Math.abs(end.y - start.y);
    return new Size(width, height);
  }
}
export { Size };
