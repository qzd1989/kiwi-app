use super::CommandResult;
use crate::app::Log;
use crate::interpreter::python::get_default_interpreter;
use crate::websocket;
use anyhow::anyhow;
use std::path::PathBuf;
use std::process::Command;
use tauri::AppHandle;

#[tauri::command]
pub fn path_exists(path: String) -> CommandResult<bool> {
    let path = PathBuf::from(path);
    Ok(path.exists())
}

#[tauri::command]
pub fn xattr_python() -> CommandResult<()> {
    let interpreter = {
        let default_interpreter = get_default_interpreter();
        default_interpreter.to_str().unwrap().to_string()
    };
    Command::new("xattr")
        .args(&["-r", "-d", "com.apple.quarantine", &interpreter])
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|error| {
            anyhow!("Failed to remove quarantine attribute.({})", error).to_string()
        })?;
    Ok(())
}

#[tauri::command]
pub fn unprotect_windows(app_handle: AppHandle, windows: Vec<String>) -> CommandResult<()> {
    crate::capture::Engine::unprotect_windows(&app_handle, &windows)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn protect_windows(app_handle: AppHandle, windows: Vec<String>) -> CommandResult<()> {
    crate::capture::Engine::protect_windows(&app_handle, &windows)
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn is_websocket_alive(port: u16) -> CommandResult<bool> {
    websocket::is_alive(port)
        .await
        .map_err(|error| error.into())
}

#[tauri::command]
pub async fn shutdown_websocket() -> CommandResult<()> {
    websocket::shutdown();
    Ok(())
}

#[tauri::command]
pub async fn open_websocket(port: u16) -> CommandResult<()> {
    if port == 0 {
        return Err("WebSocket port must be greater than 0.".into());
    }

    let serve_failed_handler = |error: anyhow::Error| {
        Log::error(error.to_string()).send_to_app_msg();
    };

    websocket::serve_in_background(port, Box::new(serve_failed_handler))
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}
