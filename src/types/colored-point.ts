import { hexColor } from "./hex-color";
import { Point } from "./point";

class ColoredPoint {
  constructor(
    public point: Point,
    public hex: hexColor,
  ) {}

  key(): string {
    return this.point.x + "-" + this.point.y;
  }

  clone(): ColoredPoint {
    return new ColoredPoint(this.point.clone(), this.hex);
  }

  static arrayToCode = (coloredPoints: ColoredPoint[]): string => {
    let rows = [];
    for (const coloredPoint of coloredPoints) {
      rows.push(
        `  ColoredPoint(point=Point(x=${coloredPoint.point.x}, y=${coloredPoint.point.y}), hex="${coloredPoint.hex}"),`,
      );
    }
    let data = `[
${rows.join(`\n`)}
]`;
    return data;
  };
}
export { ColoredPoint };
