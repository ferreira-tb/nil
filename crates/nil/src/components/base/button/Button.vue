<script setup lang="ts">
import { cn } from '@/lib/utils';
import type { HTMLAttributes } from 'vue';
import { type ButtonVariants, buttonVariants } from '.';
import { Primitive, type PrimitiveProps } from 'radix-vue';

interface Props extends PrimitiveProps {
  class?: HTMLAttributes['class'];
  size?: ButtonVariants['size'];
  variant?: ButtonVariants['variant'];
}

const props = withDefaults(defineProps<Props>(), {
  as: 'button',
});
</script>

<template>
  <Primitive :as :as-child :class="cn(buttonVariants({ variant, size }), props.class)">
    <slot></slot>
  </Primitive>
</template>
