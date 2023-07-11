import cac from "cac";
import {
  $fetch
} from "ofetch";
import semver from "semver";
import type { Release } from "vscode-schema-core";
import {
  downloadCodeSource, scan
} from "vscode-schema-core";
import pc from "picocolors";
import { version } from "../package.json";

const cli = cac("vscode-schema");

export type GlobalCLIOptions = {
  out?: string
};



cli.command("download [release] [out]", "Download ")
  .option("--out [out]", "Outdir to place the schema files in", {
    default: ".vscode-schemas"
  })
  .action(async (release: string, out: string, options: GlobalCLIOptions) => {
    if (!release) {
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt
      });
    }

    if (!semver.gte(release, "1.45.0")) {
      // set release to lastest, and notify user
      console.warn("The release you specified is not supported, using latest instead.");
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt
      });
    }

    // await download(release as Release, {
    //   outDir: out || options.out
    // });
  });

cli.command("download-src [release] [out]", "Download VSCode Source Code")
  .option("--out [out]", "Outdir to place the source code", {
    default: ".vscode-src"
  })
  .option("-f, --force", "Force download source code (will delete files in out)", {
    default: false
  })
  .action(async (release: string, out: string, options: GlobalCLIOptions & {
    force: boolean
  }) => {
    if (!release) {
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt
      });
    }

    if (!semver.gte(release, "1.45.0")) {
      // set release to lastest, and notify user
      console.warn("The release you specified is not supported, using latest instead.");
      release = await $fetch("https://latest-vscode-release.luxass.dev", {
        parseResponse: txt => txt
      });
    }

    await downloadCodeSource(release as Release, {
      out: out || options.out,
      force: options.force || false
    });

    console.log(`Downloaded source code to ${pc.green(out || options.out)}`);
  });


cli.command("scan [folder]", "Scan source code folder for schemas")
  .option("--out [type]", "Output file to place the result")
  .action(async (folder: string, options: GlobalCLIOptions) => {
    if (!folder) {
      folder = ".vscode-src";
    }

    await scan(folder, {
      out: options.out
    });
  });

cli.command("[root]", "Download and start schema generation")
  .option("--out [type]", "Output file to place the result")
  .action(async (folder: string, options: GlobalCLIOptions) => {

  });

cli.help();
cli.version(version);

cli.parse();
