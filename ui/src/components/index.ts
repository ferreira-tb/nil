// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { App } from 'vue';
import Food from './resource/Food.vue';
import Iron from './resource/Iron.vue';
import Wood from './resource/Wood.vue';
import Stone from './resource/Stone.vue';

export function registerGlobalComponents(app: App) {
  app
    .component('Food', Food)
    .component('Iron', Iron)
    .component('Stone', Stone)
    .component('Wood', Wood);
}

declare module 'vue' {
  // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
  interface GlobalComponents {
    Food: typeof Food;
    Iron: typeof Iron;
    Stone: typeof Stone;
    Wood: typeof Wood;
  }
}
