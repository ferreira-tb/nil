// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed, toRef } from 'vue';
import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { PlayerImpl } from '@/core/model/player';
import type { ResourcesImpl } from '@/core/model/resource';

export function usePlayerResources(player?: MaybeNilRef<PlayerImpl>) {
  const playerRef = player ? toRef(player) : NIL.player.refs().player;
  return computed<Option<ResourcesImpl>>(() => playerRef.value?.resources);
}
