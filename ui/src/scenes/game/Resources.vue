<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { usePlayerResources } from '@/composables/usePlayerResources';
import { usePlayerStorageCapacity } from '@/composables/usePlayerStorageCapacity';

const resources = usePlayerResources();
const capacity = usePlayerStorageCapacity();
</script>

<template>
  <div class="grid grid-cols-4 items-center gap-8">
    <Wood :amount="resources?.wood" :capacity="capacity?.warehouse" />
    <Stone :amount="resources?.stone" :capacity="capacity?.warehouse" />
    <Iron :amount="resources?.iron" :capacity="capacity?.warehouse" />
    <Food :amount="resources?.food" :capacity="capacity?.silo" />
  </div>
</template>
