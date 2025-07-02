// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::manager::ManagerExt;
use nil_core::chat::{ChatMessage, ChatMessageId, ChatMessagePlayer};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_chat_messages(app: AppHandle) -> Result<Vec<ChatMessage>> {
  app
    .client(async |cl| cl.get_chat_messages().await)
    .await?
    .map_err(Into::into)
}

#[tauri::command]
pub async fn push_chat_message(app: AppHandle, content: String) -> Result<ChatMessageId> {
  let author = app.nil().player().await?;
  let message = ChatMessagePlayer::new(author, &content);
  app
    .client(async |cl| cl.push_chat_message(message).await)
    .await?
    .map_err(Into::into)
}
