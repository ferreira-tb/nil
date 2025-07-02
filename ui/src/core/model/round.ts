// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';

export class RoundImpl implements Round {
  public readonly id: number;
  public readonly phase: RoundPhase;

  private constructor(round: Round) {
    this.id = round.id;
    this.phase = round.phase;
  }

  public isIdle() {
    return this.phase.kind === 'idle';
  }

  public isPending(id: PlayerId) {
    return this.phase.kind === 'player' && this.phase.pending.includes(id);
  }

  public static create(round: Round) {
    return new RoundImpl(round);
  }

  public static async load() {
    const round = await commands.getRound();
    return RoundImpl.create(round);
  }
}
