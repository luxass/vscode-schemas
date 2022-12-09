import { walk, extname, URI, join, dirname, colors, Input } from "../deps.ts";
import { info, success } from "../log.ts";

export async function scan(
  codeSrc: string,
  release: string,
  out?: string,
  defaultOut?: boolean
) {
  info(`Using ${colors.green.underline(codeSrc)} as VSCode Source Code`);

  const scannedFiles = await scanFiles(codeSrc);

  const { schemas, externalSchemas } = await writeSchemasUris(
    scannedFiles,
    release,
    codeSrc
  );
  info(
    `Scanned ${colors.yellow.underline(
      scannedFiles.length.toString()
    )} files - found ${colors.yellow.underline(
      schemas.length.toString()
    )} schemas and ${colors.yellow.underline(
      externalSchemas.length.toString()
    )} external schemas`
  );
  let outDir = defaultOut ? `schemas/${release}` : (out as string | undefined);
  if (!outDir) {
    outDir = await Input.prompt({
      message: "Where do you want to save the schemas uris?",
      default: `schemas/${release}`
    });
  }

  await Deno.mkdir(outDir, { recursive: true });
  const outputFile = join(outDir, ".vscode-schemas.json");
  await Deno.writeTextFile(
    outputFile,
    JSON.stringify(
      {
        version: release,
        schemas: schemas.sort(),
        externalSchemas: externalSchemas.sort()
      },
      null,
      2
    )
  );
  success(`Saved schemas uris to ${outputFile}`);
}

enum FileType {
  JSON,
  Script
}

type File = {
  name: string;
  path: string;
  type: FileType;
};

type PackageJson = {
  contributes?: Contributes;
};

type Contributes = {
  jsonValidation?: Array<JsonValidation>;
};

type JsonValidation = {
  fileMatch: Array<string> | string;
  url: string;
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
  files: Array<File>,
  release: string,
  codeSrc: string
): Promise<{
  schemas: Array<string>;
  externalSchemas: Array<string>;
}> {
  const schemas: Array<string> = [];
  const externalSchemas: Array<string> = [];
  for await (const file of files) {
    const contents = await Deno.readTextFile(file.path);
    if (file.type === FileType.JSON && file.name === "package.json") {
      const pkg: PackageJson = JSON.parse(contents);
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
            if (
              ["http", "https"].includes(scheme) &&
              authority === "raw.githubusercontent.com"
            ) {
              if (!externalSchemas.includes(validation.url)) {
                externalSchemas.push(validation.url);
              }
            } else if (scheme === "file") {
              // Normalizing the path, would remove // at https:
              const schemaPath = "https://raw.githubusercontent.com/microsoft/vscode/" + join(
                release,
                dirname(file.path.replace(codeSrc, "")),
                validation.url
              );

              if (!externalSchemas.includes(schemaPath)) {
                externalSchemas.push(schemaPath);
              }
            }
          }
        }
      }
    } else {
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
  }
  return {
    schemas,
    externalSchemas
  };
}
