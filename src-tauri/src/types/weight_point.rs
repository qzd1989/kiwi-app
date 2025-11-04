use crate::types::Size;

use super::{Point, Weight};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct WeightPoint {
    pub point: Point,
    pub weight: Weight,
}
impl WeightPoint {
    pub fn new(point: Point, weight: Weight) -> Self {
        Self { point, weight }
    }
}

pub trait WeightPointsExt {
    fn filter_close_points(&self, template_size: &Size) -> Vec<WeightPoint>;
}

impl WeightPointsExt for Vec<WeightPoint> {
    fn filter_close_points(&self, template_size: &Size) -> Vec<WeightPoint> {
        let mut filtered: Vec<WeightPoint> = Vec::new();
        let mut points = self.to_vec();

        // 按权重从高到低排序，这样我们优先保留权重高的点
        points.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        for current_point in points {
            let mut should_keep = true;

            // 检查当前点是否与已经保留的点距离太近
            for kept_point in &filtered {
                if current_point
                    .point
                    .is_too_close(&kept_point.point, template_size)
                {
                    should_keep = false;
                    break;
                }
            }

            if should_keep {
                filtered.push(current_point);
            }
        }

        filtered
    }
}
