// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ref } from 'vue';
import { Entity } from './abstract';
import { asyncRef } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { RoundImpl } from '@/core/model/round';

export class RoundEntity extends Entity {
  private readonly round: Ref<Option<RoundImpl>>;

  private readonly updateRound: () => Promise<void>;

  constructor() {
    super();

    const round = asyncRef(null, () => RoundImpl.load());
    this.round = round.state;
    this.updateRound = round.execute;

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onRoundUpdated(this.onRoundUpdated.bind(this));
  }

  public override async update() {
    await this.updateRound();
  }

  private async onRoundUpdated({ round }: RoundUpdatedPayload) {
    // Isso geralmente indica que o round atual acabou, então nós atualizamos todas as entidades.
    if (round.id !== this.id || round.phase.kind !== this.phase?.kind) {
      await NIL.update();
    } else {
      this.round.value = RoundImpl.create(round);
    }
  }

  get id() {
    return this.round.value?.id;
  }

  get phase() {
    return this.round.value?.phase;
  }

  public static use() {
    return super.get(RoundEntity) as RoundEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      round: instance.round as Readonly<typeof instance.round>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'round')) {
      const round: (typeof window.NIL)['round'] = {
        refs: RoundEntity.refs.bind(RoundEntity),
        update: RoundEntity.update.bind(RoundEntity),
        use: RoundEntity.use.bind(RoundEntity),
      };

      Object.defineProperty(window.NIL, 'round', {
        configurable: false,
        enumerable: true,
        writable: false,
        value: round,
      });
    }
  }
}
