import { Command } from "../deps.ts";
import { downloadCodeSource } from "../internal/download.ts";
import { CommandGlobalOptions } from "../utils.ts";

export const downloadCommand = new Command<CommandGlobalOptions>()
  .description("Download VSCode Source Code")
  .action(async ({ release, out, codeSrc }) => {
    await downloadCodeSource(release, { out, codeSrc });
  });
