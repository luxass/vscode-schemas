import { existsSync } from "node:fs";
import { writeFile } from "node:fs/promises";
import process from "node:process";
import cac from "cac";
import {
  $fetch,
} from "ofetch";
import semver from "semver";
import type { Release } from "vscode-schema-core";
import {
  downloadCodeSource, scan,
} from "vscode-schema-core";
import { bold, green, inverse, red, yellow } from "colorette";
import { version } from "../package.json";

const cli = cac("vscode-schema");

export interface GlobalCLIOptions {
  out?: string
}

cli.command("download [release] [out]", "Download ")
  .option("--out [out]", "Outdir to place the schema files in", {
    default: ".vscode-schemas",
  })
  .action(async (release: string, out: string, options: GlobalCLIOptions) => {
    if (!release) {
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt,
      });
    }

    if (!semver.gte(release, "1.45.0")) {
      // set release to lastest, and notify user
      console.warn("The release you specified is not supported, using latest instead.");
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt,
      });
    }

    // await download(release as Release, {
    //   outDir: out || options.out
    // });

    console.log("Currently not implemented.");
  });

cli.command("download-src [release] [out]", "Download VSCode Source Code")
  .option("--out [out]", "Outdir to place the source code", {
    default: ".vscode-src",
  })
  .option("-f, --force", "Force download source code (will delete files in out)", {
    default: false,
  })
  .action(async (release: string, out: string, options: GlobalCLIOptions & {
    force: boolean
  }) => {
    if (!release) {
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt,
      });
    }

    if (!semver.gte(release, "1.45.0")) {
      // set release to lastest, and notify user
      console.warn("The release you specified is not supported, using latest instead.");
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt,
      });
    }
    try {
      await downloadCodeSource(release as Release, {
        out: out || options.out,
        force: options.force || false,
      });
      console.log(`Downloaded source code to ${green(out || options.out || ".vscode-src")}`);
    } catch (err) {
      if (typeof err === "string") {
        console.error(err);
      }

      if (err instanceof Error && err.message === `outDir "${out || options.out}" is not empty`) {
        console.error(
          `The outDir "${out || options.out}" is not empty, use --force to force download source code.`,
        );
        return;
      }

      throw err;
    }
  });

cli.command("scan [folder]", "Scan source code folder for schemas")
  .option("--out [out]", "Output file to place the result", {
    default: ".vscode-scan-result.json",
  })
  .option("-f, --force", "Forcefully write scan results", {
    default: false,
  })
  .action(async (folder: string, options: GlobalCLIOptions & {
    force: boolean
  }) => {
    if (!folder) {
      folder = ".vscode-src";
    }

    const result = await scan(folder);

    if (!options.out) {
      options.out = ".vscode-scan-result.json";
    }

    if (existsSync(options.out) && !options.force) {
      console.warn(`File ${yellow(options.out || ".vscode-scan-result.json")} already exists, writing to file skipped.`);
      return;
    }

    await writeFile(options.out, JSON.stringify(result, null, 2), "utf8");
    console.log(`Wrote scan result to ${green(options.out)}`);
  });

cli.command("[root]", "Download and start schema generation")
  .option("--out [type]", "Output file to place the result")
  .action(async (folder: string, options: GlobalCLIOptions) => {
    console.log("Currently not implemented.");
  });

cli.help();
cli.version(version);

try {
  cli.parse(process.argv, { run: false });
  await cli.runMatchedCommand();
} catch (err) {
  console.error(`\n${red(bold(inverse(" Unhandled Error ")))}`);
  console.error(err);
  console.error("\n\n");
  process.exit(1);
}
