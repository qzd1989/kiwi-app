use crate::interpreter::code::Code;
use anyhow::Result;
use std::any::Any;
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, ExitStatus};
use std::sync::Arc;
use std::sync::atomic::AtomicU32;

pub trait Interpreter {
    fn as_any(&self) -> &dyn Any;

    fn code(&self) -> Box<dyn Code>;

    fn run(
        &self,
        file_path: &Path,
        endpoint: &str,
        on_spawned: Box<dyn Fn(u32) + Send>,
        on_stdout: Box<dyn Fn(ChildStdout) + Send>,
        on_stderr: Box<dyn Fn(ChildStderr) + Send>,
        on_exit: Box<dyn Fn(u32, ExitStatus) + Send>,
    ) -> Result<()>;

    fn stop(&self) -> bool;

    fn pid(&self) -> u32;
    fn pid_arc(&self) -> Arc<AtomicU32>;
    fn set_pid(&self, pid: u32);

    fn entry_file(&self) -> String;
}
