// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getVillage(coord: Coord) {
  return invoke<Village>('get_village', { coord });
}
