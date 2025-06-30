use serde::{Deserialize, Serialize};

use crate::types::Size;
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    /// 计算两点之间的距离是否小于给定的大小
    pub fn is_too_close(&self, other: &Point, size: &Size) -> bool {
        let dx = (self.x - other.x).abs() as u32;
        let dy = (self.y - other.y).abs() as u32;
        dx < size.width && dy < size.height
    }
}
