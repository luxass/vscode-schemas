import axios from "axios";
import {window} from "vscode";
import {SCHEMA_LIST_URL} from "./constants";
import {SchemaList} from "./types";
import {readFile} from "fs";
import {promisify} from "util";

const readFileAsync = promisify(readFile);

export async function getSchemaList(): Promise<SchemaList | undefined> {
    try {
        const {data} = await axios.get<SchemaList>(SCHEMA_LIST_URL);
        return data;
    } catch (e) {
        console.error(e);
        window.showErrorMessage(
            "Something went wrong, while trying to fetch schema list."
        );
    }
}

export async function getSchemaListLocal(localSchemaPath: string) {
    try {

        const content = await readFileAsync(localSchemaPath, {encoding: "utf8"});
        return JSON.parse(content);
    } catch (e) {
        console.error(e);
        window.showErrorMessage(
            "Something went wrong, while trying to read and parse schema list."
        );
    }
}
