<script setup lang="ts">
import { Game } from '@/core/game';
import { useLocale } from '@/locale';
import Building from './Building.vue';

const { t } = useLocale();
const { village } = Game.use();
</script>

<template>
  <div class="size-full">
    <div v-if="village" class="size-full">
      <div class="flex flex-col gap-2">
        <Building to="prefecture">
          <span>{{ t('prefecture') }}</span>
        </Building>
        <Building v-if="village.academy.level > 0" to="academy">
          <span>{{ t('academy') }}</span>
        </Building>
        <Building v-if="village.stable.level > 0" to="stable">
          <span>{{ t('stable') }}</span>
        </Building>
        <Building v-if="village.sawmill.level > 0" to="sawmill">
          <span>{{ t('sawmill') }}</span>
        </Building>
        <Building v-if="village.quarry.level > 0" to="quarry">
          <span>{{ t('quarry') }}</span>
        </Building>
        <Building v-if="village.ironMine.level > 0" to="iron-mine">
          <span>{{ t('iron-mine') }}</span>
        </Building>
        <Building v-if="village.farm.level > 0" to="farm">
          <span>{{ t('farm') }}</span>
        </Building>
        <Building v-if="village.warehouse.level > 0" to="warehouse">
          <span>{{ t('warehouse') }}</span>
        </Building>
        <Building v-if="village.silo.level > 0" to="silo">
          <span>{{ t('silo') }}</span>
        </Building>
        <Building v-if="village.wall.level > 0" to="wall">
          <span>{{ t('wall') }}</span>
        </Building>
      </div>
    </div>
  </div>
</template>
