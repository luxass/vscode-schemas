import { ExtensionContext, commands, window } from "vscode";
import { getSchemas } from "./utils";
import TOML from "@ltd/j-toml";

export function activate(context: ExtensionContext) {
  context.subscriptions.push(
    commands.registerCommand("schema-extractor.extract-all", () => {})
  );

  context.subscriptions.push(
    commands.registerCommand("schema-extractor.extract-one", async () => {
      const schemas = await getSchemas();
      const root = TOML.parse(schemas);

      if (!root.schemas) {
        window.showErrorMessage("No schemas found in list.");
        return;
      }

      if (!Array.isArray(root.schemas)) {
        window.showErrorMessage("Schemas is a non-array.");
        return;
      }

      const result = await window.showQuickPick(root.schemas as string[], {
        title: "Pick a schema",
      });
      console.log(result);
    })
  );
}
