use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt as _;
#[cfg(windows)]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

pub trait CommandExt {
    fn no_window(&mut self) -> &mut Command;
}

impl CommandExt for Command {
    fn no_window(&mut self) -> &mut Command {
        #[cfg(target_os = "windows")]
        {
            self.creation_flags(CREATE_NO_WINDOW.0);
        }
        self
    }
}
