use super::{Level, Message};
use crate::{app, extensions::AppHandleExt};
use anyhow::Result;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct Log {
    project_path: PathBuf,
}

impl Log {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    pub fn info(&self, message: impl Into<String>) -> Result<()> {
        let message = message.into();
        app::get()
            .app_handle()
            .emit_with_timestamp("project:log:info", &message.clone());
        Message::new(Level::Info, message).save(&self.project_path)
    }

    pub fn warn(&self, message: impl Into<String>) -> Result<()> {
        let message = message.into();
        app::get()
            .app_handle()
            .emit_with_timestamp("project:log:warn", &message.clone());
        Message::new(Level::Warn, message).save(&self.project_path)
    }

    pub fn error(&self, message: impl Into<String>) -> Result<()> {
        let message = message.into();
        app::get()
            .app_handle()
            .emit_with_timestamp("project:log:error", &message.clone());
        Message::new(Level::Error, message).save(&self.project_path)
    }

    pub fn success(&self, message: impl Into<String>) -> Result<()> {
        let message = message.into();
        app::get()
            .app_handle()
            .emit_with_timestamp("project:log:success", &message.clone());
        Message::new(Level::Success, message).save(&self.project_path)
    }
}
