import { hexColor } from "@types";
import { ColoredPoint } from "./colored-point";
import { Point } from "./point";

class RelativeColoredPoint {
  constructor(
    public coloredPoint: ColoredPoint,
    public relativePoint: Point,
  ) {}

  key(): string {
    return this.coloredPoint.point.x + "-" + this.coloredPoint.point.y;
  }

  isVertex(): boolean {
    return this.relativePoint.x == 0 && this.relativePoint.y == 0;
  }

  clone(): RelativeColoredPoint {
    return new RelativeColoredPoint(
      this.coloredPoint.clone(),
      this.relativePoint.clone(),
    );
  }
}

class RelativeColoredPoints extends Array<RelativeColoredPoint> {
  constructor(...items: RelativeColoredPoint[]) {
    super(...items);
  }

  vertexHex(): hexColor | null {
    if (this.length === 0) return null;
    return this.filter((item) => {
      return item.isVertex();
    }).pop()!.coloredPoint.hex;
  }

  caculateRelativePoint() {
    if (this.length === 0) return;
    if (this.length === 1) {
      this[0].relativePoint.x = 0;
      this[0].relativePoint.y = 0;
      return;
    }

    // 找到最上最左的点作为顶点
    let vertex = this[0];
    for (const point of this) {
      if (
        point.coloredPoint.point.y < vertex.coloredPoint.point.y ||
        (point.coloredPoint.point.y === vertex.coloredPoint.point.y &&
          point.coloredPoint.point.x < vertex.coloredPoint.point.x)
      ) {
        vertex = point;
      }
    }

    // 依次遍历原数组，设置每个元素的 relativePoint
    for (const point of this) {
      if (point === vertex) {
        point.relativePoint.x = 0;
        point.relativePoint.y = 0;
      } else {
        point.relativePoint.x =
          point.coloredPoint.point.x - vertex.coloredPoint.point.x;
        point.relativePoint.y =
          point.coloredPoint.point.y - vertex.coloredPoint.point.y;
      }
    }
  }
}
export { RelativeColoredPoint, RelativeColoredPoints };
