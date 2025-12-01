<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();
</script>

<template>
  <div class="game-layout">
    <Card class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('war-room') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="px-2 py-0"></CardContent>
    </Card>
  </div>
</template>
