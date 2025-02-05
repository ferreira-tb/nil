<script setup lang="ts">
import { cn } from '@/lib/utils';
import { computed, type HTMLAttributes } from 'vue';
import { SelectSeparator, type SelectSeparatorProps } from 'radix-vue';

const props = defineProps<SelectSeparatorProps & { class?: HTMLAttributes['class'] }>();

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props;

  return delegated;
});
</script>

<template>
  <SelectSeparator v-bind="delegatedProps" :class="cn('bg-muted -mx-1 my-1 h-px', props.class)" />
</template>
