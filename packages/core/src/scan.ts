import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import type { Schema } from "./types";

const ALLOWED_FILES_EXTENSIONS: string[] = [
  "ts",
  "mts",
  "cts",
  "js",
  "mjs",
  "cjs",
  "json",
  "jsonc",
];

const URI_REGEX = /vscode:\/\/schemas\/([^"']+)/gm;

export async function scan(
  codeSrc: string,
  type?: "builtin" | "extension" | "all",
): Promise<Schema[]> {
  const schemas: Schema[] = [];
  for await (const entry of walk(codeSrc)) {
    if (entry.isDirectory) continue;
    // if file is inside a test dir or file is a .test.* file, skip it
    if (entry.path.includes("test") || entry.name.includes(".test.")) continue;
    const ext = entry.name.split(".").pop();
    if (!ext) {
      console.warn(`File "${entry.path}" has no extension.`);
      continue;
    }
    if (!ALLOWED_FILES_EXTENSIONS.includes(ext)) continue;
    // TODO: Try to make it faster by using a ripgrep binary, will be published at @luxass/klow in the near future.

    const contents = await readFile(entry.path, "utf-8");
    if (ext === "json" || ext === "jsonc") {
      continue;
    }

    if (ext === "js" || ext === "mjs" || ext === "cjs" || ext === "ts") {
      const matches = contents.match(URI_REGEX);

      if (matches) {
        for (const schemaMatch of matches) {
          if (schemaMatch.includes("vscode://schemas/custom")) {
            console.warn(`Skipping custom schema "${schemaMatch}" in file "${entry.path}"`);
            continue;
          }

          if (!schemas.find((schema) => schema.name === schemaMatch)) {
            schemas.push({
              kind: "extension",
              name: schemaMatch,
            });
          }
        }
      }
      continue;
    }

    throw new Error(`File type "${ext}" is not currently implemented.`);
  }
  return schemas;
}

interface Entry {
  name: string
  isFile: boolean
  isDirectory: boolean
  path: string
}

export async function* walk(dir: string): AsyncIterableIterator<Entry> {
  try {
    const dirs = await readdir(dir, { withFileTypes: true });
    for (const entry of dirs) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) {
        yield * walk(path);
      } else {
        yield {
          name: entry.name,
          isFile: entry.isFile(),
          isDirectory: entry.isDirectory(),
          path,
        };
      }
    }
  } catch (err) {
    console.error(err);
  }
}
