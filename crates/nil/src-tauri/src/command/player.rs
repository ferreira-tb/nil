use crate::error::CResult;
use crate::manager::ManagerExt;
use nil_core::{Player, PlayerConfig, PlayerId};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, id: PlayerId) -> CResult<Player> {
  app
    .with_client(async |client| client.get_player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, config: PlayerConfig) -> CResult<PlayerId> {
  app
    .with_client(async |client| client.spawn_player(config).await)
    .await?
    .map_err(Into::into)
}
