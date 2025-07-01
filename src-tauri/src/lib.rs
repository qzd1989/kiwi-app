use std::sync::Arc;
use tauri::Manager;

pub mod app;
pub mod capture;
pub mod commands;
pub mod extensions;
pub mod input;
pub mod interpreter;
pub mod project;
pub mod record;
pub mod types;
pub mod utils;
pub mod websocket;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            let app_handle = Arc::new(app.app_handle().clone());
            let app_handle_event = app_handle.clone();
            let resource_dir = app.path().resource_dir().expect("Can't find resouce dir.");
            app::App::init_resource_dir(resource_dir)?;
            app::App::init_app_handle(app_handle.clone())?;

            //close monitor window when close main window
            let main_window = app_handle_event
                .get_webview_window("main")
                .expect("Main window not found");
            main_window.on_window_event(move |event| {
                if matches!(
                    event,
                    tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed
                ) {
                    if let Some(monitor) = app_handle_event.get_webview_window("monitor") {
                        if let Err(e) = monitor.destroy() {
                            eprintln!("Failed to destroy monitor window: {e}");
                        }
                    }
                }
            });

            // check update
            tauri::async_runtime::spawn(async move {
                app::App::update().await.unwrap();
            });

            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app_handle, _, _| {
            let main_window = app_handle
                .get_webview_window("main")
                .expect("Main window not found");
            let _ = main_window.set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::frontend::common::path_exists,
            commands::frontend::common::xattr_python,
            commands::frontend::common::unprotect_windows,
            commands::frontend::common::protect_windows,
            commands::frontend::common::open_websocket,
            commands::frontend::common::shutdown_websocket,
            commands::frontend::common::is_websocket_alive,
            commands::frontend::app::get_app_name,
            commands::frontend::app::get_app_version,
            commands::frontend::app::save_app_config,
            commands::frontend::app::get_app_config,
            commands::frontend::app::get_relative_image_data_path,
            commands::frontend::project::save_project,
            commands::frontend::project::verify_project,
            commands::frontend::project::init_project,
            commands::frontend::project::reinit_project,
            commands::frontend::project::open_project,
            commands::frontend::project::get_project,
            commands::frontend::project::open_project_in_editor,
            commands::frontend::project::reveal_project_folder,
            commands::frontend::project::save_image,
            commands::frontend::project::get_image,
            commands::frontend::project::get_image_size,
            commands::frontend::project::run_script,
            commands::frontend::project::run_recorder,
            commands::frontend::project::stop_all,
            commands::frontend::frame::find_image,
            commands::frontend::frame::find_images,
            commands::frontend::frame::find_relative_colors,
            commands::frontend::frame::find_colors,
            commands::frontend::frame::recognize_text,
            commands::frontend::capture::request_frame_data,
            commands::frontend::capture::get_monitor_size,
            commands::frontend::code::generate_find_image_code,
            commands::frontend::code::generate_find_images_code,
            commands::frontend::code::generate_find_relative_colors_code,
            commands::frontend::code::generate_find_colors_code,
            commands::frontend::code::generate_recognize_text_code,
        ]);

    if cfg!(target_os = "macos") {
        builder = builder.plugin(tauri_plugin_macos_permissions::init());
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
