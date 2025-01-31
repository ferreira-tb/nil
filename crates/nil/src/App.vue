<script setup lang="ts">
import { onMounted } from 'vue';
import { showWindow } from '@/commands';
import { handleError } from '@/lib/error';
import { useColorMode } from '@vueuse/core';

useColorMode({
  initialValue: 'dark',
  storageKey: 'nil:color-mode',
  writeDefaults: true,
  onError: handleError,
});

onMounted(() => showWindow().handleError());
</script>

<template>
  <main class="fixed inset-0 select-none">
    <div class="absolute inset-0 overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <KeepAlive>
            <component :is="Component" />
          </KeepAlive>
        </template>
      </RouterView>
    </div>
  </main>
</template>
