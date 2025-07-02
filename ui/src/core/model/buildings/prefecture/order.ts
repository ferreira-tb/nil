// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class PrefectureBuildOrderImpl implements PrefectureBuildOrder {
  public readonly id: string;
  public readonly building: BuildingId;
  public readonly level: number;
  public readonly workforce: number;
  public readonly status: PrefectureBuildOrderStatus;

  private constructor(order: PrefectureBuildOrder) {
    this.id = order.id;
    this.building = order.building;
    this.level = order.level;
    this.workforce = order.workforce;
    this.status = order.status;
  }

  public isDone() {
    return this.status.kind === 'done';
  }

  public isPending() {
    return this.status.kind === 'pending';
  }

  public static create(order: PrefectureBuildOrder) {
    return new PrefectureBuildOrderImpl(order);
  }
}
