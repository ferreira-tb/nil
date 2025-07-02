// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::chat::{ChatMessage, ChatMessageId};

impl World {
  pub fn push_chat_message(&mut self, message: impl Into<ChatMessage>) -> ChatMessageId {
    let mut message: ChatMessage = message.into();
    let id = self.chat.push(&mut message);
    self.emit_chat_message(message);
    id
  }
}
