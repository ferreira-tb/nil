// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';
import { useInfrastructure } from '../useInfrastructure';

export function usePrefecture() {
  const infrastructure = useInfrastructure();
  return computed(() => infrastructure.value?.prefecture);
}
