use super::{Config, Info, Log};
use crate::extensions::CommandExt;
use crate::interpreter::{Interpreter, PythonConfig, PythonInterpreter};
use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::{fs, thread};

pub struct Project {
    pub path: PathBuf,
    pub interpreter: Arc<dyn Interpreter + Send + Sync>,
}

impl Project {
    pub fn new(path: PathBuf, interpreter: Arc<dyn Interpreter + Send + Sync>) -> Self {
        Self { path, interpreter }
    }

    pub fn is_python_project(&self) -> bool {
        self.interpreter.as_any().is::<PythonInterpreter>()
    }

    pub fn config(&self) -> Box<dyn Config> {
        if self.is_python_project() {
            let path: PathBuf = self.path.join("pyproject.toml");
            if path.exists() {
                let config = PythonConfig::new_from_file(&path).unwrap();
                return Box::new(config);
            }
        }
        unimplemented!()
    }

    pub fn info(&self) -> Info {
        if self.is_python_project() {
            let config = self.config();
            let config = config.as_any().downcast_ref::<PythonConfig>();
            if let Some(config) = config {
                let info = Info::new(
                    config.project.name.to_owned(),
                    config.project.version.to_owned(),
                    config.project.description.to_owned(),
                    self.path.to_string_lossy().into_owned(),
                );
                return info;
            }
        }
        unimplemented!()
    }

    pub fn log(&self) -> Log {
        Log::new(self.path.clone())
    }
}

impl Project {
    pub fn update_endpoint(&self, endpoint: &str) -> Result<()> {
        if self.is_python_project() {
            let launch_path = self.path.join(".vscode").join("launch.json");
            if launch_path.exists() {
                let content = fs::read_to_string(&launch_path)?;
                let mut json: Value = serde_json::from_str(&content)?;

                if let Some(configs) = json
                    .get_mut("configurations")
                    .and_then(|c| c.as_array_mut())
                {
                    for cfg in configs {
                        if let Some(args) = cfg.get_mut("args").and_then(|a| a.as_array_mut()) {
                            for i in 0..args.len() {
                                if args[i] == "--endpoint" && i + 1 < args.len() {
                                    args[i + 1] = Value::String(endpoint.to_string());
                                }
                            }
                        }
                    }
                }

                let new_content = serde_json::to_string_pretty(&json)?;
                fs::write(&launch_path, new_content)?;
            }
        }
        Ok(())
    }

    pub fn open_folder(&self) {
        let path = self.path.clone();
        thread::spawn(move || {
            #[cfg(target_os = "windows")]
            {
                let _ = Command::new("explorer").arg("/select,").arg(&path).spawn();
            }
            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("open").arg("-R").arg(&path).spawn();
            }
        });
    }

    pub fn open_in_editor(&self) -> Result<()> {
        if cfg!(target_os = "macos") {
            Command::new("code")
                .arg(self.path.clone())
                .no_window()
                .spawn()?;
        }

        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg("code")
                .arg(self.path.clone())
                .no_window()
                .spawn()?;
        }

        Ok(())
    }
}
