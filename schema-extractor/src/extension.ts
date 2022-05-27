import {ExtensionContext, commands, window} from "vscode";
import {getSchemaListLocal, getSchemaList} from "./utils";
import {SchemaList} from "./types";

export function activate(context: ExtensionContext) {
    const {VSCODE_SCHEMAS_AUTO_RUN, VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST} = process.env;

    console.log("Extension 'schema-extractor' is now active!");

    console.log("VSCODE_SCHEMAS_AUTO_RUN:", VSCODE_SCHEMAS_AUTO_RUN);


    context.subscriptions.push(
        commands.registerCommand("schema-extractor.extract-all", async () => {
            let root: SchemaList | undefined;

            if (!VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST) {
                root = await getSchemaList();
            } else {
                root = await getSchemaListLocal(VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST)
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


        })
    );

    context.subscriptions.push(
        commands.registerCommand("schema-extractor.extract-one", async () => {
            const root = await getSchemaList();

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
            console.log(result);
        })
    );

    if (Boolean(VSCODE_SCHEMAS_AUTO_RUN)) {
        commands.executeCommand("schema-extractor.extract-all");
    }

}
