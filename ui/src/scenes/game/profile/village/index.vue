<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import Loading from '@/components/Loading.vue';
import { useRouteParams } from '@vueuse/router';
import { usePublicVillage } from '@/composables/village/usePublicVillage';
import { usePublicVillageOwner } from '@/composables/village/usePublicVillageOwner';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const continentKey = useRouteParams('ckey', null, { transform: Number.parseInt });
const { village, loading } = usePublicVillage(continentKey);

const owner = computed(() => village.value?.owner);
const { bot, player, precursor } = usePublicVillageOwner(owner);
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader v-if="village && !loading">
        <CardTitle>
          <span>{{ village.name }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0 relative size-full">
        <Loading v-if="!village || loading" class="absolute inset-0" />
        <div v-else>
          <Table id="village-profile" class="sm:max-w-max">
            <TableBody>
              <TableRow>
                <TableHead>{{ t('coordinate', 2) }}</TableHead>
                <TableCell>{{ village.coord.format() }}</TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('owner') }}</TableHead>
                <TableCell>
                  <span v-if="bot">{{ bot.name }}</span>
                  <span v-else-if="player">{{ player.id }}</span>
                  <span v-else-if="precursor">{{ precursor.id }}</span>
                </TableCell>
              </TableRow>

              <TableRow>
                <TableHead>{{ t('type') }}</TableHead>
                <TableCell>{{ t(village.owner.kind) }}</TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
:deep(#village-profile th) {
  padding-right: 2rem;
}

:deep(#village-profile td) {
  padding-right: 4rem;
}
</style>
