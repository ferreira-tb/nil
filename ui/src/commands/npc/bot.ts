// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getPublicBot(id: BotId) {
  return invoke<PublicBot>('get_public_bot', { id });
}
