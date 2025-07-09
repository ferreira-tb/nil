<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { renameVillage } from '@/commands';
import { Button, InputText, Label } from '@tb-dev/vue-components';

const { t } = useI18n();

const { coord, village } = NIL.village.refs();

const villageName = ref(village.value?.name);

function rename() {
  if (coord.value && villageName.value) {
    renameVillage(coord.value, villageName.value).err();
  }
}
</script>

<template>
  <div class="size-full px-4">
    <div class="">
      <Label class="max-w-80">
        <span class="text-muted-foreground">{{ t('rename-village') }}</span>
        <div class="flex items-center gap-2">
          <InputText v-model="villageName" min="1" max="50" spellcheck="false" />
          <Button size="sm" :disabled="!village" @click="rename">
            <span>{{ t('rename') }}</span>
          </Button>
        </div>
      </Label>
    </div>
  </div>
</template>
