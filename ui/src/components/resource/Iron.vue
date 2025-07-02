<!-- Copyright (C) Tsukilabs contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Base from './Base.vue';
import type { Option } from '@tb-dev/utils';

defineProps<{ amount?: Option<number> }>();
</script>

<template>
  <Base :amount :name="$t('iron')" color="oklch(0.7 0.1653 297.53)" />
</template>
