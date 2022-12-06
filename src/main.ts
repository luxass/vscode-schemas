import { Command } from "https://deno.land/x/cliffy@v0.25.5/mod.ts";
import { generateCommand } from "./commands/generate.ts";
import { listCommand } from "./commands/list.ts";
import { scanCommand } from "./commands/scan.ts";

await new Command()
  .name("vscode-schemas")
  .version("0.1.0")
  .description("A CLI for downloading vscode schemas")
  .globalOption("-r, --release [release:string]", "Release to use")
  .command("list", listCommand)
  .command("generate", generateCommand)
  .command("scan", scanCommand)
  .parse(Deno.args);
