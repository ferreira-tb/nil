// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { logicOr } from '@vueuse/math';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import type { MaybeNilRef } from '@tb-dev/vue';
import type { CoordImpl } from '@/core/model/coord';
import { readonly, ref, shallowRef, toRef } from 'vue';
import {
  addPrefectureBuildOrder,
  cancelPrefectureBuildOrder,
  getPrefectureBuildCatalog,
} from '@/commands';

export function usePrefectureBuildCatalog(coord?: MaybeNilRef<Option<CoordImpl>>) {
  const coordRef = coord ? toRef(coord) : NIL.village.refs().coord;
  const catalog = shallowRef<Option<PrefectureBuildCatalog>>();

  const isLoadingCatalog = ref(false);
  const isWaitingAddCmd = ref(false);
  const isWaitingCancelCmd = ref(false);

  const loading = logicOr(isLoadingCatalog, isWaitingAddCmd, isWaitingCancelCmd);

  async function load() {
    if (coordRef.value) {
      catalog.value = await getPrefectureBuildCatalog(coordRef.value);
    } else {
      catalog.value = null;
    }
  }

  async function addBuildOrder(building: BuildingId, kind: PrefectureBuildOrderKind) {
    if (coordRef.value && !loading.value) {
      try {
        isWaitingAddCmd.value = true;
        await addPrefectureBuildOrder({ coord: coordRef.value, building, kind });
        await load();
      } catch (err) {
        handleError(err);
      } finally {
        isWaitingAddCmd.value = false;
      }
    }
  }

  async function cancelBuildOrder() {
    if (coordRef.value && !loading.value) {
      try {
        isWaitingCancelCmd.value = true;
        await cancelPrefectureBuildOrder(coordRef.value);
        await load();
      } catch (err) {
        handleError(err);
      } finally {
        isWaitingCancelCmd.value = false;
      }
    }
  }

  return {
    buildCatalog: catalog as Readonly<typeof catalog>,
    loading: readonly(isLoadingCatalog),
    loadCatalog: load,
    addBuildOrder,
    cancelBuildOrder,
  };
}
