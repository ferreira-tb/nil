// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::npc::precursor::{PrecursorId, PublicPrecursor};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_public_precursor(app: AppHandle, id: PrecursorId) -> Result<PublicPrecursor> {
  app
    .client(async |cl| cl.get_public_precursor(id).await)
    .await?
    .map_err(Into::into)
}
