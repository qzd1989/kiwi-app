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
use std::time::Instant;

impl Frame {
    // ============================================
    /// 在指定区域内查找指定颜色集合，返回所有匹配到的颜色点,同一颜色也可能有多个点。
    ///
    /// # 功能说明
    /// - 遍历指定区域的每个像素点；
    /// - 对每个像素点，与目标颜色列表进行模糊匹配（RGB容差）；
    /// - 匹配成功时，记录该像素点的绝对坐标和颜色值；
    /// - 返回区域内所有匹配到的颜色点。
    ///
    /// # 参数
    /// * `hex_colors` - 目标颜色十六进制数组
    /// * `start_point` - 扫描起点（区域左上角坐标）
    /// * `size` - 扫描区域宽高
    /// * `rgb_offset` - 颜色匹配容差
    ///
    /// # 返回
    /// `(Vec<ColoredPoint>, u128)` - 匹配到的颜色点列表 + 耗时（毫秒）
    pub fn find_colors(
        &self,
        hex_colors: &[HexColor],
        start_point: &Point,
        size: &Size,
        rgb_offset: &RgbOffset,
    ) -> Result<(Vec<ColoredPoint>, u128)> {
        let now = Instant::now();

        // ---- 校验扫描区域合法性 ----
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

        // ---- 裁剪扫描区域 ----
        let buffer = self.to_buffer()?.crop(start_point, size);
        let (width, height) = buffer.dimensions();

        // ---- 转换目标颜色为 RGB ----
        let rgb_colors: Vec<RgbColor> = hex_colors
            .iter()
            .map(|hex_color| hex_color.to_rgb())
            .collect();

        let mut matched_colors = Vec::new();

        // ---- 遍历区域内所有像素点 ----
        for cropped_y in 0..height {
            for cropped_x in 0..width {
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let target = RgbColor(r, g, b);

                // ---- 对每个目标颜色进行匹配 ----
                for rgb_color in rgb_colors.iter() {
                    if rgb_color.range_compare(&rgb_offset, &target) {
                        let colored_point = ColoredPoint::new(
                            Point::new(
                                cropped_x as i32 + start_point.x,
                                cropped_y as i32 + start_point.y,
                            ),
                            rgb_color.to_hex(),
                        );
                        matched_colors.push(colored_point); // 记录匹配点
                    }
                }
            }
        }

        Ok((matched_colors, now.elapsed().as_millis()))
    }

    // ============================================
    /// 判断指定区域内是否出现目标颜色集合中的任意一种
    ///
    /// # 功能说明
    /// - 调用 `find_colors` 获取匹配结果；
    /// - 若至少匹配到一个颜色点，则返回 true，否则 false。
    ///
    /// # 参数
    /// 同 `find_colors`
    ///
    /// # 返回
    /// `bool` - 是否至少匹配到一种颜色
    pub fn has_colors(
        &self,
        colors: &[HexColor],
        point: &Point,
        size: &Size,
        rgb_offset: &RgbOffset,
    ) -> Result<bool> {
        let (matched_colors, _) = self.find_colors(colors, point, size, rgb_offset)?;
        Ok(!matched_colors.is_empty())
    }

    // ============================================
    /// 基于顶点和相对点匹配一组颜色点
    ///
    /// # 功能说明
    /// - 若只有一个颜色点，退化为 `find_colors` 功能；
    /// - 若存在多个点，逐像素扫描顶点颜色；
    /// - 顶点匹配成功后，再验证相对点是否匹配；
    /// - 全部匹配成功后返回顶点绝对坐标。
    ///
    /// # 参数
    /// * `points` - 包含一个顶点及若干相对点的颜色点集合
    /// * `start_point` - 扫描起点
    /// * `size` - 扫描区域大小
    /// * `rgb_offset` - 颜色匹配容差
    ///
    /// # 返回
    /// `(Option<ColoredPoint>, u128)` - 匹配到的顶点点信息 + 耗时（毫秒）
    pub fn find_relative_colors(
        &self,
        points: &Vec<RelativeColoredPoint>,
        start_point: &Point,
        size: &Size,
        rgb_offset: &RgbOffset,
    ) -> Result<(Option<ColoredPoint>, u128)> {
        let now = Instant::now();

        // ---- 校验扫描区域 ----
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

        if points.is_empty() {
            return Err(anyhow!(t!("The points must not be empty.")));
        }

        // ---- 单点情况 ----
        if points.len() == 1 {
            let point = points.last().cloned().unwrap().colored_point;
            let (colored_points, time) =
                self.find_colors(&[point.hex], start_point, size, rgb_offset)?;
            let point = colored_points.last().cloned();
            return Ok((point, time));
        }

        // ---- 多点情况 ----
        let Some(vertex) = points.vertex_hex() else {
            return Err(anyhow!(t!("Origin point not found.")));
        };
        let points_without_vertex: Vec<RelativeColoredPoint> = points
            .iter()
            .filter(|item| !item.is_vertex())
            .cloned()
            .collect();

        let vertex_rgb = vertex.to_rgb();
        let buffer = self.to_buffer()?.crop(start_point, size);
        let (width, height) = buffer.dimensions();

        // ---- 遍历区域扫描顶点颜色 ----
        for cropped_y in 0..height {
            for cropped_x in 0..width {
                let pixel = buffer.get_pixel(cropped_x, cropped_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                let target = RgbColor(r, g, b);

                if vertex_rgb.range_compare(&rgb_offset, &target) {
                    let origin_abs_point = Point::new(cropped_x as i32, cropped_y as i32);

                    // ---- 校验相对点是否匹配 ----
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

// ============================================
/// 校验顶点相对点是否全部匹配
///
/// # 功能说明
/// - 将相对点转换为绝对坐标；
/// — 在绝对坐标周围 5x5 区域进行模糊匹配；
/// — 任意相对点未匹配成功则返回 false；
/// - 全部匹配成功返回 true。
fn match_relatives(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    origin_abs_point: &Point,
    points_without_vertex: &Vec<RelativeColoredPoint>,
    rgb_offset: &RgbOffset,
) -> bool {
    // ---- 相对坐标转绝对坐标函数 ----
    let convert_to_abs_point = |relative_point: &Point, origin_abs_point: &Point| -> Point {
        Point::new(
            relative_point.x + origin_abs_point.x,
            relative_point.y + origin_abs_point.y,
        )
    };

    // ---- 遍历所有相对点 ----
    for relative_colored_point in points_without_vertex.iter() {
        let point = convert_to_abs_point(&relative_colored_point.relative_point, origin_abs_point);
        let rgb_color = relative_colored_point.colored_point.hex.to_owned().to_rgb();

        // ---- 在相对点附近 5x5 区域进行匹配 ----
        // 5x5 区域示意（中心点 P 为目标点）
        // [ ][ ][ ][ ][ ]
        // [ ][ ][ ][ ][ ]
        // [ ][ ][P][ ][ ]
        // [ ][ ][ ][ ][ ]
        // [ ][ ][ ][ ][ ]
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

        // ---- 如果任意点未匹配成功，则整体失败 ----
        if !matched {
            return false;
        }
    }

    true
}
