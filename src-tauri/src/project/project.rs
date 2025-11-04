use super::{Config, Info, Log};
use crate::extensions::CommandExt;
use crate::interpreter::{Interpreter, PythonConfig};
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::thread;

pub struct Project {
    pub path: PathBuf,
    pub interpreter: Arc<dyn Interpreter + Send + Sync>,
}

impl Project {
    pub fn new(path: PathBuf, interpreter: Arc<dyn Interpreter + Send + Sync>) -> Self {
        Self { path, interpreter }
    }

    pub fn config(&self) -> Box<dyn Config> {
        // python config
        {
            let path: PathBuf = self.path.join("pyproject.toml");
            if path.exists() {
                let config = PythonConfig::new_from_file(&path).unwrap();
                return Box::new(config);
            }
        }

        unimplemented!()
    }

    pub fn info(&self) -> Info {
        let config = self.config();

        // python project info
        {
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
