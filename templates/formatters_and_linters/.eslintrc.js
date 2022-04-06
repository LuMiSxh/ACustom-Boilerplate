// eslint-disable-next-line no-undef
module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/recommended',
    'google',
    'prettier',
    'next',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  plugins: ['react', '@typescript-eslint'],
  rules: {
    'require-jsdoc': 'off',
    'new-cap': 'off',
    'no-console': ['error', { allow: ['warn'] }],
    'semi': ['error', 'always'],
    'no-empty': 'warn',
    'no-cond-assign': ['error', 'always'],
    'prefer-const': 'error',
    'curly': 'error',
    '@typescript-eslint/no-explicit-any': 'off',
  },
};