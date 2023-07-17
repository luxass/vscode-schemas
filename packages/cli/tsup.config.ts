import {
  defineConfig
} from "tsup";


export default defineConfig({
  entry: ["./src/cli.ts"],
  format: ["esm"],
  clean: true,
  treeshake: true,
  outExtension(ctx) {
    return {
      js: ctx.format === "cjs" ? ".cjs" : ".mjs"
    };
  }
});
