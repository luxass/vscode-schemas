import { readFile, readdir } from "node:fs/promises";
import { dirname, join } from "node:path";
import _URI from "vscode-uri";

const {
  URI,
} = _URI;

const EXTERNAL_SCHEMES: string[] = [
  "http",
  "https",
];

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

const URI_REGEX = /vscode:\/\/schemas\/([^"']+)/g;

export async function scan(
  codeSrc: string,
): Promise<string[]> {
  const release = JSON.parse(await readFile(join(codeSrc, "package.json"), "utf-8")).version;
  const schemas: string[] = [];
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

    const contents = await readFile(entry.path, "utf-8");
    if (ext === "json" || ext === "jsonc") {
      if (!entry.name.includes("package.json")) {
        const match = contents.match(URI_REGEX);
        if (match) {
          console.warn("Skipping a json file, that includes a schema. Please fix.", entry.path);
        }
        continue;
      }
      const pkg = JSON.parse(contents);
      if (pkg.contributes?.jsonValidation) {
        for (const validation of pkg.contributes.jsonValidation) {
          const match = validation.url.match(URI_REGEX);
          if (match) {
            if (match.includes("vscode://schemas/custom")) continue;

            if (!schemas.includes(match[0])) {
              schemas.push(match[0]);
            }
          } else {
            const { scheme, authority } = URI.parse(validation.url);
            if (EXTERNAL_SCHEMES.includes(scheme) && authority === "raw.githubusercontent.com") {
              if (!schemas.includes(validation.url)) {
                schemas.push(validation.url);
              }
            } else if (scheme === "file") {
              const schemaPath = `https://raw.githubusercontent.com/microsoft/vscode/${join(
                release,
                dirname(entry.path.replace(codeSrc, "")),
                validation.url,
              )}`;
              if (!schemas.includes(schemaPath)) {
                schemas.push(schemaPath);
              }
            }
          }
        }
      }
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

          if (!schemas.find((schema) => schema === schemaMatch)) {
            schemas.push(schemaMatch);
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
  name: string;
  isFile: boolean;
  isDirectory: boolean;
  path: string;
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
