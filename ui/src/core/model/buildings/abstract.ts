// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';

export abstract class BuildingImpl implements Building {
  public abstract readonly id: BuildingId;
  public readonly level: BuildingLevel;
  public readonly enabled: boolean;

  #maxLevel: Option<BuildingLevel> = null;

  protected constructor(building: Building) {
    this.level = building.level;
    this.enabled = building.enabled;
  }

  public getStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.building.get(this.id);
  }

  public getStats() {
    return this.getStatsBy(this.level);
  }

  public getStatsBy(level: BuildingLevel) {
    return this.getStatsTable()?.get(level);
  }

  public getBaseCost() {
    return this.getBaseCostBy(this.level);
  }

  public getBaseCostBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.baseCost;
  }

  public getMaintenance() {
    return this.getMaintenanceBy(this.level);
  }

  public getMaintenanceBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.maintenance;
  }

  public getResourceCost() {
    return this.getResourceCostBy(this.level);
  }

  public getResourceCostBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.resources;
  }

  public getWorkforce() {
    return this.getWorkforceBy(this.level);
  }

  public getWorkforceBy(level: BuildingLevel) {
    return this.getStatsBy(level)?.workforce;
  }

  public isMaxLevel() {
    return this.level >= this.maxLevel;
  }

  get maxLevel() {
    if (typeof this.#maxLevel !== 'number') {
      this.#maxLevel = this.getStatsTable()
        ?.keys()
        .toArray()
        .toSorted((a, b) => a - b)
        .pop();
    }

    return this.#maxLevel ?? 0;
  }
}

export abstract class MineImpl extends BuildingImpl {
  public abstract override readonly id: MineId;

  public getMineStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.mine.get(this.id);
  }

  public getMineStats() {
    return this.getMineStatsBy(this.level);
  }

  public getMineStatsBy(level: BuildingLevel) {
    return this.getMineStatsTable()?.get(level);
  }

  public getProduction() {
    return this.getProductionBy(this.level);
  }

  public getProductionBy(level: BuildingLevel) {
    return this.getMineStatsBy(level)?.production;
  }
}

export abstract class StorageImpl extends BuildingImpl {
  public abstract override readonly id: StorageId;

  public getStorageStatsTable() {
    const { stats } = NIL.world.refs();
    return stats.value?.infrastructure.storage.get(this.id);
  }

  public getStorageStats() {
    return this.getStorageStatsBy(this.level);
  }

  public getStorageStatsBy(level: BuildingLevel) {
    return this.getStorageStatsTable()?.get(level);
  }

  public getCapacity() {
    return this.getCapacityBy(this.level);
  }

  public getCapacityBy(level: BuildingLevel) {
    return this.getStorageStatsBy(level)?.capacity;
  }
}
