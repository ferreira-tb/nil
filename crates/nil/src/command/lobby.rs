// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::lobby::LobbyState;
use nil_core::player::PlayerId;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_lobby(app: AppHandle) -> Result<LobbyState> {
  app
    .client(async |cl| cl.get_lobby().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_lobby_ready(app: AppHandle, id: PlayerId, ready: bool) -> Result<()> {
  app
    .client(async |cl| cl.set_lobby_ready(id, ready).await)
    .await?
    .map_err(Into::into)
}
