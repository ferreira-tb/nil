// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::chat::{ChatMessage, ChatMessageId, ChatMessagePlayer};

impl Client {
  /// GET `/chat`
  pub async fn get_chat_messages(&self) -> Result<Vec<ChatMessage>> {
    self.http.get_json("chat").await
  }

  /// POST `/chat/push`
  pub async fn push_chat_message(&self, message: ChatMessagePlayer) -> Result<ChatMessageId> {
    self
      .http
      .post_json("chat/push", message)
      .await
  }
}
