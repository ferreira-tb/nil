// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Prefecture = Building & {
  readonly queue: PrefectureBuildQueue;
};

type PrefectureBuildQueue = {
  readonly orders: readonly PrefectureBuildOrder[];
  readonly currentId: PrefectureBuildOrderId;
};

type PrefectureBuildOrder = {
  readonly id: PrefectureBuildOrderId;
  readonly building: BuildingId;
  readonly level: BuildingLevel;
  readonly workforce: number;
  readonly status: PrefectureBuildOrderStatus;
};

type PrefectureBuildOrderId = string;

type PrefectureBuildOrderStatus =
  | PrefectureBuildOrderStatusDone
  | PrefectureBuildOrderStatusPending;

type PrefectureBuildOrderStatusDone = {
  readonly kind: 'done';
};

type PrefectureBuildOrderStatusPending = {
  readonly kind: 'pending';
  readonly workforce: number;
};
