// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as monaco from 'monaco-editor-core';
import { shikiToMonaco } from '@shikijs/monaco';
import { createHighlighterCore } from 'shiki/core';
import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import type { editor as MonacoEditor } from 'monaco-editor-core';
import EditorWorker from 'monaco-editor-core/esm/vs/editor/editor.worker?worker';

export type CodeEditor = MonacoEditor.IStandaloneCodeEditor;

(self as any).MonacoEnvironment = {
  getWorker: () => new EditorWorker(),
};

let ready = false;

export async function createEditor(element: HTMLElement) {
  if (!ready) {
    const highlighter = await createHighlighterCore({
      engine: createOnigurumaEngine(import('shiki/wasm')),
      themes: [import('shiki/themes/vesper.mjs')],
      langs: [import('shiki/langs/lua.mjs')],
    });

    monaco.languages.register({ id: 'lua' });
    shikiToMonaco(highlighter, monaco);
    ready ||= true;
  }

  return monaco.editor.create(element, {
    language: 'lua',
    theme: 'vesper',
    value: '',
    minimap: { enabled: false },
  });
}
