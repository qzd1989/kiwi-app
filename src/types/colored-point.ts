import { HexColor, Point } from ".";

interface ColoredPoint {
  point: Point;
  hex: HexColor;
}

namespace ColoredPoint {
  export const from = (point: Point, hexColor: string): ColoredPoint => ({
    point,
    hex: HexColor.from(hexColor),
  });
}

export { ColoredPoint };
