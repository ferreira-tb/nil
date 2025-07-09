// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod prefecture;

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::infrastructure::building::BuildingId;
use nil_core::village::Coord;
use tauri::AppHandle;

#[tauri::command]
pub async fn toggle_building(
  app: AppHandle,
  coord: Coord,
  id: BuildingId,
  enabled: bool,
) -> Result<()> {
  app
    .client(async |cl| cl.toggle_building(coord, id, enabled).await)
    .await?
    .map_err(Into::into)
}
