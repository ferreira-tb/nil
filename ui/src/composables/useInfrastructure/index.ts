// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from 'vue';

export function useInfrastructure() {
  const { village } = NIL.village.refs();
  return computed(() => village.value?.infrastructure);
}
