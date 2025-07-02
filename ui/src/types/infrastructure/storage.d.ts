// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type StorageId = 'silo' | 'warehouse';

type StorageStats = {
  readonly capacity: number;
  readonly level: BuildingLevel;
};

type StorageStatsTable = ReadonlyMap<StorageId, ReadonlyMap<BuildingLevel, StorageStats>>;
