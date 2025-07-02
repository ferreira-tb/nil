import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['ui/tsconfig.json'],
  features: {
    perfectionist: true,
    unicorn: true,
    vue: true,
  },
  overrides: {
    javascript: {
      'no-undefined': 'off',
    },
    perfectionist: {
      'perfectionist/sort-object-types': 'off',
      '@typescript-eslint/sort-type-constituents': 'off',
      'perfectionist/sort-union-types': [
        'error',
        {
          order: 'asc',
          partitionByNewLine: true,
          type: 'natural',
        },
      ],
    },
    typescript: {
      '@typescript-eslint/consistent-type-definitions': ['error', 'type'],
      '@typescript-eslint/naming-convention': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
    },
  },
});
