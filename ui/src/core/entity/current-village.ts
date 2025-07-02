// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import { Entity } from './abstract';
import { until } from '@vueuse/core';
import { CoordImpl } from '@/core/model/coord';
import { type Option, sleep } from '@tb-dev/utils';
import { VillageImpl } from '@/core/model/village';
import { asyncComputed, maybe } from '@tb-dev/vue';
import { computed, nextTick, shallowRef } from 'vue';
import type { PlayerImpl } from '@/core/model/player';

/**
 * Depends on:
 * - [`CurrentPlayerEntity`](./current-player.ts)
 */
export class CurrentVillageEntity extends Entity {
  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village = asyncComputed(null, async () => {
    return maybe(this.coord, (coord) => VillageImpl.load(coord));
  });

  private readonly production = computed(() => {
    return this.village.value?.getProduction();
  });

  constructor() {
    super();

    const { player } = NIL.player.refs();
    this.watch(player, this.onPlayerUpdate.bind(this));
  }

  private onPlayerUpdate(player: Option<PlayerImpl>) {
    if (player && (!this.coord.value || !player.hasVillage(this.coord.value))) {
      this.coord.value = player.coords.at(0);
    } else if (!player) {
      this.coord.value = null;
    }
  }

  public static use() {
    return super.get(CurrentVillageEntity) as CurrentVillageEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      coord: instance.coord as Readonly<typeof instance.coord>,
      production: instance.production as Readonly<typeof instance.production>,
      village: instance.village,
    } as const;
  }

  public static readonly go = goToVillage;

  public static init() {
    if (!Object.hasOwn(window.NIL, 'village')) {
      const village: (typeof window.NIL)['village'] = {
        go: CurrentVillageEntity.go.bind(CurrentVillageEntity),
        refs: CurrentVillageEntity.refs.bind(CurrentVillageEntity),
        use: CurrentVillageEntity.use.bind(CurrentVillageEntity),
      };

      Object.defineProperty(window.NIL, 'village', {
        configurable: false,
        enumerable: true,
        value: village,
        writable: false,
      });
    }
  }
}

async function goToVillage(options?: {
  timeout?: number;
  keepTrying?: boolean;
  maxTries?: number;
}) {
  const { coord } = NIL.village.refs();
  const { timeout = 5000, keepTrying = false, maxTries = 50 } = options ?? {};

  const wait = async () => {
    if (typeof timeout === 'number' && Number.isFinite(timeout) && timeout > 0) {
      await until(coord).toBeTruthy({ timeout, throwOnTimeout: true });
    } else {
      await until(coord).toBeTruthy();
    }
  };

  const navigate = () => go('village');

  if (keepTrying) {
    let current = 0;
    const { player } = NIL.player.refs();
    const retry = async () => {
      await nextTick();
      if (player.value?.isActive()) {
        try {
          await wait();
          await navigate();
          return;
        } catch {
          await NIL.update();
        }
      } else {
        await sleep(500);
      }

      current += 1;

      if (current < maxTries) {
        await retry();
      }
    };

    await retry();
  } else {
    await navigate();
  }
}
