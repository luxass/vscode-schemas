import { defineCommand } from "citty";
import consola from "consola";
import { patches } from "@vscode-schemas/core/patches";

export default defineCommand({
  meta: {
    name: "list-patches",
    description: "List available patches",
  },
  async setup(_ctx) {
    consola.info("Available patches:");
    for (const patch of Object.keys(patches)) {
      consola.info(`- ${patch}`);
    }
  },
});
