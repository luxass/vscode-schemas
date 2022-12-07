#!/usr/bin/env -S deno run --allow-read --allow-write --allow-net --unstable --allow-env --allow-run

import { downloadCommand } from "./commands/download.ts";
import { generateCommand } from "./commands/generate.ts";
import { listCommand } from "./commands/list.ts";
import { scanCommand } from "./commands/scan.ts";
import { Command, SemVer, colors, octokit } from "./deps.ts";
import { error } from "./log.ts";

await new Command()
  .name("vscode-schemas")
  .version("0.1.0")
  .description("A CLI for downloading vscode schemas")
  .globalOption("-r, --release [release:string]", "Release to use", {
    default: await (async () => {
      const r = await octokit.request(
        "GET /repos/{owner}/{repo}/releases/latest",
        {
          owner: "microsoft",
          repo: "vscode"
        }
      );
      return r.data.tag_name;
    })(),
    action: ({ release }) => {
      if (typeof release === "boolean") {
        error("Release is a boolean, please provide a correct version");
        Deno.exit(1);
      }

      const semver = new SemVer(release);
      if (semver.compare("1.45.0") === -1) {
        error("Release must be >= 1.45.0");
        Deno.exit(1);
      }

      return release;
    }
  })
  .globalOption(
    "--code-src [codeSrc:string]",
    "Location of VSCode Source Code",
    {
      action: ({ codeSrc }) => {
        if (typeof codeSrc === "boolean") {
          console.error(
            colors.red("Code Source is a boolean, please provide a string")
          );
          Deno.exit(1);
        }
        return codeSrc;
      }
    }
  )
  .globalOption(
    "-o, --out [dir:string]",
    "Directory to place VSCode Source Code"
  )
  .action(function () {
    this.showHelp();
  })
  .command("list", listCommand)
  // .command("generate", generateCommand)
  // .command("scan", scanCommand)
  .command("download", downloadCommand)
  .parse(Deno.args);
