// @see https://www.npmjs.com/package/stylelint-stylus
// @see https://stylelint.io/user-guide/configure/
module.exports = {
  extends: [
    'stylelint-config-standard',
    'stylelint-config-prettier',
    // 'stylelint-config-standard-scss',
    // 'stylelint-config-prettier-scss',
    // @see https://www.npmjs.com/package/stylelint-stylus
    // 'stylelint-stylus/standard',
    // @see https://www.npmjs.com/package/postcss-html
    'stylelint-config-html',
    // If you are using Vue.
    'stylelint-config-recommended-vue',
    'stylelint-config-idiomatic-order',
  ],
  // plugins: [
  //   // add this plugin here:
  //   "stylelint-stylus",
  // ],
  // overrides: [
  //   {
  //     files: ["*.stylus", "*.styl", "**/*.stylus", "**/*.styl"],
  //     customSyntax: "postcss-styl",
  //   },
  // ],
  rules: {
    // override/add rules settings here, such as:
    // "stylus/declaration-colon": "never"
    // @see https://stylelint.io/user-guide/rules/list/selector-pseudo-class-no-unknown/#ignorepseudoclasses-regex-string
    'selector-pseudo-class-no-unknown': [true, { ignorePseudoClasses: ['global', 'local'] }],
    'value-keyword-case': ['lower', { camelCaseSvgKeywords: true, ignoreFunctions: ['v-bind'] }],
    'property-no-vendor-prefix': [true, { ignoreProperties: ['text-size-adjust'] }],
  },
};
