// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { compare } from '@/lib/intl';
import { asyncRef } from '@tb-dev/vue';
import { computed, toRef, watch } from 'vue';
import { CoordImpl } from '@/core/model/continent/coord';
import { getPublicVillage, getPublicVillagesBy } from '@/commands';
import { PublicVillageImpl } from '@/core/model/village/public-village';

export function usePublicVillages(keys: MaybeNilRef<readonly ContinentKey[]>) {
  const keysRef = toRef(keys);
  const coords = computed(() => {
    if (keysRef.value && keysRef.value.length > 0) {
      return keysRef.value.map((key) => CoordImpl.fromKey(key));
    }

    return null;
  });

  const { state, isLoading, execute } = asyncRef([], async () => {
    const villages: PublicVillageImpl[] = [];
    if (coords.value) {
      if (coords.value.length === 1) {
        const village = await getPublicVillage(coords.value[0]);
        villages.push(PublicVillageImpl.create(village));
      }
      else {
        for (const village of await getPublicVillagesBy(coords.value)) {
          villages.push(PublicVillageImpl.create(village));
        }
      }
    }

    villages.sort((a, b) => compare(a.name, b.name));

    return villages as readonly PublicVillageImpl[];
  });

  watch(coords, execute);

  return {
    villages: state,
    loading: isLoading,
    load: execute,
  };
}
