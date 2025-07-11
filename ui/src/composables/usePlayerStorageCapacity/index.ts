// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { PlayerImpl } from '@/core/model/player';
import { computed, type ComputedRef, toRef } from 'vue';
import type { PlayerStorageCapacityImpl } from '@/core/model/player/storage-capacity';

type CapacityRef = ComputedRef<Option<PlayerStorageCapacityImpl>>;

export function usePlayerStorageCapacity(player?: MaybeNilRef<PlayerImpl>) {
  const playerRef = player ? toRef(player) : NIL.player.refs().player;
  return computed(() => playerRef.value?.capacity) as CapacityRef;
}
