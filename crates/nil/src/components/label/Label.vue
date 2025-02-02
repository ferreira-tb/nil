<script setup lang="ts">
import { cn } from '@/lib/utils';
import type { VNode } from 'vue';
import { Label as UiLabel } from '@/components/ui/label';

interface Props {
  class?: string;
}

const props = defineProps<Props>();

defineSlots<{ default: () => VNode }>();
</script>

<template>
  <UiLabel :class="cn('flex w-full flex-col gap-2 font-normal', props.class)">
    <slot></slot>
  </UiLabel>
</template>
