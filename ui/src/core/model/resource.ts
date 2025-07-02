// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PartialNullish } from '@tb-dev/utils';

export class ResourcesImpl implements Resources {
  public readonly food: number;
  public readonly iron: number;
  public readonly stone: number;
  public readonly wood: number;

  private constructor(resources: Resources) {
    this.food = resources.food;
    this.iron = resources.iron;
    this.stone = resources.stone;
    this.wood = resources.wood;
  }

  public has(resources: PartialNullish<Resources>) {
    return (
      this.food >= (resources.food ?? 0) &&
      this.iron >= (resources.iron ?? 0) &&
      this.stone >= (resources.stone ?? 0) &&
      this.wood >= (resources.wood ?? 0)
    );
  }

  public static create(resources: PartialNullish<Resources>) {
    return new ResourcesImpl({
      food: resources.food ?? 0,
      iron: resources.iron ?? 0,
      stone: resources.stone ?? 0,
      wood: resources.wood ?? 0,
    });
  }
}
