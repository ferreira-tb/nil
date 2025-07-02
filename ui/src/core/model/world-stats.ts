// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { InfrastructureStatsImpl, type RawInfrastructureStats } from './infrastructure-stats';

export class WorldStatsImpl implements WorldStats {
  public readonly infrastructure: InfrastructureStatsImpl;

  private constructor(stats: { infrastructure: InfrastructureStatsImpl }) {
    this.infrastructure = stats.infrastructure;
  }

  public static fromRaw(raw: RawWorldStats) {
    const infrastructure = InfrastructureStatsImpl.fromRaw(raw.infrastructure);
    return new WorldStatsImpl({ infrastructure });
  }
}

export type RawWorldStats = Omit<WorldStats, 'infrastructure'> & {
  infrastructure: RawInfrastructureStats;
};
