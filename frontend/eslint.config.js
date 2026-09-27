import js from "@eslint/js";
import ts from "@typescript-eslint/eslint-plugin";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import globals from "globals";

const sourceFiles = ["**/*.ts", "**/*.tsx"];

export default [
  {
    ignores: ["dist", ".eslintrc.cjs"],
  },
  {
    files: sourceFiles,
    ...js.configs.recommended,
  },
  ...ts.configs["flat/recommended"].map((config) => ({
    ...config,
    files: sourceFiles,
  })),
  {
    files: sourceFiles,
    ...reactHooks.configs.flat.recommended,
  },
  {
    files: sourceFiles,
    languageOptions: {
      globals: globals.browser,
    },
    plugins: {
      "react-refresh": reactRefresh,
    },
    rules: {
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
    },
  },
];
