use super::CommandResult;
use crate::types::{
    Base64Png, Base64PngExt as _, ColoredPoint, HexColor, Point, RgbOffset, Size, WeightPoint,
    WeightPointsExt as _,
};

#[tauri::command]
pub fn find_image(
    origin: Base64Png,
    template: Base64Png,
    start_point: Point,
    end_point: Point,
    threshold: f64,
) -> CommandResult<Option<WeightPoint>> {
    let frame = origin.to_frame().unwrap();
    let template = template.to_buffer().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    Ok(frame.find_image(&template, start_point, size, threshold)?)
}

#[tauri::command]
pub fn find_images(
    origin: Base64Png,
    template: Base64Png,
    template_size: Size,
    start_point: Point,
    end_point: Point,
    threshold: f64,
) -> CommandResult<Vec<WeightPoint>> {
    let frame = origin.to_frame().unwrap();
    let template = template.to_buffer().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    let weight_points = frame.find_images(&template, start_point, size, threshold)?;
    Ok(weight_points.filter_close_points(&template_size))
}

#[tauri::command]
pub fn find_relative_colors(
    origin: Base64Png,
    vertex_hex: HexColor,
    relative_points: Vec<ColoredPoint>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<Option<ColoredPoint>> {
    let frame = origin.to_frame().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    Ok(frame.find_relative_colors(vertex_hex, relative_points, start_point, size, rgb_offset)?)
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
    Ok(frame.find_colors(&hex_colors, start_point, size, rgb_offset)?)
}

#[tauri::command]
pub fn recognize_text(
    origin: Base64Png,
    start_point: Point,
    end_point: Point,
) -> CommandResult<Option<String>> {
    let frame = origin.to_frame().unwrap();
    let size = Size::new_from_start_end_points(start_point, end_point)?;
    Ok(frame.recognize_text(start_point, size)?)
}
