import {
  defineConfig
} from "tsup";
import {
  version
} from "./package.json";

export default defineConfig({
  entry: ["./src/cli.ts"],
  format: ["esm"],
  clean: true,
  treeshake: true,
  outExtension(ctx) {
    return {
      js: ctx.format === "cjs" ? ".cjs" : ".mjs"
    };
  },
  define: {
    VERSION: JSON.stringify(version)
  }
});
