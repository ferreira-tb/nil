// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Village = {
  readonly coord: Coord;
  readonly infrastructure: Infrastructure;
  readonly name: string;
  readonly owner: VillageOwner;
  readonly stability: number;
};

type VillagePublicState = {
  readonly coord: Coord;
  readonly name: string;
  readonly owner: VillageOwner;
};

type Coord = {
  readonly x: number;
  readonly y: number;
};

type VillageOwner = VillageOwnerNone | VillageOwnerPlayer;

type VillageOwnerNone = {
  readonly kind: 'none';
};

type VillageOwnerPlayer = {
  readonly id: PlayerId;
  readonly kind: 'player';
};
