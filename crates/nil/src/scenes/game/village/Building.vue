<script setup lang="ts">
import type { VNode } from 'vue';
import { ButtonLink } from '@/components';
import type { GameRoute } from '@/router/types';

interface Props {
  to: GameRoute;
}

defineProps<Props>();

defineSlots<{ default: () => VNode }>();
</script>

<template>
  <ButtonLink :to button-class="flex items-center justify-start">
    <slot></slot>
  </ButtonLink>
</template>
