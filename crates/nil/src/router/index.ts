import { gameRoutes } from './game';
import type { Route } from './types';
import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home' satisfies Route,
      component: () => import('@/scenes/home/index.vue'),
    },
    {
      path: '/host-game',
      name: 'host-game' satisfies Route,
      component: () => import('@/scenes/host-game/index.vue'),
    },
    {
      path: '/join-game',
      name: 'join-game' satisfies Route,
      component: () => import('@/scenes/join-game/index.vue'),
    },
    {
      path: '/settings',
      name: 'settings' satisfies Route,
      component: () => import('@/scenes/settings/index.vue'),
    },

    {
      path: '/game',
      component: () => import('@/scenes/game/index.vue'),
      children: gameRoutes,
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
