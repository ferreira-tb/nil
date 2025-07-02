// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class WorldConfigImpl implements WorldConfig {
  public readonly name: string;

  private constructor(state: { name: string }) {
    this.name = state.name;
  }

  public static create(config: WorldConfig) {
    return new WorldConfigImpl({ name: config.name });
  }
}
