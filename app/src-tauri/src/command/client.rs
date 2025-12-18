// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_client::ServerAddr;
use nil_core::player::PlayerId;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(
  app: AppHandle,
  server_addr: ServerAddr,
  player_id: PlayerId,
) -> Result<()> {
  app
    .nil()
    .start_client(server_addr, player_id)
    .await
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
