// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function getPlayer(id: PlayerId) {
  return invoke<Player>('get_player', { id });
}

export async function getPlayerCoords(id: PlayerId) {
  return invoke<readonly Coord[]>('get_player_coords', { id });
}

export function getPlayers() {
  return invoke<readonly Player[]>('get_players');
}

export function playerExists(id: PlayerId) {
  return invoke<boolean>('player_exists', { id });
}

export function setPlayerStatus(id: PlayerId, status: PlayerStatus) {
  return invoke<null>('set_player_status', { id, status });
}

export function spawnPlayer(options: PlayerOptions) {
  return invoke<null>('spawn_player', { options });
}

export function spawnPlayerVillage(id: PlayerId) {
  return invoke<null>('spawn_player_village', { id });
}
