// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { RouteRecordRaw } from 'vue-router';
import { infrastructureRoutes } from './infrastructure';

export const gameRoutes: RouteRecordRaw[] = [
  {
    component: () => import('@/scenes/game/continent/index.vue'),
    path: 'continent',
    name: 'continent' satisfies GameScene,
  },
  {
    component: () => import('@/scenes/game/infrastructure/index.vue'),
    path: 'infrastructure',
    children: infrastructureRoutes,
  },
  {
    component: () => import('@/scenes/game/script/index.vue'),
    path: 'script',
    name: 'script' satisfies GameScene,
  },
  {
    component: () => import('@/scenes/game/village/index.vue'),
    path: 'village',
    name: 'village' satisfies GameScene,
  },
];
