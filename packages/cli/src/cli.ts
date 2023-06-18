import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { version } from "../package.json";



void yargs(hideBin(process.argv))
  .scriptName("vscode-schema")
  .usage("$0 [args]")
  .command(
    "download",
    "Download schemas/src from vscode",
    (args) => {
      console.log("ARGS 1", args);

    },
    async args => {
      console.log("ARGS 2", args);
    }
  )
  .showHelpOnFail(false)
  .alias("h", "help")
  .version("version", version)
  .alias("v", "version")
  .help()
  .argv;
