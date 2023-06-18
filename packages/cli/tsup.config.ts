import {
  defineConfig
} from "tsup";

export default defineConfig({
  format: [
    "esm"
  ],
  entry: [
    "src/cli.ts"
  ],
  treeshake: true
});
