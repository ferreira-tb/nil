import type { PlayerId } from './player';
import type * as building from './building';

export type Village = {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: VillageOwner;
};

export type Infrastructure = {
  readonly academy: building.Academy;
  readonly farm: building.Farm;
  readonly ironMine: building.IronMine;
  readonly prefecture: building.Prefecture;
  readonly quarry: building.Quarry;
  readonly sawmill: building.Sawmill;
  readonly silo: building.Silo;
  readonly stable: building.Stable;
  readonly wall: building.Wall;
  readonly warehouse: building.Warehouse;
};

export type Coord = {
  readonly x: number;
  readonly y: number;
};

export type VillageOwner = VillageOwnerNone | VillageOwnerPlayer;

export type VillageOwnerNone = {
  readonly kind: 'none';
};

export type VillageOwnerPlayer = {
  readonly id: PlayerId;
  readonly kind: 'player';
};
