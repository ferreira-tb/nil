<script setup lang="ts">
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { asyncRef } from '@/composables/async-ref';
import { Button, Card, Table, TableCell, TableRow } from '@/components';

const world = asyncRef(null, commands.getWorldState);
const players = asyncRef([], commands.getPlayers);
</script>

<template>
  <div>
    <div class="flex items-center justify-between p-4">
      <h1 v-if="world" class="text-lg font-bold">{{ world.name }}</h1>
      <Button @click="leaveGame">{{ $t('leave') }}</Button>
    </div>
    <Card>
      <Table>
        <template #header>
          <TableRow>
            <TableCell>{{ $t('player') }}</TableCell>
          </TableRow>
        </template>

        <TableRow v-for="player of players" :key="player.id">
          <TableCell>{{ player.id }}</TableCell>
        </TableRow>
      </Table>
    </Card>
  </div>
</template>
