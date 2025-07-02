// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type WorldOptions = {
  readonly name: string;
  readonly size: number;
};

type WorldConfig = {
  readonly name: string;
};

type WorldStats = {
  readonly infrastructure: InfrastructureStats;
};
