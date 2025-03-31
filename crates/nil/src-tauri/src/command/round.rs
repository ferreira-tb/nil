use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::round::Round;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_round(app: AppHandle) -> Result<Round> {
  app
    .client(async |cl| cl.round().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn is_round_idle(app: AppHandle) -> Result<bool> {
  app
    .client(async |cl| cl.round().await)
    .await?
    .map(|round| round.is_idle())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn start_round(app: AppHandle) -> Result<()> {
  app
    .client(async |cl| cl.start_round().await)
    .await?
    .map_err(Into::into)
}
