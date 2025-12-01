// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class SquadImpl implements Squad {
  public readonly unit: UnitId;
  public readonly size: number;

  private constructor(squad: Squad) {
    this.unit = squad.unit;
    this.size = squad.size;
  }

  public isEmpty() {
    return this.size === 0;
  }

  public add(squad: Squad) {
    if (this.unit === squad.unit) {
      const size = this.size + squad.size;
      return SquadImpl.create({ unit: this.unit, size });
    }
    else {
      return this;
    }
  }

  public sub(squad: Squad) {
    if (this.unit === squad.unit) {
      const size = this.size - squad.size;
      return SquadImpl.create({ unit: this.unit, size });
    }
    else {
      return this;
    }
  }

  public static create(squad: Squad) {
    if (squad instanceof SquadImpl) {
      return squad;
    }

    return new SquadImpl(squad);
  }

  public static createEmpty(unit: UnitId) {
    return SquadImpl.create({ unit, size: 0 });
  }
}
