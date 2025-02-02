<script setup lang="ts">
import type { VNode } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { ScrollArea as UiScrollArea } from '@/components/ui/scroll-area';
import {
  Card as UiCard,
  CardContent as UiCardContent,
  CardDescription as UiCardDescription,
  CardHeader as UiCardHeader,
  CardTitle as UiCardTitle,
} from '@/components/ui/card';

interface Props {
  contentClass?: string;
  descriptionClass?: string;
  headerClass?: string;
  scrollAreaClass?: string;
  scrollAreaHeight?: number | string;
  titleClass?: string;
}

withDefaults(defineProps<Props>(), {
  scrollAreaHeight: 'auto',
});

defineSlots<{
  default: () => VNode;
  description?: () => VNode;
  title?: () => VNode;
}>();
</script>

<template>
  <UiCard>
    <UiCardHeader v-if="$slots.title" :class="headerClass">
      <UiCardTitle :class="titleClass">
        <slot name="title"></slot>
      </UiCardTitle>
      <UiCardDescription v-if="$slots.description" :class="descriptionClass">
        <slot name="description"></slot>
      </UiCardDescription>
    </UiCardHeader>
    <UiCardContent :class="contentClass">
      <UiScrollArea
        v-if="scrollAreaHeight && scrollAreaHeight !== 'auto'"
        :class="scrollAreaClass"
        :style="{ height: toPixel(scrollAreaHeight) }"
      >
        <slot></slot>
      </UiScrollArea>
      <slot v-else></slot>
    </UiCardContent>
  </UiCard>
</template>
