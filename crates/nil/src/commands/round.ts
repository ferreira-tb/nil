import { invoke } from '@tauri-apps/api/core';

export function getRound() {
  return invoke<Round>('get_round');
}

export function isRoundIdle() {
  return invoke<boolean>('is_round_idle');
}

export function startRound() {
  return invoke<null>('start_round');
}
