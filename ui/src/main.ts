// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import 'vue-sonner/style.css';
import '@/assets/style.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { go, router } from '@/router';
import { createI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { initEntities } from '@/core/entity';
import { registerGlobalComponents } from '@/components';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';
import { type Locale, locales, type LocaleSchema } from '@/locale';

const app = createApp(App);
app.config.globalProperties.$go = go;
app.config.globalProperties.$c = commands;

setCurrentApp(app);
setErrorHandler(handleError, app);
registerGlobalComponents(app);

const i18n = createI18n<[LocaleSchema], Locale>({
  fallbackLocale: ['en-US', 'pt-BR'],
  legacy: false,
  locale: 'en-US',
  messages: locales,
});

app.use(i18n);
app.use(router);

router
  .push({ name: 'home' })
  .then(() => initEntities())
  .then(() => app.mount('#app'))
  .err();
