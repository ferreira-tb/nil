// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';

export function usePlayerTurn() {
  const { round } = NIL.round.refs();
  const { player } = NIL.player.refs();
  return computed(() => {
    const id = player.value?.id;
    const pending = id ? round.value?.isPending(id) : null;
    return pending ?? false;
  });
}
