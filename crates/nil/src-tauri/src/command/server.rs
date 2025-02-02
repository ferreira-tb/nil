use super::CResult;
use crate::manager::ManagerExt;
use nil_core::WorldConfig;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_server(app: AppHandle, world_config: WorldConfig) {
  let world = world_config.into_world();
  app.nil().start_server(world).await;
}

#[tauri::command]
pub async fn stop_server(app: AppHandle) {
  app.nil().stop_server().await;
}

#[tauri::command]
pub async fn get_server_version(app: AppHandle) -> CResult<String> {
  app
    .with_client(async |client| client.version().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_server_ok(app: AppHandle) -> CResult<bool> {
  app
    .with_client(async |client| client.ok().await)
    .await?
    .map_err(Into::into)
}
