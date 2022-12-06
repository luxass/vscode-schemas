import {
  Command,
  colors,
  Confirm,
  Input
} from "https://deno.land/x/cliffy@v0.25.5/mod.ts";
import { join } from "https://deno.land/std@0.167.0/path/mod.ts";
import { scanFiles, writeSchemasUris } from "../scanner.ts";

export const generateCommand = new Command<{
  release: true | string | undefined;
}>()
  .description("Generate schemas")
  .option("-cs, --code-src [codeSrc:string]", "Location of VSCode Source Code")
  
  .action(async ({ codeSrc, release }) => {
    const codeSrcPath = codeSrc as string | undefined;
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

        const scannedFiles = await scanFiles(codeSrcPath);

        const schemas = await writeSchemasUris(scannedFiles);
        console.log(
          `Scanned ${colors.yellow.underline(
            scannedFiles.length.toString()
          )} files - found ${colors.yellow.underline(
            schemas.length.toString()
          )} schemas`
        );
      } catch (error) {
        console.log(error);

        console.log("Invalid VSCode Source Code");
      }
    } else {
      console.log("No VSCode Source Code provided");

      const wantToDownload = await Confirm.prompt("Do you want to download?");

      if (!wantToDownload) {
        console.log(colors.red("Aborting."));
        return;
      }

      const downloadPath = await Input.prompt(
        {
          message: "Where do you want to download VSCode Source Code?",
          default: "vscode",
        }
      );

      console.log("Downloading VSCode Source Code");
      
    }
  });
