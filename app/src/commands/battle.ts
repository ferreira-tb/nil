// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { clamp } from 'es-toolkit';
import { toU8 } from '@/lib/number';
import { invoke } from '@tauri-apps/api/core';

export async function simulateBattle(args: {
  attacker?: Option<readonly Squad[]>;
  defender?: Option<readonly Squad[]>;
  wall?: Option<BuildingLevel>;
}) {
  args.attacker ??= [];
  args.defender ??= [];
  args.wall ??= 0;

  const stats = NIL.world.getStats();
  if (stats) {
    const minWall = stats.getBuildingMinLevel('wall');
    const maxWall = stats.getBuildingMaxLevel('wall');
    if (typeof minWall === 'number' && typeof maxWall === 'number') {
      args.wall = clamp(toU8(args.wall), minWall, maxWall);
    }
  }

  return invoke<BattleResult>('simulate_battle', { req: args });
}
