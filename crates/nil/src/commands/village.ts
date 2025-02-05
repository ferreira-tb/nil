import { invoke } from '@tauri-apps/api/core';
import type { Coord, Village } from '@/types/village';

export async function getVillage(coord: Coord): Promise<Village> {
  return invoke('get_village', { coord });
}
