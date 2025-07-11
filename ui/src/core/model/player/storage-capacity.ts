// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class PlayerStorageCapacityImpl implements PlayerStorageCapacity {
  public readonly silo: number;
  public readonly warehouse: number;

  private constructor(capacity: PlayerStorageCapacity) {
    this.silo = capacity.silo;
    this.warehouse = capacity.warehouse;
  }

  public static create(capacity: PlayerStorageCapacity) {
    return new PlayerStorageCapacityImpl(capacity);
  }
}
