use crate::types::HexColor;

use super::{ColoredPoint, Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RelativeColoredPoint {
    pub colored_point: ColoredPoint,
    pub relative_point: Point,
}

impl RelativeColoredPoint {
    pub fn new(colored_point: ColoredPoint, relative_point: Point) -> Self {
        Self {
            colored_point,
            relative_point,
        }
    }

    pub fn is_vertex(&self) -> bool {
        self.relative_point.x == 0 && self.relative_point.y == 0
    }
}

pub trait RelativeColoredPointsExt {
    fn vertex_hex(&self) -> Option<HexColor>;
}

impl RelativeColoredPointsExt for Vec<RelativeColoredPoint> {
    fn vertex_hex(&self) -> Option<HexColor> {
        let hex = self
            .iter()
            .filter(|item| item.is_vertex())
            .last()?
            .colored_point
            .hex
            .clone();
        Some(hex)
    }
}
