use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::world::WorldState;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_world_state(app: AppHandle) -> Result<WorldState> {
  app
    .client(async |cl| cl.world().await)
    .await?
    .map_err(Into::into)
}
