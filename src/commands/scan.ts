import { Command, colors, join, Input } from "../deps.ts";
import { downloadCodeSource } from "../internal/download.ts";
import { info, success } from "../log.ts";
import { scanFiles, writeSchemasUris } from "../internal/scan.ts";
import { CommandGlobalOptions } from "../utils.ts";

export const scanCommand = new Command<CommandGlobalOptions>()
  .description("Scan for Schemas")
  .option("--default-out", "Use default value in out prompt")
  .action(async ({ codeSrc, release, out, defaultOut }) => {
    codeSrc = await downloadCodeSource(release, { out, codeSrc });
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
    let outDir = defaultOut
      ? `schemas/${release}`
      : (out as string | undefined);
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
  });
