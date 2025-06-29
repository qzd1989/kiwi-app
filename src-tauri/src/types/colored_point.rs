use super::{HexColor, Point};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ColoredPoint {
    pub point: Point,
    pub hex: HexColor,
}
impl ColoredPoint {
    pub fn new(point: Point, hex: String) -> Self {
        Self { point, hex }
    }
}
