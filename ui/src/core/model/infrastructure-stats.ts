// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class InfrastructureStatsImpl implements InfrastructureStats {
  public readonly building: BuildingStatsTable;
  public readonly mine: MineStatsTable;
  public readonly storage: StorageStatsTable;

  private constructor(infrastructure: {
    building: BuildingStatsTable;
    mine: MineStatsTable;
    storage: StorageStatsTable;
  }) {
    this.building = infrastructure.building;
    this.mine = infrastructure.mine;
    this.storage = infrastructure.storage;
  }

  public static fromRaw(raw: RawInfrastructureStats) {
    const infrastructure = {
      building: new Map<BuildingId, ReadonlyMap<number, BuildingStats>>(),
      mine: new Map<MineId, ReadonlyMap<number, MineStats>>(),
      storage: new Map<StorageId, ReadonlyMap<number, StorageStats>>(),
    };

    for (const [key, value] of Object.entries(raw)) {
      switch (key as keyof RawInfrastructureStats) {
        case 'building': {
          type Entries = [BuildingId, Record<string, BuildingStats>][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const innerMap = new Map<number, BuildingStats>();
            for (const [level, stats] of Object.entries(record)) {
              innerMap.set(Number.parseInt(level), stats);
            }

            infrastructure.building.set(id, innerMap);
          }

          break;
        }
        case 'mine': {
          type Entries = [MineId, Record<string, MineStats>][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const innerMap = new Map<number, MineStats>();
            for (const [level, stats] of Object.entries(record)) {
              innerMap.set(Number.parseInt(level), stats);
            }

            infrastructure.mine.set(id, innerMap);
          }

          break;
        }
        case 'storage': {
          type Entries = [StorageId, Record<string, StorageStats>][];
          for (const [id, record] of Object.entries(value) as Entries) {
            const innerMap = new Map<number, StorageStats>();
            for (const [level, stats] of Object.entries(record)) {
              innerMap.set(Number.parseInt(level), stats);
            }

            infrastructure.storage.set(id, innerMap);
          }

          break;
        }
      }
    }

    return new InfrastructureStatsImpl(infrastructure);
  }
}

export type RawInfrastructureStats = {
  building: Record<BuildingId, Record<string, BuildingStats>>;
  mine: Record<MineId, Record<string, MineId>>;
  storage: Record<StorageId, Record<string, StorageId>>;
};
