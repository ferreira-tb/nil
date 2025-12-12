// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { CoordImpl } from '@/core/model/continent/coord';

export async function cheatGetBuildSteps(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  return invoke<readonly BuildStep[]>('cheat_get_build_steps', { req: { coord } });
}
