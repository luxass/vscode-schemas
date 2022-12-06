import {
  Confirm,
  Input,
  Number,
  Secret,
  Command
} from "https://deno.land/x/cliffy@v0.25.5/mod.ts";
import { generateCommand } from "./commands/generate.ts";
import { listCommand } from "./commands/list.ts";

await new Command()
  .name("vscode-schemas")
  .version("0.1.0")
  .description("A CLI for downloading vscode schemas")
  .globalOption("-r, --release [release:string]", "Release to use")
  .command("download [url]")
  .description("Download a schema from a url")
  .option("-o, --output [output:string]", "Output file")
  .option("-f, --force", "Force download")
  .option("-c, --confirm", "Confirm download")
  .option("-i, --input [input:string]", "Input file")
  .option("-n, --number [number:number]", "Number")
  .option("-s, --secret [secret:string]", "Secret")
  .action(async (options, url) => {
    if (options.confirm) {
      const confirmed = await Confirm.prompt("Are you sure?");
      if (!confirmed) {
        return;
      }
    }

    if (options.input) {
      const input = await Input.prompt("What is your name?");
      console.log(input);
    }

    if (options.number) {
      const number = await Number.prompt("What is your age?");
      console.log(number);
    }

    if (options.secret) {
      const secret = await Secret.prompt("What is your password?");
      console.log(secret);
    }

    console.log(
      `Downloading ${url} to ${options.output} with force: ${options.force}`
    );
  })
  .command("list", listCommand)
  .command("generate", generateCommand)
  .parse(Deno.args);
