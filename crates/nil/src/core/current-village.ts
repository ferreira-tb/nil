import { go } from '@/router';
import { shallowRef } from 'vue';
import { until } from '@vueuse/core';
import { Entity } from '@/core/entity';
import { CoordImpl } from '@/core/coord';
import type { Option } from '@tb-dev/utils';
import { VillageImpl } from '@/core/village';
import type { PlayerImpl } from '@/core/player';
import { asyncComputed, maybe } from '@tb-dev/vue';

export class CurrentVillage extends Entity {
  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village = asyncComputed(null, async () => {
    return maybe(this.coord, (coord) => VillageImpl.load(coord));
  });

  constructor() {
    super();

    const { player } = NIL.player.refs();
    this.watch(player, this.onPlayerUpdate.bind(this));
  }

  private onPlayerUpdate(player: Option<PlayerImpl>) {
    if (player && (!this.coord.value || !player.has(this.coord.value))) {
      this.coord.value = player.villages.at(0);
    } else if (!player) {
      this.coord.value = null;
    }
  }

  public static use() {
    return super.get(CurrentVillage) as CurrentVillage;
  }

  public static refs() {
    const instance = this.use();
    return {
      coord: instance.coord as Readonly<typeof instance.coord>,
      village: instance.village,
    };
  }

  public static async go() {
    const { coord } = NIL.village.refs();
    await until(coord).toBeTruthy();
    go('village');
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'village')) {
      const village: (typeof window.NIL)['village'] = {
        go: CurrentVillage.go.bind(CurrentVillage),
        refs: CurrentVillage.refs.bind(CurrentVillage),
        use: CurrentVillage.use.bind(CurrentVillage),
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
