import type { ExtensionContext } from "vscode";
import { Uri, commands, env, window } from "vscode";

export function activate(context: ExtensionContext) {
  context.subscriptions.push(
    commands.registerCommand("schema-extractor-internal.extract-all", async () => {
      const uri = await env.asExternalUri(Uri.parse(`${env.uriScheme}://luxass.vscode-schema-extractor-internal/open?vscode://schemas/icons`));
      window.showInformationMessage(uri.toString());
      window.showInformationMessage("Not implemented yet");
    }),
  );

  context.subscriptions.push(
    commands.registerCommand("schema-extractor-internal.extract-one", async () => {
      window.showInformationMessage("Not implemented yet");
    }),
  );

  context.subscriptions.push(window.registerUriHandler({
    handleUri: async (uri) => {
      console.info(uri);
    },
  }));
}
