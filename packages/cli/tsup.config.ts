import {
  defineConfig
} from "tsup";
import {
  version
} from "./package.json";

export default defineConfig({
  format: [
    "esm"
  ],
  entry: [
    "src/cli.ts"
  ],
  outExtension(ctx) {
    return {
      js: ctx.format === "cjs" ? ".cjs" : ".mjs"
    };
  },
  treeshake: true,
  clean: true,
  define: {
    VERSION: JSON.stringify(version)
  }
});
