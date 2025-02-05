<script setup lang="ts">
import { cn } from '@/lib/utils';
import { Game } from '@/core/game';
import { ButtonLink } from '@/components';

interface Props {
  class?: string;
}

const props = defineProps<Props>();

const { village } = Game.use();
</script>

<template>
  <div :class="cn('flex items-center justify-between', props.class)">
    <div>
      <ButtonLink to="village" variant="ghost" size="lg" button-class="text-lg">
        <span>{{ village?.name }}</span>
        <span>({{ village?.coord.format() }})</span>
      </ButtonLink>
    </div>
    <div></div>
  </div>
</template>
