use rust_embed::Embed;

#[cfg(target_os = "macos")]
#[derive(Embed)]
#[folder = "assets/"]
#[include = "paddle_ocr/*"]
pub struct Asset;

#[cfg(target_os = "windows")]
#[derive(Embed)]
#[folder = "assets/"]
#[include = "paddle_ocr/*"]
pub struct Asset;
