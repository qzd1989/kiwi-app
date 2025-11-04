use super::utils::*;
use crate::commands::server::Engine as ServerEngine;
use crate::{app, app_error};
use local_ip_address::local_ip;

#[tauri::command]
pub async fn start_local_server() -> CommandResult<ServerEngine> {
    let failed_handler = |e: anyhow::Error| {
        app_error!("{}", e.to_string());
    };
    let server = app::get().server();
    let mut guard = server.lock().await;
    guard
        .to_local()
        .serve_in_background(Box::new(failed_handler))
        .await?;
    Ok(ServerEngine::new(&guard.ip, guard.port))
}

#[tauri::command]
pub async fn start_any_server() -> CommandResult<ServerEngine> {
    let failed_handler = |e: anyhow::Error| {
        app_error!("{}", e.to_string());
    };
    let server = app::get().server();
    let mut guard = server.lock().await;
    guard
        .to_any()
        .serve_in_background(Box::new(failed_handler))
        .await?;
    Ok(ServerEngine::new(
        &local_ip().unwrap().to_string(),
        guard.port,
    ))
}

#[tauri::command]
pub async fn shutdown_server() -> CommandResult<()> {
    let server = app::get().server();
    let mut guard = server.lock().await;
    guard.shutdown().await;
    Ok(())
}

#[tauri::command]
pub async fn is_remote_server_alive(address: String) -> CommandResult<bool> {
    ServerEngine::is_remote_alive(&address)
        .await
        .map_err(|e| e.into())
}

#[tauri::command]
pub fn get_local_server_address() -> CommandResult<String> {
    Ok(ServerEngine::local_address())
}

#[tauri::command]
pub fn get_remote_server_address() -> CommandResult<String> {
    Ok(app::get().remote_server_address())
}

#[tauri::command]
pub fn get_lan_server_address() -> CommandResult<String> {
    Ok(ServerEngine::lan_address())
}

#[tauri::command]
pub fn set_remote_server_address(address: String) -> CommandResult<String> {
    app::get().set_remote_server_address(address);
    Ok(app::get().remote_server_address())
}
