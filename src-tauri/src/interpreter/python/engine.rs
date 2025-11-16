use super::PyProject;
use super::code::PythonCode;
use crate::{
    app,
    extensions::CommandExt as _,
    interpreter::{Interpreter, code::Code},
    utils::common::find_file_in_dir,
};
use anyhow::{Result, anyhow};
use fs_extra;
use regex::Regex;
use std::{
    any::Any,
    fs,
    path::PathBuf,
    process::Command,
    sync::{Arc, atomic::AtomicU32},
};
use std::{path::Path, process::Stdio};
use std::{
    process::{ChildStderr, ChildStdout, ExitStatus},
    sync::atomic::Ordering,
};

const PYTHON_VERSION: &str = "3.14";

#[derive(Clone)]
pub struct Engine {
    pub path: PathBuf,
    pub pid: Arc<AtomicU32>,
}

impl Interpreter for Engine {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn code(&self) -> Box<dyn Code> {
        Box::new(PythonCode::default())
    }

    fn run(
        &self,
        file_path: &Path,
        endpoint: &str,
        on_spawned: Box<dyn Fn(u32) + Send>,
        on_stdout: Box<dyn Fn(ChildStdout) + Send>,
        on_stderr: Box<dyn Fn(ChildStderr) + Send>,
        on_exit: Box<dyn Fn(u32, ExitStatus) + Send>,
    ) -> Result<()> {
        // 原子占位，防止重复启动
        if self
            .pid
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            let pid = self.pid.load(Ordering::SeqCst);
            return Err(anyhow!("Script is already running. ({})", pid));
        }

        // 构造命令
        let mut command = self.python();
        command.arg(&file_path).args(&["--endpoint", endpoint]);
        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        // 3️⃣ spawn 子进程
        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(e) => {
                self.set_pid(0); // spawn 出错时重置 PID
                return Err(e.into());
            }
        };

        // 更新真实 PID
        let pid = child.id();
        self.set_pid(pid);
        on_spawned(pid);

        // 启动 stdout/stderr 回调
        if let Some(stdout) = child.stdout.take() {
            on_stdout(stdout);
        }
        if let Some(stderr) = child.stderr.take() {
            on_stderr(stderr);
        }

        // wait出错重置 PID
        let pid_clone = self.pid.clone();
        let exit_status = child.wait().map_err(|e| {
            pid_clone.store(0, Ordering::SeqCst);
            e
        })?;
        on_exit(child.id(), exit_status);
        self.set_pid(0);
        Ok(())
    }

    fn stop(&self) -> bool {
        let pid = self.pid();

        if pid == 0 {
            return false;
        }

        #[cfg(target_os = "windows")]
        {
            let handle = Command::new("taskkill")
                .arg("/F")
                .arg("/PID")
                .arg(pid.to_string())
                .no_window()
                .spawn();
            if let Ok(mut handle) = handle {
                let _ = handle.wait();
            }
        }
        #[cfg(target_os = "macos")]
        {
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
        }
        self.set_pid(0);
        true
    }

    fn set_pid(&self, pid: u32) {
        self.pid.store(pid, Ordering::SeqCst);
    }

    fn pid(&self) -> u32 {
        self.pid.load(Ordering::SeqCst)
    }

    fn pid_arc(&self) -> Arc<AtomicU32> {
        self.pid.clone()
    }

    fn entry_file(&self) -> String {
        "main.py".to_string()
    }
}

impl Engine {
    pub fn xattr() -> Result<()> {
        if cfg!(target_os = "macos") {
            let app_interpreter = Engine::default();
            let app_python_path = app_interpreter.path;
            Command::new("xattr")
                .args(&["-r", "-d", "com.apple.quarantine"])
                .arg(&app_python_path)
                .no_window()
                .spawn()
                .and_then(|mut child| child.wait())
                .map_err(|e| {
                    anyhow!(t!(
                        "Failed to remove quarantine attribute from Python.",
                        error = e.to_string()
                    ))
                })?;
        }
        Ok(())
    }
    pub fn new_from_project(project_path: impl AsRef<Path>) -> Self {
        let project_path = project_path.as_ref().to_path_buf();
        let python_path = {
            let venv_path = project_path.join(".venv");
            if cfg!(target_os = "windows") {
                venv_path.join("Scripts").join("python.exe")
            } else {
                venv_path.join("bin").join("python")
            }
        };
        Self::new(&python_path)
    }

    /**
     * for running script
     */
    fn python(&self) -> Command {
        let mut command = Command::new(&self.path);
        command.args(&["-u"]).no_window();
        command
    }

    /**
     * uv only use for init, don't use it to run script.
     */
    fn uv(&self) -> Command {
        let mut command = Command::new(&self.path);
        command.args(&["-u", "-m", "uv"]).no_window();
        command
    }

    fn uv_init(&self, name: &str, path: &PathBuf) -> Result<()> {
        let output = self
            .uv()
            .arg("init")
            .args(&["--name", &name])
            .arg(&path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}", stderr.to_string());
            return Err(anyhow!(
                "Failed to initialize Python project. ({:?})",
                stderr
            ));
        }

        Ok(())
    }

    fn uv_venv(&self, path: &PathBuf) -> Result<()> {
        let venv_path = path.join(".venv");
        let python_path = Self::default().path;
        let output = self
            .uv()
            .arg("venv")
            .arg("--python")
            .arg(&python_path)
            .arg(&venv_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(t!(
                "Failed to genereate Python venv.",
                error = stderr.to_string()
            )));
        }

        Ok(())
    }

    fn find_latest_kiwi_whl_filename(&self, dir: &Path) -> Option<String> {
        // 匹配 kiwi_assistant-x.y.z-py3-none-any.whl
        let re = Regex::new(r"^kiwi_assistant-(\d+)\.(\d+)\.(\d+)-py3-none-any\.whl$").unwrap();

        let mut latest: Option<(u32, u32, u32, String)> = None;

        for entry in fs::read_dir(dir).ok()? {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy().to_string();

            if let Some(caps) = re.captures(&file_name) {
                let major: u32 = caps[1].parse().ok()?;
                let minor: u32 = caps[2].parse().ok()?;
                let patch: u32 = caps[3].parse().ok()?;

                if latest.is_none()
                    || (major, minor, patch)
                        > (latest.as_ref()?.0, latest.as_ref()?.1, latest.as_ref()?.2)
                {
                    latest = Some((major, minor, patch, file_name.clone()));
                }
            }
        }

        latest.map(|(_, _, _, name)| name)
    }

    fn project_template_name(&self) -> &'static str {
        "project_template"
    }

    fn uv_add_kiwi(&self, project_path: &PathBuf) -> Result<()> {
        // copy app wheels to the project to avoid using an outdated Kiwi wheel.
        {
            let from = app::get()
                .resource_dir()
                .join("python")
                .join(self.project_template_name())
                .join("wheels");
            let to = project_path.join("wheels");
            let options = fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true);
            fs_extra::dir::copy(&from, &to, &options)?;
        }

        // install kiwi
        {
            let wheels_dir = project_path.join("wheels");
            let latest_kiwi_filename = self
                .find_latest_kiwi_whl_filename(&wheels_dir)
                .ok_or_else(|| anyhow!("Kiwi whl not found."))?;
            let latest_kiwi_path = wheels_dir.join(&latest_kiwi_filename);
            let find_links = format!("--find-links={}", &wheels_dir.to_string_lossy());
            let output = self
                .uv()
                .arg("add")
                .arg("--no-index")
                .arg(&find_links)
                .arg("--directory")
                .arg(&project_path)
                .arg(&latest_kiwi_path)
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!(
                    "Failed to install kiwi_assistant lib. ({:?})",
                    stderr
                ));
            }
        }

        Ok(())
    }
}

impl Engine {
    pub fn new(path: &PathBuf) -> Self {
        let path = path.to_owned();
        let pid = Arc::new(AtomicU32::new(0));
        Self { path, pid }
    }

    pub fn init(&self, name: &str, path: &PathBuf) -> Result<()> {
        // copy template
        {
            let template_path = app::get()
                .resource_dir()
                .join("python")
                .join(self.project_template_name());

            let options = fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true);
            fs_extra::dir::copy(&template_path, &path, &options)?;
        }

        self.uv_init(&name, &path)?;
        self.uv_venv(&path)?;
        self.uv_add_kiwi(&path)?;
        Ok(())
    }

    pub fn should_reinit(&self, project_path: &PathBuf) -> Result<bool> {
        // app interpreter is moved?
        {
            let cfg_path = project_path.join(".venv").join("pyvenv.cfg");
            let content = fs::read_to_string(&cfg_path)?;
            let app_interpreter_dir = self
                .path
                .parent()
                .ok_or_else(|| anyhow!("Python bin folder not found."))?
                .to_str()
                .ok_or_else(|| anyhow!("Convert Path to String failed."))?;
            if let None = content.find(app_interpreter_dir) {
                return Ok(true);
            }
        }
        // kiwi whl is outdated?
        {
            let project_kiwi = {
                let pattern = r"kiwi_assistant-\d+(\.\d+)*-py3-none-any\.whl$";
                let path = project_path.join("pyproject.toml");
                let pyproject = PyProject::new_from_file(&path)?;
                let sources = pyproject.tool.uv.sources;
                let kiwi_source = sources
                    .get("kiwi_assistant")
                    .ok_or_else(|| anyhow!("kiwi_assistant lib not found"))?;
                let re = Regex::new(pattern)?;
                let path = re
                    .find(&kiwi_source.path)
                    .ok_or_else(|| anyhow!("Matched pyproject.toml kiwi_assistant.path failed."))?
                    .as_str();
                path.to_string()
            };
            let app_kiwi = {
                let pattern = r"^kiwi_assistant-\d+(\.\d+)*-py3-none-any\.whl$";
                let wheels_path = app::get()
                    .resource_dir()
                    .join("python")
                    .join(self.project_template_name())
                    .join("wheels");
                find_file_in_dir(&wheels_path, pattern)
                    .ok_or_else(|| anyhow!("Kiwi lib not found."))?
            };

            if project_kiwi != app_kiwi {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn reinit(&self, path: &PathBuf) -> Result<()> {
        // rm .venv
        {
            let venv_path = path.join(".venv");
            if venv_path.exists() {
                fs::remove_dir_all(&venv_path)?;
            }
        }

        // rm uv.lock
        {
            let lock_path = path.join("uv.lock");
            if lock_path.exists() {
                fs::remove_file(&lock_path)?;
            }
        }

        self.uv_venv(&path)?;
        self.uv_add_kiwi(&path)?;
        Ok(())
    }

    pub fn app_python_path() -> PathBuf {
        let python_dir = app::get().resource_dir().join("python").join("interpreter");
        let path = if cfg!(target_os = "windows") {
            python_dir.join("python.exe")
        } else {
            python_dir
                .join("bin")
                .join(format!("python{}", PYTHON_VERSION))
        };
        path
    }
}

impl Default for Engine {
    fn default() -> Self {
        let path = Engine::app_python_path();
        let pid = Arc::new(AtomicU32::new(0));
        Self { path, pid }
    }
}
