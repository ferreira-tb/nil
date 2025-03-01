import '@/assets/index.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { initCore } from '@/core';
import { go, router } from '@/router';
import { createI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { type Locale, locales, type LocaleSchema } from './locale';

const app = createApp(App);
app.config.globalProperties.$go = go;
app.config.globalProperties.$c = commands;
app.config.errorHandler = (err) => handleError(err);

Object.defineProperty(window, '__APP__', {
  configurable: false,
  enumerable: true,
  value: app,
  writable: false,
});

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
  .then(() => initCore())
  .then(() => app.mount('#app'))
  .handleError();
