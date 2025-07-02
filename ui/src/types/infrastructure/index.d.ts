// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Infrastructure = {
  readonly academy: Academy;
  readonly farm: Farm;
  readonly ironMine: IronMine;
  readonly prefecture: Prefecture;
  readonly quarry: Quarry;
  readonly sawmill: Sawmill;
  readonly silo: Silo;
  readonly stable: Stable;
  readonly wall: Wall;
  readonly warehouse: Warehouse;
};

type InfrastructureStats = {
  readonly building: BuildingStatsTable;
  readonly mine: MineStatsTable;
  readonly storage: StorageStatsTable;
};
