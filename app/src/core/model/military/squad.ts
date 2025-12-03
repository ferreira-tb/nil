// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';

export class SquadImpl implements Squad {
  public readonly unit: UnitId;
  public readonly size: number;

  private constructor(squad: Squad) {
    this.unit = squad.unit;
    this.size = squad.size;
  }

  public isEmpty() {
    return SquadImpl.isEmpty(this);
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
    return SquadImpl.create(SquadImpl.createEmptyRaw(unit));
  }

  public static createEmptyRaw(unit: UnitId): Squad {
    return { unit, size: 0 };
  }

  public static withValidSize(squad: Squad) {
    return SquadImpl.create({
      unit: squad.unit,
      size: toU32(squad.size),
    });
  }

  public static isEmpty(squad: Squad) {
    return squad.size === 0;
  }
}
