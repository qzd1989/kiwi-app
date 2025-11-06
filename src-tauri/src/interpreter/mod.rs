mod code;
mod interpreter;

pub mod python;

use code::*;
use std::path::PathBuf;
use std::process::Command;

pub use interpreter::Interpreter;
pub use python::Engine as PythonInterpreter;
pub use python::PyProject as PythonConfig;

fn macos_attr(path: &PathBuf) {
    let output = Command::new(&"xattr")
        .args(&["-r", "-d", "com.apple.quarantine", path.to_str().unwrap()])
        .output()
        .expect("Failed to xattr target_python_interpreter.");
    if !output.status.success() {
        println!("cargo:warning=init_attr xattr target_python_interpreter failed");
    }
}
