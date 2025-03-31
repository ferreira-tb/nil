use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::village::{Coord, Village};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_village(app: AppHandle, coord: Coord) -> Result<Village> {
  app
    .client(async |cl| cl.village(coord).await)
    .await?
    .map_err(Into::into)
}
