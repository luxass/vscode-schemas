import { defineCommand } from "citty";
import consola from "consola";

export default defineCommand({
  meta: {
    name: "build",
    description: "Build code prebuilt",
  },
  args: {

  },
  async setup(_ctx) {
    consola.info("Building prebuilt...");
  },
});
