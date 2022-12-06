import { walk, extname } from "./deps.ts";

enum FileType {
  JSON,
  Script
}

type File = {
  name: string;
  path: string;
  type: FileType;
};

const URI_REGEX = /vscode:\/\/schemas\/([^"']+)/gm;

/**
 * Get a list of files from a directory
 *
 * Files types is based on the extension of the file,
 * we do this because at some point in the future we may want to
 * do something different based on the file type.
 *
 * @param dir Directory to scan
 * @returns Array of files
 */
export async function scanFiles(dir: string): Promise<Array<File>> {
  const paths: Array<File> = [];
  for await (const entry of walk(dir)) {
    if (entry.isFile) {
      const ext = extname(entry.path);
      switch (ext) {
        case ".jsonc":
        case ".json":
          paths.push({
            name: entry.name,
            path: entry.path,
            type: FileType.JSON
          });
          break;
        case ".js":
        case ".mjs":
        case ".cjs":
        case ".ts":
          paths.push({
            name: entry.name,
            path: entry.path,
            type: FileType.Script
          });
          break;
      }
    }
  }
  return paths;
}

export async function writeSchemasUris(
  files: Array<File>
): Promise<Array<string>> {
  const schemas: Array<string> = [];
  for await (const file of files) {
    const contents = await Deno.readTextFile(file.path);
    const matches = contents.match(URI_REGEX);

    if (matches) {
      for (const schema of matches) {
        if (schema.includes("vscode://schemas/custom")) continue;

        if (!schemas.includes(schema)) {
          schemas.push(schema);
        }
      }
    }
  }
  return schemas;
}
