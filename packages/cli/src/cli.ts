import cac from "cac";
import {
  $fetch
} from "ofetch";
import semver from "semver";
import {
  download
} from "vscode-schema-core";

declare const VERSION: string;

const cli = cac("vscode-schema");

cli.command("download [release]", "Download ")
  .option("--out [out]", "Outdir to place the source code", {
    default: "vscode-src"
  })
  .action(async (release, options: {
    outDir?: string
  }) => {
    console.log(release, options);

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

    console.log(release);

    await download(release, {
      outDir: options.outDir
    });
  });

cli.help();
cli.version(VERSION);

cli.parse();
