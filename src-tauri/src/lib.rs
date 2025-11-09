#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en-US");

use std::sync::Arc;
use tauri::Manager;

pub mod app;
pub mod capture;
pub mod commands;
pub mod extensions;
pub mod input;
pub mod interpreter;
pub mod project;
pub mod types;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .setup(|app| {
            // init
            let app_handle = Arc::new(app.app_handle().clone());
            app::get().init_app_handle(app_handle.clone())?;

            // set locale
            app::get().set_locale(app::get().locale());

            Ok(())
        })
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
            // commands::frontend::common::xattr_python,
            commands::frontend::common::unprotect_windows,
            commands::frontend::common::protect_windows,
            commands::frontend::server::shutdown_server,
            commands::frontend::server::start_local_server,
            commands::frontend::server::start_any_server,
            commands::frontend::server::is_remote_server_alive,
            commands::frontend::server::get_local_server_address,
            commands::frontend::server::get_lan_server_address,
            commands::frontend::server::get_remote_server_address,
            commands::frontend::server::set_remote_server_address,
            commands::frontend::app::get_app_name,
            commands::frontend::app::get_app_version,
            commands::frontend::app::set_role_to_listener,
            commands::frontend::app::set_role_to_user,
            commands::frontend::app::get_relative_template_dir,
            commands::frontend::capture::run_capturer,
            commands::frontend::capture::stop_capturer,
            commands::frontend::capture::is_capturer_running,
            commands::frontend::app::get_release,
            commands::frontend::project::create_project,
            commands::frontend::project::open_project,
            commands::frontend::project::open_project_in_editor,
            commands::frontend::project::open_project_folder,
            commands::frontend::project::save_template,
            commands::frontend::project::run_script,
            commands::frontend::project::stop_run_script,
            commands::frontend::project::get_project_entry_file,
            commands::frontend::project::is_project_running,
            commands::frontend::project::set_project_pid,
            commands::frontend::project::template_exists,
            commands::frontend::project::save_screenshot,
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
            commands::frontend::code::generate_move_absolute_code
        ]);

    if cfg!(target_os = "macos") {
        // builder = builder.plugin(tauri_plugin_macos_permissions::init()); todo
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
