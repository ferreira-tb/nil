// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type MineId = 'farm' | 'iron-mine' | 'quarry' | 'sawmill';

type MineStats = {
  readonly level: BuildingLevel;
  readonly production: number;
};

type MineStatsTable = ReadonlyMap<MineId, ReadonlyMap<BuildingLevel, MineStats>>;
