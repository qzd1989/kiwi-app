use super::Frame;
use crate::extensions::ImageBufferRgbaExt as _;
use crate::types::Point;
use crate::types::Size;
use crate::types::WeightPoint;
use anyhow::{Result, anyhow};
use image::GrayImage;
use image::ImageBuffer;
use image::Luma;
use image::imageops::FilterType;
use image::imageops::resize;
use imageproc::template_matching::{
    MatchTemplateMethod, find_extremes, match_template_with_mask_parallel,
};
use serde::Deserialize;
use serde::Serialize;
use std::cmp::min;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct MatchingInfo {
    factor: f64,
    image_size: Size,
    template_size: Size,
}

impl MatchingInfo {
    pub fn new(factor: f64, image_size: Size, template_size: Size) -> Self {
        Self {
            factor,
            image_size,
            template_size,
        }
    }
}

impl Frame {
    pub fn find_image(
        &self,
        template: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        start_point: Point,
        size: Size,
        threshold: impl Into<f64>, //建议0.99以上
        min_template_side: u32,
    ) -> Result<(Option<WeightPoint>, u128, MatchingInfo)> {
        let now = Instant::now();

        if size.width > self.width || size.height > self.height {
            return Err(anyhow!(t!(
                "The find area size must not be larger than the frame size."
            )));
        }

        let (template_width, template_height) = template.dimensions();

        if size.width < template_width || size.height < template_height {
            return Err(anyhow!(t!(
                "The template size exceeds the cropped frame size."
            )));
        }
        // let template = template.crop_alpha_edges()?; //放到前台去做 todo
        //一层金字塔匹配, 保证最小边为min_template_side,不然遇到长条形模板绽放就会匹配不到
        let factor = {
            let min_side = min(template_width, template_height) as f64;
            if min_side > min_template_side as f64 {
                min_side / min_template_side as f64
            } else {
                1.0
            }
        };
        let cropped_frame = self.to_buffer()?.crop(start_point, size);
        let (frame_width, frame_height) = cropped_frame.dimensions();
        let gray_frame = {
            if factor != 1.0 {
                resize(
                    &cropped_frame,
                    (frame_width as f64 / factor) as u32,
                    (frame_height as f64 / factor) as u32,
                    FilterType::Lanczos3,
                )
            } else {
                cropped_frame
            }
        }
        .to_gray();
        let template = {
            if factor != 1.0 {
                resize(
                    template,
                    (template_width as f64 / factor) as u32,
                    (template_height as f64 / factor) as u32,
                    FilterType::Lanczos3,
                )
            } else {
                template.clone()
            }
        };
        let gray_template = template.to_gray();
        let mask = template.to_mask();
        let matching_info = MatchingInfo::new(
            factor,
            Size::new(gray_frame.width(), gray_frame.height()),
            Size::new(gray_template.width(), gray_template.height()),
        );

        match match_image(&gray_frame, &gray_template, &mask, threshold.into()) {
            Some(mut weight_point) => {
                if factor != 1.0 {
                    weight_point.point.x = (weight_point.point.x as f64 * factor) as i32;
                    weight_point.point.y = (weight_point.point.y as f64 * factor) as i32;
                    weight_point.point.x += start_point.x;
                    weight_point.point.y += start_point.y;
                } else {
                    weight_point.point.x += start_point.x;
                    weight_point.point.y += start_point.y;
                }

                Ok((Some(weight_point), now.elapsed().as_millis(), matching_info))
            }
            None => Ok((None, now.elapsed().as_millis(), matching_info)),
        }
    }

    pub fn find_images(
        &self,
        template: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        start_point: Point,
        size: Size,
        threshold: impl Into<f64>, //建议0.99以上
        min_template_side: u32,
    ) -> Result<(Vec<WeightPoint>, u128, MatchingInfo)> {
        let now = Instant::now();
        if size.width > self.width || size.height > self.height {
            return Err(anyhow!(t!(
                "The find area size must not be larger than the frame size."
            )));
        }

        let (template_width, template_height) = template.dimensions();

        if size.width < template_width || size.height < template_height {
            return Err(anyhow!(t!(
                "The template size exceeds the cropped frame size."
            )));
        }
        //一层金字塔匹配
        let factor = {
            let min_side = min(template_width, template_height) as f64;
            if min_side > min_template_side as f64 {
                min_side / min_template_side as f64
            } else {
                1.0
            }
        };
        let cropped_frame = self.to_buffer()?.crop(start_point, size);
        let (frame_width, frame_height) = cropped_frame.dimensions();
        let mut image = {
            if factor != 1.0 {
                resize(
                    &cropped_frame,
                    (frame_width as f64 / factor) as u32,
                    (frame_height as f64 / factor) as u32,
                    FilterType::Lanczos3,
                )
            } else {
                cropped_frame
            }
        }
        .to_gray();
        let template = {
            if factor != 1.0 {
                resize(
                    template,
                    (template_width as f64 / factor) as u32,
                    (template_height as f64 / factor) as u32,
                    FilterType::Lanczos3,
                )
            } else {
                template.clone()
            }
        };
        let gray_template = template.to_gray();
        let mask = template.to_mask();
        let threshold: f64 = threshold.into();
        let matching_info = MatchingInfo::new(
            factor,
            Size::new(image.width(), image.height()),
            Size::new(gray_template.width(), gray_template.height()),
        );
        let mut weight_points = Vec::new();

        loop {
            if weight_points.len() > 10 {
                break;
            }

            if let Some(weight_point) = match_image(&image, &gray_template, &mask, threshold) {
                weight_points.push(weight_point);
                filter_matched_area(&mut image, &gray_template, &weight_point.point);
            } else {
                break;
            }
        }

        for weight_point in weight_points.iter_mut() {
            if factor != 1.0 {
                weight_point.point.x = (weight_point.point.x as f64 * factor) as i32;
                weight_point.point.y = (weight_point.point.y as f64 * factor) as i32;
                weight_point.point.x += start_point.x;
                weight_point.point.y += start_point.y;
            } else {
                weight_point.point.x += start_point.x;
                weight_point.point.y += start_point.y;
            }
        }

        Ok((weight_points, now.elapsed().as_millis(), matching_info))
    }
}

fn match_image(
    image: &GrayImage,
    template: &GrayImage,
    mask: &GrayImage,
    threshold: f64,
) -> Option<WeightPoint> {
    let extremes = find_extremes(&match_template_with_mask_parallel(
        &image,
        &template,
        MatchTemplateMethod::CrossCorrelationNormalized,
        &mask,
    ));
    let weight = extremes.max_value as f64;

    if weight < threshold {
        return None;
    }

    let (x, y) = extremes.max_value_location;
    Some(WeightPoint::new(Point::new(x as i32, y as i32), weight))
}

fn filter_matched_area(image: &mut GrayImage, template: &GrayImage, start_point: &Point) {
    let tpl_w = template.width() as i32;
    let tpl_h = template.height() as i32;
    let img_w = image.width() as i32;
    let img_h = image.height() as i32;

    for dy in 0..tpl_h {
        for dx in 0..tpl_w {
            let x = start_point.x + dx;
            let y = start_point.y + dy;

            if x < img_w && y < img_h {
                let tpl_val = template.get_pixel(dx as u32, dy as u32).0[0];
                let inverted = 255u8.saturating_sub(tpl_val);
                image.put_pixel(x as u32, y as u32, Luma([inverted]));
            }
        }
    }
}
