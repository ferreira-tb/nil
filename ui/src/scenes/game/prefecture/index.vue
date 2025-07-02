<!-- Copyright (C) Tsukilabs contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { Card, Table, TableCell, TableHead, TableRow } from '@tb-dev/vue-components';

const { village } = NIL.village.refs();

const prefecture = computed(() => village.value?.prefecture);
</script>

<template>
  <div class="game-layout">
    <Card v-if="prefecture" class="w-full" content-class="px-2">
      <Table></Table>
    </Card>
  </div>
</template>
