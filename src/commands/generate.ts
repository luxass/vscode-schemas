import { Command, colors, which } from "../deps.ts";
import {
  checkVersion,
  CommandGlobalOptions,
  downloadCodeSource
} from "../utils.ts";

export const generateCommand = new Command<CommandGlobalOptions>()
  .description("Generate schemas")
  .option("--no-install", "Don't install dependencies")
  .option("--no-build", "Don't build")
  .action(async ({ codeSrc, release, out, install, build }) => {
    if (!codeSrc) {
      codeSrc = await downloadCodeSource(release, out);
    }
    console.log(
      `Using ${colors.green.underline(codeSrc)} as VSCode Source Code`
    );

    const nodeBin = await which("node");
    if (!nodeBin) {
      console.log(colors.red("Node is not installed"));
      return;
    }

    const yarnBin = await which("yarn");
    if (!yarnBin) {
      console.log(colors.red("Yarn is not installed"));
      return;
    }
    const nodeVersion = await checkVersion(nodeBin, "node.js", ">=16.14.x <17");
    const yarnVersion = await checkVersion(yarnBin, "yarn", ">=1.10.1 <2");

    if (!nodeVersion || !yarnVersion) {
      Deno.exit(1);
    }

    if (install) {
      console.log("Installing dependencies");

      const installCommand = new Deno.Command(yarnBin, {
        args: ["install"],
        cwd: codeSrc
      });

      const { code } = await installCommand.output();
      if (code !== 0) {
        console.error(colors.red("Failed to install dependencies"));
        Deno.exit(1);
      }
    }

    if (build) {
      console.log("Building code");

      const buildCommand = new Deno.Command(yarnBin, {
        args: ["compile"],
        cwd: codeSrc
      });

      const { code, stderr } = await buildCommand.output();
      if (code !== 0) {
        console.error(
          colors.red("Failed to build code - received following error")
        );
        const err = new TextDecoder().decode(stderr);
        console.error(err);
        Deno.exit(1);
      }
    }

    console.log("Running Code");
  });
