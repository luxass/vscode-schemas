import { defineCommand } from "citty";
import consola from "consola";

export default defineCommand({
  meta: {
    name: "prepare",
    description: "Prepare code prebuilt",
  },
  args: {

  },
  async setup(_ctx) {
    consola.info("Building prebuilt...");
  },
});
