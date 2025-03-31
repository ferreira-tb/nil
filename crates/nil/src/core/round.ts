import { shallowRef } from 'vue';
import * as commands from '@/commands';
import { Entity } from '@/core/entity';
import type { Option } from '@tb-dev/utils';

export class RoundImpl extends Entity {
  private readonly round = shallowRef<Option<Round>>();

  constructor() {
    super();
    this.event.onRoundUpdated(() => this.update());
  }

  public async update() {
    this.round.value = await commands.getRound();
  }

  public static use() {
    return super.get(RoundImpl) as RoundImpl;
  }

  public static refs() {
    const instance = this.use();
    return {
      round: instance.round as Readonly<typeof instance.round>,
    };
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'round')) {
      const round: (typeof window.NIL)['round'] = {
        refs: RoundImpl.refs.bind(RoundImpl),
        update: RoundImpl.update.bind(RoundImpl),
        use: RoundImpl.use.bind(RoundImpl),
      };

      Object.defineProperty(window.NIL, 'round', {
        configurable: false,
        enumerable: true,
        value: round,
        writable: false,
      });
    }
  }
}
