// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getLobby() {
  return invoke<Lobby>('get_lobby');
}

export function setLobbyReady(id: PlayerId, ready: boolean) {
  return invoke<null>('set_lobby_ready', { id, ready });
}
