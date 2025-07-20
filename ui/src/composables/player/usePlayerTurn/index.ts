// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { PlayerImpl } from '@/core/model/player';
import { toPlayerRef } from '@/composables/util/toRef';

export function usePlayerTurn(player?: MaybeNilRef<PlayerImpl>) {
  const { round } = NIL.round.refs();
  const playerRef = toPlayerRef(player);
  return computed(() => {
    const id = playerRef.value?.id;
    const isWaiting = id ? round.value?.isWaitingPlayer(id) : null;
    return isWaiting ?? false;
  });
}
