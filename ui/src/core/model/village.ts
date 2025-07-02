// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import type { Option } from '@tb-dev/utils';
import { CoordImpl } from '@/core/model/coord';
import { InfrastructureImpl } from '@/core/model/infrastructure';

export class VillageImpl implements Village {
  public readonly coord: CoordImpl;
  public readonly infrastructure: InfrastructureImpl;
  public readonly name: string;
  public readonly owner: VillageOwner;
  public readonly stability: number;

  private constructor(village: Village, infrastructure: InfrastructureImpl) {
    this.coord = CoordImpl.create(village.coord);
    this.infrastructure = infrastructure;
    this.name = village.name;
    this.owner = village.owner;
    this.stability = village.stability;
  }

  public static async load(coord: Coord) {
    const village = await commands.getVillage(coord);
    const infrastructure = InfrastructureImpl.create(village.infrastructure);
    return new VillageImpl(village, infrastructure);
  }

  public getProduction() {
    return {
      food: this.farm.getProduction(),
      iron: this.ironMine.getProduction(),
      stone: this.quarry.getProduction(),
      wood: this.sawmill.getProduction(),
    } satisfies Record<keyof Resources, Option<number>>;
  }

  get academy() {
    return this.infrastructure.academy;
  }

  get farm() {
    return this.infrastructure.farm;
  }

  get ironMine() {
    return this.infrastructure.ironMine;
  }

  get prefecture() {
    return this.infrastructure.prefecture;
  }

  get quarry() {
    return this.infrastructure.quarry;
  }

  get sawmill() {
    return this.infrastructure.sawmill;
  }

  get silo() {
    return this.infrastructure.silo;
  }

  get stable() {
    return this.infrastructure.stable;
  }

  get wall() {
    return this.infrastructure.wall;
  }

  get warehouse() {
    return this.infrastructure.warehouse;
  }
}
