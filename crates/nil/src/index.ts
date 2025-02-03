import '@/assets/index.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { go, router } from '@/router';
import { createI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { type Locale, locales, type LocaleSchema } from './locale';

const app = createApp(App);

Object.defineProperty(window, '__NIL__', {
  configurable: false,
  enumerable: true,
  writable: false,
  value: Object.freeze({ app }),
});

app.config.globalProperties.$go = go;
app.config.globalProperties.$c = commands;
app.config.errorHandler = (err) => handleError(err);

const i18n = createI18n<[LocaleSchema], Locale>({
  legacy: false,
  locale: 'en-US',
  fallbackLocale: ['en-US', 'pt-BR'],
  messages: locales,
});

app.use(i18n);
app.use(router);

router
  .push({ name: 'home' })
  .then(() => app.mount('#app'))
  .handleError();
