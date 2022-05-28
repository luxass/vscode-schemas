import { ExtensionContext, commands, window, workspace, Uri } from "vscode";
import { getSchemaListLocal, getSchemaList } from "./utils";
import { SchemaList } from "./types";

export function activate(context: ExtensionContext) {
  const {
    VSCODE_SCHEMAS_AUTO_RUN,
    VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST,
    VSCODE_SCHEMA_OUTPUT_PATH,
  } = process.env;

  context.subscriptions.push(
    commands.registerCommand("schema-extractor.extract-all", async () => {
      const workspaces = workspace.workspaceFolders;
      if (!workspaces || !workspaces.length) {
        window.showErrorMessage("No workspace opened.");
        return;
      }

      let baseUri = workspaces[0].uri;

      let root: SchemaList | undefined;

      if (!VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST) {
        root = await getSchemaList();
      } else {
        root = await getSchemaListLocal(VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST);
      }

      if (!root) {
        return;
      }

      if (!root.schemas) {
        window.showErrorMessage("No schemas found in list.");
        return;
      }

      if (!Array.isArray(root.schemas)) {
        window.showErrorMessage("Schemas is a non-array.");
        return;
      }

      let output =
        VSCODE_SCHEMA_OUTPUT_PATH ||
        (await window.showInputBox({
          title: "Output path",
          value: "./",
        }));

      if (!output) {
        output = "./";
      }
      await workspace.fs.createDirectory(Uri.joinPath(baseUri, output));

      await Promise.all(
        root.schemas.map(async (schema) => {
          const text = (
            await workspace.openTextDocument(Uri.parse(schema))
          ).getText();

          const parsedSchema = JSON.parse(text);

          await workspace.fs.writeFile(
            Uri.joinPath(
              baseUri,
              output!,
              schema.replace(/^vscode:\/\/schemas(.*)/, "$1.json")
            ),
            Buffer.from(JSON.stringify(parsedSchema, null, 2), "utf8")
          );
        })
      );
      window.showInformationMessage("Schemas extracted.");
    })
  );

  context.subscriptions.push(
    commands.registerCommand("schema-extractor.extract-one", async () => {
      const workspaces = workspace.workspaceFolders;
      if (!workspaces || !workspaces.length) {
        window.showErrorMessage("No workspace opened.");
        return;
      }

      let baseUri = workspaces[0].uri;

      let root: SchemaList | undefined;

      if (!VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST) {
        root = await getSchemaList();
      } else {
        root = await getSchemaListLocal(VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST);
      }

      if (!root) {
        return;
      }

      if (!root.schemas) {
        window.showErrorMessage("No schemas found in list.");
        return;
      }

      if (!Array.isArray(root.schemas)) {
        window.showErrorMessage("Schemas is a non-array.");
        return;
      }

      const result = await window.showQuickPick(root.schemas, {
        title: "Pick a schema",
      });

      if (!result) {
        window.showErrorMessage("No schema selected.");
        return;
      }

      let output =
        VSCODE_SCHEMA_OUTPUT_PATH ||
        (await window.showInputBox({
          title: "Output path",
          value: "./",
        }));

      if (!output) {
        output = "./";
      }

      await workspace.fs.createDirectory(Uri.joinPath(baseUri, output));

      const text = (
        await workspace.openTextDocument(Uri.parse(result))
      ).getText();

      const schema = JSON.parse(text);

      await workspace.fs.writeFile(
        Uri.joinPath(
          baseUri,
          output,
          result.replace(/^vscode:\/\/schemas(.*)/, "$1.json")
        ),
        Buffer.from(JSON.stringify(schema, null, 2), "utf8")
      );

      window.showInformationMessage("Schema extracted.");
    })
  );

  if (Boolean(VSCODE_SCHEMAS_AUTO_RUN)) {
    commands.executeCommand("schema-extractor.extract-all");
  }
}
