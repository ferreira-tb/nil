import { PlayerImpl } from '@/core/player';
import { invoke } from '@tauri-apps/api/core';
import type { Player, PlayerConfig, PlayerId } from '@/types/player';

export async function getPlayer(id: PlayerId) {
  const player = await invoke<Player>('get_player', { id });
  return new PlayerImpl(player);
}

export function spawnPlayer(config: PlayerConfig) {
  return invoke<PlayerId>('spawn_player', { config });
}
