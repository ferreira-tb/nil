// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import { computed, type ComputedRef, toRef } from 'vue';
import type { VillageImpl } from '@/core/model/village';
import type { InfrastructureImpl } from '@/core/model/infrastructure';

type InfrastructureRef = ComputedRef<Option<InfrastructureImpl>>;

export function useInfrastructure(village?: MaybeNilRef<VillageImpl>) {
  const villageRef = village ? toRef(village) : NIL.village.refs().village;
  return computed(() => villageRef.value?.infrastructure) as InfrastructureRef;
}
