<script setup lang="ts">
import { computed, ref } from 'vue';
import { useLocale } from '@/locale';
import * as dialog from '@/lib/dialog';
import * as commands from '@/commands';
import type { WorldConfig } from '@/types/world';
import type { PlayerConfig } from '@/types/player';
import { isPlayerConfig, isWorldConfig } from '@/lib/schema';
import { Button, Card, InputNumber, InputText, Label } from '@/components';

const { t } = useLocale();

const worldConfig = ref<WorldConfig>({
  size: 100,
});

const playerConfig = ref<PlayerConfig>({
  name: '',
});

const canHost = computed(() => {
  return isWorldConfig(worldConfig.value) && isPlayerConfig(playerConfig.value);
});

async function host() {
  try {
    await commands.startServer(worldConfig.value);
    await commands.startClient('127.0.0.1');
    dialog.info(await commands.spawnPlayer(playerConfig.value));
  } catch (err) {
    dialog.error(String(err));
  }
}
</script>

<template>
  <div class="bg-muted/40 flex size-full flex-col items-center justify-center gap-2">
    <Card class="p-2 sm:min-w-72">
      <template #title>
        <h1 class="text-xl">{{ t('misc.host-game') }}</h1>
      </template>

      <div class="flex flex-col gap-6">
        <div class="flex flex-col gap-4">
          <Label>
            <span>{{ t('misc.name') }}</span>
            <InputText v-model="playerConfig.name" />
          </Label>
          <Label>
            <span>{{ t('misc.world-size') }}</span>
            <InputNumber v-model="worldConfig.size" :min="10" :max="255" />
          </Label>
        </div>

        <div class="flex items-center justify-center gap-2">
          <Button :disabled="!canHost" @click="() => host()">{{ t('misc.host') }}</Button>
          <Button variant="secondary" @click="() => $go('home')">{{ t('misc.cancel') }}</Button>
        </div>
      </div>
    </Card>
  </div>
</template>
