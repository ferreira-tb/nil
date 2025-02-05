import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['crates/nil/tsconfig.json'],
  features: {
    perfectionist: true,
    react: false,
    svelte: false,
    tailwind: true,
    unicorn: true,
    vue: true,
  },
  ignores: ['**/components/bbase/*'],
  overrides: {
    perfectionist: {
      '@typescript-eslint/sort-type-constituents': 'off',
      'perfectionist/sort-union-types': [
        'error',
        {
          type: 'natural',
          order: 'asc',
          partitionByNewLine: true,
        },
      ],
    },
    vue: {
      'vue/v-on-handler-style': 'off',
    },
  },
});
