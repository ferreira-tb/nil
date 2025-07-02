// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type BuildingId =
  | 'academy'
  | 'farm'
  | 'iron-mine'
  | 'prefecture'
  | 'quarry'
  | 'sawmill'
  | 'silo'
  | 'stable'
  | 'wall'
  | 'warehouse';

type BuildingLevel = number;

type Building = {
  readonly enabled: boolean;
  readonly level: BuildingLevel;
};

type Academy = Building;
type Stable = Building;
type Sawmill = Building;
type Quarry = Building;
type IronMine = Building;
type Farm = Building;
type Warehouse = Building;
type Silo = Building;
type Wall = Building;

type BuildingStats = {
  readonly baseCost: number;
  readonly level: BuildingLevel;
  readonly maintenance: number;
  readonly resources: Resources;
  readonly workforce: number;
};

type BuildingStatsTable = ReadonlyMap<BuildingId, ReadonlyMap<BuildingLevel, BuildingStats>>;
