import { Command, colors, join, Confirm, Input, which } from "../deps.ts";
import { scanFiles, writeSchemasUris } from "../scanner.ts";
import { CommandGlobalOptions } from "../utils.ts";

export const scanCommand = new Command<CommandGlobalOptions>()
  .description("Scan for Schemas")
  .option("-cs, --code-src [codeSrc:string]", "Location of VSCode Source Code")
  .option("-d, --dir [dir:string]", "Directory to place VSCode Source Code")
  .option("-o, --out [out:string]", "Output dir to place uris")
  .action(async ({ codeSrc, release, dir }) => {
    let codeSrcPath = codeSrc as string | undefined;
    if (codeSrcPath) {
      console.log(
        `Using ${colors.green.underline(codeSrcPath)} as VSCode Source Code`
      );

      try {
        const contents = await Deno.readTextFile(
          join(codeSrcPath, "package.json")
        );

        const pkgJSON = JSON.parse(contents);

        if (typeof pkgJSON.version !== "string") {
          console.log(
            colors.red(
              `Invalid package.json - expected a string, recieved type ${typeof pkgJSON.version}`
            )
          );
          return;
        }

        if (release && release !== pkgJSON.version) {
          console.log(
            colors.red(
              `Invalid version - expected ${release}, recieved ${pkgJSON.version}`
            )
          );
          return;
        }
      } catch (error) {
        console.log(error);

        console.log("Invalid VSCode Source Code");
      }
    } else {
      let downloadDir = dir as string | undefined;
      console.log("No VSCode Source Code provided");
      if (!downloadDir) {
        const wantToDownload = await Confirm.prompt("Do you want to download?");

        if (!wantToDownload) {
          console.log(colors.red("Aborting."));
          return;
        }
        downloadDir = await Input.prompt({
          message: "Where do you want to download VSCode Source Code?",
          default: "vscode"
        });
      }

      console.log("Downloading VSCode Source Code");
      const gitBin = await which("git");
      if (!gitBin) {
        console.log(colors.red("Git is not installed"));
        Deno.exit(1);
      }
      const command = new Deno.Command(gitBin, {
        args: [
          "clone",
          "--depth",
          "1",
          "--branch",
          release || "main",
          "https://github.com/microsoft/vscode.git",
          downloadDir
        ]
      });
      const { code, stderr } = await command.output();
      // For some odd reason, git writes success to stderr. -.-
      if (code !== 0 /*|| stderr.length > 0*/) {
        console.error(
          colors.red("Failed to download VSCode - received following error")
        );
        const err = new TextDecoder().decode(stderr);
        console.error(err);
        return;
      }
      codeSrcPath = downloadDir;
    }
    const scannedFiles = await scanFiles(codeSrcPath);

    const schemas = await writeSchemasUris(scannedFiles);
    console.log(
      `Scanned ${colors.yellow.underline(
        scannedFiles.length.toString()
      )} files - found ${colors.yellow.underline(
        schemas.length.toString()
      )} schemas`
    );

    const outDir = await Input.prompt({
      message: "Where do you want to save the schemas?",
      default: `schemas/${release}`
    });

    await Deno.mkdir(outDir, { recursive: true });

    await Deno.writeTextFile(
      join(outDir, ".vscode-schemas.json"),
      JSON.stringify(schemas.sort(), null, 2)
    );
  });
