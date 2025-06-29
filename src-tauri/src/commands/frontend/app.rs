use super::{CommandResult, utils::get_relative_image_data_path_buf};
use crate::APP_NAME;
use crate::app::{App, Config as AppConfig};

#[tauri::command]
pub fn get_app_name() -> CommandResult<String> {
    Ok(APP_NAME.to_string())
}

#[tauri::command]
pub fn get_app_config() -> CommandResult<AppConfig> {
    let config = App::with_config(|config| config.clone());
    Ok(config)
}

#[tauri::command]
pub fn save_app_config(config: AppConfig) -> CommandResult<()> {
    if config.app.websocket_port == 0 {
        return Err("WebSocket port must be greater than 0.".into());
    }

    App::with_config_mut(|app_config| {
        app_config.app.websocket_port = config.app.websocket_port;
        app_config.save()
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
