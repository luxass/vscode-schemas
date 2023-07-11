import { Command, colors, which } from "../deps.ts";
import { downloadCodeSource } from "../internal/download.ts";
import { error, info, success } from "../log.ts";
import type { CommandGlobalOptions } from "../utils.ts";
import { checkVersion } from "../utils.ts";

export const generateCommand = new Command<CommandGlobalOptions>()
  .description("Generate schemas")
  .option("--no-install", "Don't install dependencies")
  .option("--no-build", "Don't build")
  .action(async ({ codeSrc, release, out, install, build }) => {
    codeSrc = await downloadCodeSource(release, { out, codeSrc });

    info(`Using ${colors.green.underline(codeSrc)} as VSCode Source Code`);

    const nodeBin = await which("node");
    if (!nodeBin) {
      error("Node is not installed");
      return;
    }

    const yarnBin = await which("yarn");
    if (!yarnBin) {
      error("Yarn is not installed");
      return;
    }
    const nodeVersion = await checkVersion(nodeBin, "node.js", ">=16.14.x <17");
    const yarnVersion = await checkVersion(yarnBin, "yarn", ">=1.10.1 <2");

    if (!nodeVersion || !yarnVersion) {
      Deno.exit(1);
    }

    if (install) {
      info("Installing dependencies");

      const installCommand = new Deno.Command(yarnBin, {
        args: ["install"],
        cwd: codeSrc
      });

      const { code } = await installCommand.output();
      if (code !== 0) {
        error("Failed to install dependencies");
        Deno.exit(1);
      }
      success("Installed dependencies");
    }

    if (build) {
      info("Building code");
      const buildCommand = new Deno.Command(yarnBin, {
        args: ["compile"],
        cwd: codeSrc
      });

      const { code, stderr } = await buildCommand.output();
      if (code !== 0) {
        error("Failed to build code - received following error");

        const err = new TextDecoder().decode(stderr);
        console.error(err);
        Deno.exit(1);
      }
      success("Built Code");
    }

    info("Running Code");

    // TODO: Patch the code to generate the schemas
    const runCommand = new Deno.Command("./scripts/code.sh", {
      args: [],
      cwd: codeSrc
    });

    const { pid, stdout, stderr } = runCommand.spawn();

    // To ensure that it is started and generated the schemas, we wait for 60 seconds
    await new Promise((r) => setTimeout(r, 60000));


    stdout.pipeTo(Deno.openSync("./output.txt", {
      create: true,
      write: true,
      read: true
    }).writable);
    stderr.pipeTo(Deno.openSync("./errput.txt", {
      create: true,
      write: true,
      read: true
    }).writable);
    Deno.kill(pid, "SIGTERM");

  });
