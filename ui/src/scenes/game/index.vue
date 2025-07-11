<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import Header from './Header.vue';
import Footer from './Footer.vue';
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { useToggle } from '@vueuse/core';
import { onMounted, useTemplateRef } from 'vue';
import { onBeforeRouteUpdate } from 'vue-router';
import { leaveGame, saveGame } from '@/core/game';
import { defineGlobalCheats } from '@/lib/global';
import Finder from '@/components/finder/Finder.vue';
import { asyncRef, onCtrlKeyDown } from '@tb-dev/vue';
import { usePlayerTurn } from '@/composables/usePlayerTurn';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { type OnClickOutsideProps, vOnClickOutside } from '@vueuse/components';
import {
  Button,
  Loading,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarProvider,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const { config } = NIL.world.refs();
const { round } = NIL.round.refs();

const isPlayerTurn = usePlayerTurn();

const { state: isHost } = asyncRef(false, commands.isHost);
const { state: serverAddr } = asyncRef(null, commands.getServerAddr);

const [isSidebarOpen, toggleSidebar] = useToggle(false);
const closeSidebar = () => void toggleSidebar(false);

const sidebarFooter = useTemplateRef('sidebarFooterEl');
const onClickOutsideOptions: OnClickOutsideProps['options'] = {
  ignore: [sidebarFooter],
};

const [isFinderOpen, toggleFinder] = useToggle(false);

onCtrlKeyDown(['b', 'B'], () => toggleSidebar());
onCtrlKeyDown(['f', 'F'], () => toggleFinder());
onCtrlKeyDown(['m', 'M'], () => go('continent'));
onCtrlKeyDown(['s', 'S'], () => save());
onCtrlKeyDown(' ', () => endTurn());

onBeforeRouteUpdate(closeSidebar);

onMounted(() => defineGlobalCheats());

function startRound() {
  if (isHost.value && round.value?.phase.kind === 'idle') {
    commands.startRound().err();
  }
}

function endTurn() {
  if (isPlayerTurn.value) {
    commands.endTurn().err();
  }
}

function save() {
  if (isHost.value && round.value?.phase.kind !== 'idle') {
    saveGame().err();
  }
}

function copyServerAddr() {
  if (serverAddr.value) {
    const addr = serverAddr.value.format();
    writeText(addr).err();
  }
}
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen">
    <Sidebar class="z-[var(--game-sidebar-z-index)]">
      <SidebarHeader>
        <div class="flex flex-col items-center">
          <h1 v-if="config" class="font-nil text-lg">{{ config.name }}</h1>
          <h2
            v-if="serverAddr"
            class="text-muted-foreground cursor-pointer text-sm"
            @click="copyServerAddr"
          >
            {{ serverAddr.format() }}
          </h2>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <div v-on-click-outside="[closeSidebar, onClickOutsideOptions]" class="size-full p-4">
          <RouterLink :to="{ name: 'script' satisfies GameScene }">Scripts</RouterLink>
        </div>
      </SidebarContent>

      <SidebarFooter>
        <div
          ref="sidebarFooterEl"
          class="grid grid-cols-2 items-center justify-center gap-4 px-6 pb-4"
        >
          <Button size="sm" :disabled="!isHost || round?.phase.kind === 'idle'" @click="save">
            <span>{{ t('save') }}</span>
          </Button>
          <Button variant="destructive" size="sm" @click="leaveGame">
            <span>{{ t('leave') }}</span>
          </Button>
        </div>
      </SidebarFooter>
    </Sidebar>

    <div class="bg-muted/40 absolute inset-0 overflow-hidden">
      <Header
        :is-host
        class="bg-background absolute inset-x-0 top-0 h-16 border-b px-4"
        @start-round="startRound"
        @end-turn="endTurn"
      />

      <div class="absolute inset-x-0 top-16 bottom-10 overflow-hidden">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <Suspense>
              <component :is="Component" />
              <template #fallback>
                <Loading class="absolute inset-0" />
              </template>
            </Suspense>
          </template>
        </RouterView>
      </div>

      <Footer class="bg-background absolute inset-x-0 bottom-0 h-10 border-t px-6" />

      <Finder v-model:open="isFinderOpen" />
    </div>
  </SidebarProvider>
</template>
