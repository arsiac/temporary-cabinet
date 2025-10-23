// eslint.config.js
import js from '@eslint/js';
import globals from 'globals';
import vuePlugin from 'eslint-plugin-vue';
import prettierRecommended from 'eslint-plugin-prettier/recommended';

export default [
  {
    ignores: ['dist/**', 'node_modules/**', '*.lock'],
  },
  {
    files: ['**/*.{js,mjs,ts,vue}'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: { ...globals.browser, ...globals.node },
      parserOptions: { ecmaFeatures: { jsx: true } },
    },
    rules: {
      ...js.configs.recommended.rules,
      'no-console': 'warn',
    },
  },
  ...vuePlugin.configs['flat/essential'],
  {
    files: ['**/*.vue'],
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },
  prettierRecommended,
];
