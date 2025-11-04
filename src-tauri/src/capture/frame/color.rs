use std::time::Instant;

use super::Frame;
use crate::{
    extensions::ImageBufferRgbaExt as _,
    types::{
        ColoredPoint, HexColor, HexColorExt as _, Point, RelativeColoredPoint,
        RelativeColoredPointsExt, RgbColor, RgbOffset, Size,
    },
};
use anyhow::{Result, anyhow};
use image::{ImageBuffer, Rgba};

impl Frame {
    pub fn find_colors(
        &self,
        hex_colors: &[HexColor],
        start_point: Point,
        size: Size,
        rgb_offset: RgbOffset,
    ) -> Result<(Vec<ColoredPoint>, u128)> {
        let now = Instant::now();
        if size.width > self.width || size.height > self.height {
            return Err(anyhow!(t!(
                "The find area size must not be larger than the frame size."
            )));
        }
        if start_point.x < 0
            || start_point.y < 0
            || start_point.x + size.width as i32 > self.width as i32
            || start_point.y + size.height as i32 > self.height as i32
        {
            return Err(anyhow!(t!(
                "The find area start point and size must be within the frame bounds."
            )));
        }
        let buffer = self.to_buffer()?.crop(start_point, size);
        let (width, height) = buffer.dimensions();
        let mut rgb_colors: Vec<RgbColor> = hex_colors
            .iter()
            .map(|hex_color| hex_color.to_rgb())
            .collect();
        let mut locating_colors = Vec::new();
        for cropped_y in 0..height {
            for cropped_x in 0..width {
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let target = RgbColor(r, g, b);
                rgb_colors.retain(|rgb_color| {
                    if rgb_color.range_compare(&rgb_offset, &target) {
                        locating_colors.push(ColoredPoint::new(
                            Point::new(
                                cropped_x as i32 + start_point.x,
                                cropped_y as i32 + start_point.y,
                            ),
                            rgb_color.to_hex(),
                        ));
                        return false;
                    }
                    return true;
                });
            }
        }
        if !rgb_colors.is_empty() {
            // 颜色只要没匹配完就返回空，必须所有颜色都匹配上。
            locating_colors.clear();
            return Ok((locating_colors, now.elapsed().as_millis()));
        }
        Ok((locating_colors, now.elapsed().as_millis()))
    }

    pub fn has_colors(
        &self,
        colors: &[HexColor],
        point: Point,
        size: Size,
        rgb_offset: RgbOffset,
    ) -> Result<bool> {
        Ok(self.find_colors(colors, point, size, rgb_offset).is_ok())
    }

    pub fn find_relative_colors(
        &self,
        points: Vec<RelativeColoredPoint>, //里面只会有一个顶点，其他都是相对点，且len 大于1
        start_point: Point,                //scan start point
        size: Size,                        //scan range
        rgb_offset: RgbOffset,
    ) -> Result<(Option<ColoredPoint>, u128)> //return vertex point
    {
        let now = Instant::now();

        if size.width > self.width || size.height > self.height {
            return Err(anyhow!(t!(
                "The find area size must not be larger than the frame size."
            )));
        }

        if start_point.x < 0
            || start_point.y < 0
            || start_point.x + size.width as i32 > self.width as i32
            || start_point.y + size.height as i32 > self.height as i32
        {
            return Err(anyhow!(t!(
                "The find area start point and size must be within the frame bounds."
            )));
        }

        if points.len() == 0 {
            return Err(anyhow!(t!("The points must not be empty.")));
        }

        if points.len() == 1 {
            // find color if provides only one point.
            let point = points.last().cloned().unwrap().colored_point;
            let (colored_points, time) =
                self.find_colors(&[point.hex], start_point, size, rgb_offset)?;
            let point = colored_points.last().cloned();
            return Ok((point, time));
        }

        let Some(vertex) = points.vertex_hex() else {
            return Err(anyhow!(t!("Origin point not found.")));
        };
        let points_without_vertex: Vec<RelativeColoredPoint> = points
            .iter()
            .filter(|item| !item.is_vertex())
            .cloned()
            .collect();
        let vertex_rgb = vertex.to_rgb();
        let buffer = self.to_buffer().unwrap().crop(start_point, size);
        let (width, height) = buffer.dimensions();

        for cropped_y in 0..height {
            for cropped_x in 0..width {
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let target = RgbColor(r, g, b);
                if vertex_rgb.range_compare(&rgb_offset, &target) {
                    let origin_abs_point = Point::new(cropped_x as i32, cropped_y as i32);
                    // compare others pixel point.
                    if match_relatives(
                        &buffer,
                        &origin_abs_point,
                        &points_without_vertex,
                        &rgb_offset,
                    ) {
                        let vertex_abs = ColoredPoint::new(
                            Point::new(
                                cropped_x as i32 + start_point.x,
                                cropped_y as i32 + start_point.y,
                            ),
                            target.to_hex(),
                        );
                        return Ok((Some(vertex_abs), now.elapsed().as_millis()));
                    }
                }
            }
        }
        Ok((None, now.elapsed().as_millis()))
    }
}

fn match_relatives(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    origin_abs_point: &Point,
    points_without_vertex: &Vec<RelativeColoredPoint>,
    rgb_offset: &RgbOffset,
) -> bool {
    let convert_to_abs_point = |relative_point: &Point, origin_abs_point: &Point| -> Point {
        Point::new(
            relative_point.x + origin_abs_point.x,
            relative_point.y + origin_abs_point.y,
        )
    };

    for relative_colored_point in points_without_vertex.iter() {
        let point = convert_to_abs_point(&relative_colored_point.relative_point, origin_abs_point);
        let rgb_color = relative_colored_point.colored_point.hex.to_owned().to_rgb();

        // Check 5x5 area centered at point
        let mut matched = false;
        for dy in -2..=2 {
            for dx in -2..=2 {
                let nx = point.x + dx;
                let ny = point.y + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                if let Some(pixel) = buffer.get_pixel_checked(nx as u32, ny as u32) {
                    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                    let target = RgbColor(r, g, b);
                    if rgb_color.range_compare(&rgb_offset, &target) {
                        matched = true;
                        break;
                    }
                }
            }
            if matched {
                break;
            }
        }
        if !matched {
            return false;
        }
    }
    true
}
