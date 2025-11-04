use super::utils::*;
use crate::{app, extensions::AppHandleExt, types::Size};
use std::thread;

#[tauri::command]
pub fn request_frame_data() -> CommandResult<()> {
    thread::spawn(move || {
        app::get().with_capturer(|capturer| {
            let frame = match capturer.grab(2000) {
                Ok(f) => f,
                Err(_) => {
                    return;
                }
            };
            match frame.to_base64_png() {
                Ok(base64_data) => {
                    app::get()
                        .app_handle()
                        .emit_with_timestamp(EmitEvent::UpdateFrame.into(), &base64_data);
                }
                Err(_) => {
                    return;
                }
            }
        });
    });

    Ok(())
}

#[tauri::command]
pub fn get_monitor_size() -> CommandResult<Size> {
    app::get()
        .with_capturer(|capturer| capturer.get_monitor_size())
        .map_err(|error| error.into())
}

#[tauri::command]
pub fn run_capturer() -> CommandResult<()> {
    if app::get().capturer().is_running() {
        return Err("Capturer is still running, please wait...".into());
    }

    if let Err(e) = app::get().with_capturer(|capturer| {
        capturer.clear_frame();
        capturer.start_background()
    }) {
        return Err(e.into());
    }

    Ok(())
}

#[tauri::command]
pub fn stop_capturer() -> CommandResult<()> {
    if app::get().capturer().is_running() {
        app::get().with_capturer(|capturer| {
            capturer.stop();
        });
    }

    Ok(())
}

#[tauri::command]
pub fn is_capturer_running() -> CommandResult<bool> {
    Ok(app::get().capturer().is_running())
}
