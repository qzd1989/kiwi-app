use super::{CommandResult, utils::get_relative_image_data_path_buf};
use crate::app::{App, Config as AppConfig};

#[tauri::command]
pub fn get_app_config() -> CommandResult<AppConfig> {
    let config = App::with_config(|config| config.clone());
    Ok(config)
}

#[tauri::command]
pub fn save_app_config(websocket_port: String) -> CommandResult<()> {
    let websocket_port = websocket_port
        .parse::<u16>()
        .map_err(|error| error.to_string())?;

    if websocket_port == 0 {
        return Err("WebSocket port must be greater than 0.".into());
    }

    App::with_config_mut(|config| {
        config.app.websocket_port = websocket_port;
        config.save()
    })?;
    Ok(())
}

#[tauri::command]
pub fn get_relative_image_data_path() -> String {
    get_relative_image_data_path_buf()
        .to_str()
        .unwrap()
        .to_string()
}

#[tauri::command]
pub fn get_app_version() -> CommandResult<String> {
    Ok(App::get_version().to_string())
}
