use super::utils::*;
use crate::{
    app,
    app::VersionOrdering,
    types::api::{PlatformInfo, Release},
};

#[tauri::command]
pub fn set_role_to_listener() -> CommandResult<()> {
    app::get().to_listener();
    Ok(())
}

#[tauri::command]
pub fn set_role_to_user() -> CommandResult<()> {
    app::get().to_user();
    Ok(())
}

#[tauri::command]
pub fn get_app_name() -> CommandResult<String> {
    Ok(app::get().name())
}

#[tauri::command]
pub fn get_relative_template_dir() -> String {
    relative_template_dir().to_str().unwrap().to_string()
}

#[tauri::command]
pub fn get_app_version() -> CommandResult<String> {
    Ok(app::get().version().to_string())
}

#[tauri::command]
pub async fn get_release() -> CommandResult<Option<PlatformInfo>> {
    let release = app::get()
        .api_get::<Release>("/version.json", [("locale", app::get().locale())])
        .await
        .map_err(|e| e.to_string())?;
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let key = format!("{}-{}", os, arch);

    if let Some(platform_info) = release.platforms.get(&key) {
        if compare_versions(&platform_info.version, &app::get().version())
            == VersionOrdering::Greater
        {
            return Ok(Some(platform_info.clone()));
        }
    }

    Ok(None)
}

pub fn compare_versions(v1: &str, v2: &str) -> VersionOrdering {
    let a: Vec<u32> = v1.split('.').filter_map(|s| s.parse().ok()).collect();
    let b: Vec<u32> = v2.split('.').filter_map(|s| s.parse().ok()).collect();
    let len = a.len().max(b.len());

    for i in 0..len {
        let num1 = *a.get(i).unwrap_or(&0);
        let num2 = *b.get(i).unwrap_or(&0);

        if num1 > num2 {
            return VersionOrdering::Greater;
        } else if num1 < num2 {
            return VersionOrdering::Less;
        }
    }

    VersionOrdering::Equal
}
