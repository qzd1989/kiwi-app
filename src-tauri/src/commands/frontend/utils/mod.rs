mod command_error;
mod emit_event;
pub mod log;

use std::path::PathBuf;

pub use {command_error::CommandResult, emit_event::EmitEvent};

pub fn relative_data_dir() -> PathBuf {
    PathBuf::from("data")
}

pub fn relative_template_dir() -> PathBuf {
    PathBuf::from("data").join("templates")
}
