import type { GameRoute } from './types';
import type { RouteRecordRaw } from 'vue-router';

export const gameRoutes: RouteRecordRaw[] = [
  {
    path: 'academy',
    name: 'academy' satisfies GameRoute,
    component: () => import('@/scenes/game/academy/index.vue'),
  },
  {
    path: 'farm',
    name: 'farm' satisfies GameRoute,
    component: () => import('@/scenes/game/farm/index.vue'),
  },
  {
    path: 'iron-mine',
    name: 'iron-mine' satisfies GameRoute,
    component: () => import('@/scenes/game/iron-mine/index.vue'),
  },
  {
    path: 'prefecture',
    name: 'prefecture' satisfies GameRoute,
    component: () => import('@/scenes/game/prefecture/index.vue'),
  },
  {
    path: 'quarry',
    name: 'quarry' satisfies GameRoute,
    component: () => import('@/scenes/game/quarry/index.vue'),
  },
  {
    path: 'sawmill',
    name: 'sawmill' satisfies GameRoute,
    component: () => import('@/scenes/game/sawmill/index.vue'),
  },
  {
    path: 'silo',
    name: 'silo' satisfies GameRoute,
    component: () => import('@/scenes/game/silo/index.vue'),
  },
  {
    path: 'stable',
    name: 'stable' satisfies GameRoute,
    component: () => import('@/scenes/game/stable/index.vue'),
  },
  {
    path: 'village',
    name: 'village' satisfies GameRoute,
    component: () => import('@/scenes/game/village/index.vue'),
  },
  {
    path: 'wall',
    name: 'wall' satisfies GameRoute,
    component: () => import('@/scenes/game/wall/index.vue'),
  },
  {
    path: 'warehouse',
    name: 'warehouse' satisfies GameRoute,
    component: () => import('@/scenes/game/warehouse/index.vue'),
  },
];
