#[derive(Debug, Clone)]
pub enum Level {
    Warn,
    Info,
    Error,
    Success,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Success => "SUCCESS",
        }
    }
}
