import { Command } from "../deps.ts";
import { downloadCodeSource } from "../internal/download.ts";
import { scan } from "../internal/scan.ts";
import { CommandGlobalOptions } from "../utils.ts";

export const scanCommand = new Command<CommandGlobalOptions>()
  .description("Scan for Schemas")
  .option("--default-out", "Use default value in out prompt")
  .action(async ({ codeSrc, release, out, defaultOut }) => {
    codeSrc = await downloadCodeSource(release, { out, codeSrc });
    await scan(codeSrc, release, out, defaultOut);
  });
