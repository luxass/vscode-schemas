import { patches } from "@vscode-schemas/core/patches";
import { defineCommand } from "citty";
import consola from "consola";

export default defineCommand({
  meta: {
    name: "patch",
    description: "Apply patch(es) to vscode source code",
  },
  args: {
    patches: {
      type: "positional",
      description: "Patch(es) to apply",
      required: false,
    },
  },
  async setup(ctx) {
    let patchesToApply: string[] = ctx.args._;
    if (patchesToApply.length === 0) {
      consola.info("no patches specified, listing available patches...");
      patchesToApply = await consola.prompt("enter patches you want to apply:", {
        type: "multiselect",
        options: Object.keys(patches),
        required: false,
      });

      if (patchesToApply.length === 0) {
        consola.info("no patches selected, exiting...");
        // eslint-disable-next-line node/prefer-global/process
        process.exit(0);
      }
    }

    consola.info("Applying patches...", patchesToApply);
  },
});
