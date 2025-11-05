use fs_extra::dir;
use pyproject::PyProject;
use regex::Regex;
use simple_zip::{compress, extract};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use tauri_build::is_dev;

const PYTHON_VERSION: &str = "3.14";
const PYTHON_TYPE: &str = "cpython";

fn profile() -> String {
    let profile = env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" | "release" => profile,
        _ => panic!("Unsupported profile: {}", profile),
    }
}

fn target_dir() -> PathBuf {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = profile();
    let dir = PathBuf::from(&cargo_manifest_dir)
        .join("target")
        .join(&profile);
    if !dir.exists() {
        panic!("Target directory does not exist: {}", dir.display());
    }
    dir
}

fn assets_dir() -> PathBuf {
    PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets")
}

fn arch() -> String {
    if cfg!(target_arch = "x86_64") {
        "x86_64".to_string()
    } else if cfg!(target_arch = "aarch64") {
        "aarch64".to_string()
    } else {
        panic!("Unsupported architecture");
    }
}

fn os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        panic!("Unsupported operating system");
    }
}

/**
 * create assets/zips/interpreter.zip
 */
fn init_python() {
    let zip_dir = assets_dir().join("zips");
    let src_dir = assets_dir()
        .join("python")
        .join("interpreters")
        .join(format!(
            "{}-{}-{}-{}",
            PYTHON_TYPE,
            PYTHON_VERSION,
            os(),
            arch()
        ));
    let dst_zip = assets_dir().join("zips").join("interpreter.zip");
    let dst_dir = target_dir().join("python").join("interpreter");

    if !zip_dir.exists() {
        dir::create_all(&zip_dir, true).expect("Failed to create assets/zips dir.");
    }

    if !dst_zip.exists() {
        println!("cargo:warning=compress python interpreter");
        compress(&src_dir, &dst_zip).expect("Compress python interpreter failed.");
    }

    if !dst_dir.exists() {
        println!("cargo:warning=extract python interpreter to target");
        dir::create_all(&dst_dir, true).expect("Failed to create target/python/interpreter dir.");
        extract(fs::File::open(dst_zip).unwrap(), &dst_dir, true)
            .expect("Failed to extract python interpreter.");
    }
}

fn install_uv() {
    let wheels_path = target_dir()
        .join("python")
        .join("project_template")
        .join("wheels");
    let pattern = r"^uv-.*\.whl$";
    println!(
        "cargo:warning=uv: {}, {}",
        &wheels_path.to_string_lossy(),
        pattern
    );
    let uv_path = match find_one_file_in_dir(&wheels_path, pattern) {
        Some(p) => p,
        None => panic!("Uv wheel not found."),
    };
    let find_links = format!("--find-links={}", &wheels_path.to_string_lossy());
    let output = system_python_command()
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("--no-index")
        .arg(&find_links)
        .arg(&uv_path)
        .output()
        .expect("Failed to install uv.");

    if !output.status.success() {
        panic!("Install uv wheel failed.");
    }
}

fn main() {
    if os() == "windows" && arch() == "aarch64" {
        // windows arm needs onnxruntime todo
        println!("cargo:rustc-link-search=native=C:\\onnxruntime\\lib");
        println!("cargo:rustc-link-lib=onnxruntime");
    }

    init_python();
    build_whl();

    if os() == "macos" {
        tauri_build_macos();
    }

    if os() == "windows" {
        tauri_build_windows();
    }

    install_uv();
}

fn tauri_build_macos() {
    tauri_build::build()
}

/// 程序启动时会弹出 UAC 提示，要求用户允许管理员权限。
fn tauri_build_windows() {
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
        .expect("Failed to run build script.");
}

fn build_whl() {
    let kiwi_package_dir = assets_dir().join("python").join("packages").join("kiwi");
    let wheels_dir = assets_dir()
        .join("python")
        .join("project_template")
        .join("wheels");
    let kiwi_whl_file_name = format!("kiwi-{}-py3-none-any.whl", kiwi_whl_version());
    let kiwi_whl_path = wheels_dir.join(&kiwi_whl_file_name);

    // debug模式下如果已经存在 kiwi whl 则跳过构建
    {
        if is_dev() && kiwi_whl_path.exists() {
            println!("cargo:warning=skip build kiwi whl in dev mode");
            return;
        }
    }

    println!("cargo:warning=clean dir project_template/wheels");
    {
        if wheels_dir.exists() {
            fs::remove_dir_all(&wheels_dir).expect("Failed to clean wheels dir.");
        }
        fs::create_dir_all(&wheels_dir).expect("Failed to create wheels dir.");
    }

    println!("cargo:warning=upgrade system python module: pip");
    {
        let output = system_python_command()
            .args(&["-u", "-m", "pip", "install", "--upgrade", "pip"])
            .output()
            .expect("Failed to upgrade pip");

        if !output.status.success() {
            panic!("Build_whl upgarde pip failed.");
        }
    }

    println!("cargo:warning=install system python module: build");
    {
        let output = system_python_command()
            .args(&["-u", "-m", "pip", "install", "build"])
            .output()
            .expect("Failed to install build");

        if !output.status.success() {
            panic!("Build_whl install build failed.");
        }
    }

    println!("cargo:warning=build module: kiwi");
    {
        let output = system_python_command()
            .args(&[
                "-u",
                "-m",
                "build",
                "-o",
                wheels_dir.to_str().unwrap(),
                kiwi_package_dir.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to build kiwi whl");

        if !output.status.success() {
            panic!("Build_whl build kiwi whl failed.");
        }
    }

    println!("cargo:warning=download dependencies of python module kiwi");
    {
        let output = system_python_command()
            .args(&[
                "-u",
                "-m",
                "pip",
                "download",
                kiwi_whl_path.to_str().unwrap(),
                "-d",
                wheels_dir.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to download dependencies of kiwi whl");

        if !output.status.success() {
            panic!("Build_whl download dependencies of kiwi whl failed.");
        }
    }

    println!("cargo:warning=delete project_template/wheels/*.tar and *.tar.gz and old kiwi whls");
    {
        find_all_files_in_dir(&wheels_dir, r".*\.tar(\.gz)?$")
            .iter()
            .for_each(|file| {
                fs::remove_file(file).expect("Failed to delete .tar or .tar.gz file.");
            });
        find_all_files_in_dir(&wheels_dir, r"^kiwi.*\.whl$")
            .iter()
            .for_each(|file| {
                if file != &kiwi_whl_path {
                    fs::remove_file(file).expect("Failed to delete old kiwi whl file.");
                }
            });
    }
}

fn system_python_command() -> Command {
    let path = {
        if os() == "macos" {
            target_dir()
                .join("python")
                .join("interpreter")
                .join("bin")
                .join(format!("python{}", PYTHON_VERSION))
        } else {
            target_dir()
                .join("python")
                .join("interpreter")
                .join("python.exe")
        }
    };
    if !path.exists() {
        panic!("{} is not exist.", path.to_str().unwrap());
    }
    Command::new(&path)
}

fn kiwi_whl_version() -> String {
    PyProject::default().project.version
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

pub fn find_one_file_in_dir(dir: &Path, pattern: &str) -> Option<PathBuf> {
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
pub fn find_all_files_in_dir(dir: &Path, pattern: &str) -> Vec<PathBuf> {
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
