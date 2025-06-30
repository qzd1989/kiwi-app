import { f64, Point } from ".";

interface WeightPoint {
  point: Point;
  weight: f64;
}

namespace WeightPoint {
  export const from = (point: Point, weight: number): WeightPoint => ({
    point,
    weight: f64.from(weight),
  });
}

export { WeightPoint };
