import axios from "axios";
import { window } from "vscode";
import { SCHEMA_LIST_URL } from "./constants";

export async function getSchemas() {
  try {
    const { data } = await axios.get(SCHEMA_LIST_URL);
    return data;
  } catch (e) {
    console.error(e);
    window.showErrorMessage("Something went wrong.");
  }
}

export const IS_NON_SCALAR = /[\uD800-\uDFFF]/u.test;