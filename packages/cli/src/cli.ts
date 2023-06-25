import cac from "cac";
import {
  $fetch
} from "ofetch";
import semver from "semver";
import type { Release } from "vscode-schema-core";
import {
  download
} from "vscode-schema-core";

declare const VERSION: string;

const cli = cac("vscode-schema");

export type GlobalCLIOptions = {
};

export type DownloadCLIOptions = GlobalCLIOptions & {
  out?: string
};

cli.command("download [release] [out]", "Download ")
  .option("--out [out]", "Outdir to place the source code", {
    default: "vscode-src"
  })
  .action(async (release: string, out: string, options: DownloadCLIOptions) => {
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

    await download(release as Release, {
      outDir: out || options.out
    });
  });

cli.command("scan [folder]", "Scan folder for source code ")
  .option("--out [type]", "Output file to place the result")
  .action(async (folder: string, out: string, options: GlobalOptions) => {
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

    await download(release as Release, {
      outDir: out || options.out
    });
  });

cli.command("download [release] [out]", "Download ")
  .option("--out [out]", "Outdir to place the source code", {
    default: "vscode-src"
  })
  .action(async (release: string, out: string, options: GlobalOptions) => {
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

    await download(release as Release, {
      outDir: out || options.out
    });
  });

cli.help();
cli.version(VERSION);

cli.parse();
