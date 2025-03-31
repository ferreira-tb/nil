use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::player::{Player, PlayerId, PlayerOptions};
use nil_core::village::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_player(app: AppHandle, id: PlayerId) -> Result<Player> {
  app
    .client(async |cl| cl.player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_player_villages(app: AppHandle, id: PlayerId) -> Result<Vec<Coord>> {
  app
    .client(async |cl| cl.villages_of(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_players(app: AppHandle) -> Result<Vec<Player>> {
  app
    .client(async |cl| cl.players().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_player(app: AppHandle, id: PlayerId) -> Result<()> {
  app
    .client(async |cl| cl.remove_player(id).await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn spawn_player(app: AppHandle, options: PlayerOptions) -> Result<()> {
  app
    .client(async |cl| cl.spawn_player(options).await)
    .await?
    .map_err(Into::into)
}
