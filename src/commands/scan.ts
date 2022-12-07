import { Command, colors, join, Input } from "../deps.ts";
import { scanFiles, writeSchemasUris } from "../scanner.ts";
import { CommandGlobalOptions, downloadCodeSource } from "../utils.ts";

export const scanCommand = new Command<CommandGlobalOptions>()
  .description("Scan for Schemas")
  .option("--default-out", "Use default value in out prompt")
  .action(async ({ codeSrc, release, out, defaultOut }) => {
    if (!codeSrc) {
      codeSrc = await downloadCodeSource(release, out);
    }
    console.log(
      `Using ${colors.green.underline(codeSrc)} as VSCode Source Code`
    );

    const scannedFiles = await scanFiles(codeSrc);

    const schemas = await writeSchemasUris(scannedFiles);
    console.log(
      `Scanned ${colors.yellow.underline(
        scannedFiles.length.toString()
      )} files - found ${colors.yellow.underline(
        schemas.length.toString()
      )} schemas`
    );
    let outDir = defaultOut ? `schemas/${release}` : (out as string | undefined);
    if (!outDir) {
      outDir = await Input.prompt({
        message: "Where do you want to save the schemas uris?",
        default: `schemas/${release}`
      });
    }

    await Deno.mkdir(outDir, { recursive: true });

    await Deno.writeTextFile(
      join(outDir, ".vscode-schemas.json"),
      JSON.stringify(
        {
          version: release,
          schemas: schemas.sort()
        },
        null,
        2
      )
    );
  });
