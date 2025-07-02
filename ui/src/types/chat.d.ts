// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type ChatMessage = ChatMessagePlayer & { readonly kind: 'player' };

type ChatMessagePlayer = {
  readonly id: ChatMessageId;
  readonly author: PlayerId;
  readonly content: ChatMessageContent;
  readonly time: string;
};

type ChatMessageId = string;
type ChatMessageContent = string;
