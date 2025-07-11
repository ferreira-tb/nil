// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import { toVillageRef } from '../toRef';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { VillageImpl } from '@/core/model/village';

export function useInfrastructure(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = toVillageRef(village);
  return computed(() => villageRef.value?.infrastructure);
}
