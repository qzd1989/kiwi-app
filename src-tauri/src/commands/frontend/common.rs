use crate::{interpreter::PythonInterpreter, project::Project};

use super::utils::*;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub fn path_exists(path: String) -> CommandResult<bool> {
    let path = PathBuf::from(path);
    Ok(path.exists())
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
pub fn xattr_interpreter() -> CommandResult<()> {
    // python is supported.
    {
        PythonInterpreter::xattr()?;
    }
    Ok(())

    // let interpreter = {
    //     let default_interpreter = get_default_interpreter();
    //     default_interpreter.to_str().unwrap().to_string()
    // };
    // Command::new("xattr")
    //     .args(&["-r", "-d", "com.apple.quarantine", &interpreter])
    //     .no_window()
    //     .spawn()
    //     .and_then(|mut child| child.wait())
    //     .map_err(|e| {
    //         anyhow!(t!(
    //             "Failed to remove quarantine attribute from Python.",
    //             error = e.to_string()
    //         ))
    //         .to_string()
    //     })?;
    // Ok(())
}
