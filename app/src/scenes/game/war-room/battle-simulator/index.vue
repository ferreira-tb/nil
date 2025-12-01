<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useMutex } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import PersonnelTable from './PersonnelTable.vue';
import { BattleResultImpl } from '@/core/model/battle';
import BattleResultTable from './BattleResultTable.vue';
import { ArmyPersonnelImpl } from '@/core/model/military/army-personnel';
import enUS from '@/locale/en-US/scenes/game/war-room/battle-simulator.json';
import ptBR from '@/locale/pt-BR/scenes/game/war-room/battle-simulator.json';
import { Button, Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const { stats } = NIL.world.refs();

const { locked, ...mutex } = useMutex();
const result = ref<Option<BattleResultImpl>>();

const attacker = ref(ArmyPersonnelImpl.createEmpty());
const defender = ref(ArmyPersonnelImpl.createEmpty());
const wallLevel = ref(stats.value?.getBuildingMinLevel('wall'));

async function simulate() {
  try {
    await mutex.acquire();
    result.value = null;
    result.value = await BattleResultImpl.simulate({
      attacker: attacker.value.getSquads(),
      defender: defender.value.getSquads(),
      wall: wallLevel.value,
    });
  }
  catch (err) {
    handleError(err);
  }
  finally {
    mutex.release();
  }
}

function clear() {
  attacker.value = ArmyPersonnelImpl.createEmpty();
  defender.value = ArmyPersonnelImpl.createEmpty();
}
</script>

<template>
  <div class="game-layout">
    <Card class="w-full">
      <CardHeader>
        <CardTitle>
          <span>{{ t('battle-simulator') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-auto px-2 py-0">
        <div class="w-full min-w-max flex flex-col gap-4 xl:flex-row-reverse xl:justify-end">
          <BattleResultTable v-if="result" :result />
          <div class="w-full flex flex-col gap-4 xl:max-w-max">
            <PersonnelTable
              v-model:attacker="attacker"
              v-model:defender="defender"
              v-model:wall="wallLevel"
              :disabled="locked"
            />

            <div class="grid grid-cols-2 items-center justify-start gap-4 max-w-max">
              <Button
                variant="default"
                :disabled="locked || attacker.isEmpty()"
                @click="simulate"
              >
                <span>{{ t('calculate') }}</span>
              </Button>
              <Button
                variant="secondary"
                :disabled="locked"
                @click="clear"
              >
                <span>{{ t('clear') }}</span>
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
