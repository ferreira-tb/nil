<!-- Copyright (C) Tsukilabs contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { onMounted } from 'vue';
import { throttle } from 'es-toolkit';
import * as commands from '@/commands';
import { showWindow } from '@/commands';
import { onKeyDown } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import { useColorMode } from '@vueuse/core';
import { Sonner } from '@tb-dev/vue-components';

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  storageKey: 'nil:color-mode',
  writeDefaults: true,
});

onKeyDown('F5', throttle(NIL.update, 1000));

onMounted(async () => {
  try {
    await showWindow();
    if (await commands.isDev()) {
      (window as any).$c = commands;
    }
  } catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <main class="fixed inset-0 select-none">
    <Sonner />
    <div class="absolute inset-0 overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
