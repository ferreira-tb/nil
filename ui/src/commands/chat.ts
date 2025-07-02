// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getChatMessages() {
  return invoke<ChatMessage[]>('get_chat_messages');
}

export function pushChatMessage(content: string) {
  return invoke<ChatMessageId>('push_chat_message', { content });
}
