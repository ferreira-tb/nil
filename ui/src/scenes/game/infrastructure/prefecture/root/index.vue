<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watch } from 'vue';
import BuildQueue from './BuildQueue.vue';
import BuildCatalog from './BuildCatalog.vue';
import { usePrefecture } from '@/composables/usePrefecture';
import { useInfrastructure } from '@/composables/useInfrastructure';
import { usePrefectureBuildCatalog } from '@/composables/usePrefectureBuildCatalog';

const { coord, village } = NIL.village.refs();

const infrastructure = useInfrastructure();
const prefecture = usePrefecture();

const { addBuildOrder, buildCatalog, cancelBuildOrder, loadCatalog, loading } =
  usePrefectureBuildCatalog(coord);

await loadCatalog();

watch(village, loadCatalog);
</script>

<template>
  <div class="flex w-full flex-col gap-4 xl:flex-row-reverse">
    <BuildQueue v-if="prefecture" :prefecture :loading @cancel="cancelBuildOrder" />
    <BuildCatalog
      v-if="infrastructure && buildCatalog"
      :catalog="buildCatalog"
      :infrastructure
      :loading
      @build-order="addBuildOrder"
    />
  </div>
</template>
