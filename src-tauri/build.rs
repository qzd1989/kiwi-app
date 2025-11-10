use fs_extra::dir::{self, CopyOptions};
use pyproject::PyProject;
use regex::Regex;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tauri_build::is_dev;

const PYTHON_VERSION: &str = "3.14";
const PYTHON_TYPE: &str = "cpython";

fn main() {
    // 1 init things and prepare to sync to {target_dir}
    {
        // 1.1 init python
        init_python();
        // 1.2 build wheels
        build_wheels();
        // 1.3 add uv to app python
        install_uv();
    }

    //2 build and sync assets/python to {target_dir}/python
    tauri_build();

    // 3 app python have no uv module in dev mode
    // if is_dev() {
    // init_app_python();
    // }
}

fn install_uv() {
    let app = App::new();
    app.xattr(&app.assets_app_python_path());
    let wheels_path = app.assets_wheels_path();
    let uv_path = match app.match_one(&wheels_path, r"^uv-.*\.whl$") {
        Some(p) => p,
        None => panic!("uv wheel not found."),
    };
    let find_links = format!("--find-links={}", &wheels_path.to_string_lossy());
    let output = app
        .assets_app_python_command()
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("--no-index")
        .arg(&find_links)
        .arg(&uv_path)
        .output()
        .expect("Failed to install uv for app python.");

    if !output.status.success() {
        panic!("Failed to install uv for app python.");
    }
}

fn init_app_python() {
    let app = App::new();
    app.xattr(&app.target_app_python_path());
    let wheels_path = app.target_wheels_path();
    let uv_path = match app.match_one(&wheels_path, r"^uv-.*\.whl$") {
        Some(p) => p,
        None => panic!("uv wheel not found."),
    };
    let find_links = format!("--find-links={}", &wheels_path.to_string_lossy());
    let output = app
        .target_app_python_command()
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("--no-index")
        .arg(&find_links)
        .arg(&uv_path)
        .output()
        .expect("Failed to install uv for app python.");

    if !output.status.success() {
        panic!("Failed to install uv for app python.");
    }
}

fn init_python() {
    let app = App::new();
    let options = dir::CopyOptions::new().overwrite(true).content_only(true);
    let from = app.assets_source_python_dir();
    let to = app.assets_app_python_dir();
    dir::copy(&from, &to, &options)
        .expect("copy assets_source_python to assets_app_python failed.");
    // for build whl
    let to = app.target_temp_python_dir();
    if !to.exists() {
        fs::create_dir_all(&to).expect("Failed to create target_temp_python_dir.");
    }
    dir::copy(&from, &to, &options)
        .expect("copy assets_source_python to target_temp_python_dir failed.");
    app.xattr(&to);
}

fn build_wheels() {
    let app = App::new();
    let package_dir = app
        .assets_dir()
        .join("python")
        .join("packages")
        .join("kiwi");
    let wheels_dir = app
        .assets_dir()
        .join("python")
        .join("project_template")
        .join("wheels");
    let kiwi_wheel_file_name = format!(
        "kiwi-{}-py3-none-any.whl",
        PyProject::default().project.version
    );
    let kiwi_wheel_path = wheels_dir.join(&kiwi_wheel_file_name);

    // skip build if in debug mode
    {
        if is_dev() && kiwi_wheel_path.exists() {
            return;
        }
    }

    // clean wheels dir
    {
        if wheels_dir.exists() {
            fs::remove_dir_all(&wheels_dir).expect("Failed to clean wheels dir.");
        }
        fs::create_dir_all(&wheels_dir).expect("Failed to create wheels dir.");
    }

    // target_temp_python_command init
    {
        let output = app
            .target_temp_python_command()
            .args(&["-u", "-m", "pip", "install", "--upgrade", "pip"])
            .output()
            .expect("Failed to upgrade target_temp_python_command module `pip`");

        if !output.status.success() {
            panic!("Failed to upgrade target_temp_python_command module `pip`");
        }

        let output = app
            .target_temp_python_command()
            .args(&["-u", "-m", "pip", "install", "build"])
            .output()
            .expect("Failed to install target_temp_python_command module `build`");

        if !output.status.success() {
            panic!("Failed to install target_temp_python_command module `build`");
        }
    }

    // build kiwi
    {
        let output = app
            .target_temp_python_command()
            .args(&[
                "-u",
                "-m",
                "build",
                "-o",
                wheels_dir.to_str().unwrap(),
                package_dir.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to build module `kiwi`");

        if !output.status.success() {
            panic!("Failed to build module `kiwi`");
        }
    }

    // download kiwi dependencies
    {
        let output = app
            .target_temp_python_command()
            .args(&[
                "-u",
                "-m",
                "pip",
                "download",
                kiwi_wheel_path.to_str().unwrap(),
                "-d",
                wheels_dir.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to download dependencies of `kiwi` module");

        if !output.status.success() {
            panic!("Failed to download dependencies of `kiwi` module");
        }
    }

    // clean up unnecessary and old files
    {
        app.match_mutiple(&wheels_dir, r".*\.tar(\.gz)?$")
            .iter()
            .for_each(|file| {
                fs::remove_file(file).expect(&format!("Failed to delete {:?}", file));
            });
        app.match_mutiple(&wheels_dir, r"^kiwi.*\.whl$")
            .iter()
            .for_each(|file| {
                if &kiwi_wheel_path != file {
                    fs::remove_file(file)
                        .expect(&format!("Failed to delete old kiwi wheel: {:?}", file));
                }
            });
    }
}

fn tauri_build() {
    match System::os() {
        Os::Macos => tauri_build::build(),
        Os::Windows => {
            let windows = tauri_build::WindowsAttributes::new().app_manifest(
                r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#,
            );
            tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
                .expect("Failed to run windows tauri_build.")
        }
    }
}
mod pyproject {
    use anyhow::Result;
    use serde::Deserialize;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct PyProject {
        #[serde(rename = "build-system")]
        build_system: BuildSystem,

        pub project: Project,

        tool: Tool,
    }
    impl Default for PyProject {
        fn default() -> Self {
            let base_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
            let toml_path = base_dir
                .join("assets")
                .join("python")
                .join("packages")
                .join("kiwi")
                .join("pyproject.toml");
            let toml_content = fs::read_to_string(&toml_path)
                .unwrap()
                .trim_start_matches('\u{feff}')
                .to_string();
            let pyproject = PyProject::load_from_toml_content(toml_content)
                .expect("load kiwi project toml failed.");
            pyproject
        }
    }

    impl PyProject {
        pub fn load_from_toml_content(toml_content: String) -> Result<PyProject> {
            let config: PyProject =
                toml::from_str(&toml_content).expect("Load config of kiwi module failed.");
            Ok(config)
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct BuildSystem {
        requires: Vec<String>,
        #[serde(rename = "build-backend")]
        build_backend: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct Project {
        name: String,
        pub version: String,
        description: String,
        authors: Vec<Author>,
        readme: String,
        #[serde(rename = "requires-python")]
        requires_python: String,
        dependencies: Vec<String>,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Author {
        name: String,
        email: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Tool {
        setuptools: ToolSetuptools,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct ToolSetuptools {
        packages: Vec<String>,
    }
}

struct System {}

#[derive(PartialEq)]
enum Os {
    Macos,
    Windows,
}

impl Os {
    pub fn to_str(&self) -> &'static str {
        match self {
            Os::Macos => "macos",
            Os::Windows => "windows",
        }
    }
}

enum Profile {
    Debug,
    Release,
}

impl Profile {
    pub fn to_str(&self) -> &'static str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
        }
    }
}

enum Arch {
    x86_64,
    Aarch64,
}

impl Arch {
    pub fn to_str(&self) -> &'static str {
        match self {
            Arch::x86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
        }
    }
}

impl System {
    fn is_macos() -> bool {
        System::os() == Os::Macos
    }

    fn os() -> Os {
        if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::Macos
        } else {
            panic!("Unsupported operating system");
        }
    }

    fn profile() -> Profile {
        match env::var("PROFILE").unwrap().as_str() {
            "debug" => Profile::Debug,
            "release" => Profile::Release,
            _ => panic!("Unsupported profile: {}", env::var("PROFILE").unwrap()),
        }
    }

    fn arch() -> Arch {
        if cfg!(target_arch = "x86_64") {
            Arch::x86_64
        } else if cfg!(target_arch = "aarch64") {
            Arch::Aarch64
        } else {
            panic!("Unsupported architecture");
        }
    }
}

struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
    //
    fn base_dir(&self) -> PathBuf {
        PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap())
    }

    // assets/
    fn assets_dir(&self) -> PathBuf {
        self.base_dir().join("assets")
    }

    // target/{profile}/
    fn target_dir(&self) -> PathBuf {
        self.base_dir()
            .join("target")
            .join(System::profile().to_str())
    }

    // assets/python/interpreters/cpython-{version}-{os}-{arch}/
    fn assets_source_python_dir(&self) -> PathBuf {
        self.assets_dir()
            .join("python")
            .join("interpreters")
            .join(format!(
                "{}-{}-{}-{}",
                PYTHON_TYPE,
                PYTHON_VERSION,
                System::os().to_str(),
                System::arch().to_str()
            ))
    }

    // assets/python/interpreter/
    fn assets_app_python_dir(&self) -> PathBuf {
        self.assets_dir().join("python").join("interpreter")
    }

    // assets/python/interpreter/(bin/python{version}|python.exe)
    fn assets_app_python_path(&self) -> PathBuf {
        match System::os() {
            Os::Macos => self
                .assets_app_python_dir()
                .join("bin")
                .join(format!("python{}", PYTHON_VERSION)),
            Os::Windows => self.assets_app_python_dir().join("python.exe"),
        }
    }

    fn assets_app_python_command(&self) -> Command {
        Command::new(&self.assets_app_python_path())
    }

    // assets/python/project_template/wheels/
    fn assets_wheels_path(&self) -> PathBuf {
        self.assets_dir()
            .join("python")
            .join("project_template")
            .join("wheels")
    }

    // for temp use
    // target/{profile}/python/temp_interpreter
    fn target_temp_python_dir(&self) -> PathBuf {
        self.target_dir().join("python").join("temp_interpreter")
    }

    // for temp use
    // target/{profile}/python/temp_interpreter/(bin/python{version}|python.exe)
    fn target_temp_python_path(&self) -> PathBuf {
        match System::os() {
            Os::Macos => self
                .target_temp_python_dir()
                .join("bin")
                .join(format!("python{}", PYTHON_VERSION)),
            Os::Windows => self.target_temp_python_dir().join("python.exe"),
        }
    }

    fn target_temp_python_command(&self) -> Command {
        Command::new(&self.target_temp_python_path())
    }

    // {target_dir}/python/interpreter/
    fn target_app_python_dir(&self) -> PathBuf {
        self.target_dir().join("python").join("interpreter")
    }

    // target/{profile}/python/interpreter/(bin/python{version}|python.exe)
    fn target_app_python_path(&self) -> PathBuf {
        match System::os() {
            Os::Macos => self
                .target_app_python_dir()
                .join("bin")
                .join(format!("python{}", PYTHON_VERSION)),
            Os::Windows => self.target_app_python_dir().join("python.exe"),
        }
    }

    fn target_app_python_command(&self) -> Command {
        Command::new(&self.target_app_python_path())
    }

    // target/{profile}/python/project_template/
    fn target_project_template_dir(&self) -> PathBuf {
        self.target_dir().join("python").join("project_template")
    }

    // target/{profile}/python/project_template/wheels/
    fn target_wheels_path(&self) -> PathBuf {
        self.target_project_template_dir().join("wheels")
    }

    fn xattr(&self, path: &PathBuf) {
        let output = Command::new(&"xattr")
            .args(&["-r", "-d", "com.apple.quarantine", path.to_str().unwrap()])
            .output()
            .expect(&format!("Failed to xattr {:?}", &path));
        if !output.status.success() {
            println!("cargo:warning=xattr {:?} failed.", &path);
        }
    }

    /// 遍历目录，返回最新的匹配正则 pattern 的文件路径
    pub fn match_one(&self, dir: &Path, pattern: &str) -> Option<PathBuf> {
        let re = Regex::new(pattern).ok()?;
        let entries = fs::read_dir(dir).ok()?;

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if re.is_match(file_name) {
                            return Some(path);
                        }
                    }
                }
            }
        }
        None
    }

    /// 遍历目录，返回所有匹配正则 pattern 的文件路径
    pub fn match_mutiple(&self, dir: &Path, pattern: &str) -> Vec<PathBuf> {
        let mut matched_files = Vec::new();
        let re = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return matched_files, // 正则无效返回空列表
        };

        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return matched_files, // 目录不存在返回空列表
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if re.is_match(file_name) {
                        matched_files.push(path);
                    }
                }
            }
        }

        matched_files
    }
}
