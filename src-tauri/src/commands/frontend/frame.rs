use super::utils::*;
use crate::app;
use crate::types::{
    Base64Png, Base64PngExt as _, ColoredPoint, HexColor, Point, RelativeColoredPoint, RgbOffset,
    Size, WeightPoint, WeightPointsExt as _,
};
use anyhow::Result;
use std::thread;
use tauri::Emitter;

#[tauri::command]
pub fn find_image(
    key: String,
    origin: Base64Png,
    template: Base64Png,
    start_point: Point,
    end_point: Point,
    threshold: f64,
    min_template_side: u32,
) -> CommandResult<()> {
    thread::spawn(move || {
        let get_result = || {
            let frame = origin.to_frame()?;
            let template = template.to_buffer()?;
            let size = Size::new_from_start_end_points(start_point, end_point)?;
            frame.find_image(&template, start_point, size, threshold, min_template_side)
        };
        let result = get_result();
        match result {
            Ok(result) => {
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateImageMatchingResult.into(), (key, result.0));
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateExecutionTime.into(), result.1);
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateImageMatchingInfo.into(), result.2);
            }
            Err(_) => todo!(),
        }
    });

    Ok(())
}

#[tauri::command]
pub fn find_images(
    key: String,
    origin: Base64Png,
    template: Base64Png,
    start_point: Point,
    end_point: Point,
    threshold: f64,
    min_template_side: u32,
) -> CommandResult<()> {
    thread::spawn(move || {
        let get_result = || {
            let frame = origin.to_frame()?;
            let template = template.to_buffer()?;
            let size = Size::new_from_start_end_points(start_point, end_point)?;
            let result =
                frame.find_images(&template, start_point, size, threshold, min_template_side)?;
            Ok(result)
        };
        let result: Result<(Vec<WeightPoint>, u128, _)> = get_result();
        let template_size = template.size().unwrap();
        match result {
            Ok(result) => {
                let points = result.0.filter_close_points(&template_size);
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateImagesMatchingResult.into(), (key, points));
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateExecutionTime.into(), result.1);
                let _ = app::get()
                    .app_handle()
                    .emit(EmitEvent::UpdateImageMatchingInfo.into(), result.2);
            }
            Err(_) => todo!(),
        }
    });

    Ok(())
}

#[tauri::command]
pub fn find_relative_colors(
    origin: Base64Png,
    points: Vec<RelativeColoredPoint>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<Option<ColoredPoint>> {
    let frame = origin.to_frame().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let result = frame.find_relative_colors(points, start_point, size, rgb_offset)?;
    let _ = app::get()
        .app_handle()
        .emit(EmitEvent::UpdateExecutionTime.into(), result.1);
    Ok(result.0)
}

#[tauri::command]
pub fn find_colors(
    origin: Base64Png,
    hex_colors: Vec<HexColor>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<Vec<ColoredPoint>> {
    let frame = origin.to_frame().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let result = frame.find_colors(&hex_colors, start_point, size, rgb_offset)?;
    let _ = app::get()
        .app_handle()
        .emit(EmitEvent::UpdateExecutionTime.into(), result.1);
    Ok(result.0)
}

#[tauri::command]
pub fn recognize_text(
    origin: Base64Png,
    start_point: Point,
    end_point: Point,
) -> CommandResult<Option<String>> {
    let frame = origin.to_frame().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let result = frame.recognize_text(start_point, size)?;
    let _ = app::get()
        .app_handle()
        .emit(EmitEvent::UpdateExecutionTime.into(), result.1);
    Ok(result.0)
}
