use crate::commands::server::Engine as ServerEngine;
use crate::project_success;
use crate::{
    app,
    commands::frontend::utils::{CommandResult, relative_template_dir},
    interpreter::PythonInterpreter,
    project::{Info, Project},
    project_error, project_info, project_warn,
    types::{Base64Png, Base64PngExt as _},
};
use anyhow::anyhow;
use encoding_rs::GBK; //command output need to support english(utf8) and chinese(gbk)
use fs_extra::dir;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::{
    io::{BufRead as _, BufReader, Read},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

#[tauri::command]
pub fn create_project(name: String, language: String, path: String) -> CommandResult<()> {
    let project_path = PathBuf::from(path);

    if project_path.exists() {
        return Err("Folder already exists.".into());
    }

    match language.as_str() {
        "python" => {
            let interpreter = PythonInterpreter::default();
            interpreter.init(&name, &project_path)?;
            Ok(())
        }
        _ => Err("Unsupported language.".into()),
    }
}

#[tauri::command]
pub fn open_project(path: String) -> CommandResult<Info> {
    let path = PathBuf::from(path);

    // python project
    {
        let config_path = path.join("pyproject.toml");
        if config_path.exists() {
            let system_interpreter = PythonInterpreter::default();
            let should = match system_interpreter.should_reinit(&path) {
                Ok(s) => s,
                Err(_) => true,
            };

            if should {
                system_interpreter.reinit(&path)?;
            }

            let project_interpreter = Arc::new(PythonInterpreter::new_from_project(&path));
            let project = Project::new(path, project_interpreter);
            let info = project.info();
            let app = app::get();
            let mut guard = app.project_mut()?;
            *guard = Some(project);
            return Ok(info);
        }
    }

    Err("Illegal project.".into())
}

#[tauri::command]
pub fn open_project_folder() -> CommandResult<()> {
    app::get().try_with_project(|project| {
        project.open_folder();
    })?;
    Ok(())
}

#[tauri::command]
pub fn open_project_in_editor() -> CommandResult<()> {
    app::get().try_with_project(|project| {
        let _ = project.open_in_editor();
    })?;
    Ok(())
}

#[tauri::command]
pub fn save_template(name: String, template: Base64Png) -> CommandResult<()> {
    let name = name + ".png";
    let buffer = template.to_buffer().unwrap();
    let project_path = app::get().try_with_project(|project| project.path.clone())?;
    let template_dir = relative_template_dir();
    let path = Path::new(&project_path).join(&template_dir).join(&name);
    let parent_path = path.parent().unwrap();

    if let Err(error) = dir::create_all(parent_path, false) {
        let msg = t!(
            "Failed to create folder.",
            path = parent_path.to_string_lossy(),
            error = error.to_string()
        );
        return Err(msg.into());
    }

    buffer.save(path)?;
    Ok(())
}

#[tauri::command]
pub fn set_project_pid(pid: u32) -> CommandResult<()> {
    let app = app::get();
    app.try_with_project(|project| {
        project.interpreter.set_pid(pid);
    })?;
    Ok(())
}

#[tauri::command]
pub fn run_script(file: String) -> CommandResult<()> {
    let app = app::get();

    // project is required.
    {
        app.try_with_project(|_| {})?;
    }

    // run capturer if local server
    {
        if app.remote_server_address() == ServerEngine::local_address()
            && !app.capturer().is_running()
        {
            if let Err(e) = app.with_capturer(|capturer| {
                capturer.clear_frame();
                capturer.start_background()
            }) {
                return Err(e.into());
            }
        }
    }

    // run script
    {
        let project_path = app.try_with_project(|project| project.path.clone())?;
        let file_path = project_path.join(&file);
        let on_spawned = move |pid| {
            let msg = t!("The script is now running.", pid = pid);
            project_success!("{}", &msg);
        };
        let on_stdout = move |stdout| {
            let reader = BufReader::new(stdout);
            output_handle(reader, move |line| {
                project_info!("{}", &line);
            });
        };
        let on_stderr = move |stderr| {
            let reader = BufReader::new(stderr);
            output_handle(reader, move |line| {
                project_error!("{}", &line);
            });
        };
        let on_exit = move |pid, exit_status| {
            let msg = t!(
                "The script has finished running.",
                pid = pid,
                status = exit_status
            );
            project_success!("{}", &msg);
        };

        thread::spawn(move || {
            // waiting to get frame if local server
            if app.remote_server_address() == ServerEngine::local_address() {
                loop {
                    if app.get_frame_arc().is_ok() {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }

            // run script
            let _ = app.try_with_project(|project| {
                if let Err(e) = project.interpreter.run(
                    &file_path,
                    &app.remote_server_address(),
                    Box::new(on_spawned),
                    Box::new(on_stdout),
                    Box::new(on_stderr),
                    Box::new(on_exit),
                ) {
                    project_error!("{}", e);
                }
            });

            // stop capturer
            app.with_capturer(|capture| {
                capture.stop();
            });
        });
    }

    Ok(())
}

#[tauri::command]
pub fn stop_run_script() -> CommandResult<()> {
    let app = app::get();
    // project is required.
    {
        app.try_with_project(|_| {})?;
    }

    // stop capturer
    {
        if app.capturer().is_running() {
            app.with_capturer(|capturer| capturer.stop());
        }
    }

    // check if script is running
    {
        let pid = app.try_with_project(|project| project.interpreter.pid())?;

        if pid == 0 {
            let msg = t!("No script is running.");
            project_warn!("{}", msg);
            return Ok(());
        }
    }

    // stop interpreter
    {
        let _ = app.try_with_project(|project| project.interpreter.stop());
    }

    Ok(())
}

#[tauri::command]
pub fn get_project_entry_file() -> CommandResult<String> {
    let file = app::get().try_with_project(|project| project.interpreter.entry_file())?;
    Ok(file)
}

#[tauri::command]
pub fn is_project_running() -> CommandResult<bool> {
    let pid = app::get().try_with_project(|project| project.interpreter.pid())?;
    Ok(pid > 0)
}

fn output_handle<R, F>(reader: BufReader<R>, mut line_handler: F)
where
    R: Read + Send + 'static,
    F: FnMut(String) + Send + 'static,
{
    thread::spawn(move || {
        let mut reader = reader;
        let mut buffer = Vec::new();

        loop {
            buffer.clear();
            let bytes_read = reader.read_until(b'\n', &mut buffer).unwrap_or(0);
            if bytes_read == 0 {
                break;
            }

            let line = match std::str::from_utf8(&buffer) {
                Ok(utf8_str) => utf8_str.trim_end_matches(&['\r', '\n'][..]).to_string(),
                Err(_) => {
                    let (cow, _, _) = GBK.decode(&buffer);
                    cow.trim_end_matches(&['\r', '\n'][..]).to_string()
                }
            };

            line_handler(line);
        }
    });
}
