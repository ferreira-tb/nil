<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { ResourcesImpl } from '@/core/model/resources';

defineProps<{
  entry: PrefectureBuildCatalogEntryAvailable;
  playerResources: Option<ResourcesImpl>;
}>();
</script>

<template>
  <div class="grid grid-cols-5 items-center justify-start gap-4">
    <Wood :amount="entry.recipe.resources.wood" :limit="playerResources?.wood" />
    <Stone :amount="entry.recipe.resources.stone" :limit="playerResources?.stone" />
    <Iron :amount="entry.recipe.resources.iron" :limit="playerResources?.iron" />
    <Food :amount="entry.recipe.maintenance" />
    <Workforce :amount="entry.recipe.workforce" />
  </div>
</template>
