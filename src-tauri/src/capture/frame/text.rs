//done
use super::Frame;
use crate::{
    extensions::{EmbeddedFileExt, ImageBufferRgbaExt as _},
    types::{Asset, Point, Size},
};
use anyhow::{Result, anyhow};
use paddle_ocr_rs::ocr_lite::OcrLite;
use std::sync::{Mutex, OnceLock};
impl Frame {
    pub fn recognize_text(&self, start_point: Point, size: Size) -> Result<Option<String>> {
        if size.width > self.width || self.height > self.height {
            return Err(anyhow!(t!(
                "The find area size must not be larger than the frame size."
            )));
        }
        let rgb_image = self.to_buffer()?.crop(start_point, size).to_rgb()?;
        OCR.get_or_init(move || -> Mutex<OcrLite> {
            let ocr = Mutex::new(OcrLite::new());
            match (
                Asset::get("paddle_ocr/models/ch_PP-OCRv5_mobile_det.onnx"),
                Asset::get("paddle_ocr/models/ch_ppocr_mobile_v2.0_cls_infer.onnx"),
                Asset::get("paddle_ocr/models/ch_PP-OCRv5_rec_mobile_infer.onnx"),
            ) {
                (Some(det), Some(cls), Some(rec)) => {
                    let (det_bytes, cls_bytes, rec_bytes) =
                        (det.to_cursor(), cls.to_cursor(), rec.to_cursor());
                    let (det_bytes, cls_bytes, rec_bytes) = (
                        det_bytes.get_ref().as_ref(),
                        cls_bytes.get_ref().as_ref(),
                        rec_bytes.get_ref().as_ref(),
                    );
                    ocr.lock()
                        .unwrap()
                        .init_models_from_memory(det_bytes, cls_bytes, rec_bytes, 2)
                        .map_err(|e| {
                            let error_string = match e {
                                paddle_ocr_rs::ocr_error::OcrError::Ort(e) => {
                                    e.message().to_string()
                                }
                                paddle_ocr_rs::ocr_error::OcrError::Io(e) => e.to_string(),
                                paddle_ocr_rs::ocr_error::OcrError::ImageError(ie) => {
                                    ie.to_string()
                                }
                                paddle_ocr_rs::ocr_error::OcrError::SessionNotInitialized => {
                                    anyhow!(t!("OCR session is uninitialized.")).to_string()
                                }
                            };
                            anyhow!(t!(
                                "Failed to initialize OCR models from memory.",
                                error = error_string
                            ))
                        })
                        .unwrap();
                }
                _ => {}
            }
            ocr
        });
        let result = OCR
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .detect(&rgb_image, 50, 1024, 0.5, 0.3, 1.6, true, false)?;
        let texts: Vec<String> = result
            .text_blocks
            .into_iter()
            .map(|text_block| text_block.text)
            .collect();
        let texts = texts.join("");
        if texts.is_empty() {
            return Ok(None);
        }
        Ok(Some(texts))
    }
}
//只想让它初始化一次
static OCR: OnceLock<Mutex<OcrLite>> = OnceLock::new();
