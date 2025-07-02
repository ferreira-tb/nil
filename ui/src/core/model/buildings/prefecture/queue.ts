// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { PrefectureBuildOrderImpl } from './order';

export class PrefectureBuildQueueImpl implements PrefectureBuildQueue {
  public readonly orders: readonly PrefectureBuildOrderImpl[];
  public readonly currentId: string;

  private constructor(queue: { orders: PrefectureBuildOrderImpl[]; currentId: string }) {
    this.orders = queue.orders;
    this.currentId = queue.currentId;
  }

  public static create(queue: PrefectureBuildQueue) {
    const orders = queue.orders.map((it) => PrefectureBuildOrderImpl.create(it));
    return new PrefectureBuildQueueImpl({ orders, currentId: queue.currentId });
  }
}
