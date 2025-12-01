// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';

export const warRoomRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/war-room/root/index.vue'),
    path: '',
    name: 'war-room' satisfies WarRoomScene,
  },
  {
    component: () => import('@/scenes/game/war-room/battle-simulator/index.vue'),
    path: 'war-room-battle-simulator',
    name: 'war-room-battle-simulator' satisfies WarRoomScene,
  },
];
