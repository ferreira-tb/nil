import { invoke } from '@tauri-apps/api/core';
import type { WorldState } from '@/types/world';

export function getWorldState() {
  return invoke<WorldState>('get_world_state');
}
