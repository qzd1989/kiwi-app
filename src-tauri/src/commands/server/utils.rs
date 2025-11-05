use crate::{
    app::{self, Role},
    commands::{frontend::relative_template_dir, server::Engine as ServerEngine},
    types::{Base64Png, Base64PngExt as _, HexColor, Point, RelativeColoredPoint, RgbOffset, Size},
};
use anyhow::{Result, anyhow};
use image::{ImageReader, RgbaImage};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub fn save_template(args: Value) -> Result<Value> {
    use fs_extra::dir;
    let (name, template) = get_template_args(&args)?;
    let name = name + ".png";
    let buffer = template.to_buffer().unwrap();
    let template_dir = relative_template_dir();
    let path = app::get()
        .resource_dir()
        .join(".cache")
        .join(&template_dir)
        .join(&name);
    let parent_path = path.parent().unwrap();

    if let Err(error) = dir::create_all(parent_path, false) {
        let msg = t!(
            "Failed to create folder.",
            path = parent_path.to_string_lossy(),
            error = error.to_string()
        );
        return Err(anyhow!(msg));
    }
    buffer.save(&path)?;
    Ok(serde_json::to_value(path)?)
}

pub fn get_required_string(args: &Value, key: &str) -> Result<String> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_u32(args: &Value, key: &str) -> Result<u32> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_i32(args: &Value, key: &str) -> Result<i32> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_f64(args: &Value, key: &str) -> Result<f64> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_point(args: &Value, key: &str) -> Result<Point> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_rgb_offset(args: &Value, key: &str) -> Result<RgbOffset> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_relative_colored_points(
    args: &Value,
    key: &str,
) -> Result<Vec<RelativeColoredPoint>> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_required_hex_colors(args: &Value, key: &str) -> Result<Vec<HexColor>> {
    args.get(key)
        .ok_or_else(|| anyhow!(t!("WebSocket argument is missing.", argument = key)))
        .and_then(|v| {
            serde_json::from_value(v.clone()).map_err(|e| {
                anyhow!(t!(
                    "Failed to deserialize WebSocket argument.",
                    argument = key,
                    error = e.to_string()
                ))
            })
        })
}

pub fn get_project_dir() -> Result<PathBuf> {
    app::get().try_with_project(|project| project.path.clone())
}

pub fn load_template_from_project(project_dir: &Path, subpath: &str) -> Result<RgbaImage> {
    let template_dir = relative_template_dir();
    let path = project_dir
        .join(&template_dir)
        .join(format!("{}.png", subpath));
    if !path.exists() {
        return Err(anyhow!(t!(
            "File does not exist.",
            path = path.to_str().unwrap()
        )));
    }
    Ok(ImageReader::open(&path)?.decode()?.into_rgba8())
}

pub fn load_template_from_cache(subpath: &str) -> Result<RgbaImage> {
    let template_dir = relative_template_dir();
    let path = app::get()
        .resource_dir()
        .join(".cache")
        .join(&template_dir)
        .join(format!("{}.png", subpath));
    if !path.exists() {
        return Err(anyhow!(t!(
            "File does not exist.",
            path = path.to_str().unwrap()
        )));
    }
    Ok(ImageReader::open(&path)?.decode()?.into_rgba8())
}

pub fn get_image_args(args: &Value) -> Result<(RgbaImage, Point, Size, f64, u32)> {
    let subpath = get_required_string(args, "subpath")?;
    let app = app::get();

    let template_buffer = if app.role() == Role::Listener {
        load_template_from_cache(&subpath)?
    } else {
        let project_dir = get_project_dir()?;
        load_template_from_project(&project_dir, &subpath)?
    };

    let start_point = get_required_point(args, "start_point")?;
    let end_point = get_required_point(args, "end_point")?;
    let threshold = get_required_f64(args, "threshold")?;
    let min_template_side = get_required_u32(args, "min_template_side")?;
    let size = Size::new_from_start_end_points(start_point, end_point)?;

    Ok((
        template_buffer,
        start_point,
        size,
        threshold,
        min_template_side,
    ))
}

pub fn get_template_args(args: &Value) -> Result<(String, Base64Png)> {
    let name = get_required_string(args, "name")?;
    let template = get_required_string(args, "template")? as Base64Png;

    Ok((name, template))
}
