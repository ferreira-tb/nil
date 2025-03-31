pub mod player;
pub mod round;
pub mod village;
pub mod world;

use crate::error::{Error, Result};
use crate::manager::ManagerExt;
use nil_core::player::PlayerId;
use nil_core::world::WorldOptions;
use std::net::SocketAddrV4;
use tauri::{AppHandle, WebviewWindow};

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> Result<SocketAddrV4> {
  app.client(async |cl| cl.server_addr()).await
}

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> Result<String> {
  app
    .client(async |cl| cl.version().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_dev() -> bool {
  tauri::is_dev()
}

#[tauri::command]
pub async fn is_host(app: AppHandle) -> bool {
  app.nil().is_host().await
}

#[tauri::command]
pub async fn is_mobile() -> bool {
  cfg!(mobile)
}

#[tauri::command]
pub async fn is_server_ready(app: AppHandle) -> Result<bool> {
  app.client(async |cl| cl.ready().await).await
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> Result<()> {
  if cfg!(desktop) {
    window
      .show()
      .and_then(|()| window.unminimize())
      .and_then(|()| window.set_focus())?;
  }

  Ok(())
}

#[tauri::command]
pub async fn start_client(
  app: AppHandle,
  player_id: PlayerId,
  server_addr: SocketAddrV4,
) -> Result<()> {
  app
    .nil()
    .start_client(player_id, server_addr)
    .await
}

#[tauri::command]
pub async fn start_server(app: AppHandle, world_options: WorldOptions) -> Result<SocketAddrV4> {
  if cfg!(desktop) {
    app.nil().start_server(world_options).await
  } else {
    Err(Error::MobileNotSupported)
  }
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) -> Result<()> {
  app.nil().stop_client().await
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}
