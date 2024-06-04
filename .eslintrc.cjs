// ESLint 检查 .vue 文件需要单独配置编辑器：
// https://eslint.vuejs.org/user-guide/#editor-integrations
module.exports = {
  root: true,
  // https://github.com/vuejs/vue-eslint-parser#readme
  // https://eslint.vuejs.org/user-guide/#usage
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
  },
  extends: [
    // lint vue 文件
    // eslint-config-taro配置包中没有引入依赖eslint-plugin-vue
    // 所以需要项目安装eslint-plugin-vue
    // 'taro/vue3',
    'plugin:vue/vue3-recommended',
    // eslint 集成prettier 规则，
    // 需要先安装
    // eslint-config-prettier
    // eslint-plugin-prettier
    // 1. disabled掉eslint中和prettier想冲突的规则
    // 2. 实时以eslint 提示的方式显示prettier规则
    // @see https://github.com/prettier/eslint-plugin-prettier
    'plugin:prettier/recommended',
  ],
  rules: {
    'vue/multi-word-component-names': [
      'error',
      {
        ignores: ['Home', 'Layout'],
      },
    ],
  },
  // 方式一：
  // lint js?(x) ts?(x)文件
  // eslint-config-taro 没有引入相关依赖
  // 需要项目单独安装：
  // "eslint-plugin-import": "^2.26.0",
  // "eslint-plugin-react": "^7.30.0",
  // "eslint-plugin-react-hooks": "^4.5.0",
  // glob functionally: https://github.com/isaacs/node-glob
  //
  // overrides: [{ files: ["**/*.js?(x)", "**/*.ts?(x)"], extends: ["taro"] }],

  // 方式二：
  // 不使用taro配置包，自定义lint ts文件
  overrides: [
    {
      files: ['src/**/*.ts', 'slaves/**/*.ts'],
      parser: '@typescript-eslint/parser',
      plugins: ['@typescript-eslint'],
      parserOptions: {
        tsconfigRootDir: __dirname,
        project: ['./tsconfig.json'],
        extraFileExtensions: ['.vue'],
      },
      extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:@typescript-eslint/recommended-requiring-type-checking',
      ],
      excludedFiles: '*.test.ts',
    },
  ],
};
