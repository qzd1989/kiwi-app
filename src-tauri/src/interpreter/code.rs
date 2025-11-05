use crate::types::{HexColor, Point, RelativeColoredPoint, RgbOffset};
pub trait Code {
    fn predefined(&self) -> String;

    fn find_image(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
        min_template_side: u32,
    ) -> String;

    fn find_images(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
        min_template_side: u32,
    ) -> String;

    fn find_relative_colors(
        &self,
        points: &[RelativeColoredPoint],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String;

    fn find_colors(
        &self,
        hex_colors: &[HexColor],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String;

    fn recognize_text(&self, start_point: &Point, end_point: &Point) -> String;
    fn absolute_code(&self, point: &Point) -> String;
}
