use crate::app;
use crate::extensions::AppHandleExt as _;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Level {
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(&self) -> String {
        match self {
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub level: Level,
    pub message: String,
}

impl Log {
    fn new(level: Level, message: impl Into<String>) -> Self {
        let message = message.into();
        Self { level, message }
    }

    // main:msg:{info|warn|error}
    fn target(&self, window_label: impl Into<String>) -> String {
        format!(
            "{}:msg:{}",
            window_label.into(),
            self.level.as_str().to_lowercase()
        )
    }

    #[allow(dead_code)]
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Level::Info, message)
    }

    #[allow(dead_code)]
    pub fn warn(message: impl Into<String>) -> Self {
        Self::new(Level::Warn, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Level::Error, message)
    }

    pub fn to_main(&self) {
        app::get()
            .app_handle()
            .emit_with_timestamp(&self.target("main"), &self.message);
    }
}
