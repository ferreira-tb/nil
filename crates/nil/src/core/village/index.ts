import * as commands from '@/commands';
import type { Coord } from '@/types/world';
import type { Option } from '@tb-dev/utils';
import type { Village } from '@/types/village';
import { CoordImpl } from '@/core/world/coord';
import { InfrastructureImpl } from './infrastructure';

export class VillageImpl implements Village {
  public readonly coord: CoordImpl;
  public readonly infrastructure: InfrastructureImpl;
  public readonly name: string;
  public readonly owner: Option<string>;

  private constructor(village: Village) {
    this.coord = CoordImpl.create(village.coord);
    this.infrastructure = InfrastructureImpl.create(village.infrastructure);
    this.name = village.name;
    this.owner = village.owner;
  }

  public static async load(coord: Coord) {
    const village = await commands.getVillage(coord);
    return new VillageImpl(village);
  }
}
