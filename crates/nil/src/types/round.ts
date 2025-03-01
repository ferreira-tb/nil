import type { PlayerId } from './player';

export type RoundState = {
  readonly id: RoundId;
  readonly phase: RoundPhase;
};

export type RoundId = number;

export type RoundPhase = RoundPhaseIdle | RoundPhasePlayer;

export type RoundPhaseIdle = {
  readonly kind: 'idle';
};

export type RoundPhasePlayer = {
  readonly kind: 'player';
  readonly pending: readonly PlayerId[];
};
