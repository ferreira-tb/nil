// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { BuildingImpl } from '../abstract';
import { PrefectureBuildQueueImpl } from './queue';

export class PrefectureImpl extends BuildingImpl implements Prefecture {
  public readonly id: BuildingId = 'prefecture';
  public readonly queue: PrefectureBuildQueue;

  private constructor(prefecture: Building & { queue: PrefectureBuildQueueImpl }) {
    super(prefecture);
    this.queue = prefecture.queue;
  }

  public static create(prefecture: Prefecture) {
    const queue = PrefectureBuildQueueImpl.create(prefecture.queue);
    return new PrefectureImpl({
      level: prefecture.level,
      enabled: prefecture.enabled,
      queue,
    });
  }
}
