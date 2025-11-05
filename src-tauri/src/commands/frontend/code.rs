use super::utils::*;
use crate::{
    app,
    types::{HexColor, Point, RelativeColoredPoint, RgbOffset},
};

#[tauri::command]
pub fn generate_find_image_code(
    subpath: String,
    start_point: Point,
    end_point: Point,
    threshold: f64,
    min_template_side: u32,
) -> CommandResult<String> {
    let code = app::get().try_with_project(|project| {
        project.interpreter.code().find_image(
            &subpath,
            &start_point,
            &end_point,
            threshold,
            min_template_side,
        )
    })?;
    Ok(code)
}

#[tauri::command]
pub fn generate_find_images_code(
    subpath: String,
    start_point: Point,
    end_point: Point,
    threshold: f64,
    min_template_side: u32,
) -> CommandResult<String> {
    let code = app::get().try_with_project(|project| {
        project.interpreter.code().find_images(
            &subpath,
            &start_point,
            &end_point,
            threshold,
            min_template_side,
        )
    })?;
    Ok(code)
}

#[tauri::command]
pub fn generate_find_relative_colors_code(
    points: Vec<RelativeColoredPoint>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<String> {
    let code = app::get().try_with_project(|project| {
        project.interpreter.code().find_relative_colors(
            &points,
            &start_point,
            &end_point,
            &rgb_offset,
        )
    })?;
    Ok(code)
}

#[tauri::command]
pub fn generate_find_colors_code(
    hex_colors: Vec<HexColor>,
    start_point: Point,
    end_point: Point,
    rgb_offset: RgbOffset,
) -> CommandResult<String> {
    let code = app::get().try_with_project(|project| {
        project
            .interpreter
            .code()
            .find_colors(&hex_colors, &start_point, &end_point, &rgb_offset)
    })?;
    Ok(code)
}

#[tauri::command]
pub fn generate_recognize_text_code(start_point: Point, end_point: Point) -> CommandResult<String> {
    let code = app::get().try_with_project(|project| {
        project
            .interpreter
            .code()
            .recognize_text(&start_point, &end_point)
    })?;
    Ok(code)
}

#[tauri::command]
pub fn generate_move_absolute_code(point: Point) -> CommandResult<String> {
    let code =
        app::get().try_with_project(|project| project.interpreter.code().absolute_code(&point))?;
    Ok(code)
}
