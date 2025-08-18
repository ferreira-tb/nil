// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getVillage(coord: Coord) {
  return invoke<Village>('get_village', { coord });
}

export async function getPublicVillage(coord: Coord) {
  return invoke<PublicVillage>('get_public_village', { coord });
}

export async function getPublicVillages() {
  return invoke<readonly PublicVillage[]>('get_public_villages');
}

export async function getPublicVillagesBy(coords: Coord[]) {
  return invoke<readonly PublicVillage[]>('get_public_villages_by', { coords });
}

export async function renameVillage(coord: Coord, name: string) {
  return invoke<null>('rename_village', { coord, name });
}
