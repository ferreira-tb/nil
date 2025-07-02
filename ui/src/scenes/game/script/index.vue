<!-- Copyright (C) Tsukilabs contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Action from './Action.vue';
import Editor from './Editor.vue';
import Sidebar from './Sidebar.vue';
import { compare } from '@/lib/intl';
import { until } from '@vueuse/core';
import * as commands from '@/commands';
import type { Option } from '@tb-dev/utils';
import type { CodeEditor } from '@/lib/editor';
import { onBeforeMount, ref, shallowRef, watch } from 'vue';
import { Card, InputText, Separator } from '@tb-dev/vue-components';

const editor = shallowRef<Option<CodeEditor>>();

const scripts = shallowRef<Script[]>([]);
const current = ref<Option<Script>>();
const loading = ref(false);

watch(scripts, (_scripts) => {
  if (!current.value && _scripts.length > 0) {
    const script = _scripts.at(0);
    if (script) setCurrent(script);
  }
});

watch(current, (script) => {
  editor.value?.setValue(script?.code ?? '');
});

onBeforeMount(loadScripts);

async function loadScripts() {
  const _scripts = await commands.getScripts();
  _scripts.sort((a, b) => compare(a.name, b.name));
  scripts.value = _scripts;
}

function setCurrent(script: Script) {
  if (editor.value && current.value) {
    current.value.code = editor.value.getValue();
  }

  current.value = script;
}

function onBeforeSave(id: ScriptId) {
  if (editor.value && current.value && current.value.id === id) {
    current.value.code = editor.value.getValue();
  }
}

async function onCreate(id: ScriptId) {
  const script = await commands.getScript(id);
  if (script) {
    setCurrent(script);
    scripts.value.push(script);
    scripts.value = scripts.value.toSorted((a, b) => compare(a.name, b.name));
  }
}

async function onSave(id: ScriptId) {
  const script = await commands.getScript(id);
  if (script) {
    if (current.value?.id === script.id) {
      setCurrent(script);
    }

    const index = scripts.value.findIndex((it) => {
      return it.id === script.id;
    });

    if (index !== -1) {
      scripts.value = scripts.value.toSpliced(index, 1, script);
    }
  }
}

function onRemove(id: ScriptId) {
  if (current.value?.id === id) {
    current.value = null;
  }

  scripts.value = scripts.value.filter((script) => {
    return script.id !== id;
  });
}

async function waitToLoad() {
  await until(loading).not.toBeTruthy();
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full p-0" content-class="size-full">
      <div class="flex size-full items-center justify-between">
        <Sidebar
          v-model:loading="loading"
          :scripts
          :wait-to-load
          @create="onCreate"
          @script-click="setCurrent"
        />

        <Separator orientation="vertical" />

        <div class="flex size-full flex-col pl-4">
          <div class="flex items-center justify-between gap-12 py-4 pr-4">
            <div class="flex w-full items-center lg:max-w-96">
              <InputText v-if="current" v-model="current.name" />
            </div>
            <Action
              v-model:loading="loading"
              :current
              :wait-to-load
              class="hidden lg:grid"
              @before-save="onBeforeSave"
              @import="loadScripts"
              @remove="onRemove"
              @save="onSave"
            />
          </div>
          <Editor v-show="current" v-model="editor" />
        </div>
      </div>
    </Card>
  </div>
</template>
