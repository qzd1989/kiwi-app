use crate::interpreter::Code;
use crate::types::{ColoredPoint, HexColor, Point, RelativeColoredPoint, RgbOffset};

pub struct PythonCode {}

impl Default for PythonCode {
    fn default() -> Self {
        Self {}
    }
}

impl Code for PythonCode {
    fn predefined(&self) -> String {
        format!(
            r#"from kiwi import ScreenClient, Point, ColoredPoint, RgbOffset, System, Key
client = ScreenClient()
"#
        )
    }

    fn find_image(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
        min_template_side: u32,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.find_image(
    subpath="{subpath}",
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    threshold={threshold},
    min_template_side={min_template_side},
)"#
        )
    }

    fn find_images(
        &self,
        subpath: &str,
        start_point: &Point,
        end_point: &Point,
        threshold: f64,
        min_template_side: u32,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.find_images(
    subpath="{subpath}",
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    threshold={threshold},
    min_template_side={min_template_side},
)"#
        )
    }

    fn find_relative_colors(
        &self,
        points: &[RelativeColoredPoint],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        let RgbOffset { r, g, b } = rgb_offset;
        let rows_str = points
            .iter()
            .map(|p| {
                let ColoredPoint { point, hex } = p.colored_point.to_owned();
                let Point { x, y } = point;
                let Point{x:rx, y:ry}= p.relative_point;
                format!("        RelativeColoredPoint(colored_point=ColoredPoint(point=Point(x={x}, y={y}), hex=\"{hex}\"), relative_point=Point(x={rx}, y={ry})),")
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"client.find_relative_colors(
    points=[
{rows_str}
    ],
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    rgb_offset=RgbOffset(r={r}, g={g}, b={b}),
)"#
        )
    }

    fn find_colors(
        &self,
        hex_colors: &[HexColor],
        start_point: &Point,
        end_point: &Point,
        rgb_offset: &RgbOffset,
    ) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        let RgbOffset { r, g, b } = rgb_offset;
        let rows_str = hex_colors
            .iter()
            .map(|hex| format!("        \"{hex}\","))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"client.find_colors(
    hex_colors=[
{rows_str}
    ],
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
    rgb_offset=RgbOffset(r={r}, g={g}, b={b}),
)"#
        )
    }

    fn recognize_text(&self, start_point: &Point, end_point: &Point) -> String {
        let Point { x: sx, y: sy } = start_point;
        let Point { x: ex, y: ey } = end_point;
        format!(
            r#"client.recognize_text(
    start_point=Point(x={sx},y={sy}),
    end_point=Point(x={ex},y={ey}),
)"#
        )
    }

    fn absolute_code(&self, point: &Point) -> String {
        let Point { x, y } = point;
        format!(r#"client.move_absolute({x}, {y})"#)
    }
}
