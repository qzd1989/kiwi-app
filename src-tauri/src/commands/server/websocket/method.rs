use crate::commands::server::{frame, input, utils};
use anyhow::{Result, anyhow};
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy)]
pub enum Method {
    SaveTemplate,
    Capture,
    HealthCheck,
    FindImage,
    FindImages,
    FindRelativeColors,
    FindColors,
    RecognizeText,
    SaveFrame,
    ClickLeft,
    ClickRight,
    PressLeft,
    PressRight,
    ReleaseLeft,
    ReleaseRight,
    MoveAbsolute,
    MoveRelative,
    GetMouseLocation,
    ScrollVertical,
    ScrollHorizontal,
    PressKey,
    ReleaseKey,
    ClickKey,
    InputText,
}

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "health_check" => Ok(Self::HealthCheck),
            "find_image" => Ok(Self::FindImage),
            "find_images" => Ok(Self::FindImages),
            "find_relative_colors" => Ok(Self::FindRelativeColors),
            "find_colors" => Ok(Self::FindColors),
            "recognize_text" => Ok(Self::RecognizeText),
            "save_frame" => Ok(Self::SaveFrame),
            "click_left" => Ok(Self::ClickLeft),
            "click_right" => Ok(Self::ClickRight),
            "press_left" => Ok(Self::PressLeft),
            "press_right" => Ok(Self::PressRight),
            "release_left" => Ok(Self::ReleaseLeft),
            "release_right" => Ok(Self::ReleaseRight),
            "move_absolute" => Ok(Self::MoveAbsolute),
            "move_relative" => Ok(Self::MoveRelative),
            "get_mouse_location" => Ok(Self::GetMouseLocation),
            "scroll_vertical" => Ok(Self::ScrollVertical),
            "scroll_horizontal" => Ok(Self::ScrollHorizontal),
            "press_key" => Ok(Self::PressKey),
            "release_key" => Ok(Self::ReleaseKey),
            "click_key" => Ok(Self::ClickKey),
            "input_text" => Ok(Self::InputText),
            "capture" => Ok(Self::Capture),
            "save_template" => Ok(Self::SaveTemplate),
            other => Err(anyhow!("Unknown method: {}", other)),
        }
    }
}

impl Method {
    pub fn handle(&self, args: Value) -> Result<Value> {
        match self {
            Method::HealthCheck => Ok(json!({"health":"ok"})),
            Method::FindImage => frame::find_image(args),
            Method::FindImages => frame::find_images(args),
            Method::FindRelativeColors => frame::find_relative_colors(args),
            Method::FindColors => frame::find_colors(args),
            Method::RecognizeText => frame::recognize_text(args),
            Method::SaveFrame => frame::save(args),
            Method::ClickLeft => input::click_left(args),
            Method::ClickRight => input::click_right(args),
            Method::PressLeft => input::press_left(args),
            Method::PressRight => input::press_right(args),
            Method::ReleaseLeft => input::release_left(args),
            Method::ReleaseRight => input::release_right(args),
            Method::MoveAbsolute => input::move_absolute(args),
            Method::MoveRelative => input::move_relative(args),
            Method::GetMouseLocation => input::get_mouse_location(args),
            Method::ScrollVertical => input::scroll_vertical(args),
            Method::ScrollHorizontal => input::scroll_horizontal(args),
            Method::PressKey => input::press_key(args),
            Method::ReleaseKey => input::release_key(args),
            Method::ClickKey => input::click_key(args),
            Method::InputText => input::input_text(args),
            Method::Capture => frame::capture(),
            Method::SaveTemplate => utils::save_template(args),
        }
    }
}
