// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Player = {
  readonly id: PlayerId;
  readonly resources: Resources;
  readonly status: PlayerStatus;
};

type PlayerId = string;

type PlayerStatus = 'active' | 'guest' | 'inactive';

type PlayerOptions = {
  readonly id: PlayerId;
};
