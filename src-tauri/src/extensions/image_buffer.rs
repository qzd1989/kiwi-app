use crate::types::{Base64Png, Point, Size};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage::ImageRgba8, GrayImage, ImageBuffer, Luma, RgbImage, Rgba, imageops};

pub trait ImageBufferRgbaExt: Sized {
    fn to_rgb(&self) -> Result<RgbImage>;
    fn to_base64png(&self) -> Result<Base64Png>;
    fn crop(&self, point: Point, size: Size) -> Self;
    fn crop_alpha_edges(&self) -> Result<Self>;
    fn to_mask(&self) -> GrayImage; //转换为grayimage并且将透明度为0的象素点设为0,以便不参与templat_matching匹配.
    fn to_gray(&self) -> GrayImage;
}

impl ImageBufferRgbaExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn to_rgb(&self) -> Result<RgbImage> {
        match ImageBuffer::from_vec(
            self.width(),
            self.height(),
            self.as_raw()
                .chunks(4)
                .flat_map(|px| &px[..3])
                .copied()
                .collect(),
        ) {
            Some(rgb_image) => Ok(rgb_image),
            None => Err(anyhow!(t!("Failed to convert RGBA buffer to RGB."))),
        }
    }

    /// too slow, don't use it.
    fn to_base64png(&self) -> Result<Base64Png> {
        let mut bytes = Vec::new();
        self.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )?;
        Ok(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(bytes)
        ))
    }

    fn crop(&self, point: Point, size: Size) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        imageops::crop_imm(
            self,
            point.x as u32,
            point.y as u32,
            size.width as u32,
            size.height as u32,
        )
        .to_image()
    }

    fn crop_alpha_edges(&self) -> Result<Self> {
        let (width, height) = self.dimensions();
        let mut found = false;
        let mut top = 0;
        let mut bottom = height - 1;
        let mut left = 0;
        let mut right = width - 1;

        // Find top
        'outer_top: for y in 0..height {
            for x in 0..width {
                if self.get_pixel(x, y)[3] != 0 {
                    top = y;
                    found = true;
                    break 'outer_top;
                }
            }
        }

        if !found {
            return Err(anyhow!(t!("Image is fully transparent.")));
        }

        // Find bottom
        'outer_bottom: for y in (0..height).rev() {
            for x in 0..width {
                if self.get_pixel(x, y)[3] != 0 {
                    bottom = y;
                    break 'outer_bottom;
                }
            }
        }

        // Find left
        'outer_left: for x in 0..width {
            for y in top..=bottom {
                if self.get_pixel(x, y)[3] != 0 {
                    left = x;
                    break 'outer_left;
                }
            }
        }

        // Find right
        'outer_right: for x in (0..width).rev() {
            for y in top..=bottom {
                if self.get_pixel(x, y)[3] != 0 {
                    right = x;
                    break 'outer_right;
                }
            }
        }

        // Compute crop width/height
        let crop_width = right - left + 1;
        let crop_height = bottom - top + 1;

        // Crop image
        Ok(image::imageops::crop_imm(self, left, top, crop_width, crop_height).to_image())
    }

    fn to_gray(&self) -> GrayImage {
        ImageRgba8(self.clone()).to_luma8()
    }

    fn to_mask(&self) -> GrayImage {
        let mut buffer = self.to_gray();

        for (x, y, pixel) in self.enumerate_pixels() {
            let [_, _, _, a] = pixel.0;
            if a == 0 {
                buffer.put_pixel(x, y, Luma([0]));
            }
        }

        buffer
    }
}
