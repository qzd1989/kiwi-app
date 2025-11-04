use super::Level;
use anyhow::{Context as _, Result};
use chrono::{DateTime, Local};
use fs_extra::dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Message {
    pub level: Level,
    pub time: DateTime<Local>,
    pub message: String,
}

impl Message {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        let time = Local::now();
        let message = message.into();
        Self {
            level,
            time,
            message,
        }
    }

    /**
     * log content format: [INFO] 2025-11-22 11:22:33.123 {message}
     */
    fn formatted(&self) -> String {
        format!(
            "[{}] {} {}",
            self.level.as_str(),
            self.time.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            self.message
        )
    }

    /**
     * log path: {resource_dir}/log/{info|warn|error}-{date}.log
     */
    pub fn save<T: AsRef<Path>>(&self, log_dir: T) -> Result<()> {
        let log_dir = log_dir.as_ref().to_path_buf();

        if !log_dir.exists() {
            dir::create_all(&log_dir, true)?;
        }

        let date = self.time.format("%Y-%m-%d").to_string();
        let log_path = log_dir.join(format!(
            "{}-{}.log",
            &self.level.as_str().to_lowercase(),
            &date
        ));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .with_context(|| format!("Failed to open log file: {:?}", &log_path))?;
        let log = self.formatted();

        writeln!(file, "{}", log)?;
        Ok(())
    }
}
