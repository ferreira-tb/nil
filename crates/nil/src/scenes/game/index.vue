<script setup lang="ts">
import Topbar from './Topbar.vue';
import Sidebar from './Sidebar.vue';
</script>

<template>
  <div class="absolute inset-0 overflow-hidden">
    <Sidebar class="absolute inset-y-0 left-0 hidden sm:block sm:w-12" />
    <div class="absolute inset-0 overflow-hidden sm:left-12">
      <Topbar class="sticky top-0 h-16" />
      <div class="size-full">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <component :is="Component" />
          </template>
        </RouterView>
      </div>
    </div>
  </div>
</template>
